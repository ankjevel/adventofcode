use crate::parse_input;
use std::{io::Result, str};

struct Node {
    size: usize,
    children: Vec<Node>,
    metadata: Vec<u128>,
}

impl Node {
    fn new(input: &[u128]) -> Option<Node> {
        if input.len() < 2 {
            return None;
        }

        let child_nodes = input[0];
        let metadata_entres = input[1];

        let mut node = Node {
            size: 2,
            children: Vec::new(),
            metadata: Vec::new(),
        };

        for _ in 0..child_nodes {
            if let Some(child) = Node::new(&input[node.size..]) {
                node.size += child.size;
                node.children.push(child);
            }
        }

        for _ in 0..metadata_entres {
            if let Some(&meta) = input.get(node.size) {
                if meta > 0 {
                    node.metadata.push(meta);
                    node.size += 1;
                }
            }
        }

        Some(node)
    }

    fn value(&self) -> u128 {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }

        self.metadata.iter().fold(0, |sum, index| {
            if let Some(child) = self.children.get(*index as usize - 1) {
                sum + child.value()
            } else {
                sum
            }
        })
    }
}

pub fn main(input: &str) -> Result<u128> {
    let entries = parse_input(input)
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<u128>>();
    Ok(Node::new(&entries).unwrap().value())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("test_fixture.txt");

    #[test]
    fn it_should_get_the_value_of_the_root_node() {
        assert_eq!(main(EXAMPLE_DATA).unwrap(), 66);
    }
}
