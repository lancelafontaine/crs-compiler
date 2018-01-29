use ropey::Rope;
use std::fs::File;
use std::io::{BufReader};
use lexer::Token;
use lexer::StateTransition;


pub struct Lexer {
    pub source_buffer: Rope,
    pub source_buffer_length: usize,
    pub current_index: usize
}

impl Lexer {
    pub fn new(path: &str) -> Lexer {
        let source_buffer = Rope::from_reader(
                BufReader::new(File::open(path).unwrap())
            ).unwrap();
        let source_buffer_length = source_buffer.clone().chars().count();
        Lexer {
            source_buffer: source_buffer,
            source_buffer_length: source_buffer_length,
            current_index: 0
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut state_transition = StateTransition::new();
        if let Some(token) = state_transition.generate_token(self) {
            Some(token)
        } else {
            None
        }
    }
}

