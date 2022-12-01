pub struct Day {
    pub part_1: fn(&str) -> String,
    pub part_2: fn(&str) -> String,
    pub day_number: usize,
}

impl Default for Day {
    fn default() -> Self {
        Day {
            part_1: |_: &str| -> String { "Not implemented yet !".to_string() },
            part_2: |_: &str| -> String { "Not implemented yet !".to_string() },
            day_number: 0,
        }
    }
}
