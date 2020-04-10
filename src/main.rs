extern crate clap;

use crate::clap::{Clap};

use crate::char_class::{lower_class, number_class, symbol_class, upper_class};
use crate::password::generate_password;

mod char_class;
mod password;

#[derive(Clap)]
#[clap(version = "1.0", author = "Phil K.")]
struct Opts {
    #[clap(short = "l", long = "length", default_value = "10", help = "The length of each password")]
    length: u32
}

fn main() {
    let opts: Opts = Opts::parse();
    let all = vec![lower_class(), upper_class(), number_class(), symbol_class()];

    println!("{}\t{}\t{}\t{}",
             generate_password(&all[0..1], opts.length),
             generate_password(&all[0..2], opts.length),
             generate_password(&all[0..3], opts.length),
             generate_password(&all[0..4], opts.length))
}
