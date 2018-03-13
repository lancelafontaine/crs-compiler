use std::fmt;
use std::cmp::PartialEq;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum TokenClass {
    Identifier,
    Keyword,
    MathOperator,
    OpenMultiLineComment,
    CloseMultiLineComment,
    SingleLineComment,
    NewLine,
    OpenParens,
    CloseParens,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenSquareBracket,
    CloseSquareBracket,
    AssignmentOperator,
    Semicolon,
    BinaryLogicalOperator,
    UnaryLogicalOperator,
    RelationalOperator,
    Float,
    Integer,
    ScopeResolutionOperator,
    InheritanceOperator,
    AccessorOperator,
    Comma,
    EndOfInput,
    UndefinedTokenClass
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Token {
    pub class: TokenClass,
    pub lexeme: String
}

impl Token {
    pub fn new(class: TokenClass, lexeme: String) -> Token {
        Token {
            class,
            lexeme
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token {{ class: '{:?}', lexeme: '{}' }}", self.class, self.lexeme)
    }
}
