use crate::errors::DatabaseError;
use crate::query::Query;

use std::collections::BTreeMap;

use super::DataType;

pub type Rows = Vec<String>;
pub type Columns = Vec<Rows>;
// pub type Table = BTreeMap<String, String>;

#[derive(Debug, Default)]
pub struct Table {
    headers: BTreeMap<String, DataType>,
    columns: BTreeMap<String, Rows>,
}

impl Table {
    pub fn new() -> Self {
        Table::default()
    }

    pub fn insert_header(&mut self, header: String, data_type: DataType) {
        self.headers.insert(header, data_type);
    }
}

#[derive(Debug, Default)]
pub struct Database {
    tables: BTreeMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database::default()
    }

    pub fn execute(&mut self, query: impl Into<Query>) -> Result<Table, DatabaseError> {
        let query = query.into();
        println!("{:#?}", self);
        println!("executing query: {:#?}", query);
        match query {
            Query::CreateTable { name, columns } => {
                self.create_table(name, columns).map(|_| Table::new())
            }
            Query::InsertInto {
                name,
                fields,
                values,
            } => self.insert_into(name, fields, values).map(|_| Table::new()),
            Query::Select { from_table, fields } => self.select(from_table, fields),
        }
    }
}

impl Database {
    fn create_table(
        &mut self,
        name: String,
        columns: Vec<crate::query::ColumnHeader>,
    ) -> Result<(), DatabaseError> {
        println!("creating table: {}", name);
        let mut table = Table::new();

        for column in columns {
            table.insert_header(column.name, column.data_type);
        }

        self.tables.insert(name, table);

        Ok(())
    }

    fn insert_into(
        &mut self,
        name: String,
        fields: Vec<String>,
        values: Vec<Vec<String>>,
    ) -> Result<(), DatabaseError> {
        println!("inserting into table: {}", name);
        let table = self
            .tables
            .get_mut(&name)
            .ok_or_else(|| DatabaseError::TableNotFound(name.clone()))?;

        for value in values {
            for (i, value) in value.iter().enumerate() {
                let field = fields
                    .get(i)
                    .ok_or_else(|| DatabaseError::ColumnNotFound(name.clone()))?;

                let column = table.columns.entry(field.clone()).or_insert_with(Vec::new);

                column.push(value.clone());
            }
        }

        Ok(())
    }

    fn select(&self, from_table: String, fields: Vec<String>) -> Result<Table, DatabaseError> {
        println!("selecting from table: {}", from_table);
        let table = self
            .tables
            .get(&from_table)
            .ok_or_else(|| DatabaseError::TableNotFound(from_table.clone()))?;

        let mut selected_table = Table::new();

        table.headers.iter().for_each(|(k, v)| {
            if fields.is_empty() || fields.contains(k) {
                selected_table.insert_header(k.clone(), v.clone());
            }
        });

        for field in fields {
            let column = table
                .columns
                .get(&field)
                .ok_or_else(|| DatabaseError::ColumnNotFound(field.clone()))?;

            selected_table.columns.insert(field.clone(), column.clone());
        }

        Ok(selected_table)
    }
}
