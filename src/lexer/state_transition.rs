use std::collections::HashMap;
use lexer::Token;
use lexer::TokenClass;
use lexer::Lexer;
use output;

lazy_static! {
    static ref STATE_TRANSITION_TABLE:  [[usize; 31]; 52] = [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,8,2,2,45,35,12,13,14,15,20,21,9,4,22,27,23,25,19,28,31,16,0,51,46,49,50,0,0,0],
        [2,3,3,2,2,2,2,3,3,3,3,3,3,3,3,3,0,3,3,3,3,3,3,2,3,3,3,3,0,0,0],
        [3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [4,5,5,5,5,5,5,5,0,0,0,0,5,6,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [5,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [6,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [7,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [9,11,11,11,11,11,11,11,0,0,0,0,11,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [10,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [11,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [12,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [13,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [14,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [15,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [16,18,18,18,18,18,18,18,0,0,0,0,18,0,18,0,0,0,0,0,0,0,17,0,0,0,0,0,0,0,0],
        [17,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [18,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [19,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [20,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [21,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [22,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [23,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,24,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [24,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [25,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,26,0,0,0,0,0,0,0,0,0,0,0,0],
        [26,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [27,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [28,29,29,29,29,29,29,29,0,0,0,0,29,0,29,0,0,0,0,0,0,0,30,0,0,0,0,0,0,0,0],
        [29,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [30,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [31,34,34,34,34,34,34,34,0,0,0,0,34,0,34,0,0,0,0,0,32,0,33,0,0,0,0,0,0,0,0],
        [32,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [33,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [34,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [35,44,44,0,0,0,0,0,44,44,44,44,44,44,44,44,0,44,44,44,44,44,44,0,36,0,0,44,0,0,0],
        [36,0,0,0,0,37,37,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [37,43,43,0,39,37,38,0,43,43,43,43,43,43,43,43,0,43,43,43,43,43,43,0,0,0,0,43,0,0,0],
        [38,0,0,0,0,37,38,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [39,0,0,0,0,42,41,0,0,0,0,40,40,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [40,0,0,0,0,42,41,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [41,43,43,0,0,0,0,0,43,43,43,43,43,43,43,43,0,43,43,43,43,43,43,0,0,0,0,43,0,0,0],
        [42,43,43,0,0,42,42,0,43,43,43,43,43,43,43,43,0,43,43,43,43,43,43,0,0,0,0,43,0,0,0],
        [43,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [44,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [45,44,44,0,0,45,45,0,44,44,44,44,44,44,44,44,0,44,44,44,44,44,44,0,36,0,0,44,0,0,0],
        [46,48,48,48,0,0,0,0,0,0,0,0,0,0,48,0,0,0,0,0,0,0,0,0,0,47,0,0,0,0,0],
        [47,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [48,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [49,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [50,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [51,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0]
    ];

    static ref STATE_TRANSITION_CHARS_BY_COLUMN: HashMap<Vec<char>, usize> = {
        let mut m = HashMap::new();
        m.insert(vec!(' ', '\t', '\r'), 1);
        m.insert(vec!('\n'), 2);
        m.insert(vec!('a','b','c','d','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'), 3);
        m.insert(vec!('e'), 4);
        m.insert(vec!('1','2','3','4','5','6','7','8','9'), 5);
        m.insert(vec!('0'), 6);
        m.insert(vec!('('), 7);
        m.insert(vec!(')'), 8);
        m.insert(vec!('{'), 9);
        m.insert(vec!('}'), 10);
        m.insert(vec!('+'), 11);
        m.insert(vec!('-'), 12);
        m.insert(vec!('*'), 13);
        m.insert(vec!('/'), 14);
        m.insert(vec!('%'), 15);
        m.insert(vec!('!'), 16);
        m.insert(vec!('&'), 17);
        m.insert(vec!('|'), 18);
        m.insert(vec!(';'), 19);
        m.insert(vec!('>'), 20);
        m.insert(vec!('<'), 21);
        m.insert(vec!('='), 22);
        m.insert(vec!('_'), 23);
        m.insert(vec!('.'), 24);
        m.insert(vec!(':'), 25);
        m.insert(vec!('['), 26);
        m.insert(vec!(']'), 27);
        m
    };

    static ref STATE_TRANSITION_LABELS_BY_COLUMN: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("states", 0);
        m.insert("final", 28);
        m.insert("backtrack", 29);
        m.insert("error", 30);
        m
    };

    static ref TOKEN_BY_FINAL_STATE: HashMap<usize, TokenClass> = {
        let mut m = HashMap::new();
        m.insert(3, TokenClass::Identifier);
        m.insert(5, TokenClass::MathOperator);
        m.insert(6, TokenClass::OpenMultiLineComment);
        m.insert(7, TokenClass::SingleLineComment);
        m.insert(8, TokenClass::NewLine);
        m.insert(10, TokenClass::CloseMultiLineComment);
        m.insert(11, TokenClass::MathOperator);
        m.insert(12, TokenClass::OpenParens);
        m.insert(13, TokenClass::CloseParens);
        m.insert(14, TokenClass::OpenCurlyBrace);
        m.insert(15, TokenClass::CloseCurlyBrace);
        m.insert(17, TokenClass::RelationalOperator);
        m.insert(18, TokenClass::AssignmentOperator);
        m.insert(19, TokenClass::Semicolon);
        m.insert(20, TokenClass::MathOperator);
        m.insert(21, TokenClass::MathOperator);
        m.insert(22, TokenClass::MathOperator);
        m.insert(24, TokenClass::BinaryLogicalOperator);
        m.insert(26, TokenClass::BinaryLogicalOperator);
        m.insert(27, TokenClass::UnaryLogicalOperator);
        m.insert(29, TokenClass::RelationalOperator);
        m.insert(30, TokenClass::RelationalOperator);
        m.insert(32, TokenClass::RelationalOperator);
        m.insert(33, TokenClass::RelationalOperator);
        m.insert(34, TokenClass::RelationalOperator);
        m.insert(43, TokenClass::Float);
        m.insert(44, TokenClass::Integer);
        m.insert(47, TokenClass::ScopeResolutionOperator);
        m.insert(48, TokenClass::InheritanceOperator);
        m.insert(49, TokenClass::OpenSquareBracket);
        m.insert(50, TokenClass::CloseSquareBracket);
        m.insert(51, TokenClass::AccessorOperator);
        m
    };

    static ref LANGUAGE_KEYWORDS: Vec<&'static str> = {
        vec!(
            "if",
            "then",
            "else",
            "for",
            "class",
            "get",
            "put",
            "return",
            "program",
            "int",
            "float",
            "bool"
        )
    };
}

pub struct StateTransition {
    current_state: usize
}

impl StateTransition {
    pub fn new() -> StateTransition {
        StateTransition {
            current_state: 1
        }
    }

    pub fn generate_token(&mut self, lexer: &mut Lexer) -> Option<Token> {
        let mut some_token: Option<Token> = None;
        let mut token_buffer = String::from("");

        let mut in_multi_line_comment = false;
        let mut in_single_line_comment = false;

        loop {
            if let Some(token) = self.transition(&mut token_buffer, lexer) {

                if lexer.current_index >= lexer.source_buffer_length {
                    some_token = None;
                    break;
                }

                if token.class == TokenClass::SingleLineComment {
                    in_single_line_comment = true;
                    continue
                }

                if token.class == TokenClass::NewLine {
                    in_single_line_comment = false;
                    // do not continue here, since newline may be a terminator
                }

                if token.class == TokenClass::OpenMultiLineComment {
                    in_multi_line_comment = true;
                    continue
                }

                if token.class == TokenClass::CloseMultiLineComment {
                    in_multi_line_comment = false;
                    continue
                }

                if in_multi_line_comment || in_single_line_comment {
                    continue
                }

                some_token = Some(token);
                break;
            }
        }
        some_token
    }

    pub fn transition(&mut self, token_buffer: &mut String, lexer: &mut Lexer) -> Option<Token> {
        // Get the transition character
        if lexer.current_index >= lexer.source_buffer_length {
            lexer.current_index -= 1;
        }
        let c = lexer.source_buffer.char(lexer.current_index);

        // Transition to next state based on transition character
        if let Some(column_index) = StateTransition::get_column_index_for_transition_char(c) {
            let next_state = STATE_TRANSITION_TABLE[self.current_state][column_index];
            self.current_state = next_state;
        } else {
            output::print_line_char_at_invalid_state(lexer);
        }

        if self.is_error_state() {
            output::error(3);
            if !self.is_backtrack_state() {
                lexer.current_index -= 1;
            }
            output::print_line_char_at_invalid_state(lexer);
            if !self.is_backtrack_state() {
                lexer.current_index += 1;
            }
            self.current_state = 1;
            token_buffer.clear();
        }

        // Look at the next character for the next transition
        if !self.is_backtrack_state() {
            lexer.current_index += 1;
            if !c.is_whitespace(){
                token_buffer.push(c);
            }
        }

        if self.is_final_state() {
            self.get_token_by_state(token_buffer)
        } else {
            None
        }
    }

    pub fn get_column_index_for_transition_char(c: char) -> Option<usize> {
        // Default: 0 results in an error state
        let mut column_index = 0;
        for (chars, possible_column_index) in STATE_TRANSITION_CHARS_BY_COLUMN.iter() {
            if chars.contains(&c) {
                column_index = *possible_column_index;
            }
        }
        if column_index == 0 {
            output::error(2); //recoverable
            return None
        }
        Some(column_index)
    }

    pub fn is_final_state(&self) -> bool {
        STATE_TRANSITION_TABLE[self.current_state][STATE_TRANSITION_LABELS_BY_COLUMN["final"]] == 1
    }

    pub fn is_backtrack_state(&self) -> bool {
        STATE_TRANSITION_TABLE[self.current_state][STATE_TRANSITION_LABELS_BY_COLUMN["backtrack"]] == 1
    }

    pub fn is_error_state(&self) -> bool {
        STATE_TRANSITION_TABLE[self.current_state][STATE_TRANSITION_LABELS_BY_COLUMN["error"]] == 1
    }

    pub fn update_token_class_by_edge_case(&self, token_class: TokenClass, token_buffer: &str) -> TokenClass {
        if token_class == TokenClass::Identifier {
            if LANGUAGE_KEYWORDS.contains(&token_buffer) {
                return TokenClass::Keyword;
            }
            if token_buffer == "and"  || token_buffer == "or" {
                return TokenClass::BinaryLogicalOperator;
            }
            if token_buffer == "not" {
                return TokenClass::UnaryLogicalOperator;
            }
        }
        token_class
    }

    pub fn get_token_by_state(&self, token_buffer: &str) -> Option<Token> {
        let mut token_class = TokenClass::UndefinedTokenClass;
        match TOKEN_BY_FINAL_STATE.get(&self.current_state) {
            Some(class) => {
                token_class = class.clone();
            },
            None => {
                return None
            }
        };
        token_class = self.update_token_class_by_edge_case(token_class, token_buffer);
        Some(Token::new(token_class, String::from(token_buffer)))
    }
}

#[cfg(test)]
mod tests {
    use lexer::state_transition::StateTransition as st;
    use lexer::lexer::Lexer;
    use lexer::TokenClass;

    #[test]
    fn test_get_token_by_state_non_final_has_no_token() {
        let state_transition = st::new();
        let some_token = state_transition.get_token_by_state(" ");
        assert_eq!(some_token.is_none(), true);
    }
    #[test]
    fn test_get_token_by_state_final_has_token() {
        let state_transition = st {
            current_state: 3 // identifier - final state
        };
        let some_token = state_transition.get_token_by_state("a");
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, TokenClass::Identifier);
            assert_eq!(token.lexeme, "a");
        }
    }
    #[test]
    fn test_get_token_by_state_final_has_token_is_keyword() {
        let state_transition = st {
            current_state: 3 // identifier - final state
        };
        let some_token = state_transition.get_token_by_state("if");
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, TokenClass::Keyword);
            assert_eq!(token.lexeme, "if");
        }
    }
    #[test]
    fn test_get_token_by_state_final_has_token_is_logical_keyword_and() {
        let state_transition = st {
            current_state: 3 // identifier - final state
        };
        let some_token = state_transition.get_token_by_state("and");
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, TokenClass::BinaryLogicalOperator);
            assert_eq!(token.lexeme, "and");
        }
    }
    #[test]
    fn test_get_token_by_state_final_has_token_is_logical_keyword_or() {
        let state_transition = st {
            current_state: 3 // identifier - final state
        };
        let some_token = state_transition.get_token_by_state("or");
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, TokenClass::BinaryLogicalOperator);
            assert_eq!(token.lexeme, "or");
        }
    }
    #[test]
    fn test_get_token_by_state_final_has_token_is_logical_keyword_not() {
        let state_transition = st {
            current_state: 3 // identifier - final state
        };
        let some_token = state_transition.get_token_by_state("not");
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, TokenClass::UnaryLogicalOperator);
            assert_eq!(token.lexeme, "not");
        }
    }
    #[test]
    fn test_is_error_state_valid() {
        let state_transition = st {
            current_state: 0 // error state
        };
        let is_error = state_transition.is_error_state();
        assert_eq!(is_error, true);
    }
    #[test]
    fn test_is_error_state_invalid() {
        let state_transition = st {
            current_state: 1 // initial state
        };
        let is_error = state_transition.is_error_state();
        assert_eq!(is_error, false);
    }
    #[test]
    fn test_is_final_state_valid() {
        let state_transition = st {
            current_state: 3 // identifier - final state
        };
        let is_final = state_transition.is_final_state();
        assert_eq!(is_final, true);
    }
    #[test]
    fn test_is_final_state_invalid() {
        let state_transition = st {
            current_state: 1 // initial state
        };
        let is_final = state_transition.is_final_state();
        assert_eq!(is_final, false);
    }
    #[test]
    fn test_is_backtrack_state_valid() {
        let state_transition = st {
            current_state: 3 // identifier - backtrack state
        };
        let is_backtrack = state_transition.is_backtrack_state();
        assert_eq!(is_backtrack, true);
    }
    #[test]
    fn test_is_backtrack_state_invalid() {
        let state_transition = st {
            current_state: 1 // initial state
        };
        let is_backtrack = state_transition.is_backtrack_state();
        assert_eq!(is_backtrack, false);
    }
    #[test]
    fn test_column_for_for_transition_char_valid() {
        let some_index = st::get_column_index_for_transition_char(' ');
        assert_eq!(some_index.is_some(), true);
        if let Some(index) = some_index {
            assert_eq!(index, 1);
        }
    }
    #[test]
    fn test_column_for_for_transition_char_invalid_character() {
        let index = st::get_column_index_for_transition_char('^');
        assert_eq!(index.is_none(), true);
    }
    #[test]
    fn test_transition_valid_intermediate_state() {
        let mut state_transition = st::new();
        let mut lexer = Lexer::new("tests/lexer/source_example.crs");
        assert_eq!(state_transition.current_state, 1);
        state_transition.transition(&mut String::from("i"), &mut lexer);
        assert_eq!(state_transition.current_state, 2);
        assert_eq!(state_transition.is_error_state(), false);
        assert_eq!(state_transition.is_final_state(), false);
        assert_eq!(state_transition.is_backtrack_state(), false);
    }
    #[test]
    fn test_generate_token() {
        let mut state_transition = st::new();
        let mut lexer = Lexer::new("tests/lexer/source_example.crs");
        assert_eq!(state_transition.current_state, 1);
        let some_token = state_transition.generate_token(&mut lexer);
        assert_eq!(some_token.is_some(), true);
        if let Some(token) = some_token {
            assert_eq!(token.class, TokenClass::Keyword);
            assert_eq!(token.lexeme, "class");
        }
    }
}
