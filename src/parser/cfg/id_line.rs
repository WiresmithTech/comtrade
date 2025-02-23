use std::str::FromStr;
use crate::error::ComtradeError;
use crate::parser::cfg::ConfigLine;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormatRevision {
    Revision1991,
    Revision1999,
    Revision2013,
}


impl Default for FormatRevision {
    fn default() -> Self {
        FormatRevision::Revision1991
    }
}

impl FromStr for FormatRevision {
    type Err = ComtradeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "1991" => Ok(FormatRevision::Revision1991),
            "1999" => Ok(FormatRevision::Revision1999),
            "2013" => Ok(FormatRevision::Revision2013),
            _ => Err(ComtradeError::BadRevisionFormat(value.to_string())),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct IdRow {
    pub station_name: String,
    pub recording_device_id: String,
    pub format_revision: FormatRevision,
}

impl IdRow {
    pub fn from_config_line<'a>(mut config_line: impl ConfigLine<'a>) -> Result<Self, ComtradeError> {
        let name = config_line.read_value()?;
        let recording_device_id = config_line.read_value()?;
        let format_revision = match config_line.read_value() {
            Ok(revision) => revision,
            Err(ComtradeError::MissingLineElements(_)) => FormatRevision::default(),
            Err(e) => return Err(e),
        };
        Ok(Self {
            station_name: name,
            recording_device_id,
            format_revision,
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::cfg::split_cfg_line;
    use super::*;

    #[test]
    fn format_revision_from_str_works() {
        assert_eq!(
            FormatRevision::from_str("1991").unwrap(),
            FormatRevision::Revision1991
        );
        assert_eq!(
            FormatRevision::from_str("1999").unwrap(),
            FormatRevision::Revision1999
        );
        assert_eq!(FormatRevision::from_str("2013").unwrap(), FormatRevision::Revision2013);
    }

    #[test]
    fn error_in_format_revision_from_str() {
        assert!(FormatRevision::from_str("1990").is_err());
    }

    #[test]
    fn id_row_from_config_line_works() {
        let line = "Station name,rec_dev_id,2013";
        let result = IdRow::from_config_line(split_cfg_line(line)).unwrap();
        assert_eq!(result, IdRow {
            station_name: "Station name".to_string(),
            recording_device_id: "rec_dev_id".to_string(),
            format_revision: FormatRevision::Revision2013,
        });
    }

    #[test]
    fn missing_version_in_id_row_shows_revision_1991() {
        let line = "Station name,rec_dev_id";
        let result = IdRow::from_config_line(split_cfg_line(line)).unwrap();
        assert_eq!(result, IdRow {
            station_name: "Station name".to_string(),
            recording_device_id: "rec_dev_id".to_string(),
            format_revision: FormatRevision::Revision1991,
        });
    }
}

