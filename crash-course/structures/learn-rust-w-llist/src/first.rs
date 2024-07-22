#[derive(Debug)]
struct Node {
    element: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

// implement methods
impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, element: i32) {
        todo!();
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!();
    }

    pub fn peek(&mut self) -> Option<i32> {
        todo!();
    }
}

// implement iterator
