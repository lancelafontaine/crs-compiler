use petgraph::Graph;
use petgraph::prelude::NodeIndex;
use lexer::Token;

#[derive(Debug)]
pub enum AstNodeType {
    Identifier
}

// It is the Ast / graph's reponsibility to track node references and
// their parents / neighbors / children through node and edges manipulation
#[derive(Debug)]
pub struct Ast {
    root: Graph<AstNode, &'static str>
}
impl Ast {
    pub fn new() -> Ast {
        Ast {
            root: Graph::<AstNode, &str>::new()
        }
    }
    pub fn make_node(&mut self, node_type: AstNodeType, token: Token) -> usize {
        self.root.add_node(AstNode::new(node_type, Some(token))).index()
    }

    pub fn get_node(&mut self, index: usize) -> Option<&AstNode>{
        self.root.node_weight(NodeIndex::new(index))
    }

    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut AstNode>{
        self.root.node_weight_mut(NodeIndex::new(index))
    }
/*
    pub fn make_family() {

    }
    pub fn makeSibling(){

    };

    pub fn adoptChildren(){

    };
*/
}

// Node is not aware of parent / neighbors / children
// Deliberate deferral of responsibility to the Ast's graph.
#[derive(Debug)]
pub struct AstNode {
    node_type: AstNodeType,
    node_token: Option<Token>,
}
impl AstNode {
    pub fn new(node_type: AstNodeType, node_token: Option<Token>) -> AstNode {
        AstNode {
            node_type,
            node_token
        }
    }
}
