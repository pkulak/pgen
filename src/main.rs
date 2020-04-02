use crate::char_class::{lower_class, number_class, symbol_class};
use crate::password::generate_password;

mod char_class;
mod password;

fn main() {
    let pass = generate_password(&vec![lower_class(), number_class()], 10);
    println!("{}", pass)
}
