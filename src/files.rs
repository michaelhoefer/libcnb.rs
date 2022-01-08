use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_one_file(path: &Path, word: &str) -> bool {
    for (_i, e) in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .enumerate()
    {
        if e.metadata().unwrap().is_file() {
            if let Ok(b) = self::contains(e.path(), &annotation_pattern(word)) {
                if b {
                    return true;
                }
            }
        }
    }
    false
}

pub fn read_file<P: AsRef<Path>>(src: P) -> Result<Vec<u8>, anyhow::Error> {
    let mut file = File::open(src)?;
    let mut buffer = Vec::new();

    // read the whole file
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_file_join<P: AsRef<Path>>(src: P) -> Result<Vec<u8>, anyhow::Error> {
    let file = File::open(src)?;
    let mut buffer = Vec::new();

    let lines = BufReader::new(file).lines();
    for mut line in lines.filter_map(|result| result.ok()) {
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        buffer.write(line.as_bytes())?;
    }

    Ok(buffer)
}

pub fn join(src: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut buffer = Vec::new();

    let lines = BufReader::new(src).lines();
    for mut line in lines.filter_map(|result| result.ok()) {
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        buffer.write(line.as_bytes())?;
    }

    Ok(buffer)
}

pub fn read_file_to_string<P: AsRef<Path>>(src: P) -> Result<String, anyhow::Error> {
    let mut file = File::open(src)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_file(content: &[u8], f: &PathBuf) {
    match File::create(f) {
        Err(why) => panic!("Failed to write file: {}", why),
        Ok(mut file) => file.write_all(content).unwrap(),
    }
}

fn annotation_pattern(s: &str) -> Regex {
    Regex::new(format!(r"@(\b)(?i:{})(\b)", s).as_str()).unwrap()
}

fn contains<P: AsRef<Path>>(src: P, pat: &Regex) -> Result<bool, anyhow::Error> {
    let s = self::read_file_to_string(src)?;
    Ok(pat.is_match(&s))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_join() {
        let data = join("THIS\nIS\nDIS\nJOINTED\n".as_bytes()).unwrap();
        let text = String::from_utf8(data).unwrap();
        assert_eq!(text, "THISISDISJOINTED");
    }

    #[test]
    fn test_read_file_join() {
        let temp = tempdir().unwrap();
        let f = temp.path().join("dumbfile.txt");
        write_file("THIS\nIS\nDIS\nJOINTED\n".as_bytes(), &f);

        let data = read_file_join(f).unwrap();
        let text = String::from_utf8(data).unwrap();
        assert_eq!(text, "THISISDISJOINTED");
    }

    #[test]
    fn test_regex_pattern() {
        let pat = annotation_pattern("IsTest");
        assert!(pat.is_match("@IsTest\npublic class Yoda {}"));
        assert!(pat.is_match("@isTest\npublic class Yoda {}"));
        assert!(pat.is_match(
            "@ISTEST public class Yoda {\n  @istest\nprivate static void testIt() {}\n}"
        ));
        assert!(!pat.is_match("@IsTesty\npublic class Yoda {}"));
        assert!(!pat.is_match("IsTest public class Yoda {}"));
        assert!(!pat.is_match("@ IsTest public class Yoda {}"));
    }

    #[test]
    fn test_find_one_file() {
        let found_content = r"
@IsTest
public class TestTests {
    @IsTest
    static void testBehavior() {
        new Test().gimmeString();
    }
}
        ";

        let ignored_content = r"
public with sharing class Test {
    public String gimmeString() {
        return 'Hi there world';
    }
}
        ";
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path();
        for f in ["file1.cls", "file2.cls", "file3.cls"] {
            write_file(ignored_content.as_bytes(), &test_dir.join(f));
        }
        for f in ["file4.cls", "file5.cls"] {
            write_file(found_content.as_bytes(), &test_dir.join(f));
        }

        assert!(
            find_one_file(test_dir, "IsTest"),
            "Should have found at least one test file, we wrote several"
        );
    }

    #[test]
    fn test_find_test_files() {
        let app_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/dummy_app")
            .canonicalize()
            .unwrap();
        assert!(
            find_one_file(app_dir.as_path(), "IsTest"),
            "Should have found at least one test file in {:?}",
            app_dir
        );
    }
}
