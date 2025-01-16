use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

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
    pub fn new(character: char, weight: u32) -> HuffNode {
        HuffNode {
            character: Some(character),
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
        let new_node = HuffNode::new(test_char, test_weight);

        assert_eq!(new_node.character.unwrap(), test_char);
        assert_eq!(new_node.weight, test_weight);
    }

    #[test]
    fn test_huffnode_ordering() {
        let heavy_node = HuffNode::new('a', 100);
        let light_node = HuffNode::new('b', 50);

        // Reversed so that BinaryHeap is a min-heap
        assert!(light_node > heavy_node);
    }

    #[test]
    fn test_min_heap() {
        let mut min_heap = BinaryHeap::new();
        let heavy_node = HuffNode::new('a', 100);
        let light_node = HuffNode::new('b', 50);

        min_heap.push(&heavy_node);
        min_heap.push(&light_node);

        assert_eq!(min_heap.len(), 2);
        assert_eq!(min_heap.pop().unwrap(), Some(&light_node).unwrap());
        assert_eq!(min_heap.pop().unwrap(), Some(&heavy_node).unwrap());
    }
}
