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

#[cfg(test)]
mod tests {
    use lexer::Lexer;

    #[test]
    fn test_lexer_new() {
        let lexer = Lexer::new("source_example.txt");
        assert_eq!(lexer.current_index, 0);
    }
    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new("source_example.txt");
        let some_token = lexer.next_token();
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, "< keyword >");
            assert_eq!(token.lexeme, "int");
        }
    }
}
