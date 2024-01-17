use crate::database::DataType;
use crate::errors::QueryBuilderError;

#[derive(Debug)]
pub struct Query {}

#[derive(Debug)]
pub struct QueryBuilder<'a> {
    fields: Vec<&'a str>,
    table_name: Option<&'a str>,
    data_type: Option<DataType>,
    values: Option<Box<[Box<[&'a str]>]>>,
    value: Option<Box<[&'a str]>>,
}

impl<'a> QueryBuilder<'a> {
    // Build
    pub fn build(&self) -> Result<Query, QueryBuilderError> {
        todo!("build")
    }

    // Create Table
    pub fn create_table(table_name: &str) -> QueryBuilder {
        todo!("create_table")
    }

    pub fn column(&self, column_name: &str, data_type: DataType) -> Self {
        todo!("column")
    }

    // Insert Into
    pub fn insert_into(table_name: &str) -> QueryBuilder {
        todo!("insert_into")
    }

    pub fn fields<T: AsRef<[&'a str]>>(&mut self, fields: T) -> &mut Self {
        // self.fields.extend(fields.as_ref().iter().copied());
        // self
        todo!("fields")
    }

    pub fn value<T: AsRef<[&'a str]>>(&self, values: T) -> Self {
        todo!("value")
    }

    // Select

    pub fn select<T: AsRef<[&'a str]>>(fields: T) -> QueryBuilder<'a> {
        todo!("select")
    }

    pub fn from(&self, table_name: &str) -> Self {
        todo!("from")
    }
}
