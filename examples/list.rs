extern crate vpk;

use std::env;

fn main() {
    println!("Hello world");
    let args: Vec<_> = env::args().collect();
    println!("{}", args[1]);
}