use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Dirs {
    pub ssds: Vec<String>,
    pub hdds: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Options {
    pub only_replace: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Cfg {
    //pub general: General,
    pub dirs: Dirs,
    pub options: Options,
}

impl Cfg {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::new();
        c.merge(File::with_name("cfg")).unwrap();
        c.try_into()
    }
}
