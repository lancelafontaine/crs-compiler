use std::collections::HashMap;
use std::collections::VecDeque;

use lexer::{ Token, TokenClass };

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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

#[derive(Debug, Clone)]
enum ParseSymbol {
    Epsilon,
    Terminal(Token),
    Nonterminal(NonterminalLabel),
    PopError,
    ScanError
}

pub struct Ast;

lazy_static! {
    static ref PARSE_TABLE:  [[usize; 42]; 57] = [
        // S
        [1, 119, 119, 119, 1, 1, 119, 119, 1, 1, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 118],
        // A
        [3, 119, 119, 119, 2, 3, 119, 119, 3, 3, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // B
        [5, 119, 119, 119, 119, 4, 119, 119, 4, 4, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // C
        [119, 118, 6, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // D
        [118, 119, 119, 119, 7, 118, 119, 119, 118, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // E
        [119, 119, 9, 119, 119, 119, 8, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // F
        [119, 119, 119, 118, 119, 10, 119, 119, 12, 12, 119, 119, 11, 119, 11, 11, 119, 11, 11, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // G
        [119, 119, 119, 118, 119, 13, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // H
        [119, 119, 15, 119, 119, 119, 119, 14, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // I
        [119, 118, 119, 118, 119, 118, 119, 118, 119, 119, 16, 118, 118, 118, 118, 118, 119, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 119, 119, 119, 119, 16, 16, 118, 119, 119],
        // J
        [119, 119, 119, 119, 119, 18, 119, 119, 17, 19, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // K
        [119, 119, 119, 119, 119, 20, 119, 119, 20, 20, 119, 21, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // L
        [119, 119, 118, 119, 119, 22, 119, 119, 22, 22, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // M
        [119, 119, 119, 119, 119, 23, 119, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // N
        [118, 119, 119, 119, 119, 24, 119, 119, 24, 24, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // O
        [119, 119, 119, 27, 119, 25, 119, 119, 119, 119, 119, 119, 26, 119, 26, 26, 119, 26, 26, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // P
        [119, 119, 119, 118, 119, 118, 119, 119, 28, 29, 119, 119, 118, 119, 118, 118, 119, 118, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // Q
        [119, 31, 119, 119, 119, 119, 119, 31, 119, 119, 119, 31, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 30, 119, 119, 119],
        // R
        [119, 118, 119, 118, 119, 118, 119, 119, 119, 119, 119, 119, 32, 119, 33, 34, 119, 35, 36, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // T
        [119, 118, 119, 118, 119, 118, 119, 119, 119, 119, 37, 118, 118, 37, 118, 118, 119, 118, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 37, 37, 119, 119, 119],
        // U
        [119, 118, 119, 118, 119, 38, 119, 118, 119, 119, 38, 118, 118, 119, 118, 118, 119, 118, 118, 119, 119, 119, 119, 119, 119, 38, 38, 119, 119, 119, 119, 119, 119, 38, 38, 38, 38, 119, 119, 119, 119, 119],
        // V
        [119, 42, 39, 119, 119, 40, 119, 119, 119, 119, 119, 119, 41, 119, 41, 41, 119, 41, 41, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // W
        [119, 119, 119, 46, 119, 44, 119, 119, 43, 45, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // X
        [119, 118, 119, 119, 119, 47, 119, 119, 119, 119, 47, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 47, 47, 119, 119, 119, 119, 119, 119, 47, 47, 47, 47, 119, 119, 119, 119, 119],
        // Y
        [119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 48, 117, 119, 117, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 48, 48, 119, 119, 119],
        // Z
        [119, 118, 119, 118, 119, 49, 119, 118, 119, 119, 49, 118, 118, 119, 118, 118, 119, 118, 118, 118, 118, 118, 118, 118, 118, 49, 49, 119, 119, 119, 119, 119, 119, 49, 49, 49, 49, 119, 119, 118, 119, 119],
        // AA
        [119, 119, 119, 119, 119, 118, 119, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 50, 51, 52, 53, 54, 55, 118, 118, 119, 119, 119, 119, 119, 119, 118, 118, 118, 118, 119, 119, 119, 119, 119],
        // AB
        [119, 119, 119, 119, 119, 118, 119, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 56, 57, 58, 59, 119, 119, 119, 119, 118, 118, 118, 118, 119, 119, 119, 119, 119],
        // AC
        [119, 118, 119, 118, 119, 60, 119, 118, 119, 119, 60, 118, 118, 119, 118, 118, 119, 118, 118, 118, 118, 118, 118, 118, 118, 60, 60, 118, 118, 119, 119, 119, 119, 60, 60, 60, 60, 119, 119, 118, 119, 119],
        // AD
        [119, 119, 119, 119, 119, 118, 119, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 61, 62, 119, 119, 119, 119, 119, 119, 118, 118, 118, 118, 119, 119, 119, 119, 119],
        // AE
        [119, 119, 119, 119, 119, 118, 119, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 118, 118, 119, 119, 63, 64, 65, 66, 118, 118, 118, 118, 119, 119, 119, 119, 119],
        // AF
        [119, 118, 119, 118, 119, 69, 119, 118, 119, 119, 67, 118, 118, 119, 118, 118, 119, 118, 118, 118, 118, 118, 118, 118, 118, 71, 71, 118, 118, 118, 118, 118, 118, 68, 70, 72, 72, 119, 119, 118, 119, 119],
        // AG
        [119, 119, 119, 76, 119, 74, 119, 119, 73, 75, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AH
        [119, 119, 119, 119, 119, 118, 119, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 118, 118, 119, 119, 119, 119, 119, 119, 118, 118, 77, 78, 119, 119, 119, 119, 119],
        // AI
        [119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 79, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AJ
        [119, 81, 119, 81, 119, 81, 119, 81, 119, 119, 119, 81, 81, 81, 81, 81, 119, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 81, 119, 119, 119, 119, 81, 80, 81, 119, 119],
        // AK
        [119, 119, 119, 119, 119, 82, 119, 119, 119, 119, 82, 83, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 82, 82, 119, 119, 119, 119, 119, 119, 82, 82, 82, 82, 119, 119, 119, 119, 119],
        // AL
        [119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 84, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 85, 85, 119, 119, 119],
        // AM
        [119, 118, 119, 118, 119, 118, 119, 118, 119, 119, 119, 118, 118, 118, 118, 118, 119, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 119, 119, 119, 119, 118, 86, 118, 119, 119],
        // AN
        [119, 118, 119, 119, 119, 119, 119, 118, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 87, 119, 119, 119],
        // AO
        [119, 119, 119, 119, 119, 119, 119, 88, 119, 119, 119, 89, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AP
        [119, 119, 119, 119, 119, 119, 119, 90, 119, 119, 119, 91, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AQ
        [119, 119, 119, 119, 119, 119, 119, 92, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AR
        [119, 119, 119, 119, 119, 119, 119, 93, 119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AS
        [119, 94, 119, 118, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 94, 119, 119, 119],
        // AT
        [119, 96, 119, 118, 119, 119, 119, 119, 119, 119, 95, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 96, 119, 119, 119],
        // AU
        [119, 119, 119, 118, 119, 97, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AV
        [119, 119, 119, 98, 119, 99, 119, 119, 119, 119, 119, 119, 98, 119, 98, 98, 119, 98, 98, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AW
        [119, 119, 119, 118, 119, 119, 119, 119, 119, 119, 100, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 100, 100, 119, 119, 119],
        // AX
        [119, 119, 119, 102, 119, 119, 119, 119, 119, 119, 119, 119, 101, 119, 101, 101, 119, 101, 101, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // AY
        [119, 119, 119, 118, 119, 103, 119, 119, 119, 119, 104, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 104, 104, 119, 119, 119],
        // AZ
        [119, 106, 119, 106, 119, 106, 119, 106, 119, 119, 119, 106, 106, 119, 106, 106, 119, 106, 106, 105, 105, 105, 105, 105, 105, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119],
        // BA
        [119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 108, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 107, 119],
        // BB
        [118, 116, 118, 116, 118, 116, 118, 116, 118, 118, 109, 116, 116, 118, 116, 116, 118, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 116, 118, 118, 118, 118, 109, 109, 116, 118, 118],
        // BC
        [119, 118, 119, 118, 119, 118, 119, 118, 119, 119, 110, 118, 118, 119, 118, 118, 119, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 118, 119, 119, 119, 119, 119, 111, 118, 119, 119],
        // BD
        [119, 113, 119, 113, 119, 113, 119, 113, 119, 119, 119, 113, 113, 119, 113, 113, 119, 113, 113, 113, 113, 113, 113, 113, 113, 113, 113, 113, 113, 112, 112, 112, 112, 119, 119, 119, 119, 119, 119, 113, 119, 119],
        // BE
        [119, 115, 119, 115, 119, 115, 119, 115, 119, 119, 119, 115, 115, 119, 115, 115, 119, 115, 115, 115, 115, 115, 115, 115, 115, 114, 114, 114, 114, 119, 119, 119, 119, 119, 119, 119, 119, 119, 119, 115, 119, 119]
    ];

    static ref PARSE_TABLE_TOKEN_BY_COLUMN: HashMap<Token, usize> = {
        let mut m = HashMap::new();
        m.insert(Token::new(TokenClass::Keyword, String::from("program")), 0);
        m.insert(Token::new(TokenClass::Semicolon, String::from(";")), 1);
        m.insert(Token::new(TokenClass::OpenCurlyBrace, String::from("{")), 2);
        m.insert(Token::new(TokenClass::CloseCurlyBrace, String::from("}")), 3);
        m.insert(Token::new(TokenClass::Keyword, String::from("class")), 4);
        m.insert(Token::new(TokenClass::Identifier, String::new()), 5); // In this case, lexeme not considered
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
        m.insert(Token::new(TokenClass::Float, String::new()), 33); // In this case, lexeme not considered
        m.insert(Token::new(TokenClass::Integer, String::new()), 34); // In this case, lexeme not considered
        m.insert(Token::new(TokenClass::UnaryLogicalOperator, String::from("!")), 35);
        m.insert(Token::new(TokenClass::UnaryLogicalOperator, String::from("not")), 36);
        m.insert(Token::new(TokenClass::AccessorOperator, String::from(".")), 37);
        m.insert(Token::new(TokenClass::OpenSquareBracket, String::from("[")), 38);
        m.insert(Token::new(TokenClass::CloseSquareBracket, String::from("]")), 39);
        m.insert(Token::new(TokenClass::ScopeResolutionOperator, String::from("::")), 40);
        m.insert(Token::new(TokenClass::EndOfInput, String::new()), 41); // In this case, lexeme not considered
        m
    };

    static ref NON_TERMINAL_LABELS_BY_ROW: HashMap<NonterminalLabel, usize> = {
        let mut m = HashMap::new();
        m.insert(NonterminalLabel::S, 0);
        m.insert(NonterminalLabel::A, 1);
        m.insert(NonterminalLabel::B, 2);
        m.insert(NonterminalLabel::C, 3);
        m.insert(NonterminalLabel::D, 4);
        m.insert(NonterminalLabel::E, 5);
        m.insert(NonterminalLabel::F, 6);
        m.insert(NonterminalLabel::G, 7);
        m.insert(NonterminalLabel::H, 8);
        m.insert(NonterminalLabel::I, 9);
        m.insert(NonterminalLabel::J, 10);
        m.insert(NonterminalLabel::K, 11);
        m.insert(NonterminalLabel::L, 12);
        m.insert(NonterminalLabel::M, 13);
        m.insert(NonterminalLabel::N, 14);
        m.insert(NonterminalLabel::O, 15);
        m.insert(NonterminalLabel::P, 16);
        m.insert(NonterminalLabel::Q, 17);
        m.insert(NonterminalLabel::R, 18);
        m.insert(NonterminalLabel::T, 19);
        m.insert(NonterminalLabel::U, 20);
        m.insert(NonterminalLabel::V, 21);
        m.insert(NonterminalLabel::W, 22);
        m.insert(NonterminalLabel::X, 23);
        m.insert(NonterminalLabel::Y, 24);
        m.insert(NonterminalLabel::Z, 25);
        m.insert(NonterminalLabel::AA, 26);
        m.insert(NonterminalLabel::AB, 27);
        m.insert(NonterminalLabel::AC, 28);
        m.insert(NonterminalLabel::AD, 29);
        m.insert(NonterminalLabel::AE, 30);
        m.insert(NonterminalLabel::AF, 31);
        m.insert(NonterminalLabel::AG, 32);
        m.insert(NonterminalLabel::AH, 33);
        m.insert(NonterminalLabel::AI, 34);
        m.insert(NonterminalLabel::AJ, 35);
        m.insert(NonterminalLabel::AK, 36);
        m.insert(NonterminalLabel::AL, 37);
        m.insert(NonterminalLabel::AM, 38);
        m.insert(NonterminalLabel::AN, 39);
        m.insert(NonterminalLabel::AO, 40);
        m.insert(NonterminalLabel::AP, 41);
        m.insert(NonterminalLabel::AQ, 42);
        m.insert(NonterminalLabel::AR, 43);
        m.insert(NonterminalLabel::AS, 44);
        m.insert(NonterminalLabel::AT, 45);
        m.insert(NonterminalLabel::AU, 46);
        m.insert(NonterminalLabel::AV, 47);
        m.insert(NonterminalLabel::AW, 48);
        m.insert(NonterminalLabel::AX, 49);
        m.insert(NonterminalLabel::AY, 50);
        m.insert(NonterminalLabel::AZ, 51);
        m.insert(NonterminalLabel::BA, 52);
        m.insert(NonterminalLabel::BB, 53);
        m.insert(NonterminalLabel::BC, 54);
        m.insert(NonterminalLabel::BD, 55);
        m.insert(NonterminalLabel::BE, 56);
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
        m.insert(116, vec!(ParseSymbol::Epsilon));
        m.insert(117, vec!(ParseSymbol::Epsilon));
        m.insert(118, vec!(ParseSymbol::PopError));
        m.insert(119, vec!(ParseSymbol::ScanError));
        m
    };
}

pub fn parse(mut token_queue: VecDeque<Token>) -> Option<Ast> {
    let mut parsing_stack: Vec<ParseSymbol> = vec!();

    // Initial conditions
    parsing_stack.push(
        ParseSymbol::Terminal(Token::new(TokenClass::EndOfInput, String::new()))
    );
    parsing_stack.push(
        ParseSymbol::Nonterminal(NonterminalLabel::S)
    );

    while token_queue.len() > 0 {
        let next_input_token = token_queue[0].clone();

        let mut current_parse_symbol : ParseSymbol;
        let parsing_stack_length = parsing_stack.len();

        if parsing_stack_length > 0 {
            current_parse_symbol = parsing_stack[parsing_stack_length - 1].clone();
        } else {
            // TODO: EndOfInput token was popped off! Error here
            unimplemented!();
        }

        match current_parse_symbol {
            ParseSymbol::Nonterminal(nonterminal_label) => {
                let next_input_token = empty_lexeme_if_insignificant(next_input_token);
                // table lookup
                let parse_table_row_index = NON_TERMINAL_LABELS_BY_ROW[&nonterminal_label];
                let parse_table_column_index = PARSE_TABLE_TOKEN_BY_COLUMN[&next_input_token];
                let production_rhs_index = PARSE_TABLE[parse_table_row_index][parse_table_column_index];
                let mut production_rhs_expansion = &PRODUCTION_EXPANSION_BY_CELL_VALUE[&production_rhs_index].to_vec();
                let mut rev_production_rhs_expansion = reverse_parse_symbols(production_rhs_expansion.to_vec());
                parsing_stack.pop();
                parsing_stack.append(&mut rev_production_rhs_expansion);
                print_parser_state(&next_input_token, &parsing_stack)
            },
            ParseSymbol::Terminal(stack_token) => {
                if (should_ignore_lexeme(&next_input_token) &&
                        next_input_token.class == stack_token.class) ||
                        stack_token == next_input_token {
                    parsing_stack.pop();
                    token_queue.pop_front();
                } else {
                    unimplemented!(); // TODO
                }
                print_parser_state(&next_input_token, &parsing_stack)
            },
            ParseSymbol::Epsilon => {
                parsing_stack.pop();
                print_parser_state(&next_input_token, &parsing_stack)
            },
            ParseSymbol::PopError => {
                println!("POP ERROR");
                unimplemented!(); // TODO
            },
            ParseSymbol::ScanError => {
                println!("SCAN ERROR");
                unimplemented!(); // TODO
            }
        }
    }
    // Checking final conditions for parsing
    if parsing_stack.len() == 1 {
        if let ParseSymbol::Terminal(last_token) = parsing_stack[0].clone() {
            if last_token.class == TokenClass::EndOfInput {
                let _ast = Ast;
                return Some(_ast)
            }
        }
    }
    None
}

fn should_ignore_lexeme(token: &Token) -> bool {
    token.class == TokenClass::Identifier ||
        token.class == TokenClass::Float ||
        token.class == TokenClass::Integer ||
        token.class == TokenClass::EndOfInput
}

fn empty_lexeme_if_insignificant(next_input_token: Token) -> Token {
    if should_ignore_lexeme(&next_input_token) {
        let mut void_next_input_token = next_input_token.clone();
        void_next_input_token.lexeme = String::new();
        return void_next_input_token;
    }
    next_input_token
}
fn reverse_parse_symbols(vec: Vec<ParseSymbol>) -> Vec<ParseSymbol> {
    vec.iter()
        .map(|e| e.clone())
        .rev()
        .collect::<Vec<ParseSymbol>>()
}

fn print_parser_state(next_input_token: &Token, parsing_stack: &Vec<ParseSymbol>) {
    println!(">>> INPUT TOKEN: {:?} <<<", next_input_token);
    println!(">>> PARSING_STACK: {:#?} <<<", parsing_stack);
}
