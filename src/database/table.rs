#![allow(unused)]

use std::collections::BTreeMap;

use crate::errors::DatabaseError;

use super::{ColumnHeader, DataType, DataValue, Row};

#[derive(Debug, Default)]
pub struct Table {
    headers: Vec<ColumnHeader>,
    pk: Option<ColumnHeader>,
    pk_map: BTreeMap<DataValue, usize>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new() -> Self {
        Table::default()
    }

    pub fn get_data_type(&self, field: String) -> Result<DataType, DatabaseError> {
        self.headers.iter().
    }

    pub fn add_column(&mut self, name: impl Into<String>, data_type: DataType) {
        self.headers.push(ColumnHeader {
            name: name.into(),
            data_type,
        });
    }

    pub fn set_pk(&mut self, name: impl Into<String>) -> Result<(), DatabaseError> {
        let name = name.into();
        let pk = self
            .headers
            .iter()
            .find(|header| header.name == name)
            .ok_or_else(|| DatabaseError::ColumnNotFound(name.clone()))?;

        if self.pk.is_some() {
            return Err(DatabaseError::PkAlreadySet);
        }

        self.pk = Some(pk.clone());

        Ok(())
    }

    pub fn insert_row(
        &mut self,
        fields: Vec<String>,
        values: Vec<DataValue>,
    ) -> Result<usize, DatabaseError> {
        todo!("insert_row")
    }
}
