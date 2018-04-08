use std::collections::HashMap;
use lexer::Token;
use ast::GENERATED_AST;
use util::Stack;

type Callback = fn(SemanticActionType, Token, &mut Stack<usize>);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum SemanticActionType {
    ProgramFamily,
    InheritanceList,
    InheritanceClass,
    ClassDeclarationList,
    ClassMemberDeclarationList,
    VariableDeclaration,
    FactorVariable,
    Variable,
    DataMember,
    FactorFunctionCall,
    FunctionCall,
    FunctionArguments,
    Expression,
    RelationalExpression,
    RelationalOperator,
    NegationOperator,
    Float,
    Integer,
    NumberSign,
    ArithmeticExpression,
    AdditiveOperator,
    MultiplicativeOperator,
    Term,
    Factor,
    FunctionParameterDeclaration,
    ClassMemberFunctionDeclaration,
    Type,
    Id,
    FunctionParameterType,
    FunctionParameterId,
    ArraySize,
    ArraySizeList,
    ArrayIndexingList,
    ClassDeclaration,
    ClassId,
    FunctionParameterList,
    FunctionDefinition,
    FunctionDefinitionList,
    StatementBlock,
    IfStatement,
    AssignmentStatement,
    ForLoopStatement,
    GetStatement,
    PutStatement,
    ReturnStatement,
}

lazy_static! {
    pub static ref SEMANTIC_ACTION_CALLBACKS_BY_TYPE: HashMap<SemanticActionType, Callback> = {
        let mut m = HashMap::new();
        m.insert(
            SemanticActionType::ClassId,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::InheritanceClass,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionParameterType,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionParameterId,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Type,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Id,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ArraySize,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::NumberSign,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Float,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Integer,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::NegationOperator,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::MultiplicativeOperator,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::AdditiveOperator,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::RelationalOperator,
            semantic_action_generic_make_node as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Factor,
            semantic_action_factor as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Term,
            semantic_action_term as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ClassMemberDeclarationList,
            semantic_action_class_member_declaration_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionParameterDeclaration,
            semantic_action_function_parameter_declaration as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::VariableDeclaration,
            semantic_action_variable_declaration as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ClassMemberFunctionDeclaration,
            semantic_action_class_member_function_declaration as fn(SemanticActionType, Token, &mut Stack<usize>)
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
            SemanticActionType::FunctionDefinition,
            semantic_action_function_definition as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionDefinitionList,
            semantic_action_function_definition_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionParameterList,
            semantic_action_function_parameter_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::StatementBlock,
            semantic_action_statement_block as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::IfStatement,
            semantic_action_if_statement as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::AssignmentStatement,
            semantic_action_assignment_statement as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ForLoopStatement,
            semantic_action_for_loop_statement as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::GetStatement,
            semantic_action_get_statement as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::PutStatement,
            semantic_action_put_statement as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::DataMember,
            semantic_action_data_member as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionArguments,
            semantic_action_function_arguments as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FactorFunctionCall,
            semantic_action_factor_function_call as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FunctionCall,
            semantic_action_function_call as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::FactorVariable,
            semantic_action_factor_variable as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Variable,
            semantic_action_variable as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::Expression,
            semantic_action_expression as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::RelationalExpression,
            semantic_action_relational_expression as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ArithmeticExpression,
            semantic_action_arithmetic_expression as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ReturnStatement,
            semantic_action_return_statement as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ArrayIndexingList,
            semantic_action_array_indexing_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m.insert(
            SemanticActionType::ArraySizeList,
            semantic_action_array_size_list as fn(SemanticActionType, Token, &mut Stack<usize>)
        );
        m
    };
}

////////////////////////////////////////////
// Generic functions for semantic actions //
// + AST leaf nodes                       //
////////////////////////////////////////////

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

////////////////////////////
// AST intermediate nodes //
////////////////////////////

fn semantic_action_class_declaration(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::ClassId,
        SemanticActionType::InheritanceList,
        SemanticActionType::ClassMemberDeclarationList
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_inheritance_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![SemanticActionType::InheritanceClass];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_function_parameter_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::FunctionParameterDeclaration,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_array_size_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![SemanticActionType::ArraySize];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_class_declaration_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![SemanticActionType::ClassDeclaration];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_program_family(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::ClassDeclarationList,
        SemanticActionType::FunctionDefinitionList,
        SemanticActionType::StatementBlock
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_function_definition(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::Type,
        SemanticActionType::Id,
        SemanticActionType::FunctionParameterList,
        SemanticActionType::StatementBlock,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_function_definition_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![SemanticActionType::FunctionDefinition];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_class_member_declaration_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::VariableDeclaration,
        SemanticActionType::ClassMemberFunctionDeclaration
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_function_parameter_declaration(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::FunctionParameterType,
        SemanticActionType::FunctionParameterId,
        SemanticActionType::ArraySizeList,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_variable_declaration(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::Type,
        SemanticActionType::Id,
        SemanticActionType::ArraySizeList,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_class_member_function_declaration(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::Type,
        SemanticActionType::Id,
        SemanticActionType::FunctionParameterList,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_statement_block(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::IfStatement,
        SemanticActionType::AssignmentStatement,
        SemanticActionType::ForLoopStatement,
        SemanticActionType::GetStatement,
        SemanticActionType::PutStatement,
        SemanticActionType::ReturnStatement,
        SemanticActionType::VariableDeclaration,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_assignment_statement(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::Variable,
        SemanticActionType::Expression,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_expression(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::RelationalExpression,
        SemanticActionType::ArithmeticExpression,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_relational_expression(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::ArithmeticExpression,
        SemanticActionType::RelationalOperator,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_factor_variable(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::DataMember,
        SemanticActionType::FactorFunctionCall,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_variable(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::DataMember,
        SemanticActionType::FunctionCall,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_data_member(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::Id,
        SemanticActionType::ArrayIndexingList,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_array_indexing_list(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![SemanticActionType::ArithmeticExpression];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_factor_function_call(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::Id,
        SemanticActionType::FunctionArguments
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_function_call(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::Id,
        SemanticActionType::FunctionArguments
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_function_arguments(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![SemanticActionType::Expression];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_arithmetic_expression(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::AdditiveOperator,
        SemanticActionType::Term
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_term(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::MultiplicativeOperator,
        SemanticActionType::Factor
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}
fn semantic_action_factor(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {
    let possible_child_node_types = vec![
        SemanticActionType::NegationOperator,
        SemanticActionType::NumberSign,
        SemanticActionType::Float,
        SemanticActionType::Integer,
        SemanticActionType::ArithmeticExpression,
        SemanticActionType::FactorVariable,
        SemanticActionType::Factor,
    ];
    semantic_action_generic_make_family(possible_child_node_types, action_type, &mut semantic_stack);
}


fn semantic_action_if_statement(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {}
fn semantic_action_for_loop_statement(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {}
fn semantic_action_get_statement(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {}
fn semantic_action_put_statement(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {}
fn semantic_action_return_statement(action_type: SemanticActionType, token: Token, mut semantic_stack: &mut Stack<usize>) {}

