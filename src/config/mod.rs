use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::{fs, io, path::PathBuf};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    pub kernel_path: PathBuf,
    pub initrd_path: PathBuf,
    pub cmdline: String,
    pub cpu: cpu::Cpu,
    pub memory: memory::Memory,
    pub filesystem: filesystem::Filesystem,
    pub network: network::Network,
    pub graphics: graphics::Graphics,
}

mod cpu;
mod memory;
mod filesystem;
mod network;
mod graphics;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("unable to read config file")]
    Io(#[from] io::Error),
    #[error("unable to parse config file")]
    Parse(#[from] serde_json::Error)
}

impl TryFrom<PathBuf> for Config {
    type Error = ConfigError;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let s = fs::read_to_string(path)?;
        serde_json::from_str(&s).map_err(|e| { ConfigError::Parse(e) })
    }
}
