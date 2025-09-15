use crate::{
    IntegerValidate,
    MIntegerTypeConstraint,
};

#[derive(Debug)]
pub struct MarbleInteger {
    pub constraint: Option<MIntegerTypeConstraint>,
}

impl MarbleInteger {
    pub const NAME: &'static str = "INTEGER";

    pub fn new() -> Self {
        Self { constraint: None }
    }
    pub fn add_less_than_constraint(&mut self, value: i32) {
        let new_constraint = MIntegerTypeConstraint::new_less_than_constraint(value);
        self.constraint = Some(new_constraint);
    }

    pub fn add_greater_than_constraint(&mut self, value: i32) {
        let constraint = MIntegerTypeConstraint::new_greater_than_constraint(value);
        self.constraint = Some(constraint);
    }

    pub fn add_equals(&mut self, value: i32) {
        let constraint = MIntegerTypeConstraint::new_equals_constraint(value);
        self.constraint = Some(constraint);
    }

    pub fn validate(&self, target: i32) -> Result<bool, &str> {
        match &self.constraint {
            Some(c) => Ok(c.validate(target)),
            None => panic!("No constraint defined for type Integer"),
        }
    }
}
