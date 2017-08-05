extern crate csv;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;

pub fn parse_csv<'a>(file_path: &str, column_names: Vec<&'a str> ) -> Result<BTreeMap<usize, BTreeMap<String, String>>, Box<Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut rows = BTreeMap::new();
    for (id, result) in rdr.records().enumerate() {
        let record = result?;
        let mut row = BTreeMap::new();
        for (val, col) in record.iter().zip(&column_names) {
            row.insert(col.to_string(), val.to_string());
        }
        rows.insert(id, row);
    }
    Ok(rows)
}
