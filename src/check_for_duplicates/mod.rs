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
