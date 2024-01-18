// This is a test file for testing database configurations
#![allow(dead_code)]
use std::collections::BTreeMap;


#[derive(Debug)]
enum DataType {
    Integer,
    String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DataValue {
    Integer(i32),
    String(String),
}

impl From<i32> for DataValue {
    fn from(value: i32) -> Self {
        DataValue::Integer(value)
    }
}

impl From<&str> for DataValue {
    fn from(value: &str) -> Self {
        DataValue::String(value.into())
    }
}

type PrimaryKey = DataValue;

#[derive(Debug, Default)]
struct Table {
    headers: Vec<(String, DataType)>,
    keys: BTreeMap<PrimaryKey, usize>,
    rows: Vec<Vec<DataValue>>,
}

fn main() {
    let mut table = Table::default();

    table.headers.push(("id".into(), DataType::Integer)); // Primary key
    table.headers.push(("name".into(), DataType::String));
    table.headers.push(("age".into(), DataType::Integer));

    table.rows.push(vec![
        DataValue::Integer(1),
        "John".into(),
        DataValue::Integer(20),
    ]);
    table.keys.insert(DataValue::Integer(1), 0);

    table.rows.push(vec![
        DataValue::Integer(2),
        "Jane".into(),
        DataValue::Integer(25),
    ]);
    table.keys.insert(DataValue::Integer(2), 1);

    println!("{:#?}", table);
}
