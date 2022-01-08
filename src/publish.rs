use std::path::PathBuf;

use crate::{data::buildpack::BuildpackToml, platform::Platform};

/// Context for a buildpack's test phase execution.
pub struct PublishContext<P: Platform, BM> {
    pub app_dir: PathBuf,
    pub buildpack_dir: PathBuf,
    pub stack_id: String,
    pub platform: P,
    pub buildpack_descriptor: BuildpackToml<BM>,
}
