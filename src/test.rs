use std::fmt::{Debug};
use std::path::{Path, PathBuf};
use serde::Serialize;

use crate::{platform::Platform, data::buildpack::BuildpackToml, write_toml_file, TomlFileError};

/// Context for a buildpack's test phase execution.
pub struct TestContext<P: Platform, BM> {
    pub app_dir: PathBuf,
    pub buildpack_dir: PathBuf,
    pub stack_id: String,
    pub platform: P,
    pub buildpack_descriptor: BuildpackToml<BM>,
}

/// Describes the outcome of the buildpack's test phase.
#[derive(Debug)]
pub enum TestOutcome {
    Pass(TestResults),
    Fail(TestResults),
}

#[derive(Serialize, Debug)]
pub enum TestStatus {
    Pass,
    Fail,
    Ready,
    Ignore
}


#[derive(Serialize, Debug)]
pub struct TestResults {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub passed: Vec<TestResult>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<TestResult>,
    pub status: TestStatus,
}

impl TestResults {
    pub fn new() -> TestResults {
        TestResults {
            passed: vec![],
            failed: vec![],
            status: TestStatus::Ready,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TestResult {
    desc: &'static str,
    status: TestStatus,
}

impl TestResult {
    pub fn new(desc: &'static str, status: TestStatus) -> TestResult {
        TestResult {
            desc,
            status,
        }
    }
}

pub fn write_test_results(
    test_results: &TestResults,
    path: impl AsRef<Path>,

) -> Result<(), TomlFileError> {
    write_toml_file(&test_results, path)?;
    println!("Tests Finished. {:?}", test_results.status);
    println!("{:?}", toml::to_string  (test_results));
    Ok(())
}
