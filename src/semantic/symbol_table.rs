use petgraph::{ Graph, Direction };
use petgraph::visit::Dfs;
use petgraph::prelude::NodeIndex;
use std::sync::Mutex;
use lexer::Token;
use ast::{ GENERATED_AST, SemanticActionType, Ast, AstNode };
use semantic::symbol_table_visitor;

lazy_static! {
    pub static ref GENERATED_SYMBOL_TABLE_GRAPH: Mutex<SymbolTableGraph> = Mutex::new(SymbolTableGraph::new());
}

// SymbolTableGraph
pub struct SymbolTableGraph {
    pub global_table_graph: Graph<SymbolTableNode, Option<usize>>,
    pub current_table_index: usize
}
impl SymbolTableGraph {
    // Constructor
    pub fn new () -> SymbolTableGraph {
        let mut graph = Graph::<SymbolTableNode, Option<usize>>::new();
        let global_table_graph_index = graph.add_node(
            SymbolTableNode::new_empty_table_node_with_label("global".to_string())
        );
        SymbolTableGraph {
            global_table_graph: graph,
            current_table_index: global_table_graph_index.index()
        }
    }

    // Getters
    pub fn get_most_recently_added_node_index(&self) -> usize {
        self.global_table_graph.node_count() - 1
    }
    pub fn get_most_recently_added_node(&self) -> Option<&SymbolTableNode> {
        self.get_node(self.get_most_recently_added_node_index())
    }
    pub fn get_node(&self, index: usize) -> Option<&SymbolTableNode> {
        self.global_table_graph.node_weight(NodeIndex::new(index))
    }
    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut SymbolTableNode> {
        self.global_table_graph.node_weight_mut(NodeIndex::new(index))
    }
    pub fn get_current_table(&self) -> Option<&SymbolTableNode> {
        self.global_table_graph.node_weight(NodeIndex::new(self.current_table_index))
    }
    pub fn get_current_table_mut(&mut self) -> Option<&mut SymbolTableNode> {
        self.global_table_graph.node_weight_mut(NodeIndex::new(self.current_table_index))
    }

    // Setters
    pub fn add_empty_table_to_table(&mut self, node_index: usize) -> usize {
        self.ensure_node_is_table(node_index);
        let previous_table_index = self.current_table_index;
        let new_table_index = self._make_node(SymbolTableNode::new_empty_table_node());
        self._make_edge(previous_table_index, new_table_index, None);
        self._enter_table_scope(new_table_index);
        new_table_index
    }
    pub fn add_empty_table_with_label_to_table(&mut self, label: String, node_index: usize) -> usize {
        self.ensure_node_is_table(node_index);
        let previous_table_index = self.current_table_index;
        let new_table_index = self._make_node(SymbolTableNode::new_empty_table_node_with_label(label));
        self._make_edge(previous_table_index, new_table_index, None);
        self._enter_table_scope(new_table_index);
        new_table_index
    }
    pub fn add_empty_record_with_type_to_table(&mut self, record_type: SymbolTableRecordType, node_index: usize) -> usize {
        self.ensure_node_is_table(node_index);
        let table_index = self.current_table_index;
        let new_record_index = self._make_node(SymbolTableNode::new_empty_record_node_with_type(record_type));
        self._make_edge(table_index, new_record_index, Some(new_record_index));
        new_record_index
    }
    pub fn set_current_table_label(&mut self, label: String) {
        let current_table_index = self.current_table_index;
        self.ensure_node_is_table(current_table_index);
        let mut current_table_node = self.get_node_mut(current_table_index);
    }
    fn _make_node(&mut self, node: SymbolTableNode) -> usize {
        self.global_table_graph.add_node(node).index()
    }
    fn _make_edge(&mut self, parent_index: usize, child_index: usize, possible_order: Option<usize>) -> usize {
        self.global_table_graph.add_edge(
            NodeIndex::new(parent_index),
            NodeIndex::new(child_index),
            possible_order
        ).index()
    }

    fn _enter_table_scope(&mut self, new_table_index: usize) {
        self.current_table_index = new_table_index;
    }

