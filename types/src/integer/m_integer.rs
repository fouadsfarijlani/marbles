use crate::{
    EqualsConstraint, GreaterThanConstraint, IntegerConstraint, LessThanConstraint,
    MIntegerConstraintType,
};

#[derive(Debug)]
pub struct MarbleInteger {
    pub constraint: Option<Box<dyn IntegerConstraint>>,
}

impl MarbleInteger {
    pub const NAME: &'static str = "INTEGER";
    pub fn new() -> Self {
        Self { constraint: None }
    }

    pub fn with_constraint(constraint_type: MIntegerConstraintType, value: i32) -> Self {
        match constraint_type {
            MIntegerConstraintType::LessThan => Self::with_less_than_constraint(value),
            MIntegerConstraintType::GreasterThan => Self::with_greater_than_constraint(value),
            MIntegerConstraintType::Equals => Self::with_equals_constraint(value),
        }
    }

    fn with_less_than_constraint(value: i32) -> Self {
        let constraint = LessThanConstraint::new(value);
        Self {
            constraint: Some(Box::new(constraint)),
        }
    }

    fn with_greater_than_constraint(value: i32) -> Self {
        let constraint = GreaterThanConstraint::new(value);
        Self {
            constraint: Some(Box::new(constraint)),
        }
    }

    fn with_equals_constraint(value: i32) -> Self {
        let constraint = EqualsConstraint::new(value);
        Self {
            constraint: Some(Box::new(constraint)),
        }
    }
}
