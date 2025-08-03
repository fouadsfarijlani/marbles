#[derive(Debug)]
pub struct MarbleBool {
    name: String,
}

impl MarbleBool {
    pub fn new() -> Self {
        Self {
            name: "BOOL".to_string(),
        }
    }
}
