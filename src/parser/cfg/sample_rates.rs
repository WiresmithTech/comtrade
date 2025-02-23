use crate::error::ComtradeError;
use crate::parser::cfg::ConfigLine;

#[derive(Debug, Clone, PartialEq)]
pub struct SamplingRate {
    pub rate_hz: f64,
    pub end_sample_number: u32,
}

impl SamplingRate {
    pub fn from_config_line<'a>(
        mut line: impl ConfigLine<'a>,
    ) -> Result<SamplingRate, ComtradeError> {
        let rate_hz = line.read_value()?;
        let end_sample_number = line.read_value()?;
        Ok(SamplingRate {
            rate_hz,
            end_sample_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::cfg::split_cfg_line;

    #[test]
    fn test_from_config_line() {
        let line = "1000, 10000";
        let mut line = split_cfg_line(line);
        let rate = SamplingRate::from_config_line(&mut line).unwrap();
        assert_eq!(rate.rate_hz, 1000.0);
        assert_eq!(rate.end_sample_number, 10000);
    }
}
