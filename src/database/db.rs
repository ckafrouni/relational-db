#![allow(unused)]

use crate::errors::DatabaseError;
use crate::query::Query;

use std::collections::BTreeMap;

use super::{table::Table, ColumnHeader, DataType, DataValue, Row};

#[derive(Debug, Default)]
pub struct Database {
    tables: BTreeMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database::default()
    }

    pub fn execute(&mut self, query: impl Into<Query>) -> Result<Option<Table>, DatabaseError> {
        let query = query.into();
        println!("executing query: {:#?}", query);
        match query {
            Query::CreateTable { name, columns, pk } => self.create_table(name, columns, pk),
            Query::InsertInto {
                name,
                fields,
                values,
            } => self.insert_into(name, fields, values),
            Query::Select { from_table, fields } => self.select(from_table, fields),
        }
    }

    fn create_table(
        &mut self,
        name: String,
        columns: Vec<ColumnHeader>,
        pk: Option<String>,
    ) -> Result<Option<Table>, DatabaseError> {
        if let Some(_) = self.tables.get(&name) {
            return Err(DatabaseError::TableAlreadyExists(name));
        }

        let mut table = Table::new();
        for column in columns {
            table.add_column(column.name, column.data_type);
        }

        if let Some(pk) = pk {
            table.set_pk(pk)?;
        }

        self.tables.insert(name, table);

        Ok(None)
    }

    fn insert_into(
        &mut self,
        name: String,
        fields: Vec<String>,
        values: Vec<Vec<String>>,
    ) -> Result<Option<Table>, DatabaseError> {
        if let Some(table) = self.tables.get_mut(&name) {
            for value in values {
                let mut row = Row::new();

                for (field, element) in fields.iter().zip(value) {}

                // let index = table.insert_row(
                //     fields.clone(),
                //     value.into_iter().map(|v| DataValue::String(v)).collect(),
                // )?;
            }
            Ok(None)
        } else {
            Err(DatabaseError::TableNotFound(name))
        }
    }

    fn select(
        &self,
        from_table: String,
        fields: Vec<String>,
    ) -> Result<Option<Table>, DatabaseError> {
        todo!()
    }
}
