use marble_node::MarbleField;
use marble_types::{LessThanConstraint, MIntegerConstraintType, MarbleInteger, MarbleString};
use std::collections::HashMap;
use std::ptr::addr_of;

fn main() {
    let new_int = MarbleInteger::with_constraint(MIntegerConstraintType::LessThan, 32);

    let result = new_int.constraint.as_ref().map(|c| c.validate(30));
    println!("Results are: {:?}", result);
}
