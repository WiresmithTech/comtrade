mod error;
pub mod parser;

use chrono::{FixedOffset, NaiveDateTime};
use derive_builder::Builder;

pub use parser::{
    AnalogChannel, AnalogConfig, AnalogScalingMode, ComtradeParser, ComtradeParserBuilder,
    DataFormat, FormatRevision, ParseError, ParseResult, SamplingRate, StatusChannel, StatusConfig,
};

#[derive(Debug, Clone, PartialEq)]
enum FileType {
    Cfg,
    Dat,
    Hdr,
    Inf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeQuality {
    /// Clock in locked and in normal operation.
    ClockLocked,

    /// Clock is unlocked and reliable to a specified precision. Value given is
    /// reliability of time as power of 10. For instances:
    ///
    /// ```rust
    /// use comtrade::TimeQuality;
    ///
    /// // Device clock time is reliable to 1 nanosecond (10^-9).
    /// let q1 = TimeQuality::ClockUnlocked(-9);
    ///
    /// // Device clock time is reliable to 10 microseconds (10^-5).
    /// let q2 = TimeQuality::ClockUnlocked(-5);
    ///
    /// // Device clock time is reliable to 10 seconds (10^1).
    /// let q3 = TimeQuality::ClockUnlocked(1);
    /// ```
    ///
    /// COMTRADE format specification expects values between -9 and 1.
    ClockUnlocked(i32),

    /// There is a fault in the clock and the time it gives is not reliable.
    ClockFailure,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeapSecondStatus {
    /// Time source does not have capability to address presence of leap seconds.
    NoCapability,

    /// A leap second has been subtracted from the record.
    Subtracted,

    /// A leap second has been added to the record.
    Added,

    /// No leap second is present in the record.
    NotPresent,
}

#[derive(Debug, Clone, Builder, PartialEq)]
pub struct Comtrade {
    pub station_name: String,
    pub recording_device_id: String,
    pub revision: FormatRevision,

    pub sample_numbers: Vec<u32>,
    pub timestamps: Vec<f64>,
    pub analog_channels: Vec<AnalogChannel>,
    pub status_channels: Vec<StatusChannel>,

    pub line_frequency: f64,

    pub sampling_rates: Vec<SamplingRate>,
    pub start_time: NaiveDateTime,
    pub trigger_time: NaiveDateTime,

    // Don't think these is necessary either, it's just used to parse / process the data file.
    pub data_format: DataFormat,

    // Below data are 1999 format onwards only.

    // Don't use option for this - just default to 1 if it's not present.
    pub timestamp_multiplication_factor: f64,

    // Below data are 2013 format onwards only.
    pub time_offset: Option<FixedOffset>,
    pub local_offset: Option<FixedOffset>,

    pub time_quality: Option<TimeQuality>,
    pub leap_second_status: Option<LeapSecondStatus>,
}

impl Default for Comtrade {
    fn default() -> Self {
        Comtrade {
            station_name: Default::default(),
            recording_device_id: Default::default(),
            revision: Default::default(),
            sample_numbers: Default::default(),
            timestamps: Default::default(),
            analog_channels: Default::default(),
            status_channels: Default::default(),
            line_frequency: Default::default(),
            sampling_rates: Default::default(),
            start_time: NaiveDateTime::from_timestamp(0, 0),
            trigger_time: NaiveDateTime::from_timestamp(0, 0),
            data_format: Default::default(),
            timestamp_multiplication_factor: 1.0,
            time_offset: Default::default(),
            local_offset: Default::default(),
            time_quality: Default::default(),
            leap_second_status: Default::default(),
        }
    }
}
