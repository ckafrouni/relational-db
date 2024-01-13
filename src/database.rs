use super::Table;

pub struct Database {
    pub tables: Vec<Box<dyn Table>>,
}

impl Database {
    pub fn new() -> Self {
        Self { tables: vec![] }
    }

    pub fn create_table<T: Table>(&mut self, table: T) {
        self.tables.push(Box::new(table));
    }
}
