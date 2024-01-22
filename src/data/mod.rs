pub const SEGMENT_COLS: [&str; 3] = ["StoreId", "Place", "DeptId"];

pub fn get_segment_cols() -> Vec<String> {
    let a: Vec<String> = SEGMENT_COLS.iter().map(|e| String::from(*e)).collect();
    a
}
