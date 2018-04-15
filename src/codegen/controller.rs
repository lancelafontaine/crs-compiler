use ast::{Ast, GENERATED_AST};
use codegen::{ codegen_visitor, memsize_visitor};
use codegen::codegen_visitor::{MOON_DATA_CODE, MOON_EXEC_CODE};
use std::fs;

static OUTPUT_FILENAME: &'static str = "output.asm";

pub fn computer_memory_size() {
    // TODO: Refactor to prevent having to perform clone of GENERATED_AST
    let graph = GENERATED_AST.lock().unwrap().clone();

    // Perform DFS tree traversal with a visitor
    let ast_root_node_index = graph.get_most_recently_added_node_index();
    Ast::dfs(
        &graph,
        ast_root_node_index,
        &mut vec![],
        &memsize_visitor::visitor,
    );
}

pub fn generate_code() {
    // TODO: Refactor to prevent having to perform clone of GENERATED_AST
    let graph = GENERATED_AST.lock().unwrap().clone();

    {
        let mut exec_code = MOON_EXEC_CODE.lock().unwrap();
        exec_code.push_str("% create moon program entry point\n");
        exec_code.push_str("         entry\n");
        exec_code.push_str("         addi r14,r0,topaddr\n");
    }

    // Perform DFS tree traversal with a visitor
    let ast_root_node_index = graph.get_most_recently_added_node_index();
    Ast::dfs(
        &graph,
        ast_root_node_index,
        &mut vec![],
        &codegen_visitor::visitor,
    );

    let mut exec_code = MOON_EXEC_CODE.lock().unwrap().clone();
    let mut data_code = MOON_DATA_CODE.lock().unwrap().clone();
    exec_code.push_str("         hlt\n");
    let mut output_data = format!("{}\n{}", exec_code, data_code);
    if let Err(_) = fs::write(OUTPUT_FILENAME, output_data) {
        unimplemented!("Unable to write generated code to file.")
    }
}
