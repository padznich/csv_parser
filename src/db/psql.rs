extern crate postgres;

use std::collections::BTreeMap;


struct Table<'a> {
    id: &'a usize,
    a: &'a String,
    b: &'a String,
}


pub fn connect(user: &str, password: &str, host: &str, port: &str, db_name: &str) -> postgres::Connection {
    let uri = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, db_name);
    let conn = postgres::Connection::connect(uri, postgres::TlsMode::None)
            .unwrap();
    conn
}


pub fn write(data: BTreeMap<usize, BTreeMap<String, String>>) {
    let conn = connect("postgres", "postgres", "localhost", "5432", "postgres");

    let (_, first_value) = data.iter().next().unwrap();
    let columns: Vec<_> = first_value.keys().cloned().collect();

    let mut values: Vec<Vec<String>> = Vec::new();
    for (_id, sub_data) in &data {
        let value = Table {
            id: _id,
            a: &sub_data.get("a").unwrap().to_string(),
            b: &sub_data.get("b").unwrap().to_string(),
        };
        conn.execute("INSERT INTO test (a, b) VALUES ($1, $2)", &[&value.a, &value.b]).unwrap();
    }
}