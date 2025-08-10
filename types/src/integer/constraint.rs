use std::fmt::Debug;

pub trait IntegerConstraint: Debug {
    fn new(value: i32) -> Self
    where
        Self: Sized;

    fn validate(&self, target: i32) -> bool;
}

#[derive(Debug)]
pub struct LessThanConstraint {
    value: i32,
}

impl IntegerConstraint for LessThanConstraint {
    fn new(new_value: i32) -> Self {
        Self { value: new_value }
    }

    fn validate(&self, target: i32) -> bool {
        // target should be less than constraint
        target < self.value
    }
}

#[derive(Debug)]
pub struct GreaterThanConstraint {
    value: i32,
}

impl IntegerConstraint for GreaterThanConstraint {
    fn new(new_value: i32) -> Self {
        Self { value: new_value }
    }

    fn validate(&self, target: i32) -> bool {
        // target should be greater than constraint
        target > self.value
    }
}

#[derive(Debug)]
pub struct EqualsConstraint {
    value: i32,
}

impl IntegerConstraint for EqualsConstraint {
    fn new(new_value: i32) -> Self {
        Self { value: new_value }
    }

    fn validate(&self, target: i32) -> bool {
        self.value == target
    }
}

#[derive(Debug)]
pub enum MIntegerConstraintType {
    LessThan,
    GreasterThan,
    Equals,
}
