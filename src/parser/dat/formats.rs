#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    Ascii,
    Binary16,
    Binary32,
    Float32,
}

impl Default for DataFormat {
    fn default() -> Self {
        DataFormat::Ascii
    }
}
