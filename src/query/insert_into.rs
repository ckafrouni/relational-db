use crate::errors::QueryBuilderError;

use super::Query;

/* #region States */
#[derive(Debug, Default)]
pub struct NoName;
#[derive(Debug, Default)]
pub struct Name(String);

#[derive(Debug, Default)]
pub struct NoFields;
#[derive(Debug, Default)]
pub struct Fields(Vec<String>);

#[derive(Debug, Default)]
pub struct NoValues;
#[derive(Debug, Default)]
pub struct Values(Vec<Vec<String>>);
/* #endregion */

#[derive(Debug, Default)]
pub struct InsertIntoQB<N, F, V> {
    name: N,
    fields: F,
    values: V,
}

impl InsertIntoQB<NoName, NoFields, NoValues> {
    pub fn new() -> Self {
        InsertIntoQB::default()
    }
}

impl InsertIntoQB<Name, Fields, Values> {
    pub fn build(self) -> Result<Query, QueryBuilderError> {
        Ok(Query::InsertInto {
            name: self.name.0,
            fields: self.fields.0,
            values: self.values.0,
        })
    }
}

impl<N, F> InsertIntoQB<N, F, NoValues> {
    pub fn value<'a, T: AsRef<[&'a str]>>(self, value: T) -> InsertIntoQB<N, F, Values> {
        InsertIntoQB {
            name: self.name,
            fields: self.fields,
            values: Values(vec![value.as_ref().iter().map(|s| s.to_string()).collect()]),
        }
    }
}

impl<N, F> InsertIntoQB<N, F, Values> {
    pub fn value<'a, T: AsRef<[&'a str]>>(mut self, value: T) -> Self {
        // Add value to values
        self.values
            .0
            .push(value.as_ref().iter().map(|s| s.to_string()).collect());
        self
    }
}

impl<N, F, V> InsertIntoQB<N, F, V> {
    pub fn name(self, name: impl Into<String>) -> InsertIntoQB<Name, F, V> {
        InsertIntoQB {
            name: Name(name.into()),
            fields: self.fields,
            values: self.values,
        }
    }

    pub fn fields<'a, T: AsRef<[&'a str]>>(self, fields: T) -> InsertIntoQB<N, Fields, V> {
        InsertIntoQB {
            name: self.name,
            fields: Fields(fields.as_ref().iter().map(|s| s.to_string()).collect()),
            values: self.values,
        }
    }
}
