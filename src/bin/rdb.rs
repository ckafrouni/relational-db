use relational_db::{Column, DataType, Database, Table};

pub struct Person;

impl Table for Person {
    fn name(&self) -> String {
        "person".to_string()
    }

    fn columns(&self) -> Vec<Column> {
        vec![
            Column {
                pk: true,
                name: "id".to_string(),
                data_type: DataType::Number,
            },
            Column {
                name: "id".to_string(),
                data_type: DataType::Number,
            },
            Column {
                name: "name".to_string(),
                data_type: DataType::String,
            },
        ]
    }
}

fn main() {
    let db = Database::new();

    db.create_table::<Person>();
}
