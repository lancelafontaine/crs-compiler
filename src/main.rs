#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quicli;
extern crate colored;
extern crate ropey;
extern crate petgraph;

pub mod output;
pub mod util;
pub mod semantic;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod args;

use std::path::Path;
use lexer::{Lexer, Token};
use output::error;
use quicli::prelude::*;
use std::collections::VecDeque;

main!(|args: args::Args| {
    if !Path::new(&args.input_file).exists() {
        error(1);
    }

    // Lexical Analysis
    let mut lex = Lexer::new(&args.input_file);
    let mut token_queue: VecDeque<Token> = VecDeque::new();
    while let Some(token) = lex.next_token() {
        token_queue.push_back(token);
    }
    //println!("{:#?}", token_queue);

    // Syntactic Analysis
    parser::parse(token_queue);
});

