pub enum LifecycleMode {
    Dev,
    CI,
    Test,
    Package,
}

impl LifecycleMode {
    pub fn from(os_str: String) -> Self {
        match os_str.as_str() {
            "Dev" => LifecycleMode::Dev,
            "Test" => LifecycleMode::Test,
            "Package" => LifecycleMode::Package,
            "CI" | _ => LifecycleMode::CI,
        }
    }
}
