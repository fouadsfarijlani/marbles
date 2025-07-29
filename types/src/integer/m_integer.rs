use crate::{MIntegerConstraint, MIntegerConstraintType};

#[derive(Debug)]
pub struct MarbleInteger {
    name: String,
    constraint: Option<MIntegerConstraint>,
}

impl MarbleInteger {
    pub fn new() -> Self {
        Self {
            name: "INTEGER".to_string(),
            constraint: None,
        }
    }

    pub fn with_constraint(
        constraint_value: u32,
        constraint: MIntegerConstraintType,
    ) -> MarbleInteger {
        match constraint {
            MIntegerConstraintType::LessThan => Self::with_less_than_constraint(constraint_value),
            MIntegerConstraintType::GreasterThan => {
                Self::with_greater_than_constraint(constraint_value)
            }
            MIntegerConstraintType::Equals => Self::with_equals_constraint(constraint_value),
        }
    }

    fn with_less_than_constraint(constraint_value: u32) -> Self {
        let constraint =
            MIntegerConstraint::new(constraint_value, MIntegerConstraintType::LessThan);
        Self {
            name: "INTEGER".to_string(),
            constraint: Some(constraint),
        }
    }

    fn with_greater_than_constraint(constraint_value: u32) -> Self {
        let constraint =
            MIntegerConstraint::new(constraint_value, MIntegerConstraintType::GreasterThan);

        Self {
            name: "INTEGER".to_string(),
            constraint: Some(constraint),
        }
    }

    fn with_equals_constraint(constraint_value: u32) -> Self {
        let constraint = MIntegerConstraint::new(constraint_value, MIntegerConstraintType::Equals);

        Self {
            name: "INTEGER".to_string(),
            constraint: Some(constraint),
        }
    }
}
