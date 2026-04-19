// Test that we have the expected output for each version from 0.1.0 through
// 1.95.0
// Use `TESTS_UPDATE_EXPECTED` to update the expected output files when the
// format of things changes
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::Path;

fn assert_file_content_matches(expected: &Path, actual: &Path, version: &OsStr) {
    let actual_content = fs::read_to_string(actual).unwrap();
    if std::env::var("TESTS_UPDATE_EXPECTED").is_ok() {
        fs::write(expected, &actual_content).expect("Able to write to the expected output file");
    }

    let expected_content = fs::read_to_string(expected).unwrap();

    // Print the content as multiline strings rather than just all on one line
    // the way assert_eq! would, so that various diff tools can be used to
    // understand the comparison
    if expected_content != actual_content {
        for diff in diff::lines(&expected_content, &actual_content) {
            match diff {
                diff::Result::Left(l) => println!("-{}", l),
                diff::Result::Both(l, _) => println!(" {}", l),
                diff::Result::Right(r) => println!("+{}", r),
            }
        }
        assert!(false, "HTML for {:?} should match", version);
    }
}

#[test]
fn verify_generated_output() {
    // Running the thanks command via `std::process::Command` is a lot slower
    // that if we just require that it have been run beforehand
    let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("output")
        .join("rust");

    // Use 1.95.0 as a canary - if the file for that release doesn't exist then
    // we assume the thanks output hasn't been generated yet
    let canary = File::open(&output_dir.join("1.95.0").join("index.html"));
    canary.expect("1.95.0 thanks file should exist");

    let expectation_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("expected");
    for entry in fs::read_dir(expectation_dir).unwrap() {
        let entry = entry.unwrap();
        let expected_file = entry.path();
        assert_eq!("html", expected_file.extension().unwrap());
        let binding = expected_file.with_extension("");
        let version = binding.file_name().unwrap();
        assert_file_content_matches(
            &expected_file,
            &output_dir.join(version).join("index.html"),
            version,
        );
    }
}
