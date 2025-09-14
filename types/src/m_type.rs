use crate::{MarbleBool, MarbleInteger, MarbleString};

#[derive(Debug)]
pub enum MarbleType {
    MInteger(MarbleInteger),
    MString(MarbleString),
    MBool(MarbleBool),
}

impl MarbleType {
    pub fn new_integer_type() -> Self {
        MarbleType::MInteger(MarbleInteger::new())
    }

    pub fn new_string_type() -> Self {
        MarbleType::MString(MarbleString::new())
    }

    pub fn new_bool_type() -> Self {
        MarbleType::MBool(MarbleBool::new())
    }
}
