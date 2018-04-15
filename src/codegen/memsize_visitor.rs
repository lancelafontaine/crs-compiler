use ast::{AstNode, SemanticActionType};
use std::sync::Mutex;
use util::Stack;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROGRAM_MAIN_DETECTOR: Mutex<Stack<usize>> = Mutex::new(Stack::new());
}

pub fn visitor(ast_node: &AstNode) {
    //if let Some(func) = CODEGEN_VISITORS_BY_NODE_TYPE.get(&ast_node.node_type) {
        //visit_print_info(ast_node);
        //func(ast_node);
    //} else {
        visit_print_info(ast_node);
    //}
}

//type Visitor = fn(&AstNode);
//lazy_static! {
    //pub static ref CODEGEN_VISITORS_BY_NODE_TYPE: HashMap<SemanticActionType, Visitor> = {
        //let mut m = HashMap::new();
        //m.insert(SemanticActionType::VariableDeclaration, visit_variable_declaration as fn(&AstNode));
        //m.insert(SemanticActionType::FunctionDefinition, visit_function_definition as fn(&AstNode));
        //m.insert(SemanticActionType::StatementBlock, visit_statement_block as fn(&AstNode));
        //m
    //};
//}

pub fn visit_print_info(ast_node: &AstNode) {
     println!("{:?}", ast_node)
}

//pub fn visit_variable_declaration(_: &AstNode) {
    //let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    //if program_main_detector.len() > 1 {
        //// this is the program's main function
        //println!("CODEGEN: PROGRAM_MAIN_DETECTOR {:?}", program_main_detector.clone());
    //}
//}

//pub fn visit_function_definition(_: &AstNode) {
    //// Required for detecting when the program's main function is reached
    //let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    //program_main_detector.pop();
//}

//pub fn visit_statement_block(_: &AstNode) {
    //// Required for detecting when the program's main function is reached
    //let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    //program_main_detector.push(0);
//}
