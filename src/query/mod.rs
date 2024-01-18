mod create_table;
mod insert_into;
mod select;

pub use create_table::CreateTableQB;
pub use insert_into::InsertIntoQB;
pub use select::SelectQB;

use crate::database::DataType;

#[derive(Debug, Clone)]
pub struct ColumnHeader {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub enum Query {
    CreateTable {
        name: String,
        columns: Vec<ColumnHeader>,
    },
    InsertInto {
        name: String,
        fields: Vec<String>,
        values: Vec<Vec<String>>,
    },
    Select {
        from_table: String,
        fields: Vec<String>,
    },
}
