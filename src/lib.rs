mod utils;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use regex::RegexBuilder;
use regex::RegexSet;
use regex::RegexSetBuilder;
use std::cmp::max;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const WORD_PATTERN: &str = r"\b(\p{L}+(?:[-']\p{L}+)?)\b";
pub const SENTENCE_PATTERN: &str = r"[.?!]+";
pub const VOWEL_GROUPS_PATTERN: &str = r"[aeiou]+";

lazy_static! {
    static ref WORD_REGEX: Regex = regex_builder(WORD_PATTERN);
    static ref SENTENCE_REGEX: Regex = regex_builder(SENTENCE_PATTERN);
    static ref VOWEL_GROUPS_REGEX: Regex = regex_builder(VOWEL_GROUPS_PATTERN);
    static ref DEDUCT_PATTERN_REGEXES: RegexSet = regexset_builder(&[
        r"e\b",
        r"ey\b",
        r"ed\b",
        r"ay\b",
        r"[kmrpbdtnvrw]es\b",
        r"ely\b",
        r"oy\b",
        r"cia",
        r"[aeilouy]le\b",
        r"tia[nl]?\b",
        r"tia([nl]s)?\b",
        r"[aeo]ym",
        r"eness\b",
        r"\bfore",
        r"ay[bclntrw]",
        r"ement",
        r"iles\b",
        r"[ao]les\b",
        r"eman\b",
        r"aying\b",
        r"oy[cln]",
        r"eful",
        r"\bey",
        r"geon",
        r"\bhome",
        r"eyn",
        r"ically",
        r"eless",
        r"sian\b",
        r"yles",
        r"\bwhite",
        r"eway",
        r"georg",
        r"lles\b",
        r"busine",
        r"illia",
        r"ules\b",
        r"\bhym",
        r"ryst",
        r"eyl",
        r"ehou",
        r"eyw",
        r"ekeep",
        r"people",
        r"every",
        r"\blife",
        r"giu",
        r"eyin",
        r"eout",
        r"oying\b",
        r"gues\b",
        r"\breine",
        r"geou",
        r"ques\b",
        r"vior",
        r"sewo",
        r"oseb",
        r"eyc",
        r"\bspace",
        r"\bstone",
        r"eover",
        r"ehol",
        r"iliar",
        r"estone",
        r"eyb",
        r"oyk",
        r"velan",
        r"piet",
        r"\bgia",
        r"somet",
        r"esvil",
        r"lyst",
        r"arriag",
        r"gior"
    ]);
    static ref ADD_PATTERN_REGEXES: RegexSet = regexset_builder(&[
        r"y\b",
        r"ia",
        r"\bmc",
        r"[il]e\b",
        r"ted\b",
        r"ee\b",
        r"io\b",
        r"ded\b",
        r"[io]er\b",
        r"y[bckglmnrstwxv]",
        r"sms?\b",
        r"eo",
        r"[eior]ed\b",
        r"iol",
        r"\bhy",
        r"iu",
        r"s'",
        r"oe\b",
        r"iot",
        r"tua",
        r"aue",
        r"ea\b",
        r"iest\b",
        r"ios",
        r"yst",
        r"nte\b",
        r"ce's",
        r"ying\b",
        r"[bcdfgkopt]led\b",
        r"ciat",
        r"lement",
        r"typ",
        r"ly[dehops]",
        r"[drv]ious",
        r"z's\b",
        r"ae\b",
        r"io[mpr]",
        r"tre\b",
        r"ione\b",
        r"[cdehlorn]ue\b",
        r"se's",
        r"nua",
        r"x'",
        r"oing",
        r"yz",
        r"creat",
        r"lua",
        r"iod",
        r"\breass",
        r"eing\b",
        r"dua",
        r"[bdprz]ion",
        r"iello\b",
        r"oa\b",
        r"ge's",
        r"phys",
        r"eact",
        r"ioc",
        r"iog",
        r"scien",
        r"dys",
        r"uou",
        r"\brein",
        r"ienn",
        r"rya",
        r"bre\b",
        r"tke\b",
        r"ryd",
        r"sh's\b",
        r"rua",
        r"ryp",
        r"rient",
        r"uing",
        r"xual",
        r"eely\b",
        r"leman\b",
        r"fluen",
        r"he'",
        r"dre\b",
        r"iet",
        r"loui",
        r"dl\b",
        r"\bio",
        r"rys",
        r"tui",
        r"rye",
        r"\bcoe",
        r"\breali",
        r"ntes\b",
        r"ch'",
        r"mye",
        r"eeman\b",
        r"ryo",
        r"linea",
        r"theat",
        r"reapp",
        r"oers\b",
        r"tys",
        r"\bcyp",
        r"eemp",
        r"nys",
        r"aic\b",
        r"cua",
        r"tl\b",
        r"tres\b",
        r"ciano",
        r"lione",
        r"eand",
        r"\bdya",
        r"gyp",
        r"croat",
        r"heroi",
        r"rearr",
        r"eex",
        r"cre\b",
        r"oniou",
        r"eum\b",
        r"fred\b",
        r"dien",
        r"oua",
        r"oincid",
        r"coordi",
        r"nucle",
        r"nyd",
        r"\breen",
        r"\breun",
        r"bys",
        r"iale\b",
        r"ifiers",
        r"rean",
        r"pre\b",
        r"iore\b",
        r"-in\b",
    ]);
}

