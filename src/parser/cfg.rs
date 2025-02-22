use crate::parser::time::{parse_time_offset, ts_base_unit};
use crate::parser::{CFG_DATETIME_FORMAT, CFG_DATETIME_FORMAT_OLD, CFG_SEPARATOR};
use crate::{
    AnalogChannel, AnalogScalingMode, ComtradeParser, DataFormat, FormatRevision, LeapSecondStatus,
    ParseError, ParseResult, SamplingRate, StatusChannel, TimeQuality,
};
use chrono::NaiveDateTime;
use std::io::BufRead;
use std::str::FromStr;

impl<T: BufRead> ComtradeParser<T> {
    pub(super) fn parse_cfg(&mut self) -> ParseResult<()> {
        // TODO: There must be a more efficient way of doing this using line iterators,
        //  I just need to figure out how to create my own line iterator in the
        //  `load_cff()` function.
        let mut lines = self.cfg_contents.split('\n');

        let early_end_err = || ParseError::new("unexpected end of .cfg file".to_string());

        let mut line_number = 1;
        let mut line = "";
        let mut line_values: Vec<&str> = vec![];

        line = lines.next().ok_or_else(early_end_err)?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Station name, identification and optionally revision year:
        // 1991:       station_name,rec_dev_id
        // 1999, 2013: station_name,rec_dev_id,rev_year

        // We need this value later to know when to quit.
        self.builder.station_name(line_values[0].to_string());
        self.builder.recording_device_id(line_values[1].to_string());

        let format_revision = match line_values.len() {
            3 => FormatRevision::from_str(line_values[2].trim())?,
            2 => FormatRevision::Revision1991,
            _ => {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )))
            }
        };
        self.builder.revision(format_revision);

        line_number += 1;

        line = lines.next().ok_or_else(early_end_err)?;
        let ChannelSizes {
            total: _,
            analog: num_analog_channels,
            status: num_status_channels,
        } = ChannelSizes::from_line(split_cfg_line(line))?;
        self.num_analog_channels = num_analog_channels;
        self.num_status_channels = num_status_channels;

        line_number += 1;

        let mut analog_channels: Vec<AnalogChannel> =
            Vec::with_capacity(self.num_analog_channels as usize);
        let mut status_channels: Vec<StatusChannel> =
            Vec::with_capacity(self.num_status_channels as usize);

        // Analog channel information:
        // An,ch_id,ph,ccbm,uu,a,b,skew,min,max,primary,secondary,PS
        for i in 0..self.num_analog_channels {
            // todo should early_end_err just be a closure?
            line = lines.next().ok_or_else(early_end_err)?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 13 {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )));
            }

            let analog_index = line_values[0]
                .trim()
                .to_string()
                .parse::<u32>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid integer value for analog channel {} index: {}",
                        i, line_values[0]
                    ))
                })?;

            let name = line_values[1].to_string();
            let phase = line_values[2].to_string(); // Non-critical.
            let circuit_component_being_monitored = line_values[3].to_string(); // Non-critical.
            let units = line_values[4].to_string();

            let multiplier = line_values[5]
                .trim()
                .to_string()
                .parse::<f64>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid real numeric value for analog channel {} multiplier: {}",
                        i, line_values[5]
                    ))
                })?;

            let offset_adder = line_values[6]
                .trim()
                .to_string()
                .parse::<f64>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid real numeric value for analog channel {} offset adder: {}",
                        i, line_values[6]
                    ))
                })?;

            let skew = line_values[7]
                .trim()
                .to_string()
                .parse::<f64>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid real numeric value for analog channel {} skew: {}",
                        i, line_values[7]
                    ))
                })?;

            let min_value = line_values[8]
                .trim()
                .to_string()
                .parse::<f64>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid real numeric value for analog channel {} minimum value: {}",
                        i, line_values[8]
                    ))
                })?;

            let max_value = line_values[9]
                .trim()
                .to_string()
                .parse::<f64>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid real numeric value for analog channel {} maximum value: {}",
                        i, line_values[9]
                    ))
                })?;

            let primary_factor =
                line_values[10]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .map_err(|_| {
                        ParseError::new(format!(
                            "invalid real numeric value for analog channel {} primary factor: {}",
                            i, line_values[10]
                        ))
                    })?;

            let secondary_factor =
                line_values[11]
                    .trim()
                    .to_string()
                    .parse::<f64>()
                    .map_err(|_| {
                        ParseError::new(format!(
                            "invalid real numeric value for analog channel {} secondary factor: {}",
                            i, line_values[11]
                        ))
                    })?;

            let scaling_mode = AnalogScalingMode::from_str(line_values[12].trim())?;

            analog_channels.push(AnalogChannel {
                index: analog_index,
                name,
                phase,
                circuit_component_being_monitored,
                units,
                min_value,
                max_value,
                multiplier,
                offset_adder,
                skew,
                primary_factor,
                secondary_factor,
                scaling_mode,
                data: vec![],
            });

            line_number += 1;
        }
        self.analog_channels = analog_channels;

        // Status (digital) channel information:
        // Dn,ch_id,ph,ccbm,y
        for i in 0..self.num_status_channels {
            line = lines.next().ok_or_else(early_end_err)?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 5 {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )));
            }

            let status_index = line_values[0]
                .trim()
                .to_string()
                .parse::<u32>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid integer value for status channel {} index: {}",
                        i, line_values[0]
                    ))
                })?;

            let name = line_values[1].to_string();
            let phase = line_values[2].to_string(); // Non-critical.
            let circuit_component_being_monitored = line_values[3].to_string(); // Non-critical.

            let normal_status_value =
                line_values[4]
                    .trim()
                    .to_string()
                    .parse::<u8>()
                    .map_err(|_| {
                        ParseError::new(format!(
                            "invalid integer value for status channel {} normal value: {}",
                            i, line_values[4]
                        ))
                    })?;
            if normal_status_value != 0 && normal_status_value != 1 {
                return Err(ParseError::new(format!("invalid normal status value for status channel {}: {}; expected one of : '0', '1'", i, line_values[4])));
            }

            status_channels.push(StatusChannel {
                index: status_index,
                name,
                phase,
                circuit_component_being_monitored,
                normal_status_value,
                data: vec![],
            });

            line_number += 1;
        }
        self.status_channels = status_channels;

        line = lines.next().ok_or_else(early_end_err)?;

        // Line frequency
        // lf
        let line_frequency = line.trim().to_string().parse::<f64>().map_err(|_| {
            ParseError::new(format!(
                "invalid real numeric value for line frequency: '{}'",
                line,
            ))
        })?;
        self.builder.line_frequency(line_frequency);

        line_number += 1;

        line = lines.next().ok_or_else(early_end_err)?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Sampling rate information
        // nrates (x 1)
        // samp,endsamp (x nrates)
        if line_values.len() != 1 {
            return Err(ParseError::new(format!(
                "unexpected number of values on line {}",
                line_number
            )));
        }

        let num_sampling_rates =
            line_values[0]
                .trim()
                .to_string()
                .parse::<u32>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid integer value for number of sample rates: {}",
                        line_values[0]
                    ))
                })?;

        let mut sampling_rates: Vec<SamplingRate> = Vec::with_capacity(num_sampling_rates as usize);

        for i in 0..num_sampling_rates {
            line = lines.next().ok_or_else(early_end_err)?;
            line_values = line.split(CFG_SEPARATOR).collect();

            if line_values.len() != 2 {
                return Err(ParseError::new(format!(
                    "unexpected number of values on line {}",
                    line_number
                )));
            }

            // The sample rate in Hertz of this sample.
            let rate_hz = line_values[0]
                .trim()
                .to_string()
                .parse::<f64>()
                .map_err(|_| {
                    ParseError::new(format!(
                        "invalid float value for sample rate frequency for rate n# {} on line {}: {}",
                        i, line_number, line_values[0]
                    ))
                })?;

            // The sample number of the final sample that uses this sample rate. Note this corresponds
            // to the sample number value in the data itself, not an index.
            let end_sample_number =
                line_values[1]
                    .trim()
                    .to_string()
                    .parse::<u32>()
                    .map_err(|_| {
                        ParseError::new(format!(
                        "invalid integer value for end sample number for rate n# {} on line {}: {}",
                        i, line_number, line_values[1]
                    ))
                    })?;

            sampling_rates.push(SamplingRate {
                rate_hz,
                end_sample_number,
            });
        }

        self.total_num_samples = sampling_rates
            .iter()
            .map(|r| r.end_sample_number)
            .max()
            .unwrap() as usize;

        // Now that we know how many samples we have in total, we can update the channel buffers
        // with the correct capacity to make `push()` operations more efficient.
        for c in self.analog_channels.iter_mut() {
            c.data = Vec::with_capacity(self.total_num_samples as usize);
        }
        for c in self.status_channels.iter_mut() {
            c.data = Vec::with_capacity(self.total_num_samples as usize);
        }

        // If file has 0 for number of sample rates, there's an extra line which just contains 0
        // indicating no fixed sample rate and the total number of samples. We don't need this data
        // so we just ignore it.
        if num_sampling_rates == 0 {
            line_number += 1;
            lines.next().ok_or_else(early_end_err)?;
        }

        self.is_timestamp_critical = num_sampling_rates == 0;
        self.builder.sampling_rates(sampling_rates);

        line_number += 1;
        line = lines.next().ok_or_else(early_end_err)?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Date/time stamps
        // dd/mm/yyyy,hh:mm:ss.ssssss
        // dd/mm/yyyy,hh:mm:ss.ssssss
        // TODO: Whether this is to micro or nano seconds determines whether how to calculate
        //       real time values from timestamps (I think - not 100% on this).

        // Time of the first data sample in data file.
        let datetime_format = if format_revision == FormatRevision::Revision1991 {
            CFG_DATETIME_FORMAT_OLD
        } else {
            CFG_DATETIME_FORMAT
        };

        let start_time =
            NaiveDateTime::parse_from_str(line.trim(), datetime_format).map_err(|_| {
                ParseError::new(format!(
                    "invalid datetime value for start time on line {}: {}",
                    line_number, line,
                ))
            })?;
        self.builder.start_time(start_time);

        self.ts_base_unit = ts_base_unit(line.trim())?;

        line_number += 1;
        line = lines.next().ok_or_else(early_end_err)?;

        // Time that the COMTRADE record recording was triggered.
        let trigger_time =
            NaiveDateTime::parse_from_str(line.trim(), datetime_format).map_err(|_| {
                ParseError::new(format!(
                    "invalid datetime value for trigger time on line {}: {}",
                    line_number, line,
                ))
            })?;
        self.builder.trigger_time(trigger_time);

        // According to the spec, if the start time is in micro/nanoseconds, the
        // other one should be too. If they are inconsistent, just take the lower one
        // to be safe. In the future this would be a good place to raise a warning.
        self.ts_base_unit = self.ts_base_unit.min(ts_base_unit(line.trim())?);

        line_number += 1;
        line = lines.next().ok_or_else(early_end_err)?;

        // Data file type
        // ft
        let data_format = DataFormat::from_str(line)?;
        self.data_format = Some(data_format.clone());
        self.builder.data_format(data_format);

        // 1991 format ends here - rest of values are 1999 and 2013 only.
        if format_revision == FormatRevision::Revision1991 {
            return Ok(());
        }

        line_number += 1;
        line = lines.next().ok_or_else(early_end_err)?;

        // Time stamp multiplication factor
        // timemult
        // The base unit for the timestamps in the data file is determined from the CFG,
        // apparently from the time/stamp. It's not clear to me how this is determined.
        // Regardless, this multiplicative factor allows you to store longer time ranges
        // within a single COMTRADE record.

        let time_mult = line.trim().parse::<f64>().map_err(|_| {
            ParseError::new(format!(
                "invalid float value for time multiplication factor on line {}: {}",
                line_number, line,
            ))
        })?;
        self.builder.timestamp_multiplication_factor(time_mult);

        // Default values for optional revision-based fields.
        self.builder.time_offset(None);
        self.builder.local_offset(None);
        self.builder.time_quality(None);
        self.builder.leap_second_status(None);

        // 1999 format ends here - rest of values are 2013 only.
        if format_revision == FormatRevision::Revision1999 {
            return Ok(());
        }

        line_number += 1;
        line = lines.next().ok_or_else(early_end_err)?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Time information and relationship between local time and UTC
        // time_code, local_code
        self.builder.time_offset(parse_time_offset(line_values[0])?);
        self.builder
            .local_offset(parse_time_offset(line_values[1])?);

        line_number += 1;
        line = lines.next().ok_or_else(early_end_err)?;
        line_values = line.split(CFG_SEPARATOR).collect();

        // Time quality of samples
        // tmq_code,leapsec
        let tmq_code = TimeQuality::from_str(line_values[0])?;
        self.builder.time_quality(Some(tmq_code));

        let leap_second_status = LeapSecondStatus::from_str(line_values[1])?;
        self.builder.leap_second_status(Some(leap_second_status));

        Ok(())
    }
}

