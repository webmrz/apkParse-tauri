use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub path: String,
    pub is_directory: bool,
    pub last_modified: String,
}

impl FileInfo {
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)?;
        
        Ok(Self {
            name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
            size: metadata.len(),
            path: path.to_string_lossy().to_string(),
            is_directory: metadata.is_dir(),
            last_modified: metadata.modified()?
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        })
    }
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nSize: {} bytes\nPath: {}\nType: {}\nLast Modified: {}",
            self.name,
            self.size,
            self.path,
            if self.is_directory { "Directory" } else { "File" },
            self.last_modified
        )
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: 0,
            path: String::new(),
            is_directory: false,
            last_modified: String::new(),
        }
    }
} 