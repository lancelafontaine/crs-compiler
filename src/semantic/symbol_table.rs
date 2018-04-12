use ast::{Ast, SemanticActionType, GENERATED_AST};
use petgraph::prelude::NodeIndex;
use petgraph::{Direction, Graph};
use semantic::symbol_table_visitor;
use std::sync::Mutex;

lazy_static! {
    pub static ref GENERATED_SYMBOL_TABLE_GRAPH: Mutex<STGraph> = Mutex::new(STGraph::new());
}

// STGraph
pub struct STGraph {
    pub global_table_graph: Graph<STNode, Option<usize>>,
    pub current_table_index: usize,
}
impl STGraph {
    // Constructor
    pub fn new() -> STGraph {
        let mut graph = Graph::<STNode, Option<usize>>::new();
        let global_table_graph_index = graph.add_node(STNode::new_empty_table_node());
        STGraph {
            global_table_graph: graph,
            current_table_index: global_table_graph_index.index(),
        }
    }

    // Getters
    pub fn get_most_recently_added_node_index(&self) -> usize {
        self.global_table_graph.node_count() - 1
    }
    pub fn get_most_recently_added_node(&self) -> Option<&STNode> {
        self.get_node(self.get_most_recently_added_node_index())
    }
    pub fn get_node(&self, index: usize) -> Option<&STNode> {
        self.global_table_graph.node_weight(NodeIndex::new(index))
    }
    pub fn get_node_mut(&mut self, index: usize) -> Option<&mut STNode> {
        self.global_table_graph
            .node_weight_mut(NodeIndex::new(index))
    }
    pub fn get_current_table(&self) -> Option<&STNode> {
        self.global_table_graph
            .node_weight(NodeIndex::new(self.current_table_index))
    }
    pub fn get_current_table_mut(&mut self) -> Option<&mut STNode> {
        self.global_table_graph
            .node_weight_mut(NodeIndex::new(self.current_table_index))
    }

    // Setters
    pub fn add_class_to_table(&mut self, node_index: usize) -> usize {
        self.ensure_node_is_table(node_index);
        let previous_table_index = self.current_table_index;

        let new_record_index =
            self._make_node(STNode::new_empty_record_node_with_type(STRecordType::Class));
        self._make_edge(previous_table_index, new_record_index, None);

        let new_table_index = self._make_node(STNode::new_empty_table_node());
        self._make_edge(new_record_index, new_table_index, None);
        new_table_index
    }
    pub fn add_empty_record_with_type_to_table(
        &mut self,
        record_type: STRecordType,
        node_index: usize,
    ) -> usize {
        self.ensure_node_is_table(node_index);
        let table_index = self.current_table_index;
        let new_record_index =
            self._make_node(STNode::new_empty_record_node_with_type(record_type));
        self._make_edge(table_index, new_record_index, Some(new_record_index));
        new_record_index
    }
    pub fn set_table_identifier(&mut self, table_index: usize, label: String) {
        let mut parent_record_index: NodeIndex;
        if let Some(index) = self.global_table_graph
            .neighbors_directed(NodeIndex::new(table_index), Direction::Incoming)
            .next()
        {
            parent_record_index = index;
        } else {
            unimplemented!("We're trying to set the table identifier for the global/root table.")
        }
        self.global_table_graph
            .node_weight_mut(parent_record_index)
            .unwrap()
            .set_record_identifier(label);
    }
    fn _make_node(&mut self, node: STNode) -> usize {
        self.global_table_graph.add_node(node).index()
    }
    fn _make_edge(
        &mut self,
        parent_index: usize,
        child_index: usize,
        possible_order: Option<usize>,
    ) -> usize {
        self.global_table_graph
            .add_edge(
                NodeIndex::new(parent_index),
                NodeIndex::new(child_index),
                possible_order,
            )
            .index()
    }

    pub fn enter_table_scope(&mut self, new_table_index: usize) {
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
        if !node.is_record() {
            unimplemented!();
        }
    }
    pub fn ensure_node_is_table(&self, node_index: usize) {
        self.ensure_node_exists(node_index);
        let node = self.get_node(node_index).unwrap();
        if !node.is_table() {
            unimplemented!();
        }
    }

    // Utility / Debugging
    pub fn print_graph(&self) {
        println!("##########################################################################################");
        for index in self.global_table_graph.node_indices() {
            println!(
                "{:?} : {:?}",
                index.index(),
                self.global_table_graph.node_weight(index).unwrap()
            );
            let incoming_edges = self.global_table_graph
                .neighbors_directed(index, Direction::Incoming);
            for incoming_edge in incoming_edges {
                println!("        Incoming edge: {} -> self", incoming_edge.index());
            }
            let outgoing_edges = self.global_table_graph
                .neighbors_directed(index, Direction::Outgoing);
            for outgoing_edge in outgoing_edges {
                println!("        Outgoing edge: self -> {}", outgoing_edge.index());
            }
        }
        println!("##########################################################################################");
    }
}

