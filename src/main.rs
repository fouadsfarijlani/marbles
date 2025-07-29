use marble_types::MarbleInteger;
use marble_types::MarbleString;

fn main() {
    let new_int = MarbleInteger::new();

    let new_string = MarbleString::new();
    println!("{:?}", new_int);
    println!("{:?}", new_string);
    println!("Hello, world!");
}
