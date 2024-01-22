use serde_json::Value;
use std::collections::HashMap;
use std::fs;

pub(crate) fn read_json_file(file_path: &String) -> Vec<HashMap<String, Value>> {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lookup: Vec<HashMap<String, Value>> = serde_json::from_str(&contents).unwrap();

    lookup
}

pub fn write_records_to_json_file(file_path: &String, records: &Vec<HashMap<String, Value>>) {
    let serialized = serde_json::to_string(&records).unwrap();
    fs::write(file_path, serialized).expect("Couldn't write")
}
pub fn write_records_to_json_file2(file_path: &String, records: &Vec<HashMap<&str, Value>>) {
    let serialized = serde_json::to_string(&records).unwrap();
    fs::write(file_path, serialized).expect("Couldn't write")
}
