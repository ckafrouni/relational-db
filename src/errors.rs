#[derive(Debug)]
pub enum DatabaseError {
    QueryError(String),
}

impl From<std::io::Error> for DatabaseError {
    fn from(error: std::io::Error) -> Self {
        DatabaseError::QueryError(error.to_string())
    }
}

#[derive(Debug)]
pub struct QueryBuilderError(Option<String>);

impl QueryBuilderError {
    pub fn new(msg: Option<String>) -> QueryBuilderError {
        QueryBuilderError(msg)
    }
}

#[derive(Debug)]
pub enum Error {
    DatabaseError(DatabaseError),
    QueryBuilderError(QueryBuilderError),
}

impl From<DatabaseError> for Error {
    fn from(error: DatabaseError) -> Self {
        Error::DatabaseError(error)
    }
}

impl From<QueryBuilderError> for Error {
    fn from(error: QueryBuilderError) -> Self {
        Error::QueryBuilderError(error)
    }
}
