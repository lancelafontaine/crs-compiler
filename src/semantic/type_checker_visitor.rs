use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;
use ast::{AstNode, SemanticActionType };
use output::{ error, write_to_symbol_table_log };
use util::Stack;
use semantic::symbol_table::GENERATED_SYMBOL_TABLE_GRAPH;

type Visitor = fn(&AstNode);
lazy_static! {
    pub static ref FACTOR_VARIABLE_DETECTOR: Mutex<Stack<usize>> = Mutex::new(Stack::new());
    pub static ref DATA_MEMBER_COUNTER: Mutex<Stack<usize>> = Mutex::new(Stack::new());
    pub static ref ID_QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
    pub static ref TYPE_CHECKER_VISITORS_BY_NODE_TYPE: HashMap<SemanticActionType, Visitor> = {
        let mut m = HashMap::new();
        m.insert(SemanticActionType::FactorVariable, visit_factor_variable as fn(&AstNode));
        m.insert(SemanticActionType::DataMember, visit_data_member as fn(&AstNode));
        m.insert(SemanticActionType::Id, visit_id as fn(&AstNode));
        m.insert(SemanticActionType::FunctionArguments, visit_function_arguments as fn(&AstNode));
        m
    };
}

pub fn visitor(ast_node: &AstNode) {
    if let Some(func) = TYPE_CHECKER_VISITORS_BY_NODE_TYPE.get(&ast_node.node_type) {
        visit_print_info(ast_node);
        func(ast_node);
    } else {
        no_visit_print_info(ast_node);
    }
}
pub fn visit_print_info(ast_node: &AstNode) {
    println!("{:?}", ast_node);
     write_to_symbol_table_log(format!("Visiting {:?}", ast_node));
}

pub fn no_visit_print_info(ast_node: &AstNode) {
    println!("{:?}", ast_node);
     write_to_symbol_table_log(format!("Ignoring {:?}", ast_node));
}
pub fn visit_factor_variable(ast_node: &AstNode) {
    let mut factor_variable_detector = FACTOR_VARIABLE_DETECTOR.lock().unwrap();
    let mut data_member_counter = DATA_MEMBER_COUNTER.lock().unwrap();
    // Empty all stacks
    while let Some(_) = factor_variable_detector.pop() {}
    while let Some(_) = data_member_counter.pop() {}
    // Indicator that we are now parsing a factor variable
    factor_variable_detector.push(0);
}

pub fn visit_data_member(ast_node: &AstNode) {
    let mut data_member_counter = DATA_MEMBER_COUNTER.lock().unwrap();
    data_member_counter.push(0);
}

pub fn visit_id(ast_node: &AstNode) {
    let mut data_member_counter = DATA_MEMBER_COUNTER.lock().unwrap();
    let mut id_queue = ID_QUEUE.lock().unwrap();

    let identifier = ast_node.clone().node_token.unwrap().lexeme;
    id_queue.push_back(identifier);
    if data_member_counter.len() > 1 {
        // Use of an accessor operator
        let symbol_table = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
        let all_record_indices = symbol_table.get_child_node_indices(0);
        for index in all_record_indices {
            let node = symbol_table.get_node(index).unwrap();
            if !node.is_variable_record() { continue }
            let node_identifier = node.get_record_identifier().unwrap();
            if &node_identifier == id_queue.get(0).unwrap() {
                // The identifier that the accessor was used on is a declared variable
                let possible_class_identifier = node.get_record_value_type().unwrap();
                if let None = symbol_table.get_class_table_index_by_identifier(possible_class_identifier) {
                    error(23);
                    unimplemented!();
                }
            }
        }
    }
}

pub fn visit_function_arguments(ast_node: &AstNode) {
    let symbol_table = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    let mut id_queue = ID_QUEUE.lock().unwrap();

    let possible_function_identifier = id_queue.get(id_queue.len()-1).unwrap();
    let mut all_table_indices = symbol_table.get_all_class_table_indices();
    all_table_indices.push(0);
    for table_index in all_table_indices {
        let record_indices = symbol_table.get_child_node_indices(table_index);
        for record_index in record_indices {
            let node = symbol_table.get_node(record_index).unwrap();
            if !node.is_function_record() { continue }
            let declared_function_identifier = node.get_record_identifier().unwrap();
            if &declared_function_identifier == possible_function_identifier {
                return
            }
        }
    }
    error(24);
    unimplemented!();
}
