mod cff;
mod cfg;
mod dat;
mod time;

use std::io::BufRead;
use std::str::FromStr;

use byteorder::ReadBytesExt;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    AnalogChannel, AnalogScalingMode, Comtrade, ComtradeBuilder, DataFormat, FileType,
    FormatRevision, LeapSecondStatus, StatusChannel, TimeQuality,
};

const CFG_SEPARATOR: &str = ",";

// 1991 revision uses mm/dd/yyyy format for date whereas 1999 and 2013 use dd/mm/yyyy.
// 1991 revision uses mm/dd/yyyy format for date whereas 1999 and 2013 use dd/mm/yyyy
const CFG_DATETIME_FORMAT_OLD: &str = "%m/%d/%Y,%H:%M:%S%.f";
const CFG_DATETIME_FORMAT: &str = "%d/%m/%Y,%H:%M:%S%.f";

// To preserve structure integrity, a special value is used in the binary16, binary32
// and float32 data formats when a timestamp is missing.
const TIMESTAMP_MISSING: u32 = 0xffffffff;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        ParseError { message }
    }
}

impl FromStr for FileType {
    type Err = ParseError;

    fn from_str(value: &str) -> ParseResult<Self> {
        match value.trim().to_lowercase().as_str() {
            "cfg" => Ok(FileType::Cfg),
            "dat" => Ok(FileType::Dat),
            "hdr" => Ok(FileType::Hdr),
            "inf" => Ok(FileType::Inf),
            _ => Err(ParseError::new(format!("invalid file type: '{}'", value))),
        }
    }
}

impl Default for FormatRevision {
    fn default() -> Self {
        FormatRevision::Revision1991
    }
}

impl FromStr for FormatRevision {
    type Err = ParseError;

    fn from_str(value: &str) -> ParseResult<Self> {
        match value {
            "1991" => Ok(FormatRevision::Revision1991),
            "1999" => Ok(FormatRevision::Revision1999),
            "2013" => Ok(FormatRevision::Revision2013),
            _ => Err(ParseError::new(format!(
                "unrecognised or invalid COMTRADE format revision: '{}'",
                value.to_owned(),
            ))),
        }
    }
}

impl FromStr for DataFormat {
    type Err = ParseError;

    fn from_str(value: &str) -> ParseResult<Self> {
        match value.trim().to_lowercase().as_str() {
            "ascii" => Ok(DataFormat::Ascii),
            "binary" => Ok(DataFormat::Binary16),
            "binary32" => Ok(DataFormat::Binary32),
            "float32" => Ok(DataFormat::Float32),
            _ => Err(ParseError::new(format!(
                "unrecognised or invalid COMTRADE data format: '{}'",
                value.to_owned(),
            ))),
        }
    }
}

impl FromStr for AnalogScalingMode {
    type Err = ParseError;
    fn from_str(value: &str) -> ParseResult<Self> {
        match value.to_lowercase().as_str() {
            "p" => Ok(AnalogScalingMode::Primary),
            "s" => Ok(AnalogScalingMode::Secondary),
            _ => Err(ParseError::new(format!(
                "invalid analog scaling mode: '{}'; must be one of: 's', 'S', 'p', 'P'",
                value,
            ))),
        }
    }
}

impl FromStr for TimeQuality {
    type Err = ParseError;

    fn from_str(value: &str) -> ParseResult<Self> {
        match value.to_lowercase().trim() {
            "f" => Ok(TimeQuality::ClockFailure),
            "b" => Ok(TimeQuality::ClockUnlocked(1)),
            "a" => Ok(TimeQuality::ClockUnlocked(0)),
            "9" => Ok(TimeQuality::ClockUnlocked(-1)),
            "8" => Ok(TimeQuality::ClockUnlocked(-2)),
            "7" => Ok(TimeQuality::ClockUnlocked(-3)),
            "6" => Ok(TimeQuality::ClockUnlocked(-4)),
            "5" => Ok(TimeQuality::ClockUnlocked(-5)),
            "4" => Ok(TimeQuality::ClockUnlocked(-6)),
            "3" => Ok(TimeQuality::ClockUnlocked(-7)),
            "2" => Ok(TimeQuality::ClockUnlocked(-8)),
            "1" => Ok(TimeQuality::ClockUnlocked(-9)),
            "0" => Ok(TimeQuality::ClockLocked),
            _ => Err(ParseError::new(format!(
                "invalid time quality code '{}'",
                value,
            ))),
        }
    }
}

impl FromStr for LeapSecondStatus {
    type Err = ParseError;

    fn from_str(value: &str) -> ParseResult<Self> {
        match value.trim() {
            "3" => Ok(LeapSecondStatus::NoCapability),
            "2" => Ok(LeapSecondStatus::Subtracted),
            "1" => Ok(LeapSecondStatus::Added),
            "0" => Ok(LeapSecondStatus::NotPresent),
            _ => Err(ParseError::new(format!(
                "invalid leap second indicator '{}'",
                value,
            ))),
        }
    }
}

