pub fn slugify(input: &str, max_words: usize) -> String {
    let lowered = input.to_lowercase();
    let words: Vec<String> = lowered
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| {
            w.chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>()
        })
        .filter(|w| !w.is_empty())
        .take(max_words)
        .collect();
    words.join("-")
}
