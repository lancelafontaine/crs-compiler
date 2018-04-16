use ast::{AstNode, SemanticActionType};
use std::sync::Mutex;
use util::Stack;
use std::collections::HashMap;
use semantic::{ STRecord, STNodeType, STRecordType, GENERATED_SYMBOL_TABLE_GRAPH };

lazy_static! {
    pub static ref MOON_EXEC_CODE: Mutex<String> = Mutex::new(String::new());
    pub static ref MOON_DATA_CODE: Mutex<String> = Mutex::new(String::new());
    pub static ref PROGRAM_MAIN_DETECTOR: Mutex<Stack<usize>> = Mutex::new(Stack::new());
    pub static ref CODEGEN_RECORD_STACK: Mutex<Stack<STRecord>> = Mutex::new(Stack::new());
}

pub fn visitor(ast_node: &AstNode) {
    if let Some(func) = CODEGEN_VISITORS_BY_NODE_TYPE.get(&ast_node.node_type) {
        visit_print_info(ast_node);
        func(ast_node);
    } else {
        visit_print_info(ast_node);
    }
}

type Visitor = fn(&AstNode);
lazy_static! {
    pub static ref CODEGEN_VISITORS_BY_NODE_TYPE: HashMap<SemanticActionType, Visitor> = {
        let mut m = HashMap::new();
        m.insert(SemanticActionType::VariableDeclaration, visit_variable_declaration as fn(&AstNode));
        m.insert(SemanticActionType::FunctionDefinition, visit_function_definition as fn(&AstNode));
        m.insert(SemanticActionType::StatementBlock, visit_statement_block as fn(&AstNode));
        m.insert(SemanticActionType::Type, visit_type as fn(&AstNode));
        m.insert(SemanticActionType::Id, visit_id as fn(&AstNode));
        m.insert(SemanticActionType::ArraySize, visit_array_size as fn(&AstNode));
        m
    };
}

pub fn visit_print_info(ast_node: &AstNode) {
     //println!("{:?}", ast_node)
}

pub fn visit_variable_declaration(_: &AstNode) {
    let mut codegen_record_stack = CODEGEN_RECORD_STACK.lock().unwrap();
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    let mut symbol_table = GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap();
    if program_main_detector.len() > 0 {
        // this is the program's main function
        let some_record = codegen_record_stack.pop();
        if some_record.is_none() { return; }
        let built_record = some_record.unwrap();

        // Match this built record against one that we
        // previously calculated the memory size for
        let all_record_indices = symbol_table.get_all_table_record_indices(0);
        for index in all_record_indices {
            let node = symbol_table.get_node_mut(index).unwrap();
            if node.is_variable_record() {
                let existing_record: STRecord;
                match node.node_type {
                    STNodeType::Record(ref record) => {
                        existing_record = record.clone().unwrap().clone();
                    },
                    _ => { continue }
                }
                if built_record.get_record_value_type() == existing_record.get_record_value_type() &&
                        built_record.get_identifier() == existing_record.get_identifier() &&
                        built_record.get_array_dimensions() == existing_record.get_array_dimensions() {
                    let memory_size = existing_record.get_memory_size().unwrap();
                    // In the future, this id should consider the scope in which it was called
                    // to avoid names collisions for reserved memory
                    let id = existing_record.get_identifier().unwrap();
                    let mut data_code = MOON_DATA_CODE.lock().unwrap();
                    node.set_record_memory_tag(id.clone());
                    let memory_allocation_string = &format!("{}{}res {}\n", &id, " ".repeat(20 - id.len()), memory_size);
                    data_code.push_str(&format!("% space for variable {}\n", id));
                    data_code.push_str(memory_allocation_string);
                }
            }
        }
    }
}

pub fn visit_function_definition(_: &AstNode) {
    // Required for detecting when the program's main function is reached
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    program_main_detector.pop();
}

pub fn visit_statement_block(_: &AstNode) {
    // Required for detecting when the program's main function is reached
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    program_main_detector.push(0);
}

pub fn visit_type(ast_node: &AstNode) {
    let mut codegen_record_stack = CODEGEN_RECORD_STACK.lock().unwrap();
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    if program_main_detector.len() > 0 {
        // variable declaration
        let value_type = ast_node.clone().node_token.unwrap().lexeme;
        let mut record = STRecord::new_empty_record(STRecordType::Variable);
        record.set_value_type(value_type);
        codegen_record_stack.push(record);
    }
}

pub fn visit_id(ast_node: &AstNode) {
    let mut codegen_record_stack = CODEGEN_RECORD_STACK.lock().unwrap();
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    if program_main_detector.len() > 0 {
        // variable declaration
        let lexeme = ast_node.clone().node_token.unwrap().lexeme;
        if let Some(_) = codegen_record_stack.top() {
            let mut record = codegen_record_stack.pop().unwrap();
            record.set_identifier(lexeme);
            codegen_record_stack.push(record);
        } else {
            let mut record = STRecord::new_empty_record(STRecordType::Variable);
            record.set_value_type(lexeme);
            codegen_record_stack.push(record);
        }
    }
}

pub fn visit_array_size(ast_node: &AstNode) {
    let mut codegen_record_stack = CODEGEN_RECORD_STACK.lock().unwrap();
    let mut program_main_detector = PROGRAM_MAIN_DETECTOR.lock().unwrap();
    if program_main_detector.len() > 0 {
        // variable declaration
        let mut array_dimension: usize;
        if let Ok(dimension) = ast_node.clone().node_token.unwrap().lexeme.parse::<usize>() {
            array_dimension = dimension;
        } else {
            array_dimension = 1;
        }
        if let Some(_) = codegen_record_stack.top() {
            let mut record = codegen_record_stack.pop().unwrap();
            record.add_array_dimension(array_dimension);
            codegen_record_stack.push(record);
        }
    }
}

