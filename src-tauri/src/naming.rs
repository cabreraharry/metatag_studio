#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_basics() {
        assert_eq!(slugify("Modern Hallandale Condo!", 8), "modern-hallandale-condo");
        assert_eq!(slugify("UPPER lower MiXeD", 8), "upper-lower-mixed");
        assert_eq!(slugify("a,b,,c", 8), "a-b-c");
    }

    #[test]
    fn slugify_word_cap() {
        assert_eq!(
            slugify("one two three four five six seven eight nine ten", 4),
            "one-two-three-four"
        );
    }
}
