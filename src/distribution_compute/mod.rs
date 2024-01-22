use serde_json::Value;
use std::collections::HashMap;

pub fn generate_distribution_data_from_resultset(
    list: &Vec<HashMap<String, Value>>,
    segment_cols: &Vec<String>,
) -> Vec<HashMap<String, Value>> {
    // count hash map.
    let mut segment_count_map: HashMap<String, (usize, usize)> = HashMap::new();

    // compute anomaly and change point counts from result set data.
    for record in list {
        let segment_value = get_segment_value(record, segment_cols);
        if !segment_count_map.contains_key(&segment_value) {
            segment_count_map.insert(segment_value.to_string(), (0, 0));
        }
        let mut count = segment_count_map.get(&segment_value).unwrap().clone();
        let is_anomaly = record.get("isAnomaly").unwrap().as_bool().unwrap();
        let is_change_point = record.get("isChangePoint").unwrap().as_bool().unwrap();

        count.0 = if is_anomaly { count.0 + 1 } else { count.0 };
        count.1 = if is_change_point {
            count.1 + 1
        } else {
            count.1
        };
        segment_count_map.insert(segment_value, count);
    }

    // generate a vector from the above count hash map.
    let mut distribution_data: Vec<HashMap<String, Value>> = Vec::new();
    for segment in segment_count_map {
        let value: String = segment.0.clone();
        let (anomaly_count, change_point_count) = segment.1;
        let value_arr: Vec<&str> = value.split("|").collect();
        let mut data_map: HashMap<String, Value> = HashMap::new();
        for (i, ele) in segment_cols.iter().enumerate() {
            let str_val: &str = *value_arr.get(i).unwrap();
            let val: Value = Value::from(String::from(str_val));
            data_map.insert(String::from(ele), val);
        }
        data_map.insert(String::from("anomalyCount"), Value::from(anomaly_count));
        data_map.insert(
            String::from("changePointCount"),
            Value::from(change_point_count),
        );
        distribution_data.push(data_map);
    }

    distribution_data
}

fn get_segment_value(record: &HashMap<String, Value>, segment_cols: &Vec<String>) -> String {
    let mut segment_value = String::new();
    for ele in segment_cols {
        if segment_value.len() > 0 {
            segment_value.push_str("|");
        }
        let v = record.get(ele).unwrap().as_str().unwrap();
        segment_value.push_str(v);
    }
    segment_value
}
