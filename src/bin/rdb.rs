use std::collections::HashMap;

use relational_db::{DataType, Database};

fn main() {
    let mut db = Database::new();

    db.create_table("person".into(), {
        let mut fields = HashMap::new();
        fields.insert("name".to_string(), DataType::String);
        fields.insert("age".to_string(), DataType::Number);
        fields
    });

    let table = db.get_table("person").unwrap();

    println!("table: {:?}", table);
    println!("table: {:#?}", table);
}
