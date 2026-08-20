use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub input_node_id: Option<u32>,
    pub output_node_id: Option<u32>,
    pub sample_rate: u32,
    pub buffer_size: u32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            input_node_id: None,
            output_node_id: None,
            sample_rate: 48_000,
            buffer_size: 128,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TanixConfig {
    #[serde(default)]
    pub audio: AudioConfig,
}

impl TanixConfig {
    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tanix")
            .join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::path();

        let config = match fs::read_to_string(&path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
            Err(_) => Self::default(),
        };

        if let Err(error) = config.save() {
            eprintln!("Failed to initialize Tanix configuration: {error}");
        }

        config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;

        Ok(())
    }
}