    // Validate pre/post-conditions
    pub fn ensure_node_exists(&self, node_index: usize) {
        if let None = self.get_node(node_index) {
            unimplemented!();
        }
    }
    pub fn ensure_node_is_record(&self, node_index: usize) {
        self.ensure_node_exists(node_index);
        let node = self.get_node(node_index).unwrap();
        if !node.is_record() { unimplemented!(); }
    }
    pub fn ensure_node_is_table(&self, node_index: usize) {
        self.ensure_node_exists(node_index);
        let node = self.get_node(node_index).unwrap();
        if !node.is_table() { unimplemented!(); }
    }

    // Utility / Debugging
    pub fn print_graph(&self) {
        println!("##########################################################################################");
        for index in self.global_table_graph.node_indices() {
            println!("{:?} : {:?}", index.index(), self.global_table_graph.node_weight(index).unwrap());
            let incoming_edges = self.global_table_graph.neighbors_directed(index, Direction::Incoming);
            for incoming_edge in incoming_edges {
                println!("        Incoming edge: {} -> self", incoming_edge.index());
            }
            let outgoing_edges = self.global_table_graph.neighbors_directed(index, Direction::Outgoing);
            for outgoing_edge in outgoing_edges {
                println!("        Outgoing edge: self -> {}", outgoing_edge.index());
            }
        }
        println!("##########################################################################################");
    }
}

// SymbolTableNode
#[derive(Debug, PartialEq)]
pub enum SymbolTableNodeType {
    Table(Option<String>, Vec<String>), // label, inheritance labels
    Record(Option<SymbolTableRecord>)
}
#[derive(Debug, PartialEq)]
pub struct SymbolTableNode {
    pub node_type: SymbolTableNodeType
}
impl SymbolTableNode {
    pub fn new_empty_table_node() -> SymbolTableNode {
        SymbolTableNode {
            node_type: SymbolTableNodeType::Table(None, vec![])
        }
    }
    pub fn new_empty_table_node_with_label(label: String) -> SymbolTableNode {
        SymbolTableNode {
            node_type: SymbolTableNodeType::Table(Some(label), vec![])
        }
    }
    pub fn new_empty_record_node() -> SymbolTableNode {
        SymbolTableNode {
            node_type: SymbolTableNodeType::Record(None)
        }
    }
    pub fn new_empty_record_node_with_type(record_type: SymbolTableRecordType) -> SymbolTableNode {
        SymbolTableNode {
            node_type: SymbolTableNodeType::Record(Some(
                SymbolTableRecord::new_empty_record(record_type)
            ))
        }
    }
    pub fn set_table_label(&mut self, label: String) {
        let mut inheritance_list: Vec<String>;
        match self.node_type {
            SymbolTableNodeType::Table(_, ref existing_list) => {
                inheritance_list = existing_list.clone();
            },
            _ => unimplemented!()
        }
        self.node_type = SymbolTableNodeType::Table(Some(label), inheritance_list);
    }
    pub fn set_record_identifier(&mut self, identifier: String) {
        let mut some_record: Option<SymbolTableRecord>;
        match self.node_type {
            SymbolTableNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => { unimplemented!("Can't set record identifier on a non-record node"); }
        }
        if let Some(mut record) = some_record {
            record.set_identifier(identifier);
            self.node_type = SymbolTableNodeType::Record(Some(record));
        } else { unimplemented!("Can't set record identifier on an empty record node since we don't know what record_type it should be"); }
    }
    pub fn set_record_value_type(&mut self, value_type: String) {
        let mut some_record: Option<SymbolTableRecord>;
        match self.node_type {
            SymbolTableNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => { unimplemented!("Can't set record value type on a non-record node"); }
        }
        if let Some(mut record) = some_record {
            record.set_value_type(value_type);
            self.node_type = SymbolTableNodeType::Record(Some(record));
        } else { unimplemented!("Can't set record value type on an empty record node since we don't know what record_type it should be"); }
    }
    pub fn add_function_parameter_fragment_to_record(&mut self, function_parameter_fragment: String) {
        let mut some_record: Option<SymbolTableRecord>;
        match self.node_type {
            SymbolTableNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => { unimplemented!("Can't add record function parameter on a non-record node"); }
        }
        if let Some(mut record) = some_record {
            record.add_function_parameter_fragment(function_parameter_fragment);
            self.node_type = SymbolTableNodeType::Record(Some(record));
        } else { unimplemented!("Can't add record function parameter on an empty record node since we don't know what record_type it should be"); }
    }
    pub fn increment_record_array_size(&mut self) {
        let mut some_record: Option<SymbolTableRecord>;
        match self.node_type {
            SymbolTableNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => { unimplemented!("Can't set record array size on a non-record node"); }
        }
        if let Some(mut record) = some_record {
            record.increment_array_size();
            self.node_type = SymbolTableNodeType::Record(Some(record));
        } else { unimplemented!("Can't set record array on an empty record node since we don't know what record_type it should be"); }
    }

