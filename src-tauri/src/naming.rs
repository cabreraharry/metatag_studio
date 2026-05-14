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

/// Build the output filename stem from the user's metadata, with optional prefix.
/// Falls back from alt → title → original stem when each is empty or slugs to nothing.
pub fn build_output_stem(alt: &str, title: &str, fallback_stem: &str, prefix: &str) -> String {
    let body = first_non_empty_slug(&[alt, title, fallback_stem]);
    let prefix_slug = slugify(prefix, 4);
    if prefix_slug.is_empty() {
        body
    } else if body.is_empty() {
        prefix_slug
    } else {
        format!("{prefix_slug}-{body}")
    }
}

fn first_non_empty_slug(sources: &[&str]) -> String {
    for s in sources {
        let slug = slugify(s, 8);
        if !slug.is_empty() {
            return slug;
        }
    }
    String::new()
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

    #[test]
    fn slugify_strips_non_ascii() {
        // Naive ASCII filter — café becomes "caf" (accent stripped, e dropped because
        // it was part of a non-ASCII codepoint). This is good enough for v1; if we
        // need diacritic flattening later, swap in the `deunicode` crate.
        let s = slugify("Café Modern", 8);
        assert!(s.contains("modern"));
        assert!(!s.contains('é'));
    }

    #[test]
    fn build_stem_uses_alt_first() {
        assert_eq!(
            build_output_stem("Living Room", "Title", "photo123", ""),
            "living-room"
        );
    }

    #[test]
    fn build_stem_falls_back() {
        assert_eq!(
            build_output_stem("", "", "WhatsApp Image", ""),
            "whatsapp-image"
        );
    }

    #[test]
    fn build_stem_prefix() {
        assert_eq!(
            build_output_stem("Office Interior", "", "x", "Hallandale"),
            "hallandale-office-interior"
        );
    }

    #[test]
    fn build_stem_prefix_only() {
        assert_eq!(build_output_stem("", "", "", "Hallandale"), "hallandale");
    }
}
