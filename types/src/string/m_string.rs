#[derive(Debug)]
pub struct MarbleString {
    name: String,
}

impl MarbleString {
    pub fn new() -> Self {
        Self {
            name: "STRING".to_string(),
        }
    }
}
