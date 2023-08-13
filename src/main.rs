mod anagram;
mod auto_complete_using_trie;
mod binary_search;
mod binary_search_tree;
mod breadth_first_search;
mod check_for_duplicates;
mod depth_first_search;
mod hamming_distance;
mod linked_list;
mod palindrome;
mod run_length_encoding;
mod two_sum;
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
        "Hellooooo, my name is Tom. I liiiiive at 1234444 Jacksooon Laaane. IiiiiI like   to swim.
         I like tooo read books.",
    );

    println!("res: {:?}", res);

    let decoded = run_length_decode(&res);

    println!("decoded: {}", decoded);

    let str = "racecar";

    let is_palindrome = palindrome::is_palindrome(str);

    println!("{} is palindrome: {}", str, is_palindrome);

    let pref = "ap";

    let words = vec![
        "apple".to_owned(),
        "ape".to_owned(),
        "apricot".to_owned(),
        "apocalypse".to_owned(),
        "yo".to_owned(),
        "alien".to_owned(),
    ];

    let mut auto_complete = auto_complete_using_trie::Autocomplete::new();

    auto_complete.insert_words(words);

    let results = auto_complete.find_words(pref.to_owned());

    println!("results: {:?}", results);

    let str1 = "hello";
    let str2 = "jelly";

    let distance = hamming_distance::hamming_distance(str1, str2);

    println!("distance: {}", distance);

    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let target = 7;

    let index = binary_search::binary_search(&target, &items);

    match index {
        Some(i) => println!("found {} at index {}", target, i),
        None => println!("{} not found", target),
    }
}
