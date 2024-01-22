// Group By counts

use std::collections::HashMap;

use serde_json::Value;

pub fn group_distribution_data(
    group_by_cols: &Vec<String>,
    distribution_data: &Vec<HashMap<String, Value>>,
    aggregate_col: &String,
) -> Vec<HashMap<std::string::String, Value>> {
    let mut count_map: HashMap<String, u64> = HashMap::new();

    // Perform group by - sum aggregation
    for record in distribution_data {
        let mut record_value = String::from("");
        for col in group_by_cols {
            if record_value.len() > 0 {
                record_value.push_str("|");
            }
            let v = record.get(col).unwrap().as_str().unwrap();
            record_value.push_str(v);
        }
        count_map.entry(record_value.to_string()).or_insert(0);
        let count: u64 = record.get(aggregate_col).unwrap().as_u64().unwrap();
        let prev_count: &u64 = count_map.get(&record_value).unwrap();
        count_map.insert(record_value, count + prev_count);
    }

    // convert HashMap to Vec<HashMap>
    let mut grouped_distribution: Vec<HashMap<String, Value>> = Vec::new();
    for segment in count_map {
        let value: String = segment.0.clone();
        let count = segment.1;
        let value_arr: Vec<&str> = value.split("|").collect();
        let mut data_map: HashMap<String, Value> = HashMap::new();
        for (i, ele) in group_by_cols.iter().enumerate() {
            let str_val: &str = *value_arr.get(i).unwrap();
            let val: Value = Value::from(String::from(str_val));
            data_map.insert(String::from(ele), val);
        }
        data_map.insert(String::from("count"), Value::from(count));
        grouped_distribution.push(data_map);
    }

    grouped_distribution
}
