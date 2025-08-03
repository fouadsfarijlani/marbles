use std::collections::HashMap;

use marble_node::MarbleField;
use marble_types::{MIntegerConstraintType, MarbleInteger, MarbleString};

fn main() {
    let new_int = MarbleInteger::new();
    let new_string = MarbleString::new();

    let int_field = MarbleField::new_integer();
    let string_field = MarbleField::new_string();
    let bool_field = MarbleField::new_bool();

    println!("{:?}", new_int);

    println!("{:?}", new_string);
    println!("{:?}", int_field);
    println!("{:?}", string_field);
    println!("{:?}", bool_field);
}
