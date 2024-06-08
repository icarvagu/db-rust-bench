use std::fs::File;
use std::io::{BufReader};
use std::error::Error;
use csv::Reader;
use crate::modal::csvStruct::Record;

pub fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = Reader::from_reader(reader);
    let mut records = Vec::new();

    
    for result in csv_reader.deserialize::<Record>() {
        let mut record: Record = result?;
        records.push(record);
    }

    Ok(records)
}
