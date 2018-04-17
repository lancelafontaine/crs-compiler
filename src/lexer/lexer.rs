use lexer::StateTransition;
use lexer::Token;
use lexer::TokenClass;
use ropey::Rope;
use std::fs::File;
use std::io::BufReader;
use std::collections::VecDeque;
use output::write_to_token_log;

pub struct Lexer {
    pub source_buffer: Rope,
    pub source_buffer_length: usize,
    pub current_index: usize,
}

impl Lexer {
    pub fn new(path: &str) -> Lexer {
        let source_buffer = Rope::from_reader(BufReader::new(File::open(path).unwrap())).unwrap();
        let source_buffer_length = source_buffer.clone().chars().count();
        Lexer {
            source_buffer: source_buffer,
            source_buffer_length: source_buffer_length,
            current_index: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut token_generator = || -> Option<Token> {
            let mut state_transition = StateTransition::new();
            if let Some(token) = state_transition.generate_token(self) {
                Some(token)
            } else {
                None
            }
        };

        // Complexity delegated to this funciton to filter NewLine tokens
        // Must be done by creating a new state_transition struct each time
        loop {
            let some_token = token_generator();
            if let None = some_token {
                return None;
            }
            if let Some(token) = some_token {
                if token.class != TokenClass::NewLine {
                    return Some(token);
                }
            }
        }
    }
}

pub fn tokenize(input_file: String) -> VecDeque<Token> {
    let mut lex = Lexer::new(&input_file);
    let mut token_queue: VecDeque<Token> = VecDeque::new();
    while let Some(token) = lex.next_token() {
        token_queue.push_back(token.clone());
        write_to_token_log(format!("{:?}", token));
    }
    return token_queue
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use lexer::TokenClass;

    #[test]
    fn test_lexer_new() {
        let lexer = Lexer::new("tests/lexer/source_example.crs");
        assert_eq!(lexer.current_index, 0);
    }
    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new("tests/lexer/source_example.crs");
        let some_token = lexer.next_token();
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, TokenClass::Keyword);
            assert_eq!(token.lexeme, "class");
        }
    }
}
