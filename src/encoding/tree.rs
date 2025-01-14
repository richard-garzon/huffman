use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::rc::Rc;

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
}
