#[cfg(test)]
mod integration_tests {
    use std::fs;
    use std::path::PathBuf;
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;
    use std::str::FromStr; // Run programs

    #[test]
    fn test_encode_chunk_in_file() {
        let mut cmd = Command::cargo_bin("pngme-cli").unwrap();

        let file_path = PathBuf::from_str("Capture.PNG").unwrap();
        let output_path = PathBuf::from_str("output.PNG").unwrap();

        cmd.arg("encode")
            .arg(&file_path)
            .arg("RUST")
            .arg("Test message")
            .arg(output_path);

        let contents = fs::read(&file_path).unwrap();

        assert_eq!(47470, contents.len());
    }
}

