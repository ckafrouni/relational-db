use std::collections::HashMap;

use crate::errors::DatabaseError;
use crate::query::Query;

pub struct Database {}

pub type TableSlice = HashMap<String, Vec<String>>;

impl Default for Database {
    fn default() -> Self {
        Self {}
    }
}

impl Database {
    #[allow(unused)]
    pub fn execute(&mut self, query: impl Into<Query>) -> Result<TableSlice, DatabaseError> {
        println!("executing query: {:#?}", query.into());
        Ok(TableSlice::new())
    }
}
