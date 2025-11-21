use anyhow::{Context, Result};
use dicom::object::open_file;
use std::path::Path;

/// Validates if a file can be parsed as DICOM and prints a brief summary.
pub fn check_file(path: &Path) -> Result<()> {
    let obj = open_file(path).context("Falha ao abrir arquivo DICOM")?;
    let meta = obj.meta();

    println!("Arquivo válido: {}", path.display());
    println!("  Transfer Syntax: {}", meta.transfer_syntax());
    println!("  Media Storage SOP Class UID: {}", meta.media_storage_sop_class_uid);

    Ok(())
}
