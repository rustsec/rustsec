#![cfg(all(feature = "v2", feature = "std"))]

use cvss::v2::Vector;
use std::{fs, str::FromStr};

// Run the test set from Red Hat's Security Python implementation: https://github.com/RedHatProductSecurity/cvss

fn run_tests_from_file(name: &str) {
    let content = fs::read_to_string(format!("tests/cvss-redhat/tests/{}", name)).unwrap();
    for l in content.lines() {
        let parts = l.split(" - ").collect::<Vec<&str>>();
        let vector = parts[0];
        if vector.len() > 44 {
            // more than base, skip
            continue;
        }
        // "(base, _, _)"
        let base_score = parts[1].split(',').next().unwrap().trim_start_matches('(');
        // "(_, temporal, _)"
        let temporal_score = parts[1].split(',').nth(1).unwrap().trim();

        let cvss = Vector::from_str(vector).unwrap();

        // Test correct serialization.
        assert_eq!(cvss.to_string(), parts[0]);
        assert!(cvss.base_score().value() >= 0.0);
        assert!(cvss.base_score().value() <= 10.0);

        assert!(cvss.temporal_score().value() >= 0.0);
        assert!(cvss.temporal_score().value() <= 10.0);

        let base_diff: f64 = cvss.base_score().value() - base_score.parse::<f64>().unwrap();
        assert!(base_diff.abs() < 0.0001);

        if temporal_score != "None" {
            let temporal_diff: f64 =
                cvss.temporal_score().value() - temporal_score.parse::<f64>().unwrap();
            assert!(temporal_diff.abs() < 0.0001);
        }
    }
}

#[test]
fn cvss_v2_simple() {
    run_tests_from_file("vectors_simple2");
}

#[test]
fn cvss_v2_random() {
    run_tests_from_file("vectors_random2");
}
