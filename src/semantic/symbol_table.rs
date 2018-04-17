use petgraph::prelude::NodeIndex;
use petgraph::{Direction, Graph};
use std::sync::Mutex;
use output::{ error, write_to_symbol_table_log };

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
    pub fn get_parent_node_index(&self, node_index: usize) -> Option<usize> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(node_index), Direction::Incoming)
            .next()
            .clone()
            .map(|n| n.index())
    }
    pub fn get_child_node_indices(&self, node_index: usize) -> Vec<usize> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(node_index), Direction::Outgoing)
            .map(|n| n.index())
            .collect()
    }
    pub fn get_child_node_index(&self, node_index: usize) -> Option<usize> {
        self.ensure_node_is_record(node_index);
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(node_index), Direction::Outgoing)
            .next()
            .clone()
            .map(|n| n.index())
    }
    pub fn get_table_identifier(&mut self, table_index: usize) -> Option<String> {
        let mut parent_record_index: usize;
        if let Some(index) = self.get_parent_node_index(table_index) {
            parent_record_index = index;
        } else {
            error(16);
            unimplemented!();
        }
        self.global_table_graph
            .node_weight(NodeIndex::new(parent_record_index))
            .unwrap()
            .get_record_identifier()
    }
    pub fn get_all_function_record_indices(&self) -> Vec<usize> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(0), Direction::Outgoing)
            .map(|n| n.index())
            .filter(|i| self.get_node(*i).unwrap().is_function_record())
            .collect()
    }
    pub fn get_all_function_table_indices(&self) -> Vec<usize> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(0), Direction::Outgoing)
            .map(|n| n.index())
            .filter(|i| self.get_node(*i).unwrap().is_function_record())
            .filter_map(|i| self.get_child_node_index(i))
            .collect()
    }
    pub fn get_all_table_indices(&self) -> Vec<usize> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(0), Direction::Outgoing)
            .map(|n| n.index())
            .filter(|i| self.get_node(*i).unwrap().is_class_record() || self.get_node(*i).unwrap().is_function_record())
            .filter_map(|i| self.get_child_node_index(i))
            .collect()
    }
    pub fn get_all_class_table_indices(&self) -> Vec<usize> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(0), Direction::Outgoing)
            .map(|n| n.index())
            .filter(|i| self.get_node(*i).unwrap().is_class_record())
            .filter_map(|i| self.get_child_node_index(i))
            .collect()
    }
    pub fn get_all_table_record_indices(&self, table_index: usize) -> Vec<usize> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(table_index), Direction::Outgoing)
            .map(|n| n.index())
            .collect()
    }
    pub fn get_all_class_identifiers(&self) -> Vec<String> {
        self.global_table_graph
            .neighbors_directed(NodeIndex::new(0), Direction::Outgoing)
            .filter_map(|n| self.get_node(n.index()))
            .filter(|n| n.is_class_record())
            .filter_map(|n| n.get_record_identifier())
            .collect()
    }
    pub fn get_class_table_index_by_identifier(&self, label: String) -> Option<usize> {
        let global_table_record_indices = self.global_table_graph
            .neighbors_directed(NodeIndex::new(0), Direction::Outgoing);
        for record_index in global_table_record_indices {
            let node = self.get_node(record_index.index()).unwrap();
            if node.is_class_record() {
                if let Some(record_identifier) = node.get_record_identifier() {
                    if record_identifier == label {
                        return self.get_child_node_index(record_index.index()).clone()
                    }
                }
            }
        }
        None
    }
    pub fn calculate_table_memory_sizes(&self, table_index: usize) -> Vec<usize> {
        let mut calculated_sizes = vec![];
        let all_record_indices = self.get_all_table_record_indices(table_index);
        for index in all_record_indices {
            let node = self.get_node(index).unwrap();
            if node.is_variable_record() {
                let mut temp_memory_size: usize;
                let value_type = node.get_record_value_type().unwrap();
                if value_type == "int" {
                    temp_memory_size = 4;
                } else {
                    // Composed class type / data member
                    let mut class_table_index: usize;
                    if let Some(i) = self.get_class_table_index_by_identifier(value_type) {
                        class_table_index = i;
                    } else {
                        error(20);
                        unimplemented!();
                    }
                    temp_memory_size = self.calculate_table_memory_sizes(class_table_index)
                        .iter()
                        .fold(0, |x, &y| x + y);

                    // Add size of inherited classes
                    let inheritance_list = self.get_node(class_table_index).unwrap().get_table_inheritance_list();
                    for class_label in inheritance_list {
                        let mut class_table_index: usize;
                        if let Some(i) = self.get_class_table_index_by_identifier(class_label) {
                            class_table_index = i;
                        } else {
                            error(20);
                            unimplemented!();
                        }
                        temp_memory_size += self.calculate_table_memory_sizes(class_table_index)
                            .iter()
                            .fold(0, |x, &y| x + y);
                    }
                }
                let mut temp_memory_multiplier: usize;
                let array_dimensions = node.get_record_array_dimensions();
                if array_dimensions.is_some() {
                    temp_memory_multiplier = array_dimensions
                        .unwrap()
                        .iter()
                        .fold(1, |x, &y| x * y);
                } else {
                    temp_memory_multiplier = 1;
                }
                calculated_sizes.push(temp_memory_size * temp_memory_multiplier);
            }
        }
        calculated_sizes
    }
    pub fn set_table_memory_sizes(&mut self, table_index: usize, calculated_sizes: Vec<usize>) {
        let mut calculated_sizes: Vec<&usize> = calculated_sizes.iter().rev().collect();
        let all_record_indices = self.get_all_table_record_indices(table_index);
        for index in all_record_indices {
            let node = self.get_node_mut(index).unwrap();
            if node.is_variable_record() {
                node.set_record_memory_size(*calculated_sizes.pop().unwrap())
            }
        }
    }

    // Setters
    pub fn add_table_to_table(&mut self, record_type: STRecordType, node_index: usize) -> usize {
        self.ensure_node_is_table(node_index);
        let previous_table_index = self.current_table_index;

        let new_record_index =
            self._make_node(STNode::new_empty_record_node_with_type(record_type));
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
    pub fn move_table_scope(&mut self, index_of_table_to_be_moved: usize, index_of_table_to_move_to: usize) {
        let record_index = self.get_parent_node_index(index_of_table_to_be_moved).unwrap();
        let parent_record_index = self.get_parent_node_index(record_index).unwrap();

        let edge_index_to_remove = self.global_table_graph.find_edge(
            NodeIndex::new(parent_record_index),
            NodeIndex::new(record_index)
        ).unwrap();
        self.global_table_graph.remove_edge(edge_index_to_remove);
        self._make_edge(index_of_table_to_move_to, record_index, None);
    }
    pub fn set_table_value_type(&mut self, table_index: usize, value_type: String) {
        let mut parent_record_index: usize;
        if let Some(index) = self.get_parent_node_index(table_index) {
            parent_record_index = index;
        } else {
            error(16);
            unimplemented!();
        }
        self.global_table_graph
            .node_weight_mut(NodeIndex::new(parent_record_index))
            .unwrap()
            .set_record_value_type(value_type);
    }
    pub fn initialize_table_function_parameters(&mut self, table_index: usize) {
        let mut parent_record_index: usize;
        if let Some(index) = self.get_parent_node_index(table_index) {
            parent_record_index = index;
        } else {
            error(16);
            unimplemented!();
        }
        self.global_table_graph
            .node_weight_mut(NodeIndex::new(parent_record_index))
            .unwrap()
            .initialize_record_function_parameters();
    }
    pub fn set_table_identifier(&mut self, table_index: usize, label: String) {
        let mut parent_record_index: usize;
        if let Some(index) = self.get_parent_node_index(table_index) {
            parent_record_index = index;
        } else {
            error(16);
            unimplemented!();
        }
        self.global_table_graph
            .node_weight_mut(NodeIndex::new(parent_record_index))
            .unwrap()
            .set_record_identifier(label);
    }
    pub fn add_function_parameter_fragment_to_table(
        &mut self,
        table_index: usize,
        function_parameter_fragment: String,
    ) {
        let mut parent_record_index: usize;
        if let Some(index) = self.get_parent_node_index(table_index) {
            parent_record_index = index;
        } else {
            error(16);
            unimplemented!();
        }
        self.global_table_graph
            .node_weight_mut(NodeIndex::new(parent_record_index))
            .unwrap()
            .add_function_parameter_fragment_to_record(function_parameter_fragment);
    }
    pub fn remove_node(&mut self, node_index: usize) {
        self.global_table_graph.remove_node(NodeIndex::new(node_index));
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
            error(17);
            unimplemented!();
        }
    }
    pub fn is_node_record(&self, node_index: usize) -> bool {
        self.ensure_node_exists(node_index);
        let node = self.get_node(node_index).unwrap();
        node.is_record()
    }
    pub fn is_node_table(&self, node_index: usize) -> bool {
        self.ensure_node_exists(node_index);
        let node = self.get_node(node_index).unwrap();
        node.is_table()
    }
    pub fn ensure_node_is_record(&self, node_index: usize) {
        self.ensure_node_exists(node_index);
        let node = self.get_node(node_index).unwrap();
        if !node.is_record() {
            error(18);
            unimplemented!();
        }
    }
    pub fn ensure_node_is_table(&self, node_index: usize) {
        self.ensure_node_exists(node_index);
        let node = self.get_node(node_index).unwrap();
        if !node.is_table() {
            error(19);
            unimplemented!();
        }
    }

    // Utility / Debugging
    pub fn print_graph(&self) {
        write_to_symbol_table_log(format!("##########################################################################################"));
        for index in self.global_table_graph.node_indices() {
            write_to_symbol_table_log(format!(
                "{:?} : {:?}",
                index.index(),
                self.global_table_graph.node_weight(index).unwrap()
            ));
            let incoming_edges = self.global_table_graph
                .neighbors_directed(index, Direction::Incoming);
            for incoming_edge in incoming_edges {
                write_to_symbol_table_log(format!("        Incoming edge: {} -> self", incoming_edge.index()));
            }
            let outgoing_edges = self.global_table_graph
                .neighbors_directed(index, Direction::Outgoing);
            for outgoing_edge in outgoing_edges {
                write_to_symbol_table_log(format!("        Outgoing edge: self -> {}", outgoing_edge.index()));
            }
        }
        write_to_symbol_table_log(format!("##########################################################################################"));
    }
}

