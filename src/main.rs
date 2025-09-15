// Main is just testing gorunds so far

use marble_types::{IntegerValidate, MType, MarbleInteger};

fn main() {
    let mut new_int = MarbleInteger::new();
    new_int.add_less_than_constraint(4);
    println!("{:?}", new_int);

    new_int.constraint = None;
    let results = new_int.validate(7);
    println!("{:?}", results);

    match &new_int.constraint {
        Some(c) => println!("value 7 is {:?}", c.validate(7)),
        None => println!("No constraint found"),
    }

    let new_type = MType::new_integer_type();
}
