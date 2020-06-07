extern crate clap;

use crate::char_class::{lower_class, number_class, symbol_class, upper_class};
use crate::clap::Clap;
use crate::pronounce::make_default_graph;

mod char_class;
mod password;
mod pronounce;

#[derive(Clap)]
#[clap(version = "1.0", author = "Phil K.")]
struct Opts {
    #[clap(short = "l", long = "length", default_value = "10", help = "The length of each password")]
    length: u32,

    #[clap(short = "c", long = "complexity", default_value="0")]
    complexity: usize
}

fn main() {
    let opts: Opts = Opts::parse();
    let all = vec![lower_class(), upper_class(), number_class(), symbol_class()];

    let complexity = if opts.complexity > 5 {
        5
    } else {
        opts.complexity
    };

    if complexity > 0 {
        let pass = if complexity == 1 {
            pronounce::generate_password(make_default_graph(), opts.length)
        } else {
            password::generate_password(&all[0..complexity - 1], opts.length)
        };

        println!("{}", pass);
        return
    }

    println!("{}\t{}\t{}\t{}\t{}",
             pronounce::generate_password(make_default_graph(), opts.length),
             password::generate_password(&all[0..1], opts.length),
             password::generate_password(&all[0..2], opts.length),
             password::generate_password(&all[0..3], opts.length),
             password::generate_password(&all[0..4], opts.length))
}
