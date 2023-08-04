use std::collections::{HashMap, HashSet};

pub fn has_duplicates_set<T: Eq + std::hash::Hash>(items: &[T]) -> bool {
    let mut set = HashSet::new();
    for item in items {
        match set.insert(item) {
            false => return true,
            true => (),
        };
    }
    false
}

pub fn has_duplicates_map<T: Copy + Eq + std::hash::Hash>(items: &[T]) -> bool {
    let mut map = HashMap::<T, usize>::new();
    for (i, item) in items.iter().enumerate() {
        match map.insert(*item, i) {
            Some(_) => return true,
            None => (),
        };
    }
    false
}

#[test]
fn test_has_duplicates_set_with_integers() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 2];
    let arr3 = [4, 4, 4, 4, 4];
    let arr4: [i32; 0] = [];

    assert_eq!(has_duplicates_set(&arr1), false);
    assert_eq!(has_duplicates_set(&arr2), true);
    assert_eq!(has_duplicates_set(&arr3), true);
    assert_eq!(has_duplicates_set(&arr4), false);
}

#[test]
fn test_has_duplicates_set_with_strings() {
    let arr1 = ["apple", "banana", "orange"];
    let arr2 = ["apple", "banana", "apple", "orange"];
    let arr3 = ["apple", "apple", "apple", "apple"];
    let arr4: [&str; 0] = [];

    assert_eq!(has_duplicates_set(&arr1), false);
    assert_eq!(has_duplicates_set(&arr2), true);
    assert_eq!(has_duplicates_set(&arr3), true);
    assert_eq!(has_duplicates_set(&arr4), false);
}

#[test]
fn test_has_duplicates_map_with_integers() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 2];
    let arr3 = [4, 4, 4, 4, 4];
    let arr4: [i32; 0] = [];

    assert_eq!(has_duplicates_map(&arr1), false);
    assert_eq!(has_duplicates_map(&arr2), true);
    assert_eq!(has_duplicates_map(&arr3), true);
    assert_eq!(has_duplicates_map(&arr4), false);
}

#[test]
fn test_has_duplicates_map_with_strings() {
    let arr1 = ["apple", "banana", "orange"];
    let arr2 = ["apple", "banana", "apple", "orange"];
    let arr3 = ["apple", "apple", "apple", "apple"];
    let arr4: [&str; 0] = [];

    assert_eq!(has_duplicates_map(&arr1), false);
    assert_eq!(has_duplicates_map(&arr2), true);
    assert_eq!(has_duplicates_map(&arr3), true);
    assert_eq!(has_duplicates_map(&arr4), false);
}
