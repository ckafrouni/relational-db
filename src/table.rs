use std::collections::HashMap;

use super::{Column, DataType};

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn new(name: String, fields: HashMap<String, DataType>) -> Self {
        Self {
            name,
            columns: fields
                .into_iter()
                .map(|(name, data_type)| Column::new(name, data_type))
                .collect(),
        }
    }
}
