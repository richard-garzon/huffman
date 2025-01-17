use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

use super::frequency::Freq;

pub fn generate_tree(freq: Freq) -> Rc<RefCell<HuffNode>> {
    let mut min_heap = BinaryHeap::new();

    for (character, weight) in freq.counter.into_iter() {
        let curr_node = Rc::new(RefCell::new(HuffNode::new(Some(character), weight)));
        min_heap.push(curr_node);
    }

    while min_heap.len() > 1 {
        let first = min_heap.pop().unwrap();
        let second = min_heap.pop().unwrap();

        let new_node = Rc::new(RefCell::new(HuffNode::new(
            None,
            first.borrow().weight + second.borrow().weight,
        )));

        new_node.borrow_mut().left = Some(Rc::clone(&first));
        new_node.borrow_mut().right = Some(Rc::clone(&second));

        min_heap.push(new_node);
    }

    min_heap.pop().unwrap()
}

#[derive(Debug)]
pub struct HuffNode {
    character: Option<char>,
    weight: u32,
    left: Option<Rc<RefCell<HuffNode>>>,
    right: Option<Rc<RefCell<HuffNode>>>,
    parent: Option<Rc<RefCell<HuffNode>>>,
}

/* make this cache friendly later
pub struct HuffmanTree {
    nodes: Vec<HuffNode>,
    pub root: usize
}
*/

impl HuffNode {
    pub fn new(character: Option<char>, weight: u32) -> HuffNode {
        HuffNode {
            character: character,
            weight: weight,
            left: None,
            right: None,
            parent: None,
        }
    }
}

impl PartialEq for HuffNode {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for HuffNode {}

impl PartialOrd for HuffNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let test_char = 'a';
        let test_weight = 400;
        let new_node = HuffNode::new(Some(test_char), test_weight);

        assert_eq!(new_node.character.unwrap(), test_char);
        assert_eq!(new_node.weight, test_weight);
    }

    #[test]
    fn test_huffnode_ordering() {
        let heavy_node = HuffNode::new(Some('a'), 100);
        let light_node = HuffNode::new(Some('b'), 50);

        // Reversed so that BinaryHeap is a min-heap
        assert!(light_node > heavy_node);
    }

    #[test]
    fn test_min_heap() {
        let mut min_heap = BinaryHeap::new();
        let heavy_node = HuffNode::new(Some('a'), 100);
        let light_node = HuffNode::new(Some('b'), 50);

        min_heap.push(&heavy_node);
        min_heap.push(&light_node);

        assert_eq!(min_heap.len(), 2);
        assert_eq!(min_heap.pop().unwrap(), Some(&light_node).unwrap());
        assert_eq!(min_heap.pop().unwrap(), Some(&heavy_node).unwrap());
    }

    #[test]
    fn test_single_node_tree() {
        let mut freq = Freq::new();
        let test_input = "aaa".as_bytes();

        freq.update(test_input);

        let root = generate_tree(freq);

        assert_eq!(root.borrow().character.unwrap(), 'a');
        assert_ne!(root.borrow().character.unwrap(), 'b');
        assert_eq!(root.borrow().weight, 3);
        assert_eq!(root.borrow().left, None);
        assert_eq!(root.borrow().right, None);
    }
}
