use std::collections::HashMap;

use crate::errors::DatabaseError;
use crate::query::Query;

pub struct Database {}

pub type TableSlice = HashMap<String, Vec<String>>;

impl Default for Database {
    fn default() -> Self {
        todo!("default database creation")
    }
}

impl Database {
    #[allow(unused)]
    pub fn execute(&mut self, query: Query) -> Result<TableSlice, DatabaseError> {
        todo!("execute")
    }
}
