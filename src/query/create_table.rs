use crate::{database::DataType, errors::QueryBuilderError};

#[derive(Debug)]
pub struct CreateTableQuery {}

/* #region States */
#[derive(Debug, Default)]
pub struct NoName;
#[derive(Debug, Default)]
pub struct Name(String);
/* #endregion */

#[derive(Debug, Default)]
pub struct CreateTableQB<T> {
    name: T,
    columns: Option<Vec<(String, DataType)>>,
}

impl CreateTableQB<NoName> {
    pub fn new() -> Self {
        CreateTableQB::default()
    }
}

impl<T> CreateTableQB<T> {
    pub fn name(self, name: impl Into<String>) -> CreateTableQB<Name> {
        CreateTableQB {
            name: Name(name.into()),
            columns: self.columns,
        }
    }

    pub fn column(mut self, name: impl Into<String>, data_type: DataType) -> Self {
        if let Some(columns) = &mut self.columns {
            columns.push((name.into(), data_type));
        } else {
            self.columns = Some(vec![(name.into(), data_type)]);
        }

        self
    }
}

impl CreateTableQB<Name> {
    pub fn build(self) -> Result<CreateTableQuery, QueryBuilderError> {
        println!("{:#?}", self);
        todo!("build")
    }
}
