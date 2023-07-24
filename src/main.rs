mod check_for_duplicates;
mod tests;
use check_for_duplicates::{has_duplicates_map, has_duplicates_set};

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

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
}
