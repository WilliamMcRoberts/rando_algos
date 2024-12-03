pub fn run_length_encode(text: &str) -> Vec<(char, i32)> {
    let mut count = 1;
    let mut encoded: Vec<(char, i32)> = vec![];
    let text_len = text.len();

    for (i, c) in text.chars().enumerate() {
        let next_idx = i + 1;
        if next_idx < text_len && c == text.chars().nth(next_idx).unwrap() {
            count += 1;
        } else {
            encoded.push((c, count));
            count = 1;
        }
    }

    encoded
}

pub fn run_length_decode(encoded: &[(char, i32)]) -> String {
    encoded
        .iter()
        .map(|x| (x.0).to_string().repeat(x.1 as usize))
        .collect::<String>()
}

#[test]
fn test_run_length_decode() {
    let res = run_length_decode(&[('A', 0)]);
    assert_eq!(res, "");
    let res = run_length_decode(&[('B', 1)]);
    assert_eq!(res, "B");
    let res = run_length_decode(&[('A', 5), ('z', 3), ('B', 1)]);
    assert_eq!(res, "AAAAAzzzB");
}

#[test]
fn test_run_length_encode() {
    let res = run_length_encode("");
    assert_eq!(res, []);

    let res = run_length_encode("A");
    assert_eq!(res, [('A', 1)]);

    let res = run_length_encode("YOOOOOOOOOO");
    assert_eq!(res, [('Y', 1), ('O', 10)]);

    let res = run_length_encode("HELLO");
    assert_eq!(res, [('H', 1), ('E', 1), ('L', 2), ('O', 1)]);

    let res = run_length_encode("Rust-Trends");
    assert_eq!(
        res,
        [
            ('R', 1),
            ('u', 1),
            ('s', 1),
            ('t', 1),
            ('-', 1),
            ('T', 1),
            ('r', 1),
            ('e', 1),
            ('n', 1),
            ('d', 1),
            ('s', 1)
        ]
    );
}
