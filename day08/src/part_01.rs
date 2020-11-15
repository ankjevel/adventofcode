use crate::parse_input;
use std::io::Result;

fn query_children(entries: &mut Vec<u8>, result_metadata_entries: &mut Vec<Vec<usize>>) {
    let number_of_child_nodes = entries.remove(0);
    let number_of_metadata_entries = entries.remove(0);

    for _ in 0..number_of_child_nodes {
        query_children(entries, result_metadata_entries);
    }

    let mut metadata_entries: Vec<usize> = Vec::new();
    for _ in 0..number_of_metadata_entries {
        let entry = entries.remove(0) as usize;
        metadata_entries.push(entry);
    }

    result_metadata_entries.push(metadata_entries);
}

pub fn main(input: &str) -> Result<usize> {
    let parsed = parse_input(input.clone());
    let mut metadata_entries: Vec<Vec<usize>> = Vec::new();

    query_children(&mut parsed.clone(), &mut metadata_entries);

    Ok(metadata_entries
        .iter()
        .flatten()
        .fold(0, |acc, curr| acc + curr))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("test_fixture_1.txt");

    #[test]
    fn it_should_get_the_same_answer_as_part_example() {
        assert_eq!(main(EXAMPLE_DATA).unwrap(), 138);
    }
}
