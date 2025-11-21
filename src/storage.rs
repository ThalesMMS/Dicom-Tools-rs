use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct FileStore {
    root: PathBuf,
}

impl FileStore {
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root).context("Failed to create upload directory")?;
        Ok(Self { root })
    }

    pub fn save(&self, original_name: Option<&str>, bytes: &[u8]) -> Result<String> {
        let stem = original_name
            .and_then(|n| Path::new(n).file_stem().and_then(|s| s.to_str()))
            .map(sanitize_filename)
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "dicom".to_string());

        let hash = hex::encode(Sha256::digest(bytes));
        let filename = format!("{}-{}.dcm", stem, &hash[..12]);
        let path = self.root.join(&filename);
        fs::write(&path, bytes).context("Failed to persist uploaded file")?;
        Ok(filename)
    }

    pub fn resolve(&self, name: &str) -> Result<PathBuf> {
        let candidate = self.root.join(name);
        let canonical_root = self
            .root
            .canonicalize()
            .unwrap_or_else(|_| self.root.clone());
        let canonical = candidate
            .canonicalize()
            .context("Requested file not found")?;
        if !canonical.starts_with(&canonical_root) {
            bail!("Attempt to access file outside storage root");
        }
        Ok(canonical)
    }

    pub fn derived_path(
        &self,
        source_name: &str,
        suffix: &str,
        extension: &str,
    ) -> Result<(String, PathBuf)> {
        let base = Path::new(source_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(sanitize_filename)
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "dicom".to_string());

        let filename = format!("{}-{}.{}", base, suffix, extension);
        Ok((filename.clone(), self.root.join(filename)))
    }
}

fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}
