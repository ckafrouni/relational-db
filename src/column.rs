use super::DataType;

pub trait Column {
    fn name(&self) -> &str;
    fn data_type(&self) -> DataType;
}
