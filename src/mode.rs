use std::ffi::OsStr;
use std::str::FromStr;

pub enum LifecycleMode {
    Dev,
    CI,
    Test,
    Package,
}

impl FromStr for LifecycleMode {
    type Err = anyhow::Error;

    fn from_str(mode: &str) -> Result<LifecycleMode, Self::Err> {
        match mode {
            "Dev" => Ok(LifecycleMode::Dev),
            "Test" => Ok(LifecycleMode::Test),
            "Package" => Ok(LifecycleMode::Package),
            "CI" => Ok(LifecycleMode::CI),
            _ => Err(anyhow::Error::msg("Invalid mode string")),
        }
    }
}

impl AsRef<OsStr> for LifecycleMode {
    fn as_ref(&self) -> &OsStr {
        let str = match self {
            LifecycleMode::Dev => "Dev",
            LifecycleMode::Test => "Test",
            LifecycleMode::Package => "Package",
            LifecycleMode::CI => "CI",
        };
        OsStr::new(str)
    }
}
