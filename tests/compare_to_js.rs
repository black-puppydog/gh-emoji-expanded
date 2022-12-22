use node_emoji;
use once_cell::sync::Lazy;
use pretty_assertions::assert_eq;
use serde_json;
use std::{collections::HashSet, fs};

const NAMES_FILE: &str = "tests/emoji.json";

fn read_json_file(filename: &str) -> serde_json::Value {
    let file =
        fs::File::open(filename).expect(&format!("Could not open {} for JSON parsing.", filename));
    serde_json::from_reader(file).expect(&format!("Found invalid JSON in {}", filename))
}

fn generate_tests() -> Vec<(String, String)> {
    let names_file = read_json_file(NAMES_FILE);
    names_file
        .as_object()
        .unwrap()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
        .collect()
}

static TEST_CASES: Lazy<Vec<(String, String)>> = Lazy::new(generate_tests);
static SHORTCODES: Lazy<HashSet<String>> = Lazy::new(|| {
    TEST_CASES
        .iter()
        .map(|(shortcode, _)| shortcode.clone())
        .collect()
});

// This somewhat involved solution lets us define the number of tests exactly
// once, namely in the getLimit macro. That way we can make sure that if the
// number of tests in the input files changes (up or down) we get _at least_ one
// failing test and can adjust the number of tests here.
// Taken from here:
// https://github.com/dtolnay/seq-macro/issues/3#issuecomment-904500519
macro_rules! id {
    ($limit:literal) => {
        $limit
    };
}

macro_rules! getlimit {
    ($($expand:ident)::*) => {
        $($expand)::* !{1904}
    };
}

const NUM_TESTS: usize = getlimit!(id);

#[test]
/// Tests that the number of tests used here in the test generation macro
/// actually matches the number of tests in the files This is necessary because
/// I didn't find a way to really dynamically generate test cases at runtime, so
/// instead I just hard-coded the number of tests into NUM_TESTS, and used that
/// for the seq macro to generate that number of functions
fn consistent_number_of_tests() {
    let test_cases = generate_tests();
    assert_eq!(test_cases.len(), NUM_TESTS);
}

#[test]
fn cannot_find_any_non_entries() {
    let too_much: Vec<&str> = node_emoji::all()
        .filter_map(|(shortcode, _)| {
            if SHORTCODES.contains(shortcode) {
                None
            } else {
                Some(shortcode)
            }
        })
        .collect();

    assert_eq!(too_much, Vec::<&str>::new());
}

#[test]
fn can_get_every_key_from_JS_lib() {
    let could_not_find: Vec<&str> = SHORTCODES
        .iter()
        .filter_map(|shortcode| match node_emoji::get(shortcode) {
            None => Some(shortcode.as_str()),
            Some(_) => None,
        })
        .collect();

    assert_eq!(could_not_find, Vec::<&str>::new());
}
