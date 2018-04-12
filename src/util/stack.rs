use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T: Debug + Clone> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { stack: vec![] }
    }

    pub fn push(&mut self, semantic_symbol: T) {
        self.stack.push(semantic_symbol);
    }

    pub fn append(&mut self, vector: &mut Vec<T>) {
        self.stack.append(vector);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn top(&self) -> Option<T> {
        self.stack.last().cloned()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}
