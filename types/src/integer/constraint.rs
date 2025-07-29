#[derive(Debug)]
pub struct MIntegerConstraint {
    value: u32,
    constraint_type: MIntegerConstraintType,
}

impl MIntegerConstraint {
    pub fn new(value: u32, constraint_type: MIntegerConstraintType) -> Self {
        Self {
            value: value,
            constraint_type: constraint_type,
        }
    }

    pub fn less_than(&self, target: u32) -> bool {
        // taret should be less than constraint
        target < self.value
    }

    pub fn greater_than(&self, target: u32) -> bool {
        // target should be greater than constraint
        target > self.value
    }

    pub fn equals(&self, target: u32) -> bool {
        // target should be equals to constraint
        target == self.value
    }
}

#[derive(Debug)]
pub enum MIntegerConstraintType {
    LessThan,
    GreasterThan,
    Equals,
}
