#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quicli;
extern crate colored;
extern crate petgraph;
extern crate ropey;

pub mod args;
pub mod ast;
pub mod lexer;
pub mod output;
pub mod parser;
pub mod semantic;
pub mod util;

use lexer::{Lexer, Token};
use output::error;
use quicli::prelude::*;
use std::collections::VecDeque;
use std::path::Path;
use semantic::GENERATED_SYMBOL_TABLE_GRAPH;

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

    // AST is built at this point
    // But no semantic checks have occurred
    semantic::build_symbol_tables();
    semantic::prune_symbol_tables();
    GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap().print_graph();
});
