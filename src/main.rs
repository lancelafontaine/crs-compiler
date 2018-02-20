#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quicli;
extern crate colored;
extern crate ropey;

pub mod output;
pub mod lexer;
pub mod args;

use std::path::Path;
use lexer::Lexer;
use output::error;
use quicli::prelude::*;

main!(|args: args::Args| {
    if !Path::new(&args.input_file).exists() {
        error(1);
    }
    let mut lex = Lexer::new(&args.input_file);
    while let Some(token) = lex.next_token() {
        println!("{}", token);
    }
});

