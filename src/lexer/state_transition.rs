use std::collections::HashMap;
use lexer::Token;
use lexer::Lexer;

lazy_static! {
    static ref STATE_TRANSITION_TABLE:  [[usize; 28]; 46] = [
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,8,2,2,45,35,12,13,14,15,20,21,9,4,22,27,23,25,19,28,31,16,0,0,0,0,0],
        [2,3,3,2,2,2,2,3,3,3,3,3,3,3,3,3,0,3,3,3,3,3,3,2,0,0,0,0],
        [3,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [4,5,5,5,5,5,5,5,0,0,0,0,5,6,7,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [5,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [6,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [7,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [9,11,11,11,11,11,11,11,0,0,0,0,11,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [10,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [11,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [12,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [13,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [14,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [15,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [16,18,18,18,18,18,18,18,0,0,0,0,18,0,18,0,0,0,0,0,0,0,17,0,0,0,0,0],
        [17,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [18,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [19,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [20,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [21,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [22,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [23,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,24,0,0,0,0,0,0,0,0,0,0],
        [24,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [25,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,26,0,0,0,0,0,0,0,0,0],
        [26,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [27,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [28,29,29,29,29,29,29,29,0,0,0,0,29,0,29,0,0,0,0,0,0,0,30,0,0,0,0,0],
        [29,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [30,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [31,34,34,34,34,34,34,34,0,0,0,0,34,0,34,0,0,0,0,0,32,0,33,0,0,0,0,0],
        [32,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [33,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
        [34,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [35,44,44,0,0,0,0,0,44,44,44,44,44,44,44,44,0,44,44,44,44,44,44,0,36,0,0,0],
        [36,0,0,0,0,37,37,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [37,43,43,0,39,37,38,0,43,43,43,43,43,43,43,43,0,43,43,43,43,43,43,0,0,0,0,0],
        [38,0,0,0,0,37,38,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [39,0,0,0,0,42,41,0,0,0,0,40,40,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [40,0,0,0,0,42,41,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [41,43,43,0,0,0,0,0,43,43,43,43,43,43,43,43,0,43,43,43,43,43,43,0,0,0,0,0],
        [42,43,43,0,0,42,42,0,43,43,43,43,43,43,43,43,0,43,43,43,43,43,43,0,0,0,0,0],
        [43,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [44,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
        [45,44,44,0,0,45,45,0,44,44,44,44,44,44,44,44,0,44,44,44,44,44,44,0,36,0,0,0]
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
        m
    };

    static ref STATE_TRANSITION_LABELS_BY_COLUMN: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("states", 0);
        m.insert("final", 25);
        m.insert("backtrack", 26);
        m.insert("error", 27);
        m
    };

    static ref TOKEN_BY_FINAL_STATE: HashMap<usize, &'static str> = {
        let mut m = HashMap::new();
        m.insert(3, "< identifier >");
        m.insert(5, "< math_operator, '/' >");
        m.insert(6, "< open_multi_line_comment >");
        m.insert(7, "< single_line_comment >");
        m.insert(8, "< newline >");
        m.insert(10, "< close_multi_line_comment >");
        m.insert(11, "< math_operator, '*' >");
        m.insert(12, "< open_parens >");
        m.insert(13, "< close_parens >");
        m.insert(14, "< open_curly_brace >");
        m.insert(15, "< close_curly_brace >");
        m.insert(17, "< relational_operator, '==' >");
        m.insert(18, "< assignment_operator >");
        m.insert(19, "< semicolon >");
        m.insert(20, "< math_operator, '+' >");
        m.insert(21, "< math_operator, '-' >");
        m.insert(22, "< math_operator, '%' >");
        m.insert(24, "< logical_operator, and >");
        m.insert(26, "< logical_operator, or >");
        m.insert(27, "< logical_operator, not >");
        m.insert(29, "< relational_operator, '>' >");
        m.insert(30, "< relational_operator, '>=' >");
        m.insert(32, "< relational_operator, '<>' >");
        m.insert(33, "< relational_operator, '<=' >");
        m.insert(34, "< relational_operator, '<' >");
        m.insert(43, "< float >");
        m.insert(44, "< int >");
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
        let mut some_token: Option<Token>;
        let mut token_buffer = String::from("");

        loop {
            if let Some(token) = self.transition(&mut token_buffer, lexer) {
                some_token = Some(token);
                if lexer.current_index >= lexer.source_buffer_length {
                    some_token = None;
                }
                break;
            }
        }
        some_token
    }

    pub fn transition(&mut self, token_buffer: &mut String, lexer: &mut Lexer) -> Option<Token> {
        // Get the transition character
        let c = lexer.source_buffer.char(lexer.current_index);

        // Transition to next state based on transition character
        let column_index = StateTransition::get_column_index_for_transition_char(c);
        let next_state = STATE_TRANSITION_TABLE[self.current_state][column_index];
        self.current_state = next_state;

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

    pub fn get_column_index_for_transition_char(c: char) -> usize {
        // Default: 0 results in an error state
        let mut column_index = 0;
        for (chars, possible_column_index) in STATE_TRANSITION_CHARS_BY_COLUMN.iter() {
            if chars.contains(&c) {
                column_index = *possible_column_index;
            }
        }
        if column_index == 0 {
            // TODO: ERROR
        }
        column_index
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

    pub fn get_token_by_state(&self, token_buffer: &str) -> Option<Token> {
        let mut token_class = "";
        match TOKEN_BY_FINAL_STATE.get(&self.current_state) {
            Some(class) => {
                token_class = class;
            },
            None => {
                //TODO: throw error
            }
        };

        if token_class == "< identifier >" && LANGUAGE_KEYWORDS.contains(&token_buffer) {
            token_class = "< keyword >";
        }

        Some(Token::new(String::from(token_class), String::from(token_buffer)))
    }
}
