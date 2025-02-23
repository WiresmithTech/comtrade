use crate::error::ComtradeError;
use crate::parser::cfg::ConfigLine;
use crate::FormatRevision;

#[derive(Debug, Clone, PartialEq)]
pub struct IdRow {
    pub station_name: String,
    pub recording_device_id: String,
    pub format_revision: FormatRevision,
}

impl IdRow {
    pub fn from_config_line<'a>(
        mut config_line: impl ConfigLine<'a>,
    ) -> Result<Self, ComtradeError> {
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
    use super::*;
    use crate::parser::cfg::split_cfg_line;
    use crate::FormatRevision;

    #[test]
    fn id_row_from_config_line_works() {
        let line = "Station name,rec_dev_id,2013";
        let result = IdRow::from_config_line(split_cfg_line(line)).unwrap();
        assert_eq!(
            result,
            IdRow {
                station_name: "Station name".to_string(),
                recording_device_id: "rec_dev_id".to_string(),
                format_revision: FormatRevision::Revision2013,
            }
        );
    }

    #[test]
    fn missing_version_in_id_row_shows_revision_1991() {
        let line = "Station name,rec_dev_id";
        let result = IdRow::from_config_line(split_cfg_line(line)).unwrap();
        assert_eq!(
            result,
            IdRow {
                station_name: "Station name".to_string(),
                recording_device_id: "rec_dev_id".to_string(),
                format_revision: FormatRevision::Revision1991,
            }
        );
    }
}
