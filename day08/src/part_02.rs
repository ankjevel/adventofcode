use crate::parse_input;
use std::{io::Result, sync::Mutex};

struct Entry {
    index: usize,
    children: Vec<usize>,
    entries: Vec<u128>,
}

struct Query<'a> {
    entries: &'a mut Vec<u128>,
    metadata: &'a mut Vec<Entry>,
}

fn query_children<F>(query: Query, mut next: F) -> usize
where
    F: FnMut() -> usize + Copy,
{
    let number_of_child_nodes = query.entries.remove(0);
    let number_of_metadata_entries = query.entries.remove(0);
    let index = next();

    let mut children = Vec::new();
    for _ in 0..number_of_child_nodes {
        children.push(query_children(
            Query {
                entries: query.entries,
                metadata: query.metadata,
            },
            next,
        ));
    }

    let mut entries = Vec::new();
    for _ in 0..number_of_metadata_entries {
        entries.push(query.entries.remove(0));
    }

    query.metadata.push(Entry {
        index,
        children,
        entries,
    });

    query.metadata.len()
}

fn sum_of_entries(index: usize, entry: &Entry, entries: &Vec<Entry>) -> u128 {
    let current_index = entry.index;
    let index_to_find = current_index + index as usize;

    if index_to_find >= entries.len() {
        return 0;
    }

    let child = entries.get(index_to_find).unwrap();
    if child.children.len() > 0 {
        sum_of_entries(child.index as usize, child, entries)
    } else {
        child.entries.clone().into_iter().sum()
    }
}

pub fn main(input: &str) -> Result<u128> {
    let entries = parse_input(input)
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<u128>>();
    let mut metadata = Vec::new();
    let current_index = &Mutex::new(0);
    query_children(
        Query {
            entries: &mut entries.clone(),
            metadata: &mut metadata,
        },
        move || {
            let mut index = current_index.lock().unwrap();
            *index += 1;
            index.clone() - 1
        },
    );

    metadata.sort_by(|a, b| a.index.cmp(&b.index));

    let first = metadata.first().unwrap();
    Ok(first.entries.iter().fold(0, |acc, index| {
        acc + sum_of_entries(index.to_owned() as usize, &first, &metadata)
    }))
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
