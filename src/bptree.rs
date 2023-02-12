use std::ptr::NonNull;

#[derive(Debug)]
enum BpTreeNode {
    InnerNode(InnerNode),
    LeafNode(LeafNode),
}

#[derive(Debug)]
struct InnerNode {
    children: Vec<BpTreeNode>,
    keys: Vec<i32>,
}

#[derive(Debug)]
struct LeafNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    next: Option<NonNull<LeafNode>>,
    prev: Option<NonNull<LeafNode>>,
}

pub struct BpTree {
    root: BpTreeNode,
}

impl BpTree {
    pub fn new() {
        todo!()
    }
}
