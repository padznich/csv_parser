extern crate postgres;

use std::collections::BTreeMap;


pub fn connect(user: &str, password: &str, host: &str, port: &str, db_name: &str) -> postgres::Connection {
    let uri = format!("postgresql://{}:{}@{}:{}/{}", user, password, host, port, db_name);
    let conn = postgres::Connection::connect(uri, postgres::TlsMode::None)
            .unwrap();
    conn
}


pub fn write(data: BTreeMap<usize, BTreeMap<String, String>>) {

    let conn = connect("postgres", "postgres", "localhost", "5432", "postgres");

    for (_id, sub_data) in data {
        println!("{:?}", sub_data.get("a").unwrap());
        conn.execute("INSERT INTO test (a, b, c, d) VALUES ($1, $2, $3, $4)",
                     &[
                         &sub_data.get("a").unwrap(),
                         &sub_data.get("b").unwrap(),
                         &sub_data.get("c").unwrap(),
                         &sub_data.get("d").unwrap()
                     ]).unwrap();
    }
}