// STNode
#[derive(Debug, PartialEq)]
pub enum STNodeType {
    Table(Vec<String>), // label, inheritance labels
    Record(Option<STRecord>),
}
#[derive(Debug, PartialEq)]
pub struct STNode {
    pub node_type: STNodeType,
}
impl STNode {
    pub fn new_empty_table_node() -> STNode {
        STNode {
            node_type: STNodeType::Table(vec![]),
        }
    }
    pub fn new_empty_record_node() -> STNode {
        STNode {
            node_type: STNodeType::Record(None),
        }
    }
    pub fn new_empty_record_node_with_type(record_type: STRecordType) -> STNode {
        STNode {
            node_type: STNodeType::Record(Some(STRecord::new_empty_record(record_type))),
        }
    }
    pub fn set_record_identifier(&mut self, identifier: String) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                unimplemented!("Can't set record identifier on a non-record node");
            }
        }
        if let Some(mut record) = some_record {
            record.set_identifier(identifier);
            self.node_type = STNodeType::Record(Some(record));
        } else {
            unimplemented!("Can't set record identifier on an empty record node since we don't know what record_type it should be");
        }
    }
    pub fn set_record_value_type(&mut self, value_type: String) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                unimplemented!("Can't set record value type on a non-record node");
            }
        }
        if let Some(mut record) = some_record {
            record.set_value_type(value_type);
            self.node_type = STNodeType::Record(Some(record));
        } else {
            unimplemented!("Can't set record value type on an empty record node since we don't know what record_type it should be");
        }
    }
    pub fn add_function_parameter_fragment_to_record(
        &mut self,
        function_parameter_fragment: String,
    ) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                unimplemented!("Can't add record function parameter on a non-record node");
            }
        }
        if let Some(mut record) = some_record {
            record.add_function_parameter_fragment(function_parameter_fragment);
            self.node_type = STNodeType::Record(Some(record));
        } else {
            unimplemented!("Can't add record function parameter on an empty record node since we don't know what record_type it should be");
        }
    }
    pub fn increment_record_array_size(&mut self) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                unimplemented!("Can't set record array size on a non-record node");
            }
        }
        if let Some(mut record) = some_record {
            record.increment_array_size();
            self.node_type = STNodeType::Record(Some(record));
        } else {
            unimplemented!("Can't set record array on an empty record node since we don't know what record_type it should be");
        }
    }

    pub fn add_to_table_inheritance_list(&mut self, new_label: String) {
        let mut inheritance_list: Vec<String>;
        match self.node_type {
            STNodeType::Table(ref existing_list) => {
                inheritance_list = existing_list.clone();
            }
            _ => unimplemented!(),
        }
        inheritance_list.push(new_label);
        self.node_type = STNodeType::Table(inheritance_list);
    }
    pub fn initialize_record_function_parameters(&mut self) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                unimplemented!("Can't set record array size on a non-record node");
            }
        }
        if let Some(mut record) = some_record {
            record.initialize_function_parameters();
            self.node_type = STNodeType::Record(Some(record));
        } else {
            unimplemented!("Can't set record array on an empty record node since we don't know what record_type it should be");
        }
    }

    pub fn is_table(&self) -> bool {
        match self.node_type {
            STNodeType::Table(_) => true,
            _ => false,
        }
    }
    pub fn is_record(&self) -> bool {
        match self.node_type {
            STNodeType::Record(_) => true,
            _ => false,
        }
    }
}

// STRecord
#[derive(Debug, PartialEq, Clone)]
pub enum STRecordType {
    Class,
    Function,
    Parameter,
    Variable,
}
#[derive(Debug, PartialEq, Clone)]
pub struct STRecord {
    identifier: Option<String>,
    record_type: STRecordType,
    value_type: Option<String>,
    array_depth: Option<usize>,
    function_parameters: Option<Vec<String>>,
}
impl STRecord {
    pub fn new_empty_record(record_type: STRecordType) -> STRecord {
        STRecord {
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
    Ast::dfs(
        &graph,
        ast_root_node_index,
        &mut vec![],
        &symbol_table_visitor::visitor,
    );
    GENERATED_SYMBOL_TABLE_GRAPH.lock().unwrap().print_graph();
}
