use std::collections::HashMap;
use lexer::Token;
use ast::GENERATED_AST;
use util::Stack;

type Callback = fn(SemanticActionType, Token, &mut Stack<usize>);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum SemanticActionType {
    VariableId,
    ProgramFamily,
    InheritanceList,
    InheritanceClass,
    ClassDeclarationList,
    ClassMemberList,
    ClassMember,
    ClassDeclaration,
    ClassId,
    FunctionDefinitionList,
    ProgramMainFunction,
}

lazy_static! {
    pub static ref SEMANTIC_ACTION_CALLBACKS_BY_TYPE: HashMap<SemanticActionType, Callback> = {
        let mut m = HashMap::new();
        m.insert(
            SemanticActionType::VariableId,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ClassId,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::InheritanceClass,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ClassMember,
            semantic_action_class_member as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ClassMemberList,
            semantic_action_class_member_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::InheritanceList,
            semantic_action_inheritance_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ProgramFamily,
            semantic_action_program_family as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ClassDeclarationList,
            semantic_action_class_declaration_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ClassDeclaration,
            semantic_action_class_declaration as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionDefinitionList,
            semantic_action_function_definition_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ProgramMainFunction,
            semantic_action_program_main_function as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m
    };
}

fn semantic_action_generic_make_node(action_type: SemanticActionType, token: Token, semantic_stack: &mut Stack<usize>) {
    let node_index = GENERATED_AST.lock().unwrap().make_node(action_type, Some(token));
    println!(">>> Created node: {:?}", GENERATED_AST.lock().unwrap().get_node(node_index));
    semantic_stack.push(node_index);
}

fn semantic_action_generic_make_family(possible_child_node_types: Vec<SemanticActionType>, action_type: SemanticActionType, semantic_stack: &mut Stack<usize>) {
    let parent_node_index = GENERATED_AST.lock().unwrap().make_node(action_type, None);

    // For children nodes relevant to this parent node:
    // - Pop from the semantic stack
    // - Add edges from parent node to child nodes
    let mut counter = 0;
    while let Some(child_node_index) = semantic_stack.top() {
        let child_node = GENERATED_AST.lock().unwrap().get_node(child_node_index).cloned().unwrap();
        if possible_child_node_types.contains(&child_node.node_type) {
            semantic_stack.pop(); // No need to keep this, already have it
            GENERATED_AST.lock().unwrap().make_edge(parent_node_index, child_node_index, Some(counter));
            counter += 1;
        } else { break }
    }
    semantic_stack.push(parent_node_index);
}

fn semantic_action_class_declaration(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::ClassId,
        SemanticActionType::InheritanceList
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}

fn semantic_action_inheritance_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![SemanticActionType::InheritanceClass];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}

fn semantic_action_class_declaration_list(action_type: SemanticActionType, token: Token, semantic_stack: &mut Stack<usize>) {
    println!(">>> CLASS DECLARATION LIST ACTION: {:?}", semantic_stack.top());
    while let Some(node_index) = semantic_stack.top() {
        let previous_node = GENERATED_AST.lock().unwrap().get_node(node_index).cloned();
        semantic_stack.pop();
        println!(">>> Node at top of the stack is: {:?}", previous_node);
    }
}

fn semantic_action_program_family(action_type: SemanticActionType, token: Token, semantic_stack: &mut Stack<usize>) {}
fn semantic_action_function_definition_list(action_type: SemanticActionType, token: Token, semantic_stack: &mut Stack<usize>) {}
fn semantic_action_class_member(action_type: SemanticActionType, token: Token, semantic_stack: &mut Stack<usize>) {}
fn semantic_action_class_member_list(action_type: SemanticActionType, token: Token, semantic_stack: &mut Stack<usize>) {}
fn semantic_action_program_main_function(action_type: SemanticActionType, token: Token, semantic_stack: &mut Stack<usize>) {}
