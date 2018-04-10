use petgraph::Graph;
use petgraph::visit::Dfs;
use petgraph::prelude::NodeIndex;
use std::sync::Mutex;
use lexer::Token;
use ast::{ GENERATED_AST, SemanticActionType, Ast, AstNode };
use semantic::symbol_table_visitor;

lazy_static! {
    pub static ref GENERATED_SYMBOL_TABLE_GRAPH: Mutex<SymbolTableGraph> = Mutex::new(SymbolTableGraph::new());
}

pub enum SymbolTableNodeType {
    Table,
    Record
}

pub enum SymbolTableRecordType {
    Class,
    Function,
    Parameter,
    Variable
}

pub struct SymbolTableGraph {
    pub global_table: Graph<SymbolTableNode, Option<usize>>,
    pub current_table_index: usize
}

impl SymbolTableGraph {

    pub fn new () -> SymbolTableGraph {
        let mut graph = Graph::<SymbolTableNode, Option<usize>>::new();
        let global_table_index = graph.add_node(SymbolTableNode::new_table());
        SymbolTableGraph {
            global_table: graph,
            current_table_index: global_table_index.index()
        }
    }

    pub fn add_record_to_table() {}
    pub fn get_table_records() {}
    pub fn get_parent_table_records() {}
}



pub struct SymbolTableNode {
    pub node_type: SymbolTableNodeType,
    pub record: Option<SymbolTableRecord>
}

impl SymbolTableNode {
    pub fn new_table() -> SymbolTableNode {
        SymbolTableNode {
            node_type: SymbolTableNodeType::Table,
            record: None
        }
    }
}

pub struct SymbolTableRecord {
    identifier: String,
    record_type: SymbolTableRecordType,
    value_type: Option<String>,
    token: Token
}

pub fn build_symbol_tables() {
    // TODO: Refactor to prevent having to perform clone of GENERATED_AST
    let mut graph = GENERATED_AST.lock().unwrap().clone();

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
    Ast::dfs(&graph, ast_root_node_index, &mut vec![], &symbol_table_visitor::visitor);
}