/// Implement a line as a trait alias for clearer implementation.
trait ConfigLine<'a>: Iterator<Item = &'a str> {
    /// Read the next value as any parsable type.
    fn read_value<T: FromStr>(&mut self) -> Result<T, ParseError> {
        let str_value = self
            .next()
            .ok_or(ParseError::new("No value on line.".to_string()))?;
        str_value
            .parse()
            .map_err(|e| ParseError::new(format!("Unable to parse the value. Value: {str_value}")))
    }

    /// This is used when there is an additional character on the end.
    /// For example 16A for channels where we want 16.
    fn read_value_with_trailing_char<T: FromStr>(&mut self) -> Result<T, ParseError> {
        let str_value = self
            .next()
            .ok_or(ParseError::new("No value on line.".to_string()))?;
        let trimmed_value = &str_value[..str_value.len().saturating_sub(1)];
        trimmed_value.parse().map_err(|e| {
            ParseError::new(format!("Unable to parse the value. Value: {trimmed_value}"))
        })
    }
}
/// Broad implementation of this trait so it acts as an alias.
impl<'a, T: Iterator<Item = &'a str>> ConfigLine<'a> for T {}

fn split_cfg_line(line: &str) -> impl ConfigLine {
    line.split(CFG_SEPARATOR).map(|s| s.trim())
}

struct ChannelSizes {
    total: usize,
    analog: usize,
    status: usize,
}

impl ChannelSizes {
    fn from_line<'a>(mut cfg_line: impl ConfigLine<'a>) -> Result<Self, ParseError> {
        let total = cfg_line.read_value()?;
        let analog = cfg_line.read_value_with_trailing_char()?;
        let status = cfg_line.read_value_with_trailing_char()?;
        Ok(Self {
            total,
            analog,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_line_uses_csv_seperator_and_trims_whitespace() {
        let line = "  1, 2, 3 ,4   ";
        let mut iter = split_cfg_line(line);
        assert_eq!(iter.next(), Some("1"));
        assert_eq!(iter.next(), Some("2"));
        assert_eq!(iter.next(), Some("3"));
        assert_eq!(iter.next(), Some("4"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn get_channel_counts() {
        let line = "20,4A,16D ";
        let line = split_cfg_line(line);
        let sizes = ChannelSizes::from_line(line).unwrap();
        assert_eq!(sizes.total, 20);
        assert_eq!(sizes.analog, 4);
        assert_eq!(sizes.status, 16);
    }
}
