use ast::{Ast, AstNode, GENERATED_AST, SemanticActionType };
use semantic::{ GENERATED_SYMBOL_TABLE_GRAPH };
use codegen::{ codegen_visitor, memsize_visitor};
use codegen::codegen_visitor::{MOON_DATA_CODE, MOON_EXEC_CODE};
use std::fs;

static OUTPUT_FILENAME: &'static str = "output.asm";

pub fn compute_memory_size() {
    let mut symbol_table = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    let calculated_sizes = symbol_table.calculate_table_memory_sizes(0);
    symbol_table.set_table_memory_sizes(0, calculated_sizes);
}

pub fn generate_code() {
    // TODO: Refactor to prevent having to perform clone of GENERATED_AST
    let graph = GENERATED_AST.lock().unwrap().clone();

    {
        let mut exec_code = MOON_EXEC_CODE.lock().unwrap();
        exec_code.push_str("% create moon program entry point\n");
        exec_code.push_str("                    entry\n");
        exec_code.push_str("                    addi r14,r0,topaddr\n");
    }

    // Perform DFS tree traversal with a visitor
    let ast_root_node_index = graph.get_most_recently_added_node_index();
    Ast::dfs(
        &graph,
        ast_root_node_index,
        &mut vec![],
        &codegen_visitor::visitor,
    );
    // Do one last variable declaration invocation
    // TODO Consider refactoring into a post-order DFS traversal for code generation
    codegen_visitor::visit_variable_declaration(&AstNode::new(SemanticActionType::ProgramFamily, None));

    let mut exec_code = MOON_EXEC_CODE.lock().unwrap().clone();
    let mut data_code = MOON_DATA_CODE.lock().unwrap().clone();
    exec_code.push_str("                    hlt\n");
    let mut output_data = format!("{}\n{}", exec_code, data_code);
    if let Err(_) = fs::write(OUTPUT_FILENAME, output_data) {
        unimplemented!("Unable to write generated code to file.")
    }
}
