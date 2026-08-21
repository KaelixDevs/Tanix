use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TanixApplication {
    Tonex,
    Amplitube5,
}

impl TanixApplication {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Tonex => "TONEX",
            Self::Amplitube5 => "AmpliTube 5",
        }
    }

    pub fn directory_name(&self) -> &'static str {
        match self {
            Self::Tonex => "tonex",
            Self::Amplitube5 => "amplitube5",
        }
    }

    pub fn prefix_path(&self, base: &Path) -> PathBuf {
        base.join(self.directory_name())
    }
}

#[derive(Debug, Clone)]
pub struct InstalledApplication {
    pub application: TanixApplication,
    pub executable: Option<PathBuf>,
    pub install_directory: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ApplicationManager {
    base_directory: PathBuf,
}

impl ApplicationManager {
    pub fn new() -> Self {
        Self {
            base_directory: application_base_directory(),
        }
    }

    pub fn base_directory(&self) -> &Path {
        &self.base_directory
    }

    pub fn application_directory(&self, application: TanixApplication) -> PathBuf {
        self.base_directory.join(application.directory_name())
    }

    pub fn ensure_directories(&self) -> Result<(), String> {
        for application in [TanixApplication::Tonex, TanixApplication::Amplitube5] {
            fs::create_dir_all(self.application_directory(application)).map_err(|error| {
                format!(
                    "Failed to create {} directory: {error}",
                    application.display_name()
                )
            })?;
        }

        Ok(())
    }

    pub fn detect(&self, application: TanixApplication) -> InstalledApplication {
        let directory = self.application_directory(application);

        let executable = find_executable(&directory, application);

        InstalledApplication {
            application,
            executable,
            install_directory: directory.exists().then_some(directory),
        }
    }

    pub fn detect_all(&self) -> Vec<InstalledApplication> {
        [TanixApplication::Tonex, TanixApplication::Amplitube5]
            .into_iter()
            .map(|application| self.detect(application))
            .collect()
    }
}

impl Default for ApplicationManager {
    fn default() -> Self {
        Self::new()
    }
}

fn application_base_directory() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local")
        .join("share")
        .join("tanix")
        .join("applications")
}

fn find_executable(directory: &Path, application: TanixApplication) -> Option<PathBuf> {
    if !directory.is_dir() {
        return None;
    }

    let candidates: &[&str] = match application {
        TanixApplication::Tonex => &["TONEX.exe", "TONEX", "TONEXApp.exe"],

        TanixApplication::Amplitube5 => &["AmpliTube 5.exe", "AmpliTube 5", "AmpliTube.exe"],
    };

    for candidate in candidates {
        let path = directory.join(candidate);

        if path.is_file() {
            return Some(path);
        }
    }

    find_executable_recursive(directory, candidates, 0)
}

fn find_executable_recursive(
    directory: &Path,
    candidates: &[&str],
    depth: usize,
) -> Option<PathBuf> {
    // Don't recursively scan an enormous directory tree.
    if depth > 5 {
        return None;
    }

    let entries = fs::read_dir(directory).ok()?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                if candidates
                    .iter()
                    .any(|candidate| name.eq_ignore_ascii_case(candidate))
                {
                    return Some(path);
                }
            }
        } else if path.is_dir() {
            if let Some(found) = find_executable_recursive(&path, candidates, depth + 1) {
                return Some(found);
            }
        }
    }

    None
}