fn regex_builder(pattern: &str) -> Regex {
    RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()
        .unwrap()
}

fn regexset_builder(patterns: &[&str]) -> RegexSet {
    RegexSetBuilder::new(patterns)
        .case_insensitive(true)
        .build()
        .unwrap()
}

#[wasm_bindgen]
pub fn word_count(text: &str) -> usize {
    WORD_REGEX.find_iter(text).count()
}

#[wasm_bindgen]
pub fn sentence_count(text: &str) -> usize {
    max(SENTENCE_REGEX.find_iter(text).count(), 1)
}

#[wasm_bindgen]
pub fn syllables_in_text(text: &str) -> usize {
    WORD_REGEX
        .find_iter(text)
        .map(|word| syllables_in_word(word.as_str()))
        .sum()
}

fn syllables_in_word(word: &str) -> usize {
    let vowel_groups: usize = VOWEL_GROUPS_REGEX.find_iter(word).count();
    let add_count: usize = ADD_PATTERN_REGEXES.matches(word).iter().count();
    let minus_count: usize = DEDUCT_PATTERN_REGEXES.matches(word).iter().count();

    if vowel_groups + add_count < minus_count + 1 {
        return 1;
    }
    return vowel_groups + add_count - minus_count;
}

#[wasm_bindgen]
pub fn flesh_kincaid_reading_ease_score(text: &str) -> f32 {
    let word_count = word_count(text);
    let sentence_count = sentence_count(text);
    let syllables_in_text = syllables_in_text(text);

    206.835
        - 1.015 * word_count as f32 / sentence_count as f32
        - 84.6 * syllables_in_text as f32 / word_count as f32
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
        assert_eq!(syllables_in_text("Zylka"), 2);
    }

    #[test]
    fn test_syllables_in_word() {
        assert_eq!(syllables_in_word("unaware"), 3);
        assert_eq!(syllables_in_word("sum"), 1);
        assert_eq!(syllables_in_word("some"), 1);
        assert_eq!(syllables_in_word("pernicious"), 3);
        assert_eq!(syllables_in_word("egregious"), 3);
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("sample text"), 2);
        assert_eq!(word_count("$5 only"), 1);
        assert_eq!(word_count("This is noted in the book(1)"), 6);
        assert_eq!(
            word_count(
                "A sentence that's long—very-long, and contains some real world stuff;funny!"
            ),
            12
        );
    }
}
