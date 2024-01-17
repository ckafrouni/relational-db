use crate::{database::DataType, errors::QueryBuilderError};

use super::{ColumnHeader, Query};

/* #region States */
#[derive(Debug, Default)]
pub struct NoName;
#[derive(Debug, Default)]
pub struct Name(String);

#[derive(Debug, Default)]
pub struct NoColumns;
#[derive(Debug, Default)]
pub struct Columns(Vec<ColumnHeader>);
/* #endregion */

#[derive(Debug, Default)]
pub struct CreateTableQB<N, C> {
    name: N,
    columns: C,
}

impl CreateTableQB<NoName, NoColumns> {
    pub fn new() -> Self {
        CreateTableQB::default()
    }
}

impl CreateTableQB<Name, Columns> {
    pub fn build(self) -> Result<Query, QueryBuilderError> {
        Ok(Query::CreateTable {
            name: self.name.0,
            columns: self.columns.0,
        })
    }
}

impl<N> CreateTableQB<N, Columns> {
    pub fn column(mut self, name: impl Into<String>, data_type: DataType) -> Self {
        self.columns.0.push(ColumnHeader {
            name: name.into(),
            data_type,
        });

        self
    }
}

impl<N> CreateTableQB<N, NoColumns> {
    pub fn column(self, name: impl Into<String>, data_type: DataType) -> CreateTableQB<N, Columns> {
        CreateTableQB {
            name: self.name,
            columns: Columns(vec![ColumnHeader {
                name: name.into(),
                data_type,
            }]),
        }
    }
}

impl<N, C> CreateTableQB<N, C> {
    pub fn name(self, name: impl Into<String>) -> CreateTableQB<Name, C> {
        CreateTableQB {
            name: Name(name.into()),
            columns: self.columns,
        }
    }
}
