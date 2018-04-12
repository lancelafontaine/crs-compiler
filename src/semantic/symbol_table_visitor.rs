use ast::{AstNode, SemanticActionType};
use semantic::symbol_table::{STRecordType, GENERATED_SYMBOL_TABLE_GRAPH};
use std::collections::HashMap;
use std::sync::Mutex;
use util::Stack;

lazy_static! {
    pub static ref SYMBOL_TABLE_STACK: Mutex<Stack<usize>> = Mutex::new(Stack::new());
}

pub fn visitor(ast_node: &AstNode) {
    if let Some(func) = SYMBOL_TABLE_GENERATION_VISITORS_BY_NODE_TYPE.get(&ast_node.node_type) {
        visit_print_info(ast_node);
        func(ast_node);
    } else {
        visit_print_info(ast_node);
    }
}

type Visitor = fn(&AstNode);
lazy_static! {
    pub static ref SYMBOL_TABLE_GENERATION_VISITORS_BY_NODE_TYPE: HashMap<SemanticActionType, Visitor> = {
        let mut m = HashMap::new();
        m.insert(SemanticActionType::ClassId, visit_class_id as fn(&AstNode));
        m.insert(
            SemanticActionType::InheritanceClass,
            visit_inheritance_class as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::ClassDeclaration,
            visit_class_declaration as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::VariableDeclaration,
            visit_variable_declaration as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::ProgramFamily,
            visit_program_family as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::FunctionParameterType,
            visit_function_parameter as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::FunctionParameterId,
            visit_function_parameter as fn(&AstNode),
        );
        m.insert(SemanticActionType::Type, visit_type as fn(&AstNode));
        m.insert(SemanticActionType::Id, visit_id as fn(&AstNode));
        m.insert(
            SemanticActionType::ArraySize,
            visit_array_size as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::ClassMemberFunctionDeclaration,
            visit_class_member_function_declaration as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::FunctionParameterList,
            visit_function_parameter_list as fn(&AstNode),
        );
        m
    };
}

pub fn visit_program_family(_: &AstNode) {
    // Sanity check: global scope should have been been created with STGraph::new()
    let global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    assert!(0 == global_table_graph.current_table_index);
    assert!(0 == global_table_graph.get_most_recently_added_node_index());
}
pub fn visit_class_id(ast_node: &AstNode) {
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    let mut current_table_index = global_table_graph.current_table_index;
    let new_table_label = ast_node.clone().node_token.unwrap().lexeme;
    global_table_graph.set_table_identifier(current_table_index, new_table_label);
}
pub fn visit_class_declaration(_: &AstNode) {
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    global_table_graph.current_table_index = 0; // Class definitions always extend from root
    let current_table_index = global_table_graph.current_table_index;
    let new_table_index = global_table_graph.add_class_to_table(current_table_index);
    global_table_graph.enter_table_scope(new_table_index);
}
pub fn visit_inheritance_class(ast_node: &AstNode) {
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    let mut current_table_node = global_table_graph.get_current_table_mut().unwrap();
    let inheritance_class = ast_node.clone().node_token.unwrap().lexeme;
    current_table_node.add_to_table_inheritance_list(inheritance_class)
}
pub fn visit_print_info(ast_node: &AstNode) {
    println!("{:?}", ast_node);
}

pub fn visit_variable_declaration(_: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    // If there's something on the symbol table stack, pop it
    // This is the node index of the *previous* declaration node
    symbol_table_stack.pop();

    // Make a new record node in the current table/scope
    let current_table_index = global_table_graph.current_table_index;
    let new_record_index = global_table_graph
        .add_empty_record_with_type_to_table(STRecordType::Variable, current_table_index);

    // Push this new record's index onto the symbol table stack so that this record can be
    // pointed to and modified once the variable declaration's children in the AST are reached
    symbol_table_stack.push(new_record_index);
}

pub fn visit_class_member_function_declaration(_: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    // Pop the symbol table stack twice
    // Once for the previous FunctionParameterList
    // Once for the previous ClassMemberFunctionDeclaration
    symbol_table_stack.pop();
    symbol_table_stack.pop();

    // Make a new record node in the current table/scope
    let current_table_index = global_table_graph.current_table_index;
    let new_record_index = global_table_graph
        .add_empty_record_with_type_to_table(STRecordType::Function, current_table_index);

    // Push this new record's index onto the symbol table stack so that this record can be
    // pointed to and modified once the variable declaration's children in the AST are reached
    symbol_table_stack.push(new_record_index);
}

pub fn visit_function_parameter(ast_node: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    if let Some(node_index) = symbol_table_stack.top() {
        global_table_graph.ensure_node_is_record(node_index);
        let record_node = global_table_graph.get_node_mut(node_index).unwrap();
        let fragment = ast_node.clone().node_token.unwrap().lexeme;
        record_node.add_function_parameter_fragment_to_record(fragment);
    } else {
        unimplemented!()
    } // A type without any context means something went horribly wrong
}

pub fn visit_type(ast_node: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    if let Some(node_index) = symbol_table_stack.top() {
        global_table_graph.ensure_node_is_record(node_index);
        let record_node = global_table_graph.get_node_mut(node_index).unwrap();
        let value_type = ast_node.clone().node_token.unwrap().lexeme;
        record_node.set_record_value_type(value_type);
    } else {
        unimplemented!()
    } // A type without any context means something went horribly wrong
}

pub fn visit_id(ast_node: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    if let Some(node_index) = symbol_table_stack.top() {
        global_table_graph.ensure_node_is_record(node_index);
        let record_node = global_table_graph.get_node_mut(node_index).unwrap();
        let identifier = ast_node.clone().node_token.unwrap().lexeme;
        record_node.set_record_identifier(identifier);
    } else {
        unimplemented!()
    } // An identifier without any context means something went horribly wrong
}

pub fn visit_array_size(ast_node: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    if let Some(node_index) = symbol_table_stack.top() {
        global_table_graph.ensure_node_is_record(node_index);
        let record_node = global_table_graph.get_node_mut(node_index).unwrap();
        if symbol_table_stack.len() > 1 {
            let fragment = ast_node.clone().node_token.unwrap().lexeme;
            record_node.add_function_parameter_fragment_to_record(fragment);
        } else {
            record_node.increment_record_array_size();
        }
    } else {
        unimplemented!()
    } // An array size without any context means something went horribly wrong
}

pub fn visit_function_parameter_list(_: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    if let Some(node_index) = symbol_table_stack.top() {
        // Push a symbol onto the symbol table stack as an indication
        // for other visitors that we are now parsing function parameters
        symbol_table_stack.push(node_index);

        let record_node = global_table_graph.get_node_mut(node_index).unwrap();
        record_node.initialize_record_function_parameters()
    } else {
        unimplemented!("There was a function parameter list without any sort of declaration?")
    }
}
