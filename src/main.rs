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
mod quick_sort;
mod run_length_encoding;
mod sorting;
mod two_sum;

use std::sync::Arc;

use anagram::angrm::is_anagram;
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
        "Hellooooo, my name is Tom. 
            I liiiiive at 1234444 Jacksooon Laaane. 
            IiiiiI like   to swim.
             I like tooo read books.",
    );

    println!("res: {:?}", res);

    let decoded = run_length_decode(&res);

    println!("decoded: {}", decoded);

    let str = "racecar";

    let is_palindrome = palindrome::is_palindrome(str);

    println!("{} is palindrome: {}", str, is_palindrome);

    let pref = "ap";

    let words = vec!["apple", "ape", "apricot", "apocalypse", "yo", "alien"];

    let mut auto_complete = auto_complete_using_trie::Autocomplete::new();

    auto_complete.insert_words(&words);

    let results = auto_complete.find_words(pref);

    println!("results: {:?}", results);

    let str1 = "hello";
    let str2 = "jelly";

    let distance = hamming_distance::hamming_distance(str1, str2);

    println!("distance: {}", distance);

    let items = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

    let target = 7;

    if let Some(i) = binary_search::binary_search(&target, items) {
        println!("found {} at index {}", target, i);
    } else {
        println!("{} not found", target);
    }

    let s1 = "anagram";

    let s2 = "nagaraM";

    let is_anagram = is_anagram(s1, s2);

    println!("{} and {} are anagrams: {}", s1, s2, is_anagram);

    let a = "yooooooooo".to_string();
    let b = "what up".to_string();

    let c = get_biggest(&a, &b);

    println!("Biggest: {}", c);

    println!("A and B: {}, {}", a, b);
}

fn get_biggest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() < b.len() {
        return b;
    } else {
        return a;
    }
}

struct MyId(String);

impl From<MyId> for String {
    fn from(MyId(value): MyId) -> Self {
        value
    }
}

#[allow(dead_code)]
pub struct Person {
    id: Option<u32>,
    name: String,
    age: u32,
}
