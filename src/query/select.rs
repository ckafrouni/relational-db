use crate::errors::QueryBuilderError;

use super::Query;

/* #region States */
#[derive(Debug, Default)]
pub struct NoFromTable;
#[derive(Debug, Default)]
pub struct FromTable(String);

#[derive(Debug, Default)]
pub struct NoFields;
#[derive(Debug, Default)]
pub struct Fields(Vec<String>);
/* #endregion */

#[derive(Debug, Default)]
pub struct SelectQB<T, F> {
    from_table: T,
    fields: F,
}

impl SelectQB<NoFromTable, NoFields> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SelectQB<FromTable, Fields> {
    pub fn build(self) -> Result<Query, QueryBuilderError> {
        Ok(Query::Select {
            from_table: self.from_table.0,
            fields: self.fields.0,
        })
    }
}

impl<T, F> SelectQB<T, F> {
    pub fn from(self, name: impl Into<String>) -> SelectQB<FromTable, F> {
        SelectQB {
            from_table: FromTable(name.into()),
            fields: self.fields,
        }
    }

    pub fn fields(
        self,
        fields: impl IntoIterator<Item = impl Into<String>>,
    ) -> SelectQB<T, Fields> {
        SelectQB {
            from_table: self.from_table,
            fields: Fields(fields.into_iter().map(Into::into).collect()),
        }
    }
}
