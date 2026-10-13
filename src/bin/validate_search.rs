use std::{env, fs, process::ExitCode};
use fhir_search_exhibit::validate_search_request;

fn main() -> ExitCode {
    let path = env::args().nth(1).expect("usage: validate-search <file.txt>");
    let line = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
    let report = validate_search_request(&line);
    println!("{}", serde_json::to_string_pretty(&report.issues).unwrap());
    ExitCode::from(if report.is_valid() { 0 } else { 1 })
}
