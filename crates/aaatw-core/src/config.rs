use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub data_dir: Option<PathBuf>,
    pub resource_dll: Option<PathBuf>,
    pub skip_splash: bool,
}
