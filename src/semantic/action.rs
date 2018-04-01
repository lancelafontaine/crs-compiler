use std::collections::HashMap;
use std::sync::Mutex;

use lexer::Token;
use ast::{ Ast, AstNodeType };

type Callback = fn(Token, &mut Vec<usize>);

lazy_static! {
    #[derive(Debug)]
    pub static ref GENERATED_AST: Mutex<Ast> = Mutex::new(Ast::new());

    pub static ref SEMANTIC_ACTION_CALLBACKS_BY_TYPE: HashMap<SemanticActionType, Callback> = {
        let mut m = HashMap::new();
        m.insert(SemanticActionType::VariableId, semantic_action_variable_id as fn(Token, &mut Vec<usize>));
        m
    };
}

fn semantic_action_variable_id(token: Token, semantic_stack: &mut Vec<usize>) {
    let node_index = GENERATED_AST.lock().unwrap().make_node(AstNodeType::Identifier, token);
    semantic_stack.push(node_index);
    println!(">> semantic stack: {:?}", semantic_stack);
    println!(">> node in AST: {:?}", GENERATED_AST.lock().unwrap().get_node(node_index));
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum SemanticActionType {
    VariableId
}
