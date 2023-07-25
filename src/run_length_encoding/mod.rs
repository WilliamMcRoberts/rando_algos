pub fn run_length_encode(text: &str) -> Vec<(char, i32)> {
    let mut count = 1;
    let mut encoded: Vec<(char, i32)> = vec![];

    for (i, c) in text.chars().enumerate() {
        if i + 1 < text.len() && c == text.chars().nth(i + 1).unwrap() {
            count += 1;
        } else {
            encoded.push((c, count));
            count = 1;
        }
    }

    encoded
}

pub fn run_length_decode(encoded: &[(char, i32)]) -> String {
    let res = encoded
        .iter()
        .map(|x| (x.0).to_string().repeat(x.1 as usize))
        .collect::<String>();

    res
}
