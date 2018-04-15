use ast::{AstNode};

pub fn visitor(ast_node: &AstNode) {
    //if let Some(func) = SYMBOL_TABLE_GENERATION_VISITORS_BY_NODE_TYPE.get(&ast_node.node_type) {
        //visit_print_info(ast_node);
        //func(ast_node);
    //} else {
        visit_print_info(ast_node);
    //}
}
pub fn visit_print_info(ast_node: &AstNode) {
     println!("{:?}", ast_node);
}

