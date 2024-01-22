// use crate::jsonreader;
use rand::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

pub mod data;
pub mod distribution;
pub mod distribution_compute;
pub mod json_reader;

fn main() {
    // let mut test: HashMap<String, usize> = HashMap::new();
    // test.insert(String::from("hi"), 0);
    // test.insert(String::from("hi"), 2);
    // test.insert(String::from("hi"), 10);

    // let path = String::from("./src/data/resultset_updated.json");
    // let json_content: Vec<HashMap<String, Value>> = json_reader::read_json_file(&path);
    // for (index, &ref item) in json_content.iter().enumerate() {
    //     println!("{} - {}", index, &item.get("Place").expect("msg"))
    // }
    // let dist: Vec<HashMap<String, Value>> =
    //     distribution_compute::generate_distribution_data_from_resultset(
    //         &json_content,
    //         &data::get_segment_cols(),
    //     );

    // let dist_path: String = String::from("./src/data/distribution.json");

    // json_reader::write_records_to_json_file(&dist_path, &dist);

    let path = String::from("./src/data/distribution.json");
    let json_content: Vec<HashMap<String, Value>> = json_reader::read_json_file(&path);

    let result = distribution::group_distribution_data(
        &vec![String::from("Place"), String::from("StoreId")],
        &json_content,
        &String::from("anomalyCount"),
    );
    let len = result.len();
    let dist_path: String = String::from("./src/data/group_by_result.json");

    json_reader::write_records_to_json_file(&dist_path, &result);

    println!("{len}");
}

#[allow(dead_code)]
fn generate_million_records() {
    let path = String::from("./src/data/resultset.json");
    let write_path = String::from("./src/data/resultset_updated.json");

    let mut json_content = json_reader::read_json_file(&path);

    let mut counter = 0;
    let initial_size = json_content.len();

    while counter < 1000000 {
        generate_random_record(&mut json_content, initial_size);
        counter = counter + 1;
    }

    json_reader::write_records_to_json_file(&write_path, &json_content)
}

fn generate_random_record(list: &mut Vec<HashMap<String, Value>>, limit: usize) {
    let cols = ["StoreId", "Place", "DeptId"];
    let indexes: (usize, usize, usize) = (
        generate_random_number(limit),
        generate_random_number(limit),
        generate_random_number(limit),
    );
    let a = (
        list.get(indexes.0)
            .expect("no record")
            .get(cols[0])
            .expect("no value in record"),
        list.get(indexes.1)
            .expect("no record")
            .get(cols[1])
            .expect("no value in record"),
        list.get(indexes.2)
            .expect("no record")
            .get(cols[2])
            .expect("no value in record"),
    );
    let mut new_elem = HashMap::<String, Value>::new();
    new_elem.insert(cols.get(0).expect("test").to_string(), a.0.clone());
    new_elem.insert(cols.get(1).expect("test").to_string(), a.1.clone());
    new_elem.insert(cols.get(2).expect("test").to_string(), a.2.clone());
    new_elem.insert("isAnomaly".to_string(), serde_json::Value::Bool(random()));
    new_elem.insert(
        "isChangePoint".to_string(),
        serde_json::Value::Bool(random()),
    );

    list.insert(list.len(), new_elem);
    // let values: (String, String, String)  = ( list[indexes.0][cols].get().copy(), list[indexes.1][cols[1])
}

pub fn generate_random_number(limit: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..=limit)
}
