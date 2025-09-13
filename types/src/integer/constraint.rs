use std::fmt::Debug;

pub trait IntegerValidate {
    fn validate(&self, target: i32) -> bool;
}

#[derive(Debug)]
pub struct LessThanConstraint {
    value: i32,
}

impl LessThanConstraint {
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

impl GreaterThanConstraint {
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

impl EqualsConstraint {
    fn new(new_value: i32) -> Self {
        Self { value: new_value }
    }

    fn validate(&self, target: i32) -> bool {
        self.value == target
    }
}

#[derive(Debug)]
pub enum MIntegerTypeConstraint {
    LessThan(LessThanConstraint),
    GreasterThan(GreaterThanConstraint),
    Equals(EqualsConstraint),
}

impl MIntegerTypeConstraint {
    pub fn new_less_than_constraint(new_value: i32) -> Self {
        MIntegerTypeConstraint::LessThan(LessThanConstraint::new(new_value))
    }

    pub fn new_greater_than_constraint(new_value: i32) -> Self {
        MIntegerTypeConstraint::GreasterThan(GreaterThanConstraint::new(new_value))
    }

    pub fn new_equals_constraint(new_value: i32) -> Self {
        MIntegerTypeConstraint::Equals(EqualsConstraint::new(new_value))
    }
}

impl IntegerValidate for MIntegerTypeConstraint {
    fn validate(&self, target: i32) -> bool {
        match self {
            MIntegerTypeConstraint::LessThan(l) => l.validate(target),
            MIntegerTypeConstraint::GreasterThan(g) => g.validate(target),
            MIntegerTypeConstraint::Equals(e) => e.validate(target),
        }
    }
}
