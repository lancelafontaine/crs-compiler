use std::collections::HashMap;
use ast::{ SemanticActionType, AstNode };

pub fn visitor(node_index: usize, node: &AstNode) {
    if let Some(func) = SYMBOL_TABLE_GENERATION_VISITORS_BY_NODE_TYPE.get(&node.node_type) {
        func(node_index, node);
    };
}

type Visitor = fn(usize, &AstNode);
lazy_static! {
    pub static ref SYMBOL_TABLE_GENERATION_VISITORS_BY_NODE_TYPE: HashMap<SemanticActionType, Visitor> = {
        let mut m = HashMap::new();
        m.insert(SemanticActionType::ClassId, visit_class_id as fn(usize, &AstNode));
        m
    };
}

pub fn visit_class_id(node_index: usize, node: &AstNode) {
        println!("node_index: {}", node_index);
        println!("{:?}", node);
}

