use relational_db::utils::colorise::Coloured;

use relational_db::{
    database::{DataType, Database},
    errors::Error,
    query::{Query, QueryBuilder as QB},
};

fn main() -> Result<(), Error> {
    println!("{}", "Relational DB".italic().bold());

    let mut db = Database::default();

    let create_query: Query = QB::create_table("user")
        .column("id", DataType::Integer)
        .column("name", DataType::String)
        .column("age", DataType::Integer)
        .column("email", DataType::String)
        .column("password", DataType::String)
        .build()?;

    db.execute(create_query)?;

    let insert_one_query: Query = QB::insert_into("user")
        .fields(["id", "name", "age", "email", "password"])
        .value(["1", "John Doe", "21", "john.doe@email.com", "password123"])
        .build()?;

    db.execute(insert_one_query)?;

    let insert_many_query: Query = QB::insert_into("user")
        .fields(["id", "name", "age", "email", "password"])
        .value(["2", "Jane Doe", "21", "jane.doe@email.com", "password"])
        .value(["3", "John Smith", "21", "john.smith@email.com", "password"])
        .build()?;

    db.execute(insert_many_query)?;

    let select_query: Query = QB::select(["id", "name", "age", "email"])
        .from("user")
        .build()?;

    let res = db.execute(select_query)?;

    println!("{:#?}", res);

    Ok(())
}
