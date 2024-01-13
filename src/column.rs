use super::DataType;

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub data: Vec<DataType>,
}

impl Column {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
            data: vec![],
        }
    }
}
