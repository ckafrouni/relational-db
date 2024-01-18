#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Integer,
    String,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Integer => write!(f, "INTEGER"),
            DataType::String => write!(f, "STRING"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataValue {
    Integer(i32),
    String(String),
}

#[derive(Debug, Clone)]
pub struct ColumnHeader {
    pub name: String,
    pub data_type: DataType,
}

pub type Row = Vec<DataValue>;

#[derive(Debug, Clone)]
pub enum ColumnOption {
    PK,
    AutoIncrement,
    NotNull,
    Unique,
}
