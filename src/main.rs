#![feature(fs_read_write)]

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
pub mod codegen;
pub mod util;

use lexer::{Lexer, Token};
use quicli::prelude::*;
use std::path::Path;
use semantic::GENERATED_SYMBOL_TABLE_GRAPH;

main!(|args: args::Args| {
    if !Path::new(&args.input_file).exists() {
        output::error(1);
    }
    output::initialize_logs();

    // Lexical Analysis
    let token_queue = lexer::tokenize(args.input_file);

    // Syntactic Analysis
    parser::parse(token_queue);

    // AST is built at this point
    // But no semantic checks have occurred
    semantic::build_symbol_tables();
    semantic::prune_symbol_tables();
    semantic::check_types();
    codegen::compute_memory_size();
    codegen::generate_code();
    GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap().print_graph();
});
