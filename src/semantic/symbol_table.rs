use petgraph::Graph;
use petgraph::visit::Dfs;
use petgraph::prelude::NodeIndex;
use std::sync::Mutex;
use lexer::Token;
use ast::{ GENERATED_AST, SemanticActionType, AstNode };
use std::ops::Deref;

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
    let graph = GENERATED_AST.lock().unwrap();
    let ast_root_node_index = graph.root.node_count() - 1;

    // Some sanity checks
    let some_ast_root_node = graph.get_node(ast_root_node_index);
    if let Some(ast_root_node) = some_ast_root_node {
        if ast_root_node.node_type != SemanticActionType::ProgramFamily {
            unimplemented!();
        }
    } else {
        unimplemented!();
    }

    // TODO: Optimize this DFS tree traversal
    let mut ast_graph = graph.deref().root.map(
        |node_index, node| node.clone(),
        |edge_index, edge| edge.clone()
    );
    let mut dfs_graph = graph.deref().root.map(
        |node_index, node| node_index.index(),
        |edge_index, edge| edge_index.index()
    );

    let mut visited = vec![];
    dfs(&dfs_graph, &ast_graph, ast_root_node_index, &mut visited);
    println!("{:?}", visited)
}

// TODO: Optimize this DFS tree traversal
pub fn dfs(dfs_graph: &Graph<usize, usize>, ast_graph: &Graph<AstNode, Option<usize>>, node_index: usize, visited: &mut Vec<usize>) {
    if !visited.contains(&node_index) {
        visited.push(node_index);
        println!("{}", node_index);
        let node = ast_graph.node_weight(NodeIndex::new(node_index)).unwrap();
        println!("{:?}", node);
        for child_node_index in dfs_graph.neighbors(NodeIndex::new(node_index)){
            dfs(dfs_graph, ast_graph, child_node_index.index(), visited);
        }
    }
}
