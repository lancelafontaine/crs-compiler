use ast::SemanticActionType;
use lexer::Token;
use petgraph::prelude::NodeIndex;
use petgraph::{Direction, Graph};
use std::sync::Mutex;
use output::write_to_ast_log;

lazy_static! {
    #[derive(Debug, Clone)]
    pub static ref GENERATED_AST: Mutex<Ast> = Mutex::new(Ast::new());
}

// It is the Ast / graph's reponsibility to track node references and
// their parents / neighbors / children through node and edges manipulation
#[derive(Debug, Clone)]
pub struct Ast {
    pub root: Graph<AstNode, Option<usize>>,
}
impl Ast {
    pub fn new() -> Ast {
        Ast {
            root: Graph::<AstNode, Option<usize>>::new(),
        }
    }

    pub fn dfs(ast: &Ast, node_index: usize, visited: &mut Vec<usize>, visitor: &Fn(&AstNode)) {
        if !visited.contains(&node_index) {
            visited.push(node_index);
            visitor(ast.get_node(node_index).unwrap());
            for child_node_index in ast.get_node_children(node_index) {
                Ast::dfs(ast, child_node_index, visited, visitor);
            }
        }
    }

    pub fn make_node(
        &mut self,
        node_type: SemanticActionType,
        maybe_token: Option<Token>,
    ) -> usize {
        self.root
            .add_node(AstNode::new(node_type, maybe_token))
            .index()
    }

    pub fn get_node(&self, index: usize) -> Option<&AstNode> {
        self.root.node_weight(NodeIndex::new(index))
    }

    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut AstNode> {
        self.root.node_weight_mut(NodeIndex::new(index))
    }

    pub fn make_edge(
        &mut self,
        parent_index: usize,
        child_index: usize,
        possible_order: Option<usize>,
    ) -> usize {
        self.root
            .add_edge(
                NodeIndex::new(parent_index),
                NodeIndex::new(child_index),
                possible_order,
            )
            .index()
    }

    pub fn get_most_recently_added_node_index(&self) -> usize {
        self.root.node_count() - 1
    }

    pub fn get_most_recently_added_node(&self) -> Option<&AstNode> {
        self.get_node(self.get_most_recently_added_node_index())
    }

    pub fn get_node_children(&self, index: usize) -> Vec<usize> {
        self.root
            .neighbors_directed(NodeIndex::new(index), Direction::Outgoing)
            .map(|x| x.index())
            .collect()
    }

    pub fn print_graph(&self) {
        write_to_ast_log(format!("##########################################################################################"));
        for index in self.root.node_indices() {
           write_to_ast_log(format!(
                "{:?} : {:?}",
                index.index(),
                self.root.node_weight(index).unwrap()
            ));
            let incoming_edges = self.root.neighbors_directed(index, Direction::Incoming);
            for incoming_edge in incoming_edges {
                write_to_ast_log(format!("        Incoming edge: {} -> self", incoming_edge.index()));
            }
            let outgoing_edges = self.root.neighbors_directed(index, Direction::Outgoing);
            for outgoing_edge in outgoing_edges {
                write_to_ast_log(format!("        Outgoing edge: self -> {}", outgoing_edge.index()));
            }
        }
        write_to_ast_log(format!("##########################################################################################"));
    }
}

// Node is not aware of parent / neighbors / children
// Deliberate deferral of responsibility to the Ast's graph.
#[derive(Debug, Clone)]
pub struct AstNode {
    pub node_type: SemanticActionType,
    pub node_token: Option<Token>,
}
impl AstNode {
    pub fn new(node_type: SemanticActionType, node_token: Option<Token>) -> AstNode {
        AstNode {
            node_type,
            node_token,
        }
    }
}
