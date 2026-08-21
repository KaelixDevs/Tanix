use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone)]
pub struct WineRuntime {
    pub executable: PathBuf,
    pub version: Option<String>,
    pub prefix: PathBuf,
}

#[derive(Debug, Clone)]
pub enum RuntimeStatus {
    Available(WineRuntime),
    NotInstalled,
    Failed(String),
}

impl WineRuntime {
    pub fn detect() -> RuntimeStatus {
        let executable = match find_wine() {
            Some(path) => path,
            None => return RuntimeStatus::NotInstalled,
        };

        let version = Command::new(&executable)
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            });

        RuntimeStatus::Available(Self {
            executable,
            version,
            prefix: default_prefix(),
        })
    }

    pub fn prefix_exists(&self) -> bool {
        self.prefix.join("drive_c").is_dir()
    }

    pub fn initialize_prefix(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.prefix)
            .map_err(|error| format!("Failed to create Tanix prefix: {error}"))?;

        let status = Command::new(&self.executable)
            .arg("wineboot")
            .env("WINEPREFIX", &self.prefix)
            .status()
            .map_err(|error| format!("Failed to start wineboot: {error}"))?;

        if !status.success() {
            return Err(format!("wineboot exited with status {status}"));
        }

        if !self.prefix_exists() {
            return Err("Wineboot completed but the Wine prefix was not created".to_string());
        }

        Ok(())
    }

    pub fn launch(&self, executable: &Path, args: &[String]) -> Result<u32, String> {
        if !self.prefix_exists() {
            return Err("Tanix Wine prefix is not initialized".to_string());
        }

        if !executable.is_file() {
            return Err(format!(
                "Windows executable does not exist: {}",
                executable.display()
            ));
        }

        let child = Command::new(&self.executable)
            .arg(executable)
            .args(args)
            .env("WINEPREFIX", &self.prefix)
            .spawn()
            .map_err(|error| format!("Failed to launch Windows application: {error}"))?;

        Ok(child.id())
    }

    pub fn winecfg(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.prefix)
            .map_err(|error| format!("Failed to create Tanix prefix: {error}"))?;

        Command::new(&self.executable)
            .arg("winecfg")
            .env("WINEPREFIX", &self.prefix)
            .spawn()
            .map_err(|error| format!("Failed to launch winecfg: {error}"))?;

        Ok(())
    }

    pub fn executable(&self) -> &Path {
        &self.executable
    }

    pub fn prefix(&self) -> &Path {
        &self.prefix
    }
}

fn find_wine() -> Option<PathBuf> {
    let candidates = ["/usr/bin/wine", "/usr/bin/wine64", "/opt/wine/bin/wine"];

    candidates
        .iter()
        .map(PathBuf::from)
        .find(|path| path.is_file())
}

fn default_prefix() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local")
        .join("share")
        .join("tanix")
        .join("wine")
        .join("default")
}