// STNode
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum STNodeType {
    Table(Vec<String>), // inheritance labels
    Record(Option<STRecord>),
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
    pub fn get_table_inheritance_list(&self) -> Vec<String> {
        match self.node_type {
            STNodeType::Table(ref inheritance_list) => {
                inheritance_list.clone()
            }
            _ => {
                error(19);
                unimplemented!();
            }
        }
    }
    pub fn get_record_identifier(&self) -> Option<String> {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.get_identifier()
        } else {
            None
        }
    }
    pub fn get_record_value_type(&self) -> Option<String>{
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        some_record.unwrap().get_record_value_type()
    }
    pub fn get_record_type(&self) -> STRecordType {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        some_record.unwrap().get_record_type()
    }
    pub fn get_record_array_dimensions(&self) -> Option<Vec<usize>> {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        some_record.unwrap().get_array_dimensions()
    }

    pub fn set_record_identifier(&mut self, identifier: String) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.set_identifier(identifier);
            self.node_type = STNodeType::Record(Some(record));
        }
    }
    pub fn set_record_memory_tag(&mut self, tag: String) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.set_memory_tag(tag);
            self.node_type = STNodeType::Record(Some(record));
        }
    }

    pub fn set_record_value_type(&mut self, value_type: String) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.set_value_type(value_type);
            self.node_type = STNodeType::Record(Some(record));
        }
    }
    pub fn set_record_memory_size(&mut self, memory_size: usize) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.set_memory_size(memory_size);
            self.node_type = STNodeType::Record(Some(record));
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
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.add_function_parameter_fragment(function_parameter_fragment);
            self.node_type = STNodeType::Record(Some(record));
        }
    }
    pub fn add_record_array_dimension(&mut self, array_dimension: usize) {
        let mut some_record: Option<STRecord>;
        match self.node_type {
            STNodeType::Record(ref some_existing_record) => {
                some_record = some_existing_record.clone()
            }
            _ => {
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.add_array_dimension(array_dimension);
            self.node_type = STNodeType::Record(Some(record));
        }
    }

    pub fn add_to_table_inheritance_list(&mut self, new_label: String) {
        let mut inheritance_list: Vec<String>;
        match self.node_type {
            STNodeType::Table(ref existing_list) => {
                inheritance_list = existing_list.clone();
            }
            _ => {
                error(19);
                unimplemented!()
            },

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
                error(18);
                unimplemented!();
            }
        }
        if let Some(mut record) = some_record {
            record.initialize_function_parameters();
            self.node_type = STNodeType::Record(Some(record));
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
    pub fn is_function_record(&self) -> bool {
        match self.node_type {
            STNodeType::Record(ref record) => {
                match record.clone().unwrap().record_type {
                    STRecordType::Function => true,
                    _ => false
                }
            },
            _ => false,
        }
    }
    pub fn is_variable_record(&self) -> bool {
        match self.node_type {
            STNodeType::Record(ref record) => {
                match record.clone().unwrap().record_type {
                    STRecordType::Variable => true,
                    _ => false
                }
            },
            _ => false,
        }
    }
    pub fn is_class_record(&self) -> bool {
        match self.node_type {
            STNodeType::Record(ref record) => {
                match record.clone().unwrap().record_type {
                    STRecordType::Class => true,
                    _ => false
                }
            },
            _ => false,
        }
    }
}

// STRecord
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum STRecordType {
    Class,
    Function,
    Parameter,
    Variable,
}
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct STRecord {
    identifier: Option<String>,
    record_type: STRecordType,
    value_type: Option<String>,
    array_dimensions: Option<Vec<usize>>,
    function_parameters: Option<Vec<String>>,
    memory_size: Option<usize>,
    memory_tag: Option<String>
}
impl STRecord {
    pub fn new_empty_record(record_type: STRecordType) -> STRecord {
        STRecord {
            record_type,
            identifier: None,
            value_type: None,
            array_dimensions: None,
            function_parameters: None,
            memory_size: None,
            memory_tag: None,
        }
    }
    pub fn get_identifier(&self) -> Option<String> {
        self.identifier.clone()
    }
    pub fn get_record_type(&self) -> STRecordType {
        self.record_type.clone()
    }
    pub fn get_record_value_type(&self) -> Option<String> {
        self.value_type.clone()
    }
    pub fn get_array_dimensions(&self) -> Option<Vec<usize>> {
        self.array_dimensions.clone()
    }
    pub fn get_memory_size(&self) -> Option<usize> {
        self.memory_size.clone()
    }
    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = Some(identifier);
    }
    pub fn set_value_type(&mut self, value_type: String) {
        self.value_type = Some(value_type);
    }
    pub fn set_memory_size(&mut self, memory_size: usize) {
        self.memory_size = Some(memory_size);
    }
    pub fn set_memory_tag(&mut self, memory_tag: String) {
        self.memory_tag = Some(memory_tag);
    }
    pub fn add_array_dimension(&mut self, array_dimension: usize) {
        let has_dimensions = self.array_dimensions.is_some();
        if has_dimensions {
            let mut dimensions = self.array_dimensions.clone().unwrap().clone();
            dimensions.push(array_dimension);
            self.array_dimensions = Some(dimensions);
        } else {
            self.array_dimensions = Some(vec![array_dimension]);
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
            list_of_fragments = vec![];
        }
        list_of_fragments.push(function_parameter_fragment);
        self.function_parameters = Some(list_of_fragments)
    }
}
