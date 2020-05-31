use kincaid::syllables_in_text;
use kincaid::WORD_PATTERN;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[macro_use]
extern crate lazy_static;

const DICTIONARY_PATH: &str = "tests/cmudict.txt";

const PRONUNCIATION_PATTERN: &str = r"((?:[\w\n]+ ?)+)";
const VOWEL_PATTERN: &str = r"\b[AEIOU]";

lazy_static! {
    pub static ref LINE_REGEX: Regex =
        Regex::new(&format!("{}.*  {}", WORD_PATTERN, PRONUNCIATION_PATTERN)).unwrap();
    pub static ref VOWEL_REGEX: Regex = Regex::new(VOWEL_PATTERN).unwrap();
}

#[test]
fn test_against_cmudict() {
    // The original encoding of the CMU dictionary on their website is ISO 8859-1
    // Save it to UTF-8 when replacing with a new copy
    let file_contents =
        fs::read_to_string(DICTIONARY_PATH).expect("Failed to read the dictionary to a string");

    let mut word_to_valid_syllable_counts: HashMap<&str, HashSet<usize>> = HashMap::new();
    file_contents
        .lines()
        .filter_map(|line| extract_word_and_syllable_count(line))
        .for_each(|(word, syllable_count)| {
            word_to_valid_syllable_counts
                .entry(word)
                .or_insert(HashSet::new())
                .insert(syllable_count);
        });

    let mut mistake_count = 0;
    for (word, expected_syllables) in word_to_valid_syllable_counts {
        let predicted_syllables = syllables_in_text(word);
        if !expected_syllables.contains(&predicted_syllables) {
            mistake_count += 1;
            println!(
                "{}: {:?} (expected) {} (calculated)",
                word, expected_syllables, predicted_syllables
            )
        }
    }

    assert_eq!(mistake_count, 6842);
}

fn extract_word_and_syllable_count(line: &str) -> Option<(&str, usize)> {
    let capture = LINE_REGEX.captures(line)?;

    let word = capture.get(1).unwrap().as_str();
    let phonemes = capture.get(2).unwrap().as_str();
    let syllables = VOWEL_REGEX.find_iter(phonemes).count();

    return Some((word, syllables));
}
