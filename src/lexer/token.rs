use std::fmt;

pub struct Token {
    pub class: String,
    pub lexeme: String
}

impl Token {
    pub fn new(class: String, lexeme: String) -> Token {
        Token {
            class,
            lexeme
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token {{ class: '{}', lexeme: '{}' }}", self.class, self.lexeme)
    }

}
