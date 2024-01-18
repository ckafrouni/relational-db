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
