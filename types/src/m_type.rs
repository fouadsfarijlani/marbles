use crate::{MarbleBool, MarbleInteger, MarbleString};

#[derive(Debug)]
pub enum MType {
    MInteger(MarbleInteger),
    MString(MarbleString),
    MBool(MarbleBool),
}

pub trait TypeConstraintValidate<T>
where
    T: Sized,
{
    fn validate(&self, target: T) -> Result<bool, &str>;
}

impl MType {
    pub fn new_integer_type() -> Self {
        MType::MInteger(MarbleInteger::new())
    }

    pub fn new_string_type() -> Self {
        MType::MString(MarbleString::new())
    }

    pub fn new_bool_type() -> Self {
        MType::MBool(MarbleBool::new())
    }
}

impl TypeConstraintValidate<i32> for MType {
    fn validate(&self, target: i32) -> Result<bool, &str> {
        match self {
            MType::MInteger(integer_type) => integer_type.validate(target),
            MType::MBool(bool_type) => panic!("No TypeConstraint for type 'BOOL'"),
            MType::MString(string_type) => panic!("No TypeConstraint for type 'STRING'"),
        }
    }
}
