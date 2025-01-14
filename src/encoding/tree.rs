use std::collections::BinaryHeap;

pub struct HuffNode {
    character: char,
    weight: u32,
    left: Option<usize>,
    right: Option<usize>,
}

pub struct HuffmanTree {
    nodes: Vec<HuffNode>,
    pub root: usize
}