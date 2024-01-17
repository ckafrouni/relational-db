mod create_table;
mod insert_into;
mod select;

pub use create_table::{CreateTableQB, CreateTableQuery};
pub use insert_into::{InsertIntoQB, InsertIntoQuery};
pub use select::{SelectQB, SelectQuery};

#[derive(Debug)]
pub enum Query {
    CreateTable(CreateTableQuery),
    InsertInto(InsertIntoQuery),
    Select(SelectQuery),
}
