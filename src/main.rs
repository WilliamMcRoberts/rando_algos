mod check_for_duplicates;
mod depth_first_search;
mod run_length_encoding;
mod tests;
use check_for_duplicates::{has_duplicates_map, has_duplicates_set};

use crate::{
    depth_first_search::*,
    run_length_encoding::{run_length_decode, run_length_encode},
};

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

    let res = run_length_decode(&[('A', 0)]);

    println!("res: {}", res);

    let res = run_length_decode(&[('B', 1)]);

    println!("res: {}", res);

    let res = run_length_decode(&[('A', 5), ('z', 3), ('B', 1)]);

    println!("res: {}", res);

    let res = run_length_encode("");

    println!("res: {:?}", res);

    let res = run_length_encode("A");

    println!("res: {:?}", res);

    let res = run_length_encode("AA");

    println!("res: {:?}", res);

    let res = run_length_encode("AAAABBBCCDAA");

    println!("res: {:?}", res);

    let res = run_length_encode(
        "Hellooooo, my name is Tom. I liiiiive at 1234444 Jacksooon Laaane. IiiiiI like   to swim. I like tooo read books.",
    );

    println!("res: {:?}", res);

    let decoded = run_length_decode(&res);

    println!("decoded: {}", decoded);
}
