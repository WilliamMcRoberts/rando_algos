mod check_for_duplicates;
mod depth_first_search;
mod tests;
use check_for_duplicates::{has_duplicates_map, has_duplicates_set};

use crate::depth_first_search::*;

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 4, 9];

    let strings = vec!["yo", "hello", "hi", "sup", "wasup"];

    println!("vec: {:?}", nums);

    println!("vec: {:?}", strings);

    println!("nums contains_duplicate_set: {}", has_duplicates_set(&nums));

    println!("nums contains_duplicate_map: {}", has_duplicates_map(&nums));

    println!(
        "strings contains_duplicate_set: {}",
        has_duplicates_set(&strings)
    );

    println!(
        "strings contains_duplicate_map: {}",
        has_duplicates_map(&strings)
    );

    let vertices = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7), (4, 9)];

    let root = 1;

    let objective = 9;

    let graph = Graph::new(
        vertices.into_iter().map(|v| v.into()).collect(),
        edges.into_iter().map(|e| e.into()).collect(),
    );

    let result = depth_first_search(&graph, root.into(), objective.into());

    match result {
        Some(path) => println!("path: {:?}", path),
        None => println!("no path found"),
    }
}
