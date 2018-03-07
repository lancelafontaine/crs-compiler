use std::collections::HashMap;
use lexer::{ Token, TokenClass };

enum NonterminalLabel {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    AA,
    AB,
    AC,
    AD,
    AE,
    AF,
    AG,
    AH,
    AI,
    AJ,
    AK,
    AL,
    AM,
    AN,
    AO,
    AP,
    AQ,
    AR,
    AS,
    AT,
    AU,
    AV,
    AW,
    AX,
    AY,
    AZ,
    BA,
    BB,
    BC,
    BD,
    BE
}

enum ParseSymbol {
    Epsilon,
    Terminal(Token),
    Nonterminal(NonterminalLabel),
    PopError,
    ScanError
}


lazy_static! {
    static ref PARSE_TABLE:  [[usize; 42]; 57] = [
        [1, 117, 117, 117, 1, 1, 117, 117, 1, 1, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 116],
        [3, 117, 117, 117, 2, 3, 117, 117, 3, 3, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [5, 117, 117, 117, 117, 4, 117, 117, 4, 4, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 116, 6, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [116, 117, 117, 117, 7, 116, 117, 117, 116, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 9, 117, 117, 117, 8, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 116, 117, 10, 117, 117, 12, 12, 117, 117, 11, 117, 11, 11, 117, 11, 11, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 116, 117, 13, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 15, 117, 117, 117, 117, 14, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 116, 117, 116, 117, 116, 117, 116, 117, 117, 16, 116, 116, 116, 116, 116, 117, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 117, 117, 117, 117, 16, 16, 116, 117, 117],
        [117, 117, 117, 117, 117, 18, 117, 117, 17, 19, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 20, 117, 117, 20, 20, 117, 21, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 116, 117, 117, 22, 117, 117, 22, 22, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 23, 117, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [116, 117, 117, 117, 117, 24, 117, 117, 24, 24, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 27, 117, 25, 117, 117, 117, 117, 117, 117, 26, 117, 26, 26, 117, 26, 26, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 116, 117, 116, 117, 117, 28, 29, 117, 117, 116, 117, 116, 116, 117, 116, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 31, 117, 117, 117, 117, 117, 31, 117, 117, 117, 31, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 30, 117, 117, 117],
        [117, 116, 117, 116, 117, 116, 117, 117, 117, 117, 117, 117, 32, 117, 33, 34, 117, 35, 36, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 116, 117, 116, 117, 116, 117, 117, 117, 117, 37, 116, 116, 117, 116, 116, 117, 116, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 37, 37, 117, 117, 117],
        [117, 116, 117, 116, 117, 38, 117, 116, 117, 117, 38, 116, 116, 117, 116, 116, 117, 116, 116, 117, 117, 117, 117, 117, 117, 38, 38, 117, 117, 117, 117, 117, 117, 38, 38, 38, 38, 117, 117, 117, 117, 117],
        [117, 42, 39, 117, 117, 40, 117, 117, 117, 117, 117, 117, 41, 117, 41, 41, 117, 41, 41, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 46, 117, 44, 117, 117, 43, 45, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 116, 117, 117, 117, 47, 117, 117, 117, 117, 47, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 47, 47, 117, 117, 117, 117, 117, 117, 47, 47, 47, 47, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 48, 116, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 48, 48, 117, 117, 117],
        [117, 116, 117, 116, 117, 49, 117, 116, 117, 117, 49, 116, 116, 117, 116, 116, 117, 116, 116, 116, 116, 116, 116, 116, 116, 49, 49, 117, 117, 117, 117, 117, 117, 49, 49, 49, 49, 117, 117, 116, 117, 117],
        [117, 117, 117, 117, 117, 116, 117, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 50, 51, 52, 53, 54, 55, 116, 116, 117, 117, 117, 117, 117, 117, 116, 116, 116, 116, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 116, 117, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 56, 57, 58, 59, 117, 117, 117, 117, 116, 116, 116, 116, 117, 117, 117, 117, 117],
        [117, 116, 117, 116, 117, 60, 117, 116, 117, 117, 60, 116, 116, 117, 116, 116, 117, 116, 116, 116, 116, 116, 116, 116, 116, 60, 60, 116, 116, 117, 117, 117, 117, 60, 60, 60, 60, 117, 117, 116, 117, 117],
        [117, 117, 117, 117, 117, 116, 117, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 61, 62, 117, 117, 117, 117, 117, 117, 116, 116, 116, 116, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 116, 117, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 116, 116, 117, 117, 63, 64, 65, 66, 116, 116, 116, 116, 117, 117, 117, 117, 117],
        [117, 116, 117, 116, 117, 69, 117, 116, 117, 117, 67, 116, 116, 117, 116, 116, 117, 116, 116, 116, 116, 116, 116, 116, 116, 71, 71, 116, 116, 116, 116, 116, 116, 68, 70, 72, 72, 117, 117, 116, 117, 117],
        [117, 117, 117, 76, 117, 74, 117, 117, 73, 75, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 116, 117, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 116, 116, 117, 117, 117, 117, 117, 117, 116, 116, 77, 78, 117, 117, 117, 117, 117],
        [117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 79, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 81, 117, 81, 117, 81, 117, 81, 117, 117, 117, 81, 81, 81, 81, 81, 117, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 117, 117, 117, 117, 81, 80, 81, 117, 117],
        [117, 117, 117, 117, 117, 82, 117, 117, 117, 117, 82, 83, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 82, 82, 117, 117, 117, 117, 117, 117, 82, 82, 82, 82, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 84, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 85, 85, 117, 117, 117],
        [117, 116, 117, 116, 117, 116, 117, 116, 117, 117, 117, 116, 116, 116, 116, 116, 117, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 117, 117, 117, 117, 116, 86, 116, 117, 117],
        [117, 116, 117, 117, 117, 117, 117, 116, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 87, 117, 117, 117],
        [117, 117, 117, 117, 117, 117, 117, 88, 117, 117, 117, 89, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 117, 117, 90, 117, 117, 117, 91, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 117, 117, 92, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 117, 117, 93, 117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 94, 117, 116, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 94, 117, 117, 117],
        [117, 95, 117, 116, 117, 117, 117, 117, 117, 117, 96, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 95, 117, 117, 117],
        [117, 117, 117, 116, 117, 97, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 116, 117, 98, 117, 117, 117, 117, 117, 117, 99, 117, 99, 99, 117, 99, 99, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 116, 117, 117, 117, 117, 117, 117, 100, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 100, 100, 117, 117, 117],
        [117, 117, 117, 102, 117, 117, 117, 117, 117, 117, 117, 117, 101, 117, 101, 101, 117, 101, 101, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 116, 117, 103, 117, 117, 117, 117, 104, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 104, 104, 117, 117, 117],
        [117, 106, 117, 106, 117, 106, 117, 106, 117, 117, 117, 106, 106, 117, 106, 106, 117, 106, 106, 105, 105, 105, 105, 105, 105, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117],
        [117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 108, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 107, 117],
        [117, 116, 117, 116, 117, 116, 117, 116, 117, 117, 109, 116, 116, 117, 116, 116, 117, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 117, 117, 117, 117, 109, 109, 116, 117, 117],
        [117, 116, 117, 116, 117, 116, 117, 116, 117, 117, 110, 116, 116, 117, 116, 116, 117, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 117, 117, 117, 117, 117, 111, 116, 117, 117],
        [117, 113, 117, 113, 117, 113, 117, 113, 117, 117, 117, 113, 113, 117, 113, 113, 117, 113, 113, 113, 113, 113, 113, 113, 113, 113, 113, 113, 113, 112, 112, 112, 112, 117, 117, 117, 117, 117, 117, 113, 117, 117],
        [117, 115, 117, 115, 117, 115, 117, 115, 117, 117, 117, 115, 115, 117, 115, 115, 117, 115, 115, 115, 115, 115, 115, 115, 115, 114, 114, 114, 114, 117, 117, 117, 117, 117, 117, 117, 117, 117, 117, 115, 117, 117]
    ];

    static ref PARSE_TABLE_TOKEN_BY_COLUMN: HashMap<Token, usize> = {
        let mut m = HashMap::new();
        m.insert(Token::new(TokenClass::Keyword, String::from("program")), 0);
        m.insert(Token::new(TokenClass::Semicolon, String::from(";")), 1);
        m.insert(Token::new(TokenClass::OpenCurlyBrace, String::from("{")), 2);
        m.insert(Token::new(TokenClass::CloseCurlyBrace, String::from("}")), 3);
        m.insert(Token::new(TokenClass::Keyword, String::from("class")), 4);
        m.insert(Token::new(TokenClass::Identifier, String::from("id")), 5);
        m.insert(Token::new(TokenClass::InheritanceOperator, String::from(":")), 6);
        m.insert(Token::new(TokenClass::Comma, String::from(",")), 7);
        m.insert(Token::new(TokenClass::Keyword, String::from("float")), 8);
        m.insert(Token::new(TokenClass::Keyword, String::from("int")), 9);
        m.insert(Token::new(TokenClass::OpenParens, String::from("(")), 10);
        m.insert(Token::new(TokenClass::CloseParens, String::from(")")), 11);
        m.insert(Token::new(TokenClass::Keyword, String::from("for")), 12);
        m.insert(Token::new(TokenClass::AssignmentOperator, String::from("=")), 13);
        m.insert(Token::new(TokenClass::Keyword, String::from("get")), 14);
        m.insert(Token::new(TokenClass::Keyword, String::from("if")), 15);
        m.insert(Token::new(TokenClass::Keyword, String::from("then")), 16);
        m.insert(Token::new(TokenClass::Keyword, String::from("put")), 17);
        m.insert(Token::new(TokenClass::Keyword, String::from("return")), 18);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("<")), 19);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("<=")), 20);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("<>")), 21);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("==")), 22);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from(">")), 23);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from(">=")), 24);
        m.insert(Token::new(TokenClass::MathOperator, String::from("+")), 25);
        m.insert(Token::new(TokenClass::MathOperator, String::from("-")), 26);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("or")), 27);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("||")), 28);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("&&")), 29);
        m.insert(Token::new(TokenClass::MathOperator, String::from("*")), 30);
        m.insert(Token::new(TokenClass::MathOperator, String::from("/")), 31);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("and")), 32);
        m.insert(Token::new(TokenClass::Float, String::from("1.0")), 33); // In this case, lexeme not considered
        m.insert(Token::new(TokenClass::Integer, String::from("1")), 34); // In this case, lexeme not considered
        m.insert(Token::new(TokenClass::UnaryLogicalOperator, String::from("!")), 35);
        m.insert(Token::new(TokenClass::UnaryLogicalOperator, String::from("not")), 36);
        m.insert(Token::new(TokenClass::AccessorOperator, String::from(".")), 37);
        m.insert(Token::new(TokenClass::OpenSquareBracket, String::from("[")), 38);
        m.insert(Token::new(TokenClass::CloseSquareBracket, String::from("]")), 39);
        m.insert(Token::new(TokenClass::ScopeResolutionOperator, String::from("::")), 40);
        m.insert(Token::new(TokenClass::EndOfInput, String::from("$")), 41); // In this case, lexeme not considered
        m
    };

    static ref NON_TERMINAL_LABELS_BY_ROW: HashMap<usize, NonterminalLabel> = {
        let mut m = HashMap::new();
        m.insert(0, NonterminalLabel::S);
        m.insert(1, NonterminalLabel::A);
        m.insert(2, NonterminalLabel::B);
        m.insert(3, NonterminalLabel::C);
        m.insert(4, NonterminalLabel::D);
        m.insert(5, NonterminalLabel::E);
        m.insert(6, NonterminalLabel::F);
        m.insert(7, NonterminalLabel::G);
        m.insert(8, NonterminalLabel::H);
        m.insert(9, NonterminalLabel::I);
        m.insert(10, NonterminalLabel::J);
        m.insert(11, NonterminalLabel::K);
        m.insert(12, NonterminalLabel::L);
        m.insert(13, NonterminalLabel::M);
        m.insert(14, NonterminalLabel::N);
        m.insert(15, NonterminalLabel::O);
        m.insert(16, NonterminalLabel::P);
        m.insert(17, NonterminalLabel::Q);
        m.insert(18, NonterminalLabel::R);
        m.insert(19, NonterminalLabel::T);
        m.insert(20, NonterminalLabel::U);
        m.insert(21, NonterminalLabel::V);
        m.insert(22, NonterminalLabel::W);
        m.insert(23, NonterminalLabel::X);
        m.insert(24, NonterminalLabel::Y);
        m.insert(25, NonterminalLabel::Z);
        m.insert(26, NonterminalLabel::AA);
        m.insert(27, NonterminalLabel::AB);
        m.insert(28, NonterminalLabel::AC);
        m.insert(29, NonterminalLabel::AD);
        m.insert(30, NonterminalLabel::AE);
        m.insert(31, NonterminalLabel::AF);
        m.insert(32, NonterminalLabel::AG);
        m.insert(33, NonterminalLabel::AH);
        m.insert(34, NonterminalLabel::AI);
        m.insert(35, NonterminalLabel::AJ);
        m.insert(36, NonterminalLabel::AK);
        m.insert(37, NonterminalLabel::AL);
        m.insert(38, NonterminalLabel::AM);
        m.insert(39, NonterminalLabel::AN);
        m.insert(40, NonterminalLabel::AO);
        m.insert(41, NonterminalLabel::AP);
        m.insert(42, NonterminalLabel::AQ);
        m.insert(43, NonterminalLabel::AR);
        m.insert(44, NonterminalLabel::AS);
        m.insert(45, NonterminalLabel::AT);
        m.insert(46, NonterminalLabel::AU);
        m.insert(47, NonterminalLabel::AV);
        m.insert(48, NonterminalLabel::AW);
        m.insert(49, NonterminalLabel::AX);
        m.insert(50, NonterminalLabel::AY);
        m.insert(51, NonterminalLabel::AZ);
        m.insert(52, NonterminalLabel::BA);
        m.insert(53, NonterminalLabel::BB);
        m.insert(54, NonterminalLabel::BC);
        m.insert(55, NonterminalLabel::BD);
        m.insert(56, NonterminalLabel::BE);
        m
    };


    static ref PRODUCTION_EXPANSION_BY_CELL_VALUE: HashMap<usize, Vec<ParseSymbol>> = {
        let mut m = HashMap::new();
        m.insert(1, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::A),
            ParseSymbol::Nonterminal(NonterminalLabel::B),
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("program"))),
            ParseSymbol::Nonterminal(NonterminalLabel::C),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(2, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::D),
            ParseSymbol::Nonterminal(NonterminalLabel::A)
        ));
        m.insert(3, vec!(ParseSymbol::Epsilon));
        m.insert(4, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::N),
            ParseSymbol::Nonterminal(NonterminalLabel::B)
        ));
        m.insert(5, vec!(ParseSymbol::Epsilon));
        m.insert(6, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenCurlyBrace, String::from("{"))),
            ParseSymbol::Nonterminal(NonterminalLabel::F),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseCurlyBrace, String::from("}")))
        ));
        m.insert(7, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("class"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::E),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenCurlyBrace, String::from("{"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AG),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseCurlyBrace, String::from("}"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(8, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::InheritanceOperator, String::from(":"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::H)
        ));
        m.insert(9, vec!(ParseSymbol::Epsilon));
        m.insert(10, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AY)
        ));
        m.insert(11, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AX)
        ));
        m.insert(12, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::P),
            ParseSymbol::Nonterminal(NonterminalLabel::F)
        ));
        m.insert(13, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::Q),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::F)
        ));
        m.insert(14, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Comma, String::from(";"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::H)
        ));
        m.insert(15, vec!(ParseSymbol::Epsilon));
        m.insert(16, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AL),
            ParseSymbol::Nonterminal(NonterminalLabel::I)
        ));
        m.insert(17, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("float")))
        ));
        m.insert(18, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id")))
        ));
        m.insert(19, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("int")))
        ));
        m.insert(20, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::J),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::Q),
            ParseSymbol::Nonterminal(NonterminalLabel::AO)
        ));
        m.insert(21, vec!(ParseSymbol::Epsilon));
        m.insert(22, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::J),
            ParseSymbol::Nonterminal(NonterminalLabel::M),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::K),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")")))
        ));
        m.insert(23, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::BA)
        ));
        m.insert(24, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::L),
            ParseSymbol::Nonterminal(NonterminalLabel::C),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(25, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::T),
            ParseSymbol::Nonterminal(NonterminalLabel::O)
        ));
        m.insert(26, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::R),
            ParseSymbol::Nonterminal(NonterminalLabel::O)
        ));
        m.insert(27, vec!(ParseSymbol::Epsilon));
        m.insert(28, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("float"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::Q),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(29, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("int"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::Q),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(30, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AN),
            ParseSymbol::Nonterminal(NonterminalLabel::Q)
        ));
        m.insert(31, vec!(ParseSymbol::Epsilon));
        m.insert(32, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("for"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::J),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Terminal(Token::new(TokenClass::AssignmentOperator, String::from("="))),
            ParseSymbol::Nonterminal(NonterminalLabel::U),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::X),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::T),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Nonterminal(NonterminalLabel::V),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(33, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("get"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::Y),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(34, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("if"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::U),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("then"))),
            ParseSymbol::Nonterminal(NonterminalLabel::V),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(35, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("put"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::U),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(36, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("return"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::U),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(37, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Y),
            ParseSymbol::Terminal(Token::new(TokenClass::AssignmentOperator, String::from("="))),
            ParseSymbol::Nonterminal(NonterminalLabel::U)
        ));
        m.insert(38, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Z),
            ParseSymbol::Nonterminal(NonterminalLabel::AZ)
        ));
        m.insert(39, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenCurlyBrace, String::from("{"))),
            ParseSymbol::Nonterminal(NonterminalLabel::O),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseCurlyBrace, String::from("}"))),
        ));
        m.insert(40, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::T)
        ));
        m.insert(41, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::R)
        ));
        m.insert(42, vec!(ParseSymbol::Epsilon));
        m.insert(43, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("float"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AI)
        ));
        m.insert(44, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AI)
        ));
        m.insert(45, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("int"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AI)
        ));
        m.insert(46, vec!(ParseSymbol::Epsilon));
        m.insert(47, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Z),
            ParseSymbol::Nonterminal(NonterminalLabel::AA),
            ParseSymbol::Nonterminal(NonterminalLabel::Z)
        ));
        m.insert(48, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::I),
            ParseSymbol::Nonterminal(NonterminalLabel::AJ)
        ));
        m.insert(49, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AC),
            ParseSymbol::Nonterminal(NonterminalLabel::BE)
        ));
        m.insert(50, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("<")))
        ));
        m.insert(51, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("<=")))
        ));
        m.insert(52, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("<>")))
        ));
        m.insert(53, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("==")))
        ));
        m.insert(54, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from(">")))
        ));
        m.insert(55, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from(">=")))
        ));
        m.insert(56, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("+")))
        ));
        m.insert(57, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("-")))
        ));
        m.insert(58, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("or")))
        ));
        m.insert(59, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("||")))
        ));
        m.insert(60, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AF),
            ParseSymbol::Nonterminal(NonterminalLabel::BD)
        ));
        m.insert(61, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("+")))
        ));
        m.insert(62, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("-")))
        ));
        m.insert(63, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("&&")))
        ));
        m.insert(64, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("*")))
        ));
        m.insert(65, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("/")))
        ));
        m.insert(66, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("and")))
        ));
        m.insert(67, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::Z),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
        ));
        m.insert(68, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Float, String::from("1.0"))),
        ));
        m.insert(69, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::BB)
        ));
        m.insert(70, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Integer, String::from("1"))),
        ));
        m.insert(71, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AH),
            ParseSymbol::Nonterminal(NonterminalLabel::AF)
        ));
        m.insert(72, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AD),
            ParseSymbol::Nonterminal(NonterminalLabel::AF)
        ));
        m.insert(73, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("float"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AT)
        ));
        m.insert(74, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AT)
        ));
        m.insert(75, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("int"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AT)
        ));
        m.insert(76, vec!(ParseSymbol::Epsilon));
        m.insert(77, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::UnaryLogicalOperator, String::from("!")))
        ));
        m.insert(78, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::UnaryLogicalOperator, String::from("not")))
        ));
        m.insert(79, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::K),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::W)
        ));
        m.insert(80, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AM),
            ParseSymbol::Nonterminal(NonterminalLabel::AJ)
        ));
        m.insert(81, vec!(ParseSymbol::Epsilon));
        m.insert(82, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::U),
            ParseSymbol::Nonterminal(NonterminalLabel::AP)
        ));
        m.insert(83, vec!(ParseSymbol::Epsilon));
        m.insert(84, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::AK),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::AccessorOperator, String::from(".")))
        ));
        m.insert(85, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AJ),
            ParseSymbol::Terminal(Token::new(TokenClass::AccessorOperator, String::from(".")))
        ));
        m.insert(86, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenSquareBracket, String::from("["))),
            ParseSymbol::Nonterminal(NonterminalLabel::Z),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseSquareBracket, String::from("]")))
        ));
        m.insert(87, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenSquareBracket, String::from("["))),
            ParseSymbol::Terminal(Token::new(TokenClass::Integer, String::from("1"))),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseSquareBracket, String::from("]")))
        ));
        m.insert(88, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AQ),
            ParseSymbol::Nonterminal(NonterminalLabel::AO)
        ));
        m.insert(89, vec!(ParseSymbol::Epsilon));
        m.insert(90, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AR),
            ParseSymbol::Nonterminal(NonterminalLabel::AP)
        ));
        m.insert(91, vec!(ParseSymbol::Epsilon));
        m.insert(92, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Comma, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::J),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::Q)
        ));
        m.insert(93, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Comma, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::U)
        ));
        m.insert(94, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Q),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AG)
        ));
        m.insert(95, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AI)
        ));
        m.insert(96, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AS)
        ));
        m.insert(97, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AW)
        ));
        m.insert(98, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AX)
        ));
        m.insert(99, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AU)
        ));
        m.insert(100, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::T),
            ParseSymbol::Nonterminal(NonterminalLabel::AV)
        ));
        m.insert(101, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::R),
            ParseSymbol::Nonterminal(NonterminalLabel::AV)
        ));
        m.insert(102, vec!(ParseSymbol::Epsilon));
        m.insert(103, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AW)
        ));
        m.insert(104, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::G)
        ));
        m.insert(105, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AA),
            ParseSymbol::Nonterminal(NonterminalLabel::Z)
        ));
        m.insert(106, vec!(ParseSymbol::Epsilon));
        m.insert(107, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::ScopeResolutionOperator, String::from("::"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
        ));
        m.insert(108, vec!(ParseSymbol::Epsilon));
        m.insert(109, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::I),
            ParseSymbol::Nonterminal(NonterminalLabel::BC)
        ));
        m.insert(110, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::AK),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")")))
        ));
        m.insert(111, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AJ)
        ));
        m.insert(112, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AE),
            ParseSymbol::Nonterminal(NonterminalLabel::AF),
            ParseSymbol::Nonterminal(NonterminalLabel::BD)
        ));
        m.insert(113, vec!(ParseSymbol::Epsilon));
        m.insert(114, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AB),
            ParseSymbol::Nonterminal(NonterminalLabel::AC),
            ParseSymbol::Nonterminal(NonterminalLabel::BE)
        ));
        m.insert(115, vec!(ParseSymbol::Epsilon));
        m.insert(116, vec!(ParseSymbol::PopError));
        m.insert(117, vec!(ParseSymbol::ScanError));
        m
    };

}

