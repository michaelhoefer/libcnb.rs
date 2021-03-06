//! This crate provides a library to implement [Cloud Native Buildpacks](https://buildpacks.io/).

// Enable rustc and Clippy lints that are disabled by default.
// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unused-crate-dependencies
#![warn(unused_crate_dependencies)]
// https://rust-lang.github.io/rust-clippy/stable/index.html
#![warn(clippy::pedantic)]
// Re-disable pedantic lints that are currently failing, until they are triaged and fixed/wontfixed.
// https://github.com/Malax/libcnb.rs/issues/60
#![allow(clippy::doc_markdown)]
// https://github.com/Malax/libcnb.rs/issues/65
#![allow(clippy::implicit_clone)]
// https://github.com/Malax/libcnb.rs/issues/56
#![allow(clippy::items_after_statements)]
// https://github.com/Malax/libcnb.rs/issues/59
#![allow(clippy::map_unwrap_or)]
// https://github.com/Malax/libcnb.rs/issues/62
#![allow(clippy::match_wildcard_for_single_variants)]
// https://github.com/Malax/libcnb.rs/issues/53
#![allow(clippy::missing_errors_doc)]
// https://github.com/Malax/libcnb.rs/issues/54
#![allow(clippy::missing_panics_doc)]
// https://github.com/Malax/libcnb.rs/issues/83
#![allow(clippy::module_name_repetitions)]
// https://github.com/Malax/libcnb.rs/issues/57
#![allow(clippy::must_use_candidate)]
// https://github.com/Malax/libcnb.rs/issues/63
#![allow(clippy::needless_pass_by_value)]
// https://github.com/Malax/libcnb.rs/issues/61
#![allow(clippy::redundant_closure_for_method_calls)]
// https://github.com/Malax/libcnb.rs/issues/58
#![allow(clippy::semicolon_if_nothing_returned)]
// https://github.com/Malax/libcnb.rs/issues/55
#![allow(clippy::single_match_else)]
// https://github.com/Malax/libcnb.rs/issues/64
#![allow(clippy::unnecessary_wraps)]

pub mod data;
pub mod layer_env;

pub mod layer_lifecycle;

use crate::data::buildpack::BuildpackApi;
pub use build::BuildContext;
pub use detect::DetectContext;
pub use detect::DetectOutcome;
pub use env::*;
pub use error::*;
pub use files::find_one_file;
pub use files::join;
pub use files::read_file;
pub use files::read_file_join;
pub use files::read_file_to_string;
pub use files::write_file;
pub use generic::*;
pub use mode::*;
pub use platform::*;
pub use publish::PublishContext;
pub use runtime::cnb_runtime;
pub use runtime::cnb_runtime_all;
pub use test::TestContext;
pub use test::TestOutcome;
pub use test::TestResult;
pub use test::TestResults;
pub use test::TestStatus;
pub use toml_file::*;
pub use transfer::compress_and_put;
pub use transfer::get;
pub use transfer::get_and_extract;
pub use transfer::put;

mod build;
mod detect;
mod env;
mod error;
mod files;
mod generic;
mod mode;
mod platform;
mod publish;
mod runtime;
mod test;
mod toml_file;
mod transfer;

const LIBCNB_SUPPORTED_BUILDPACK_API: BuildpackApi = BuildpackApi { major: 0, minor: 6 };
