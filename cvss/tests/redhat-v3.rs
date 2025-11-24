#![cfg(all(feature = "v3", feature = "std"))]

use cvss::v3::Vector;
use std::{fs, str::FromStr};

// Run the test set from Red Hat's Security Python implementation: https://github.com/RedHatProductSecurity/cvss

fn run_tests_from_file(name: &str) {
    let content = fs::read_to_string(format!("tests/cvss-redhat/tests/{}", name)).unwrap();
    for l in content.lines() {
        let parts = l.split(" - ").collect::<Vec<&str>>();
        let vector = parts[0];

        // "(base, _, _)"
        let base_score = parts[1].split(',').next().unwrap().trim_start_matches('(');
        let temporal_score = parts[1].split(',').nth(1).unwrap().trim();

        let cvss = Vector::from_str(vector).unwrap();

        // Test correct serialization.
        assert_eq!(cvss.to_string(), parts[0]);

        // Test score calculation.
        assert!(cvss.base_score().value() >= 0.0);
        assert!(cvss.base_score().value() <= 10.0);
        assert!(cvss.temporal_score().value() >= 0.0);
        assert!(cvss.temporal_score().value() <= 10.0);

        let diff: f64 = cvss.base_score().value() - base_score.parse::<f64>().unwrap();
        assert!(
            diff.abs() < 0.0001,
            "Base score mismatch for vector {}: expected {}, got {}",
            vector,
            base_score,
            cvss.base_score().value()
        );

        let diff: f64 = cvss.temporal_score().value() - temporal_score.parse::<f64>().unwrap();
        assert!(
            diff.abs() < 0.0001,
            "Temporal score mismatch for vector {}: expected {}, got {}",
            vector,
            temporal_score,
            cvss.temporal_score().value()
        );
    }
}

#[test]
fn cvss_v3_simple() {
    run_tests_from_file("vectors_simple3");
    run_tests_from_file("vectors_simple31");
}

#[test]
fn cvss_v3_random() {
    run_tests_from_file("vectors_random3");
    run_tests_from_file("vectors_random31");
}
