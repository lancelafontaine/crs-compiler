#[macro_use]
extern crate lazy_static;
extern crate colored;
extern crate ropey;
extern crate clap;

pub mod output;

use ropey::Rope;
use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use std::process;
use clap::{Arg, App};


fn main() {
    let exit_code = run();
    process::exit(exit_code);
}

fn run() -> i32 {
    let arg_matches = App::new("compiler")
        .version("0.1")
        .about("Compiler built for the COMP 442 course at Concordia University")
        .author("Lance Lafontaine")
        .arg(Arg::with_name("input_file")
            .help("Path to input source file")
            .required(true)
            .index(1))
        .get_matches();
    let input_file = arg_matches.value_of("input_file").unwrap();

    if !Path::new(input_file).exists() {
        return output::error(1);
    }

    let text = Rope::from_reader(
        BufReader::new(File::open(input_file).unwrap())
    ).unwrap();

    for (i, c) in text.chars().enumerate() {
        println!("{} - {}", i, c);
    }
    return 0;
}
