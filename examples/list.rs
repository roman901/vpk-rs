extern crate vpk;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        panic!("Input file is not specified");
    }

    let vpk_file = match vpk::from_path(&args[1]) {
        Err(e) => panic!("Error while open file {}, err {}", &args[1], e),
        Ok(vpk_file) => vpk_file,
    };

    for file in vpk_file.tree.keys() {
        println!("{}", file);
    }
}
