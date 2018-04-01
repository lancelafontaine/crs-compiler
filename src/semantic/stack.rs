// The usize refers to a node index reference in the AST's graph

pub struct SemanticStack {
    stack: Vec<usize>
}

impl SemanticStack {
    pub fn push(&mut self, semantic_symbol: usize) {
        self.stack.push(semantic_symbol);
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

}
