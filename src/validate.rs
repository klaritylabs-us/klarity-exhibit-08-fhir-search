//! FHIR search query construction checks (path + query string).

use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_search_request(line: &str) -> ValidationReport {
    let mut report = ValidationReport::ok();
    let line = line.trim();
    if line.is_empty() {
        report.push(ValidationIssue::error("request", "empty search line", "empty"));
        return report;
    }

    let Some((path, query)) = line.split_once('?') else {
        report.push(ValidationIssue::error("request", "expected Resource?param=value format", "missing-query"));
        return report;
    };

    let resource = path.trim_start_matches('/').trim();
    if resource != "Patient" && resource != "Observation" {
        report.push(ValidationIssue::warning("resource", "exhibit focuses on Patient/Observation search", "resource-type"));
    }

    if query.is_empty() {
        report.push(ValidationIssue::error("query", "search needs at least one parameter", "no-params"));
        return report;
    }

    for pair in query.split('&') {
        let (name, value) = match pair.split_once('=') {
            Some((n, v)) => (n, v),
            None => {
                report.push(ValidationIssue::error("query", format!("bad pair {:?}", pair), "malformed-pair"));
                continue;
            }
        };
        if name.is_empty() || value.is_empty() {
            report.push(ValidationIssue::error("query", format!("empty name or value in {:?}", pair), "empty-pair"));
        }
        if name.ends_with(":missing") && value != "true" && value != "false" {
            report.push(ValidationIssue::error(
                format!("query.{}", name),
                ":missing modifier expects true or false",
                "missing-modifier",
            ));
        }
        if (name == "birthdate" || name == "date") && !value.contains('|') && !starts_with_prefix(value) {
            report.push(ValidationIssue::warning(
                format!("query.{}", name),
                "date search often uses eq|gt|lt|ge|le prefix (e.g. gt1980-01-01)",
                "date-prefix",
            ));
        }
    }

    report
}

fn starts_with_prefix(v: &str) -> bool {
    ["eq", "ne", "gt", "lt", "ge", "le", "sa", "eb", "ap"].iter().any(|p| v.starts_with(p))
}
