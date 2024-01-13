use std::collections::HashMap;

use super::{DataType, Table};

pub struct Database {
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Self {
        Self { tables: vec![] }
    }

    pub fn create_table(&mut self, name: String, fields: HashMap<String, DataType>) {
        self.tables.push(Table::new(name, fields));
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.iter().find(|t| t.name == name)
    }
}
