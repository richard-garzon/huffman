use super::frequency::Freq;
use std::collections::BinaryHeap;

pub fn generate_tree(freq: &Freq) -> Option<Box<HuffNode>> {
    let mut min_heap = BinaryHeap::new();

    for (character, weight) in freq.counter.iter() {
        let curr_node = Box::new(HuffNode::new(Some(*character), *weight));
        // let curr_node = Rc::new(RefCell::new(HuffNode::new(Some(character), weight)));
        min_heap.push(curr_node);
    }

    while min_heap.len() > 1 {
        let first = min_heap.pop().unwrap();
        let second = min_heap.pop().unwrap();

        let mut new_node = Box::new(HuffNode::new(None, first.weight + second.weight));

        new_node.left = Some(first);
        new_node.right = Some(second);

        min_heap.push(new_node);
    }

    min_heap.pop()
}

#[derive(Debug)]
pub struct HuffNode {
    pub character: Option<char>,
    pub weight: u32,
    pub left: Option<Box<HuffNode>>,
    pub right: Option<Box<HuffNode>>,
}

/* make this cache friendly later
pub struct HuffmanTree {
    nodes: Vec<HuffNode>,
    pub root: usize
}
*/

impl HuffNode {
    pub fn new(character: Option<char>, weight: u32) -> Self {
        Self {
            character,
            weight,
            left: None,
            right: None,
        }
    }
}

// Implementing these traits so that nodes in min-heap
// order as they are added to the BinaryHeap
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
        let cmp_result = other.weight.cmp(&self.weight);

        if cmp_result != std::cmp::Ordering::Equal {
            // weights aren't equal, return cmp result to get min-heap by weight behavior
            return cmp_result;
        }

        match (self.character, other.character) {
            // weights are equal, so we order lexicographically by character if possible
            (Some(c1), Some(c2)) => c2.cmp(&c1), // Returns Less if c2 < c1
            (Some(_), None) => std::cmp::Ordering::Greater, // Any char value is greater than no char value
            (None, Some(_)) => std::cmp::Ordering::Less, // None value is Less than some char value
            (None, None) => std::cmp::Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encoding::test_cases;

    use super::*;

    pub fn verify_tree(node: &Option<Box<HuffNode>>, freq: &Freq) {
        let curr_node = node.as_ref().unwrap();
        println!("printing node:");
        println!("{:?}", curr_node);
        if let Some(character) = curr_node.character {
            assert_eq!(freq.counter.get(&character).unwrap(), &curr_node.weight)
        } else {
            let left = curr_node.left.as_ref().unwrap();
            let right = curr_node.right.as_ref().unwrap();

            let left_weight = left.weight;
            let right_weight = right.weight;

            assert_eq!(curr_node.weight, left_weight + right_weight);

            verify_tree(&curr_node.left, &freq);
            verify_tree(&curr_node.right, &freq);
        }
    }
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

        let root = generate_tree(&freq);

        assert_eq!(root.as_ref().unwrap().character.unwrap(), 'a');
        assert_ne!(root.as_ref().unwrap().character.unwrap(), 'b');
        assert_eq!(root.as_ref().unwrap().weight, 3);
        assert_eq!(root.as_ref().unwrap().left, None);
        assert_eq!(root.as_ref().unwrap().right, None);

        verify_tree(&root, &freq);
    }

    #[test]
    fn test_two_node_tree() {
        let mut freq = Freq::new();
        let test_input = "acc".as_bytes();

        freq.update(test_input);

        let root = generate_tree(&freq);

        assert_eq!(root.as_ref().unwrap().weight, 3);
        assert_eq!(
            root.as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .character
                .unwrap(),
            'a'
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .character
                .unwrap(),
            'c'
        );
        assert_eq!(root.as_ref().unwrap().left.as_ref().unwrap().weight, 1);
        assert_eq!(root.as_ref().unwrap().right.as_ref().unwrap().weight, 2);
    }

    #[test]
    fn test_three_node_tree() {
        let mut freq = Freq::new();
        let test_input = "aaabbcd".as_bytes();

        freq.update(test_input);

        let root = generate_tree(&freq);

        verify_tree(&root, &freq);
    }

    // running with no capture shows that generate tree isn't determinate!
    // write a test to lock in an order and fix
    #[test]
    fn test_tree_no_duplicate_characters() {
        let mut freq = Freq::new();
        let test_input = "abcd".as_bytes();

        freq.update(test_input);

        let root = generate_tree(&freq);

        verify_tree(&root, &freq);
    }

    #[test]
    fn test_tree_bigger_string() {
        let mut freq = Freq::new();
        let test_input = test_cases::SAMPLE_TEST;
        freq.update(test_input.as_bytes());

        let root = generate_tree(&freq);

        verify_tree(&root, &freq);
    }
}
