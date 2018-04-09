use petgraph::{ Graph, Direction };
use petgraph::graph::{ Node, Edge, WalkNeighbors };
use petgraph::prelude::NodeIndex;
use lexer::Token;
use std::sync::Mutex;
use ast::SemanticActionType;

lazy_static! {
    #[derive(Debug, Clone)]
    pub static ref GENERATED_AST: Mutex<Ast> = Mutex::new(Ast::new());
}

// It is the Ast / graph's reponsibility to track node references and
// their parents / neighbors / children through node and edges manipulation
#[derive(Debug)]
pub struct Ast {
    pub root: Graph<AstNode, Option<usize>>
}
impl Ast {
    pub fn new() -> Ast {
        Ast {
            root: Graph::<AstNode, Option<usize>>::new()
        }
    }

    pub fn make_node(&mut self, node_type: SemanticActionType, maybe_token: Option<Token>) -> usize {
        self.root.add_node(AstNode::new(node_type, maybe_token)).index()
    }

    pub fn get_node(&self, index: usize) -> Option<&AstNode>{
        self.root.node_weight(NodeIndex::new(index))
    }

    pub fn print_graph(&self) {
        println!("##########################################################################################");
        for index in self.root.node_indices() {
            println!("{:?} : {:?}", index.index(), self.root.node_weight(index).unwrap());
            let incoming_edges = self.root.neighbors_directed(index, Direction::Incoming);
            for incoming_edge in incoming_edges {
                println!("        Incoming edge: {} -> self", incoming_edge.index());
            }
            let outgoing_edges = self.root.neighbors_directed(index, Direction::Outgoing);
            for outgoing_edge in outgoing_edges {
                println!("        Outgoing edge: self -> {}", outgoing_edge.index());
            }
        }
        println!("##########################################################################################");
    }

    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut AstNode>{
        self.root.node_weight_mut(NodeIndex::new(index))
    }

    pub fn make_edge(&mut self, parent_index: usize, child_index: usize, possible_order: Option<usize>) -> usize {
        self.root.add_edge(
            NodeIndex::new(parent_index),
            NodeIndex::new(child_index),
            possible_order
        ).index()
    }

    pub fn get_node_children(&self, index: usize) -> Vec<usize> {
        self.root.neighbors_directed(NodeIndex::new(index), Direction::Outgoing)
            .map(|x| x.index())
            .collect()
    }

    /* pub fn get_node_parent(&self, index: usize) -> () {
        self.root.neighbors(NodeIndex::new(index))
    }
    */
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
            node_token
        }
    }
}
