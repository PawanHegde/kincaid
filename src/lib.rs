use std::collections::HashSet;

const DELIMITERS: [char; 7] = ['.', '!', '?', ';', ',', '–', '-'];

fn get_words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| c.is_whitespace() || DELIMITERS.contains(&c))
        .filter(|word| !word.is_empty())
}

pub fn word_count(text: &str) -> usize {
    get_words(text).count()
}

pub fn sentence_count(text: &str) -> usize {
    text.split_terminator('.').count()
}

pub fn syllables_in_text(text: &str) -> usize {
    get_words(text).map(&syllables_in_word).sum()
}

fn syllables_in_word(word: &str) -> usize {
    let mut syllables = 0;
    let mut previous_was_vowel = false;
    for letter in word.chars() {
        if is_vowel(letter) {
            if previous_was_vowel == false {
                syllables = syllables + 1;
            }
            previous_was_vowel = true;
        } else {
            previous_was_vowel = false;
        }
    }

    if word.ends_with('e') {
        syllables = syllables - 1;
    }

    if syllables > 0 {
        syllables
    } else {
        1
    }
}

fn is_vowel(character: char) -> bool {
    let vowels: HashSet<&char> = ['a', 'e', 'i', 'o', 'u'].iter().collect();
    vowels.contains(&character)
}

pub fn flesh_kincaid_reading_ease_score(text: &str) -> f32 {
    206.835
        - 1.015 * word_count(text) as f32 / sentence_count(text) as f32
        - 84.6 * syllables_in_text(text) as f32 / word_count(text) as f32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_syllables_in_text() {
        assert_eq!(syllables_in_text(""), 0);
        assert_eq!(syllables_in_text("Hello"), 2);
        assert_eq!(syllables_in_text("Hello World"), 3);
        assert_eq!(syllables_in_text("Test-case"), 2);
        assert_eq!(syllables_in_text("Hello, World! This is a test"), 7);
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("sample text"), 2);
        assert_eq!(word_count("$5 only"), 2);
        assert_eq!(
            word_count(
                "A sentence that's long–very-long, and contains some real world stuff;funny!"
            ),
            13
        );
    }

    #[test]
    fn test_is_vowel() {
        assert_eq!(is_vowel('u'), true)
    }

    #[test]
    fn test_syllables_in_word() {
        assert_eq!(syllables_in_word("sum"), 1);
        assert_eq!(syllables_in_word("some"), 1);
        assert_eq!(syllables_in_word("pernicious"), 3);
        assert_eq!(syllables_in_word("egregious"), 3);
    }

    #[test]
    fn test_flesh_kincaid_reading_ease_score() {
        let test_string = "This is some, long text!";
        assert_eq!(flesh_kincaid_reading_ease_score(&test_string), 117.16001);
    }
}
