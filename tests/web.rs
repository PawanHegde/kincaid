//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use kincaid::{syllables_in_text, word_count};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(syllables_in_text("Hello World"), 3);
    assert_eq!(word_count("This is noted in the book(1)"), 6);
}
