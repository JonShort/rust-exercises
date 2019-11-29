use regex::Regex;

lazy_static! {
    static ref FIRST_CHAR_VOWEL_REGEX: Regex = Regex::new(r"^[aeiou].*$").unwrap();
}

pub fn to_pig_latin(provided: &str) -> String {
    let individual_words = provided.split_whitespace();

    let converted_words: Vec<String> = individual_words
        .map(|word| word_to_pig_latin(&word.to_lowercase()))
        .collect();

    converted_words.join(" ")
}

fn word_to_pig_latin(word: &str) -> String {
    let is_first_char_vowel = FIRST_CHAR_VOWEL_REGEX.is_match(word);

    if is_first_char_vowel {
        return format!("{}-hay", word);
    }

    let word_parts = word.split_at(1);
    format!("{}-{}ay", word_parts.1, word_parts.0)
}
