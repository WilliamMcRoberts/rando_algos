pub fn is_anagram(s: &str, t: &str) -> bool {
    let mut s = s.to_ascii_lowercase().chars().collect::<Vec<char>>();
    s.sort_unstable();

    let mut t = t.to_ascii_lowercase().chars().collect::<Vec<char>>();
    t.sort_unstable();

    s == t
}
