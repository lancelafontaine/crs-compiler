use std::collections::HashMap;
use std::collections::VecDeque;

use lexer::{ Token, TokenClass };
use parser::symbol::{ ParseSymbol, NonterminalLabel};
use semantic::{ SemanticActionType, SEMANTIC_ACTION_CALLBACKS_BY_TYPE };

pub struct Ast;

lazy_static! {
    static ref PARSE_TABLE:  [[usize; 43]; 54] = [
        // Program
        [1, 109, 109, 109, 109, 109, 109, 109, 109, 109, 1, 1, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 1, 1, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108],
        // AdditiveOperator
        [109, 109, 2, 3, 4, 5, 109, 108, 109, 109, 109, 108, 109, 109, 108, 109, 108, 109, 109, 109, 109, 109, 109, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // ArithmeticExpression
        [109, 108, 6, 6, 109, 109, 109, 6, 108, 109, 109, 6, 109, 109, 6, 108, 6, 108, 109, 109, 109, 109, 109, 6, 6, 109, 109, 109, 109, 108, 108, 108, 108, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109],
        // ArithmeticExpressionTail
        [109, 8, 7, 7, 7, 7, 109, 109, 8, 109, 109, 109, 109, 109, 109, 8, 109, 8, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 8, 8, 8, 8, 8, 8, 109, 109, 109, 109, 109, 109, 109, 109],
        // ArithmeticOrRelationalExpression
        [109, 10, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 10, 109, 10, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 9, 9, 9, 9, 9, 9, 109, 109, 109, 109, 109, 109, 109, 109],
        // ArraySize
        [109, 108, 109, 109, 109, 109, 11, 109, 109, 109, 109, 109, 109, 109, 109, 108, 109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // ArraySizeRecursion
        [109, 13, 109, 109, 109, 109, 12, 109, 109, 109, 109, 109, 109, 109, 109, 13, 109, 13, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // AssignmentStatement
        [109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 14, 109, 109, 109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // ClassDeclaration
        [108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 15, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // ClassDeclarationRecursion
        [17, 109, 109, 109, 109, 109, 109, 109, 109, 109, 16, 17, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 17, 17, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // Expression
        [109, 108, 18, 18, 109, 109, 109, 18, 109, 109, 109, 18, 109, 109, 18, 108, 18, 108, 109, 109, 109, 109, 109, 18, 18, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // Factor
        [109, 108, 22, 22, 108, 108, 109, 24, 108, 109, 109, 20, 109, 109, 19, 108, 23, 108, 109, 108, 108, 108, 108, 21, 21, 109, 109, 109, 109, 108, 108, 108, 108, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionArguments
        [109, 109, 25, 25, 109, 109, 109, 25, 109, 109, 109, 25, 109, 109, 25, 26, 25, 109, 109, 109, 109, 109, 109, 25, 25, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionArgumentsTail
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108, 109, 27, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionArgumentsTailRecursion
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 29, 109, 28, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionBody
        [109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 30, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionCallOrVariable
        [109, 108, 108, 108, 108, 108, 109, 109, 108, 109, 109, 31, 109, 109, 109, 108, 109, 108, 109, 108, 108, 108, 108, 109, 109, 109, 109, 109, 109, 108, 108, 108, 108, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionCallOrVariableTail
        [109, 32, 32, 32, 32, 32, 32, 109, 32, 32, 109, 109, 109, 109, 32, 32, 109, 32, 32, 32, 32, 32, 32, 109, 109, 109, 109, 109, 109, 32, 32, 32, 32, 32, 32, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionCallOrVariableTailRecursion
        [109, 34, 34, 34, 34, 34, 109, 109, 34, 109, 109, 109, 109, 109, 109, 34, 109, 34, 33, 34, 34, 34, 34, 109, 109, 109, 109, 109, 109, 34, 34, 34, 34, 34, 34, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionCallParensOrIndexing
        [109, 36, 36, 36, 36, 36, 36, 109, 36, 36, 109, 109, 109, 109, 35, 36, 109, 36, 36, 36, 36, 36, 36, 109, 109, 109, 109, 109, 109, 36, 36, 36, 36, 36, 36, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionDeclarationRecursionStart
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 37, 109, 38, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 37, 37, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionDeclarationRecursionTail
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108, 39, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionDefinition
        [108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 40, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 40, 40, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionDefinitionRecursion
        [42, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 41, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 41, 41, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionHeader
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 43, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 43, 43, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionParameters
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 44, 109, 109, 109, 45, 109, 109, 109, 109, 109, 109, 109, 109, 109, 44, 44, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionParametersTail
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108, 109, 46, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // FunctionParametersTailRecursion
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 48, 109, 47, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // IdListRecursion
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 50, 109, 109, 109, 109, 49, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // Indexing
        [109, 108, 108, 108, 108, 108, 51, 109, 108, 108, 109, 109, 109, 109, 109, 108, 109, 108, 108, 108, 108, 108, 108, 109, 109, 109, 109, 109, 109, 108, 108, 108, 108, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109],
        // IndexingRecursion
        [109, 53, 53, 53, 53, 53, 52, 109, 53, 53, 109, 109, 109, 109, 109, 53, 109, 53, 53, 53, 53, 53, 53, 109, 109, 109, 109, 109, 109, 53, 53, 53, 53, 53, 53, 109, 109, 109, 109, 109, 109, 109, 109],
        // MultiplicativeOperator
        [109, 109, 108, 108, 109, 109, 109, 108, 109, 109, 109, 108, 109, 109, 108, 109, 108, 109, 109, 54, 55, 56, 57, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // NegationOperator
        [109, 109, 108, 108, 109, 109, 109, 108, 109, 109, 109, 108, 109, 109, 108, 109, 108, 109, 109, 109, 109, 109, 109, 58, 59, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // NumberSign
        [109, 109, 60, 61, 109, 109, 109, 108, 109, 109, 109, 108, 109, 109, 108, 109, 108, 109, 109, 109, 109, 109, 109, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // NumberType
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 62, 63, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // OptionalInheritanceList
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 65, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 64, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // OptionalNamespacing
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 66, 109, 109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // OptionalNamespacingTail
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 68, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 67, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // RelationalExpression
        [109, 108, 69, 69, 109, 109, 109, 69, 109, 109, 109, 69, 109, 109, 69, 109, 69, 109, 109, 109, 109, 109, 109, 69, 69, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // RelationalOperator
        [109, 109, 108, 108, 109, 109, 109, 108, 109, 109, 109, 108, 109, 109, 108, 109, 108, 109, 109, 109, 109, 109, 109, 108, 108, 109, 109, 109, 109, 70, 71, 72, 73, 74, 75, 109, 109, 109, 109, 109, 109, 109, 109],
        // Statement
        [109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 76, 109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 77, 77, 77, 109, 108, 77, 77, 109],
        // StatementBlock
        [109, 79, 109, 109, 109, 109, 109, 109, 109, 109, 109, 78, 80, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 78, 78, 78, 109, 79, 78, 78, 109],
        // StatementRecursion
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 81, 109, 82, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 81, 81, 81, 109, 109, 81, 81, 109],
        // StatementWithoutAssignment
        [109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 108, 109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 83, 84, 85, 109, 108, 86, 87, 109],
        // Term
        [109, 108, 88, 88, 108, 108, 109, 88, 108, 109, 109, 88, 109, 109, 88, 108, 88, 108, 109, 109, 109, 109, 109, 88, 88, 109, 109, 109, 109, 108, 108, 108, 108, 108, 108, 109, 109, 109, 109, 109, 109, 109, 109],
        // TermRecursion
        [109, 90, 90, 90, 90, 90, 109, 109, 90, 109, 109, 109, 109, 109, 109, 90, 109, 90, 109, 89, 89, 89, 89, 109, 109, 109, 109, 109, 109, 90, 90, 90, 90, 90, 90, 109, 109, 109, 109, 109, 109, 109, 109],
        // Type
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 92, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 91, 91, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // Variable
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 108, 109, 93, 109, 109, 109, 108, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // VariableDeclarationRecursionThenStatementRecursionA
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 97, 109, 95, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 94, 94, 109, 109, 109, 109, 109, 109, 109, 109, 96, 96, 96, 109, 109, 96, 96, 109],
        // VariableDeclarationRecursionThenStatementRecursionB
        [109, 109, 109, 109, 109, 109, 98, 109, 109, 98, 109, 99, 109, 108, 98, 109, 109, 109, 98, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // VariableTail
        [109, 109, 109, 109, 109, 109, 101, 109, 109, 101, 109, 109, 109, 109, 100, 101, 109, 109, 101, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // VariableTailTail
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 103, 109, 109, 109, 109, 109, 103, 109, 109, 102, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // VariableThenFunctionDeclarationRecursion
        [109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 104, 109, 105, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 104, 104, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109],
        // VariableThenFunctionDeclarationRecursionTail
        [109, 106, 109, 109, 109, 109, 106, 109, 109, 109, 109, 109, 109, 108, 107, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109, 109]
    ];

    static ref PARSE_TABLE_TOKEN_BY_COLUMN: HashMap<Token, usize> = {
        let mut m = HashMap::new();
        m.insert(Token::new(TokenClass::Keyword, String::from("program")), 0);
        m.insert(Token::new(TokenClass::Semicolon, String::from(";")), 1);
        m.insert(Token::new(TokenClass::MathOperator, String::from("+")), 2);
        m.insert(Token::new(TokenClass::MathOperator, String::from("-")), 3);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("or")), 4);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("||")), 5);
        m.insert(Token::new(TokenClass::OpenSquareBracket, String::from("[")), 6);
        m.insert(Token::new(TokenClass::Integer, String::new()), 7); // In this case, lexeme not considered
        m.insert(Token::new(TokenClass::CloseSquareBracket, String::from("]")), 8);
        m.insert(Token::new(TokenClass::AssignmentOperator, String::from("=")), 9);
        m.insert(Token::new(TokenClass::Keyword, String::from("class")), 10);
        m.insert(Token::new(TokenClass::Identifier, String::new()), 11); // In this case, lexeme not considered
        m.insert(Token::new(TokenClass::OpenCurlyBrace, String::from("{")), 12);
        m.insert(Token::new(TokenClass::CloseCurlyBrace, String::from("}")), 13);
        m.insert(Token::new(TokenClass::OpenParens, String::from("(")), 14);
        m.insert(Token::new(TokenClass::CloseParens, String::from(")")), 15);
        m.insert(Token::new(TokenClass::Float, String::new()), 16); // In this case, lexeme not considered
        m.insert(Token::new(TokenClass::Comma, String::from(",")), 17);
        m.insert(Token::new(TokenClass::AccessorOperator, String::from(".")), 18);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("&&")), 19);
        m.insert(Token::new(TokenClass::MathOperator, String::from("*")), 20);
        m.insert(Token::new(TokenClass::MathOperator, String::from("/")), 21);
        m.insert(Token::new(TokenClass::BinaryLogicalOperator, String::from("and")), 22);
        m.insert(Token::new(TokenClass::UnaryLogicalOperator, String::from("!")), 23);
        m.insert(Token::new(TokenClass::UnaryLogicalOperator, String::from("not")), 24);
        m.insert(Token::new(TokenClass::Keyword, String::from("float")), 25);
        m.insert(Token::new(TokenClass::Keyword, String::from("int")), 26);
        m.insert(Token::new(TokenClass::InheritanceOperator, String::from(":")), 27);
        m.insert(Token::new(TokenClass::ScopeResolutionOperator, String::from("::")), 28);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("<>")), 29);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("<")), 30);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("<=")), 31);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from("==")), 32);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from(">")), 33);
        m.insert(Token::new(TokenClass::RelationalOperator, String::from(">=")), 34);
        m.insert(Token::new(TokenClass::Keyword, String::from("for")), 35);
        m.insert(Token::new(TokenClass::Keyword, String::from("get")), 36);
        m.insert(Token::new(TokenClass::Keyword, String::from("if")), 37);
        m.insert(Token::new(TokenClass::Keyword, String::from("then")), 38);
        m.insert(Token::new(TokenClass::Keyword, String::from("else")), 39);
        m.insert(Token::new(TokenClass::Keyword, String::from("put")), 40);
        m.insert(Token::new(TokenClass::Keyword, String::from("return")), 41);
        m.insert(Token::new(TokenClass::EndOfInput, String::new()), 42); // In this case, lexeme not considered
        m
    };

    static ref NON_TERMINAL_LABELS_BY_ROW: HashMap<NonterminalLabel, usize> = {
        let mut m = HashMap::new();
        m.insert(NonterminalLabel::Program, 0);
        m.insert(NonterminalLabel::AdditiveOperator, 1);
        m.insert(NonterminalLabel::ArithmeticExpression, 2);
        m.insert(NonterminalLabel::ArithmeticExpressionTail, 3);
        m.insert(NonterminalLabel::ArithmeticOrRelationalExpression, 4);
        m.insert(NonterminalLabel::ArraySize, 5);
        m.insert(NonterminalLabel::ArraySizeRecursion, 6);
        m.insert(NonterminalLabel::AssignmentStatement, 7);
        m.insert(NonterminalLabel::ClassDeclaration, 8);
        m.insert(NonterminalLabel::ClassDeclarationRecursion, 9);
        m.insert(NonterminalLabel::Expression, 10);
        m.insert(NonterminalLabel::Factor, 11);
        m.insert(NonterminalLabel::FunctionArguments, 12);
        m.insert(NonterminalLabel::FunctionArgumentsTail, 13);
        m.insert(NonterminalLabel::FunctionArgumentsTailRecursion, 14);
        m.insert(NonterminalLabel::FunctionBody, 15);
        m.insert(NonterminalLabel::FunctionCallOrVariable, 16);
        m.insert(NonterminalLabel::FunctionCallOrVariableTail, 17);
        m.insert(NonterminalLabel::FunctionCallOrVariableTailRecursion, 18);
        m.insert(NonterminalLabel::FunctionCallParensOrIndexing, 19);
        m.insert(NonterminalLabel::FunctionDeclarationRecursionStart, 20);
        m.insert(NonterminalLabel::FunctionDeclarationRecursionTail, 21);
        m.insert(NonterminalLabel::FunctionDefinition, 22);
        m.insert(NonterminalLabel::FunctionDefinitionRecursion, 23);
        m.insert(NonterminalLabel::FunctionHeader, 24);
        m.insert(NonterminalLabel::FunctionParameters, 25);
        m.insert(NonterminalLabel::FunctionParametersTail, 26);
        m.insert(NonterminalLabel::FunctionParametersTailRecursion, 27);
        m.insert(NonterminalLabel::IdListRecursion, 28);
        m.insert(NonterminalLabel::Indexing, 29);
        m.insert(NonterminalLabel::IndexingRecursion, 30);
        m.insert(NonterminalLabel::MultiplicativeOperator, 31);
        m.insert(NonterminalLabel::NegationOperator, 32);
        m.insert(NonterminalLabel::NumberSign, 33);
        m.insert(NonterminalLabel::NumberType, 34);
        m.insert(NonterminalLabel::OptionalInheritanceList, 35);
        m.insert(NonterminalLabel::OptionalNamespacing, 36);
        m.insert(NonterminalLabel::OptionalNamespacingTail, 37);
        m.insert(NonterminalLabel::RelationalExpression, 38);
        m.insert(NonterminalLabel::RelationalOperator, 39);
        m.insert(NonterminalLabel::Statement, 40);
        m.insert(NonterminalLabel::StatementBlock, 41);
        m.insert(NonterminalLabel::StatementRecursion, 42);
        m.insert(NonterminalLabel::StatementWithoutAssignment, 43);
        m.insert(NonterminalLabel::Term, 44);
        m.insert(NonterminalLabel::TermRecursion, 45);
        m.insert(NonterminalLabel::Type, 46);
        m.insert(NonterminalLabel::Variable, 47);
        m.insert(NonterminalLabel::VariableDeclarationRecursionThenStatementRecursionA, 48);
        m.insert(NonterminalLabel::VariableDeclarationRecursionThenStatementRecursionB, 49);
        m.insert(NonterminalLabel::VariableTail, 50);
        m.insert(NonterminalLabel::VariableTailTail, 51);
        m.insert(NonterminalLabel::VariableThenFunctionDeclarationRecursion, 52);
        m.insert(NonterminalLabel::VariableThenFunctionDeclarationRecursionTail, 53);
        m
    };

    static ref PRODUCTION_EXPANSION_BY_CELL_VALUE: HashMap<usize, Vec<ParseSymbol>> = {
        let mut m = HashMap::new();
        m.insert(1, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::ClassDeclarationRecursion),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionDefinitionRecursion),
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("program"))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionBody),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(2, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("+")))
        ));
        m.insert(3, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("-")))
        ));
        m.insert(4, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("or")))
        ));
        m.insert(5, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("||")))
        ));
        m.insert(6, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Term),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpressionTail)
        ));
        m.insert(7, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AdditiveOperator),
            ParseSymbol::Nonterminal(NonterminalLabel::Term),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpressionTail)
        ));
        m.insert(8, vec!(ParseSymbol::Epsilon));
        m.insert(9, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::RelationalOperator),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpression)
        ));
        m.insert(10, vec!(ParseSymbol::Epsilon));
        m.insert(11, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenSquareBracket, String::from("["))),
            ParseSymbol::Terminal(Token::new(TokenClass::Integer, String::from("1"))),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseSquareBracket, String::from("]"))),
        ));
        m.insert(12, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::ArraySize),
            ParseSymbol::Nonterminal(NonterminalLabel::ArraySizeRecursion)
        ));
        m.insert(13, vec!(ParseSymbol::Epsilon));
        m.insert(14, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Variable),
            ParseSymbol::Terminal(Token::new(TokenClass::AssignmentOperator, String::from("="))),
            ParseSymbol::Nonterminal(NonterminalLabel::Expression)
        ));
        m.insert(15, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("class"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::OptionalInheritanceList),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenCurlyBrace, String::from("{"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableThenFunctionDeclarationRecursion),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseCurlyBrace, String::from("}"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(16, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::ClassDeclaration),
            ParseSymbol::Nonterminal(NonterminalLabel::ClassDeclarationRecursion)
        ));
        m.insert(17, vec!(ParseSymbol::Epsilon));
        m.insert(18, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpression),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticOrRelationalExpression)
        ));
        m.insert(18, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpression),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticOrRelationalExpression)
        ));
        m.insert(19, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpression),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")")))
        ));
        m.insert(20, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionCallOrVariable)
        ));
        m.insert(21, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::NegationOperator),
            ParseSymbol::Nonterminal(NonterminalLabel::Factor)
        ));
        m.insert(22, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::NumberSign),
            ParseSymbol::Nonterminal(NonterminalLabel::Factor)
        ));
        m.insert(23, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Float, String::from("1.0")))
        ));
        m.insert(24, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Integer, String::from("1")))
        ));
        m.insert(25, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Expression),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionArgumentsTailRecursion)
        ));
        m.insert(26, vec!(ParseSymbol::Epsilon));
        m.insert(27, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Comma, String::from(","))),
            ParseSymbol::Nonterminal(NonterminalLabel::Expression)
        ));
        m.insert(28, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionArgumentsTail),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionArgumentsTailRecursion)
        ));
        m.insert(29, vec!(ParseSymbol::Epsilon));
        m.insert(30, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenCurlyBrace, String::from("{"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableDeclarationRecursionThenStatementRecursionA),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseCurlyBrace, String::from("}")))
        ));
        m.insert(31, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionCallOrVariableTail)
        ));
        m.insert(32, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionCallParensOrIndexing),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionCallOrVariableTailRecursion)
        ));
        m.insert(33, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::AccessorOperator, String::from("."))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionCallOrVariableTail)
        ));
        m.insert(34, vec!(ParseSymbol::Epsilon));
        m.insert(35, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionArguments),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")")))
        ));
        m.insert(36, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::IndexingRecursion),
        ));
        m.insert(37, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Type),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionDeclarationRecursionTail),
        ));
        m.insert(38, vec!(ParseSymbol::Epsilon));
        m.insert(39, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionParameters),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionDeclarationRecursionStart),
        ));
        m.insert(40, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionHeader),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionBody),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(41, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionDefinition),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionDefinitionRecursion)
        ));
        m.insert(42, vec!(ParseSymbol::Epsilon));
        m.insert(43, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Type),
            ParseSymbol::Nonterminal(NonterminalLabel::OptionalNamespacing),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionParameters),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
        ));
        m.insert(44, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Type),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::ArraySizeRecursion),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionParametersTailRecursion)
        ));
        m.insert(45, vec!(ParseSymbol::Epsilon));
        m.insert(46, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Comma, String::from(","))),
            ParseSymbol::Nonterminal(NonterminalLabel::Type),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::ArraySizeRecursion)
        ));
        m.insert(47, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionParametersTail),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionParametersTailRecursion)
        ));
        m.insert(48, vec!(ParseSymbol::Epsilon));
        m.insert(49, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Comma, String::from(","))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::IdListRecursion)
        ));
        m.insert(50, vec!(ParseSymbol::Epsilon));
        m.insert(51, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenSquareBracket, String::from("["))),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpression),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseSquareBracket, String::from("]")))
        ));
        m.insert(52, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Indexing),
            ParseSymbol::Nonterminal(NonterminalLabel::IndexingRecursion)
        ));
        m.insert(53, vec!(ParseSymbol::Epsilon));
        m.insert(54, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("&&"))),
        ));
        m.insert(55, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("*"))),
        ));
        m.insert(56, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("/"))),
        ));
        m.insert(57, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::BinaryLogicalOperator, String::from("and"))),
        ));
        m.insert(58, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::UnaryLogicalOperator, String::from("!"))),
        ));
        m.insert(59, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::UnaryLogicalOperator, String::from("not"))),
        ));
        m.insert(60, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("+"))),
        ));
        m.insert(61, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::MathOperator, String::from("-"))),
        ));
        m.insert(62, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("float"))),
        ));
        m.insert(63, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("int"))),
        ));
        m.insert(64, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::InheritanceOperator, String::from(":"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::IdListRecursion)
        ));
        m.insert(65, vec!(ParseSymbol::Epsilon));
        m.insert(66, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::OptionalNamespacingTail)
        ));
        m.insert(67, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::ScopeResolutionOperator, String::from("::"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
        ));
        m.insert(68, vec!(ParseSymbol::Epsilon));
        m.insert(69, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpression),
            ParseSymbol::Nonterminal(NonterminalLabel::RelationalOperator),
            ParseSymbol::Nonterminal(NonterminalLabel::ArithmeticExpression),
        ));
        m.insert(70, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("<>")))
        ));
        m.insert(71, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("<")))
        ));
        m.insert(72, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("<=")))
        ));
        m.insert(73, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from("==")))
        ));
        m.insert(74, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from(">")))
        ));
        m.insert(75, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::RelationalOperator, String::from(">=")))
        ));
        m.insert(76, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::AssignmentStatement),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(77, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::StatementWithoutAssignment),
        ));
        m.insert(78, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Statement),
        ));
        m.insert(79, vec!(ParseSymbol::Epsilon));
        m.insert(80, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenCurlyBrace, String::from("{"))),
            ParseSymbol::Nonterminal(NonterminalLabel::StatementRecursion),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseCurlyBrace, String::from("}"))),
        ));
        m.insert(81, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Statement),
            ParseSymbol::Nonterminal(NonterminalLabel::StatementRecursion)
        ));
        m.insert(82, vec!(ParseSymbol::Epsilon));
        m.insert(83, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("for"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::Type),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Terminal(Token::new(TokenClass::AssignmentOperator, String::from("="))),
            ParseSymbol::Nonterminal(NonterminalLabel::Expression),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::RelationalExpression),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::AssignmentStatement),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Nonterminal(NonterminalLabel::StatementBlock),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(84, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("get"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::Variable),

            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(85, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("if"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::Expression),

            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("then"))),
            ParseSymbol::Nonterminal(NonterminalLabel::StatementBlock),
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("else"))),
            ParseSymbol::Nonterminal(NonterminalLabel::StatementBlock),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(86, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("put"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::Expression),

            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(87, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Keyword, String::from("return"))),
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::Expression),

            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";")))
        ));
        m.insert(88, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Factor),
            ParseSymbol::Nonterminal(NonterminalLabel::TermRecursion)
        ));
        m.insert(89, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::MultiplicativeOperator),
            ParseSymbol::Nonterminal(NonterminalLabel::Factor),
            ParseSymbol::Nonterminal(NonterminalLabel::TermRecursion),
        ));
        m.insert(90, vec!(ParseSymbol::Epsilon));
        m.insert(91, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::NumberType)
        ));
        m.insert(92, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id")))
        ));
        m.insert(93, vec!(
            ParseSymbol::SemanticAction(SemanticActionType::VariableId),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableTail)
        ));
        m.insert(94, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::NumberType),

            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::ArraySizeRecursion),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableDeclarationRecursionThenStatementRecursionA)
        ));
        m.insert(95, vec!(ParseSymbol::Epsilon));
        m.insert(96, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::StatementWithoutAssignment),
            ParseSymbol::Nonterminal(NonterminalLabel::StatementRecursion)
        ));
        m.insert(97, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableDeclarationRecursionThenStatementRecursionB)
        ));
        m.insert(98, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::VariableTail),
            ParseSymbol::Terminal(Token::new(TokenClass::AssignmentOperator, String::from("="))),
            ParseSymbol::Nonterminal(NonterminalLabel::Expression),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::StatementRecursion)
        ));
        m.insert(99, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::ArraySizeRecursion),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableDeclarationRecursionThenStatementRecursionA)
        ));
        m.insert(100, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::OpenParens, String::from("("))),
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionArguments),
            ParseSymbol::Terminal(Token::new(TokenClass::CloseParens, String::from(")"))),
            ParseSymbol::Terminal(Token::new(TokenClass::AccessorOperator, String::from("."))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableTail)
        ));
        m.insert(101, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::IndexingRecursion),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableTailTail)
        ));
        m.insert(102, vec!(
            ParseSymbol::Terminal(Token::new(TokenClass::AccessorOperator, String::from("."))),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableTail)
        ));
        m.insert(103, vec!(ParseSymbol::Epsilon));
        m.insert(104, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::Type),
            ParseSymbol::Terminal(Token::new(TokenClass::Identifier, String::from("id"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableThenFunctionDeclarationRecursionTail)
        ));
        m.insert(105, vec!(ParseSymbol::Epsilon));
        m.insert(106, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::ArraySizeRecursion),
            ParseSymbol::Terminal(Token::new(TokenClass::Semicolon, String::from(";"))),
            ParseSymbol::Nonterminal(NonterminalLabel::VariableThenFunctionDeclarationRecursion)
        ));
        m.insert(107, vec!(
            ParseSymbol::Nonterminal(NonterminalLabel::FunctionDeclarationRecursionTail)
        ));
        m.insert(118, vec!(ParseSymbol::PopError));
        m.insert(119, vec!(ParseSymbol::ScanError));
        m
    };
}

pub fn parse(mut token_queue: VecDeque<Token>) -> Option<Ast> {
    let mut parsing_stack: Vec<ParseSymbol> = vec!();
    let mut semantic_stack: Vec<usize> = vec!();

    // Initial conditions
    parsing_stack.push(
        ParseSymbol::Terminal(Token::new(TokenClass::EndOfInput, String::new()))
    );
    parsing_stack.push(
        ParseSymbol::Nonterminal(NonterminalLabel::Program)
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
            ParseSymbol::SemanticAction(semantic_action_type) => {
                println!(">>> We found a semantic action! It's type is {:?}", semantic_action_type);
                SEMANTIC_ACTION_CALLBACKS_BY_TYPE[&semantic_action_type](next_input_token.clone(), &mut semantic_stack);
                parsing_stack.pop();
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
