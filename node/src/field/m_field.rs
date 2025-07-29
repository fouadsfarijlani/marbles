use marble_types::MarbleInteger;

#[derive(Debug)]
pub struct MarbleIntegerField {
    field_type: MarbleInteger,
    value: u32,
}

impl MarbleIntegerField {
    pub fn new() -> Self {
        let new_int = MarbleInteger::new();
        Self {
            field_type: new_int,
            value: 100,
        }
    }
}
