use std::collections::{ HashSet, HashMap };
use ast::GENERATED_AST;
use semantic::{ STNode, GENERATED_SYMBOL_TABLE_GRAPH };
use output::error;

pub fn check_double_declarations(){
    let symbol_table = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    let mut all_table_indices = symbol_table.get_all_table_indices();
    all_table_indices.push(0);
    for table_index in all_table_indices {
        let mut existing_nodes: HashSet<STNode> = HashSet::new();
        let all_record_indices = symbol_table.get_all_table_record_indices(table_index);
        for record_index in all_record_indices {
            let node = symbol_table.get_node(record_index).unwrap();
            if !existing_nodes.contains(node) {
                existing_nodes.insert(node.clone());
            } else {
                error(21);
                unimplemented!();
            }
        }
    }
}

pub fn check_circular_class_dependencies() {
    let symbol_table = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    let mut class_graph: HashMap<String, Vec<String>> = HashMap::new();

    let mut all_record_indices = symbol_table.get_all_table_record_indices(0);
    for index in all_record_indices {
        let node = symbol_table.get_node(index).unwrap();
        if !node.is_class_record() { continue }
        let class_identifier = node.get_record_identifier().unwrap();
        let mut class_table_index: usize;
        if let  Some(i) = symbol_table.get_child_node_index(index) {
            class_table_index = i;
        } else {
            error(20);
            unimplemented!();
        }
        let table_node = symbol_table.get_node(class_table_index).unwrap();
        let mut inheritance_list = table_node.get_table_inheritance_list();

        let mut class_declaration_indices = symbol_table.get_child_node_indices(class_table_index);
        let mut class_declaration_identifiers = vec![];
        for record_index in class_declaration_indices {
            let node = symbol_table.get_node(record_index).unwrap();
            if !node.is_variable_record() { continue }
            let node_identifier = node.get_record_value_type().unwrap();
            if let Some(_)= symbol_table.get_class_table_index_by_identifier(node_identifier.clone()) {
                class_declaration_identifiers.push(node_identifier);
            }
        }
        inheritance_list.append(&mut class_declaration_identifiers);
        class_graph.insert(class_identifier, inheritance_list);
    }

    // For every class, perform DFS to see if a cycle exists
    for node in class_graph.keys() {
        let mut visited = vec![];
        let has_cycle = has_cycle_via_dfs(&class_graph, node, &mut visited);
        if has_cycle {
            error(22);
            unimplemented!();
        }
    }
}

fn has_cycle_via_dfs(class_graph: &HashMap<String, Vec<String>>, node: &String, visited: &mut Vec<String>) -> bool{
    if !visited.contains(&node) {
        visited.push(node.clone());
        for child_node in class_graph.get(node).unwrap() {
            return has_cycle_via_dfs(class_graph, child_node, visited)
        }
        return false;
    }
    return true;
}
