use ast::{Ast, SemanticActionType, GENERATED_AST};
use semantic::{
    symbol_table_visitor,
    type_checker_visitor,
    STRecordType,
    GENERATED_SYMBOL_TABLE_GRAPH
};
use std::collections::HashMap;

pub fn build_symbol_tables() {
    // TODO: Refactor to prevent having to perform clone of GENERATED_AST
    let graph = GENERATED_AST.lock().unwrap().clone();

    // Some sanity checks
    let some_ast_root_node = graph.get_most_recently_added_node();
    if let Some(ast_root_node) = some_ast_root_node {
        if ast_root_node.node_type != SemanticActionType::ProgramFamily {
            unimplemented!();
        }
    } else {
        unimplemented!();
    }

    // Perform DFS tree traversal with a visitor
    let ast_root_node_index = graph.get_most_recently_added_node_index();
    Ast::dfs(
        &graph,
        ast_root_node_index,
        &mut vec![],
        &symbol_table_visitor::visitor,
    );
}

pub fn prune_symbol_tables() {
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    // Ensure that class method definitions have 2 records
    let all_table_indices = global_table_graph.get_all_class_table_indices();
    for index in all_table_indices {
        let all_table_record_indices = global_table_graph.get_all_table_record_indices(index);
        let mut variable_count: HashMap<String, usize> = HashMap::new();
        let mut function_count: HashMap<String, usize> = HashMap::new();
        for record_index in all_table_record_indices {
            let node = global_table_graph.get_node(record_index).unwrap();
            let identifier = node.get_record_identifier().unwrap();
            match node.get_record_type() {
                STRecordType::Variable => {
                    let has_seen_variable = variable_count.contains_key(&identifier);
                    if has_seen_variable {
                        let new_count = variable_count.get(&identifier).unwrap() + 1;
                        variable_count.insert(identifier, new_count);
                    } else {
                        variable_count.insert(identifier, 1);
                    }
                },
                STRecordType::Function => {
                    let has_seen_function = function_count.contains_key(&identifier);
                    if  has_seen_function {
                        let new_count = function_count.get(&identifier).unwrap() + 1;
                        function_count.insert(identifier, new_count);
                    } else {
                        function_count.insert(identifier, 1);
                    }
                },
                _ => unimplemented!("Unexpected record type for a class declaration"),

            }
        }
        if !variable_count.values().all(|&x| x == 1) {
            unimplemented!("a data member of a class has been declared twice")
        }
        if !function_count.values().all(|&x| x == 2) {
            unimplemented!("A class method was defined without being declared")
        }

        // Now get rid of that extra function definition node
        let all_table_record_indices = global_table_graph.get_all_table_record_indices(index);
        for record_index in all_table_record_indices {
            if global_table_graph.get_node(record_index).unwrap().is_function_record() {
                if global_table_graph.get_child_node_indices(record_index).len() == 0 {
                    global_table_graph.remove_node(record_index);
                }
            }
        }
    }
}

pub fn check_types() {
    // TODO: Refactor to prevent having to perform clone of GENERATED_AST
    let graph = GENERATED_AST.lock().unwrap().clone();

    // Perform DFS tree traversal with a visitor
    let ast_root_node_index = graph.get_most_recently_added_node_index();
    Ast::dfs(
        &graph,
        ast_root_node_index,
        &mut vec![],
        &type_checker_visitor::visitor,
    );
}