    pub fn add_to_table_inheritance_list(&mut self, new_label: String) {
        let mut label: String;
        let mut inheritance_list: Vec<String>;
        match self.node_type {
            SymbolTableNodeType::Table(ref existing_label, ref existing_list) => {
                label = existing_label.clone().unwrap().clone();
                inheritance_list = existing_list.clone();
            },
            _ => unimplemented!()
        }
        inheritance_list.push(new_label);
        self.node_type = SymbolTableNodeType::Table(Some(label), inheritance_list);
    }
    pub fn initialize_record_function_parameters(&mut self) {
        let mut some_record: Option<SymbolTableRecord>;
        match self.node_type {
            SymbolTableNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => { unimplemented!("Can't set record array size on a non-record node"); }
        }
        if let Some(mut record) = some_record {
            record.initialize_function_parameters();
            self.node_type = SymbolTableNodeType::Record(Some(record));
        } else { unimplemented!("Can't set record array on an empty record node since we don't know what record_type it should be"); }
    }

    pub fn is_table(&self) -> bool {
        match self.node_type {
            SymbolTableNodeType::Table(_, _) => true,
            _ => false
        }
    }
    pub fn is_record(&self) -> bool {
        match self.node_type {
            SymbolTableNodeType::Record(_) => true,
            _ => false
        }
    }
}

// SymbolTableRecord
#[derive(Debug, PartialEq, Clone)]
pub enum SymbolTableRecordType {
    Class,
    Function,
    Parameter,
    Variable
}
#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTableRecord {
    identifier: Option<String>,
    record_type: SymbolTableRecordType,
    value_type: Option<String>,
    array_depth: Option<usize>,
    function_parameters: Option<Vec<String>>
}
impl SymbolTableRecord {
    pub fn new_empty_record(record_type: SymbolTableRecordType) -> SymbolTableRecord {
        SymbolTableRecord {
            record_type,
            identifier: None,
            value_type: None,
            array_depth: None,
            function_parameters: None,
        }
    }
    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = Some(identifier);
    }
    pub fn set_value_type(&mut self, value_type: String) {
        self.value_type = Some(value_type);
    }
    pub fn increment_array_size(&mut self) {
        if let Some(mut depth) = self.array_depth {
            self.array_depth = Some(depth + 1);
        } else {
            self.array_depth = Some(1);
        }
    }
    pub fn initialize_function_parameters(&mut self) {
        self.function_parameters = Some(vec![])
    }

    pub fn add_function_parameter_fragment(&mut self, function_parameter_fragment: String) {
        let mut list_of_fragments: Vec<String>;
        if let Some(ref mut existing_list) = self.function_parameters {
            list_of_fragments = existing_list.clone();
        } else {
            unimplemented!("FunctionParameterList hasn't been seen yet, something wrong with the AST, initialize_function_parameters was never called");
        }
        list_of_fragments.push(function_parameter_fragment);
        self.function_parameters = Some(list_of_fragments)

    }
}

// Free Functions
pub fn build_symbol_tables() {
    // TODO: Refactor to prevent having to perform clone of GENERATED_AST
    let graph = GENERATED_AST.lock().unwrap().clone();

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
    GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap().print_graph();

}
