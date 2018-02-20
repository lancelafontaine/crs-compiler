use quicli::prelude::*;

/// crs: a compiler implemented in Rust for a C++-like language. Built for the COMP 442 course at Concordia University
#[derive(Debug, StructOpt)]
pub struct Args {
    /// Path to the input source file
    pub input_file: String,
}

