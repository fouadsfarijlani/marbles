use marble_types::{MIntegerTypeConstraint, MarbleBool, MarbleInteger, MarbleString};

#[derive(Debug)]
// TODO: split this up for required fields
pub struct MarbleField<T, V> {
    field_type: T,
    pub value: Option<V>,
}

impl MarbleField<MarbleInteger, u32> {
    pub fn new_integer() -> Self {
        let int_type = MarbleInteger::new();
        Self {
            field_type: int_type,
            value: None,
        }
    }
}

impl MarbleField<MarbleString, String> {
    pub fn new_string() -> Self {
        let string_type = MarbleString::new();
        Self {
            field_type: string_type,
            value: None,
        }
    }
}

impl MarbleField<MarbleBool, bool> {
    pub fn new_bool() -> Self {
        let bool_type = MarbleBool::new();
        Self {
            field_type: bool_type,
            value: None,
        }
    }
}

#[derive(Debug)]
pub enum MField {
    MIntegerField(MarbleField<MarbleInteger, u32>),
    MStringField(MarbleField<MarbleString, String>),
}
