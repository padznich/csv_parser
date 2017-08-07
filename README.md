Try RUST for postgres
===
    conn.execute("INSERT INTO test (a, b) VALUES ($1, $2)", &[&value.a, &value.b]).unwrap();
    conn.execute("INSERT INTO test (a, b, c) VALUES ($1, $2, $3)", &[&value.a, &value.b]).unwrap();
### Hardcode only - can't transform from Vec --> tuple

There is no clickhouse for rust yet.
===
