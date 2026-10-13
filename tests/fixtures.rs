use std::{fs, path::PathBuf};
use fhir_search_exhibit::validate_search_request;

fn f(n: &str) -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("examples"); p.push(n);
    fs::read_to_string(p).unwrap()
}

#[test]
fn valid_search() { assert!(validate_search_request(&f("valid-patient-search.txt")).is_valid()); }
#[test]
fn invalid_no_query() { assert!(!validate_search_request(&f("invalid-no-query.txt")).is_valid()); }
