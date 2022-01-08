use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fmt::Display;
use std::str::FromStr;
use anyhow::anyhow;

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
            "Dev" | "dev" | "DEV" => Ok(LifecycleMode::Dev),
            "Test" | "test" | "TEST" => Ok(LifecycleMode::Test),
            "Package" | "package" | "PACKAGE" => Ok(LifecycleMode::Package),
            "CI" | "ci" => Ok(LifecycleMode::CI),
            _ => Err(anyhow::Error::msg("Invalid mode string")),
        }
    }
}

impl Display for LifecycleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.as_ref();
        write!(f, "{:?}", name)
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

const MODE_ENV_NAME: &str = "CNB_LIFECYCLE_MODE";

/// Return currently configured LifecycleMode
///
/// # Example
///
/// ```
/// use libcnb::{get_lifecycle_mode, LifecycleMode};
/// match get_lifecycle_mode() {
///     Ok(mode) => println!("Current mode is {}", mode),
///     Err(e) => eprintln!("Configured mode is invalid. {}", e)
/// }
/// ```
/// Returns an error only if the environment variable is set, but set to an invalid mode.
pub fn get_lifecycle_mode() -> Option<LifecycleMode> {
    match std::env::var_os(MODE_ENV_NAME) {
        Some(os_str) => {
            match LifecycleMode::from_str(os_str.to_str().unwrap()) {
                Ok(mode) => Some(mode),
                Err(e) => {
                    eprintln!("Warning: invalid lifecycle mode is set but ignored: {:?}.\n{}", os_str.to_str().unwrap(), e);
                    None
                }
            }
        }
        _ => None
    }
}

pub fn set_lifecycle_mode(mode: &str) -> Result<LifecycleMode, anyhow::Error> {
    let result = LifecycleMode::from_str(mode);
    match result {
        Ok(mode) => {
            std::env::set_var(MODE_ENV_NAME, mode.borrow());
            Ok(mode)
        }
        Err(e) => Err(anyhow!(e))
    }
}