lazy_static! {
    static ref CFF_HEADER_REGEXP: Regex = Regex::new(r#"(?i)---\s*file type:\s*(?P<file_type>[a-z]+)(\s+(?P<data_format>[a-z]+))?\s*(:\s*(?P<data_size>\d+))?\s*---$"#).unwrap();
    static ref DATE_REGEXP: Regex = Regex::new("([0-9]{1,2})/([0-9]{1,2})/([0-9]{2,4})").unwrap();
    static ref TIME_REGEXP: Regex = Regex::new("([0-9]{2}):([0-9]{2}):([0-9]{2})(\\.([0-9]{1,12}))?").unwrap();
}

// Cannot derive builder for this because of complexity of wrapping `T: BufRead` in
// `Option` - I can't figure out how to stop the default implementation from complaining
// that `BufReader<File>` doesn't implement `Copy`.
pub struct ComtradeParserBuilder<T: BufRead> {
    cff_file: Option<T>,
    cfg_file: Option<T>,
    dat_file: Option<T>,
    hdr_file: Option<T>,
    inf_file: Option<T>,
}

impl<T: BufRead> ComtradeParserBuilder<T> {
    pub fn new() -> Self {
        Self {
            cff_file: None,
            cfg_file: None,
            dat_file: None,
            hdr_file: None,
            inf_file: None,
        }
    }

    pub fn cff_file(mut self, file: T) -> Self {
        self.cff_file = Some(file);
        self
    }

    pub fn cfg_file(mut self, file: T) -> Self {
        self.cfg_file = Some(file);
        self
    }

    pub fn dat_file(mut self, file: T) -> Self {
        self.dat_file = Some(file);
        self
    }

    pub fn hdr_file(mut self, file: T) -> Self {
        self.hdr_file = Some(file);
        self
    }

    pub fn inf_file(mut self, file: T) -> Self {
        self.inf_file = Some(file);
        self
    }

    pub fn build(self) -> ComtradeParser<T> {
        ComtradeParser::new(
            self.cff_file,
            self.cfg_file,
            self.dat_file,
            self.hdr_file,
            self.inf_file,
        )
    }
}

pub struct ComtradeParser<T: BufRead> {
    cff_file: Option<T>,
    cfg_file: Option<T>,
    dat_file: Option<T>,
    hdr_file: Option<T>,
    inf_file: Option<T>,

    cfg_contents: String,
    ascii_dat_contents: String,
    binary_dat_contents: Vec<u8>,
    hdr_contents: String,
    inf_contents: String,

    builder: ComtradeBuilder,
    total_num_samples: u32,
    num_analog_channels: u32,
    num_status_channels: u32,
    analog_channels: Vec<AnalogChannel>,
    status_channels: Vec<StatusChannel>,
    is_timestamp_critical: bool,
    ts_base_unit: f64,
    data_format: Option<DataFormat>,
}

impl<T: BufRead> ComtradeParser<T> {
    pub fn new(
        cff_file: Option<T>,
        cfg_file: Option<T>,
        dat_file: Option<T>,
        hdr_file: Option<T>,
        inf_file: Option<T>,
    ) -> Self {
        Self {
            cff_file,
            cfg_file,
            dat_file,
            hdr_file,
            inf_file,

            cfg_contents: String::new(),
            ascii_dat_contents: String::new(),
            binary_dat_contents: vec![],
            hdr_contents: String::new(),
            inf_contents: String::new(),

            builder: ComtradeBuilder::default(),
            total_num_samples: 0,
            num_analog_channels: 0,
            num_status_channels: 0,
            analog_channels: vec![],
            status_channels: vec![],
            is_timestamp_critical: false,
            ts_base_unit: 0.0,
            data_format: None,
        }
    }

    pub fn dat_file(mut self, file: T) -> Self {
        self.dat_file = Some(file);
        self
    }

    pub fn hdr_file(mut self, file: T) -> Self {
        self.hdr_file = Some(file);
        self
    }

    pub fn inf_file(mut self, file: T) -> Self {
        self.inf_file = Some(file);
        self
    }

    pub fn parse(mut self) -> ParseResult<Comtrade> {
        if self.cff_file.is_some() {
            self.load_cff()?;
            self.parse_cfg()?;
            self.parse_dat()?;
        } else {
            if let Some(ref mut cfg_file) = self.cfg_file {
                cfg_file
                    .read_to_string(&mut self.cfg_contents)
                    .map_err(|_| {
                        ParseError::new("unable to read specified .cfg file".to_string())
                    })?;
            } else {
                return Err(ParseError::new(
                    "you must specify either .cff or .cfg file".to_string(),
                ));
            }

            self.parse_cfg()?;

            if let Some(ref mut dat_file) = self.dat_file {
                match self.data_format {
                    Some(DataFormat::Ascii) => {
                        dat_file
                            .read_to_string(&mut self.ascii_dat_contents)
                            .map_err(|_| {
                                ParseError::new("unable to read specified .dat file".into())
                            })?;
                    }
                    None => {
                        return Err(ParseError::new("unknown data format for data file.".into()));
                    }
                    // Other binary format.
                    _ => {
                        dat_file
                            .read_to_end(&mut self.binary_dat_contents)
                            .map_err(|_| {
                                ParseError::new("unable to read specified .dat file".into())
                            })?;
                    }
                }
            } else {
                return Err(ParseError::new(
                    "you must specify either .cff or .dat file".to_string(),
                ));
            }

            self.parse_dat()?;

            if let Some(ref mut hdr_file) = self.hdr_file {
                hdr_file
                    .read_to_string(&mut self.hdr_contents)
                    .map_err(|_| {
                        ParseError::new("unable to read specified .hdr file".to_string())
                    })?;
            }

            if let Some(ref mut inf_file) = self.inf_file {
                inf_file
                    .read_to_string(&mut self.inf_contents)
                    .map_err(|_| {
                        ParseError::new("unable to read specified .inf file".to_string())
                    })?;
            }
        }

        // `.hdr` and `.inf` files don't need parsing - if present they're
        // non-machine-readable text files for reference for humans to look at.

        self.builder.analog_channels(self.analog_channels);
        self.builder.status_channels(self.status_channels);

        Ok(self.builder.build().unwrap())
    }
}
