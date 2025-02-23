use super::ConfigLine;
use crate::error::ComtradeError;
use std::num::NonZeroUsize;

#[derive(Debug, Clone, PartialEq)]
pub enum AnalogScalingMode {
    Primary,
    Secondary,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogConfig {
    /// 1-indexed counter to determine which channel this is in a COMTRADE record.
    pub index: NonZeroUsize,
    pub name: String,
    pub phase: String,
    pub circuit_component_being_monitored: String,
    pub units: String,
    pub min_value: f64,
    pub max_value: f64,
    /// Use to calculate real values from data points.
    pub multiplier: f64,
    pub offset_adder: f64,
    /// Value in microseconds.
    pub skew: f64,
    /// Used to convert between primary and secondary values in channel.
    pub primary_factor: f64,
    /// Used to convert between primary and secondary values in channel.
    pub secondary_factor: f64,
    pub scaling_mode: AnalogScalingMode,
}

impl AnalogConfig {
    pub fn from_cfg_row<'a>(mut config_line: impl ConfigLine<'a>) -> Result<Self, ComtradeError> {
        let index = config_line.read_value()?;
        let name = config_line.read_value()?;
        let phase = config_line.read_value()?;
        let circuit_component_being_monitored = config_line.read_value()?;
        let units = config_line.read_value()?;
        let multiplier = config_line.read_value()?;
        let offset_adder = config_line.read_value()?;
        let skew = config_line.read_value()?;
        let min_value = config_line.read_value()?;
        let max_value = config_line.read_value()?;
        let primary_factor = config_line.read_value()?;
        let secondary_factor = config_line.read_value()?;
        let scaling_mode = config_line.read_value()?;
        Ok(Self {
            index,
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
        })
    }
}
