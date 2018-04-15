use ast::{AstNode, SemanticActionType};
use semantic::symbol_table::{STRecordType, GENERATED_SYMBOL_TABLE_GRAPH};
use std::collections::HashMap;
use std::sync::Mutex;
use util::Stack;

lazy_static! {
    pub static ref SYMBOL_TABLE_STACK: Mutex<Stack<usize>> = Mutex::new(Stack::new());
    pub static ref PROGRAM_MAIN_DETECTOR: Mutex<Stack<usize>> = Mutex::new(Stack::new());
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
            SemanticActionType::FunctionDefinitionList,
            visit_function_definition_list as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::FunctionDefinition,
            visit_function_definition as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::VariableDeclaration,
            visit_variable_declaration as fn(&AstNode),
        );
        m.insert(
            SemanticActionType::StatementBlock,
            visit_statement_block as fn(&AstNode),
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
pub fn visit_function_definition_list(_: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    // Pop the symbol table stack twice
    // Once for the previous FunctionParameterList
    // Once for the previous ClassMemberFunctionDeclaration
    symbol_table_stack.pop();
    symbol_table_stack.pop();
    global_table_graph.current_table_index = 0;
}
pub fn visit_statement_block(_: &AstNode) {
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    program_main_detector.push(0);
    symbol_table_stack.pop();
    symbol_table_stack.pop();
}
pub fn visit_function_definition(_: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();

    // Pop the symbol table stack once for the previous FunctionDefinition
    // Once for the previous FunctionParameterList
    symbol_table_stack.pop();
    symbol_table_stack.pop();
    // Function definitions are always at the root
    global_table_graph.current_table_index = 0;
    let current_table_index = global_table_graph.current_table_index;

    let new_function_index = global_table_graph.add_table_to_table(STRecordType::Function, current_table_index);
    global_table_graph.enter_table_scope(new_function_index);

    // Push this new record's index onto the symbol table stack so that this record can be
    // pointed to and modified once the variable declaration's children in the AST are reached
    symbol_table_stack.push(new_function_index);
    program_main_detector.pop();
}
pub fn visit_class_declaration(_: &AstNode) {
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    // Class declaration are always at the root
    global_table_graph.current_table_index = 0;
    let current_table_index = global_table_graph.current_table_index;
    let new_table_index = global_table_graph.add_table_to_table(STRecordType::Class, current_table_index);
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
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    // If there's something on the symbol table stack, pop it
    // This is the node index of the *previous* declaration node
    symbol_table_stack.pop();

    if program_main_detector.len() > 1 {
        // this is the program's main function
        global_table_graph.current_table_index = 0;
    }

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

    let mut node_index: usize;
    let fragment = ast_node.clone().node_token.unwrap().lexeme;

    if let Some(index) = symbol_table_stack.top() {
        node_index = index;
    } else {
        unimplemented!("An identifier without any context means something went horribly wrong")
    }

    if global_table_graph.is_node_record(node_index) {
        let node = global_table_graph.get_node_mut(node_index).unwrap();
        node.add_function_parameter_fragment_to_record(fragment);
    } else {
        global_table_graph.add_function_parameter_fragment_to_table(node_index, fragment);
    }
}

pub fn visit_type(ast_node: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    let mut node_index: usize;
    let value_type = ast_node.clone().node_token.unwrap().lexeme;

    if let Some(index) = symbol_table_stack.top() {
        node_index = index;
    } else {
        unimplemented!("An identifier without any context means something went horribly wrong")
    }

    if global_table_graph.is_node_record(node_index) {
        // variable/function declaration
        let node = global_table_graph.get_node_mut(node_index).unwrap();
        node.set_record_value_type(value_type);
    } else {
        // function definition
        global_table_graph.set_table_value_type(node_index, value_type);
    }
}

pub fn visit_id(ast_node: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    let mut node_index: usize;
    let identifier = ast_node.clone().node_token.unwrap().lexeme;

    if let Some(index) = symbol_table_stack.top() {
        node_index = index;
    } else {
        unimplemented!("An identifier without any context means something went horribly wrong")
    }

    if global_table_graph.is_node_record(node_index) {
        // variable/function declaration
        let node = global_table_graph.get_node_mut(node_index).unwrap();
        if node.get_record_identifier().is_some() {
            // Previously set identifier was actually a class type!
            let class_type = node.get_record_identifier().unwrap();
            node.set_record_value_type(class_type);
        }
        node.set_record_identifier(identifier);
    } else {
        // class declaration or function definition
        let some_table_identifier = global_table_graph.get_table_identifier(node_index);
        if let Some(table_identifier) = some_table_identifier {
            // this is a class method
            let all_class_identifiers = global_table_graph.get_all_class_identifiers();
            if all_class_identifiers.contains(&table_identifier) {
                let some_table_index = global_table_graph.get_class_table_index_by_identifier(table_identifier);
                if let Some(index_of_table_to_move_to) = some_table_index {
                    let index_of_table_to_be_moved = node_index;
                    global_table_graph.move_table_scope(
                        index_of_table_to_be_moved,
                        index_of_table_to_move_to
                    );
                    global_table_graph.set_table_identifier(node_index, identifier);
                } else {
                    unimplemented!("Class method defined for a class that does not exist.")
                }
            } else {
                unimplemented!("Class method defined for a class that does not exist.")
            }
        } else {
            // this is a free function
            global_table_graph.set_table_identifier(node_index, identifier);
        }
    }
}

pub fn visit_array_size(ast_node: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    let mut node_index: usize;
    let fragment = ast_node.clone().node_token.unwrap().lexeme;

    if let Some(index) = symbol_table_stack.top() {
        node_index = index;
    } else {
        unimplemented!("An array size without any context means something went horribly wrong")
    }
    if global_table_graph.is_node_record(node_index) {
        let record_node = global_table_graph.get_node_mut(node_index).unwrap();
        if symbol_table_stack.len() > 1 {
            record_node.add_function_parameter_fragment_to_record(fragment);
        } else {
            if let Ok(array_dimension) = fragment.parse::<usize>() {
                record_node.add_record_array_dimension(array_dimension);
            }
        }
    } else {
        // Free Function Definition
        global_table_graph.add_function_parameter_fragment_to_table(node_index, fragment);
    }
}

pub fn visit_function_parameter_list(_: &AstNode) {
    let mut symbol_table_stack = SYMBOL_TABLE_STACK.lock().unwrap();
    let mut global_table_graph = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();

    let mut node_index: usize;
    if let Some(index) = symbol_table_stack.top() {
        node_index = index;
    } else {
        unimplemented!("An identifier without any context means something went horribly wrong")
    }

    // Push a symbol onto the symbol table stack as an indication
    // for other visitors that we are now parsing function parameters
    symbol_table_stack.push(node_index);
    if global_table_graph.is_node_record(node_index) {
        // ClassMemberFunctionDeclaration
        let node = global_table_graph.get_node_mut(node_index).unwrap();
        node.initialize_record_function_parameters()
    } else {
        // free function table
        global_table_graph.initialize_table_function_parameters(node_index)
    }
}
