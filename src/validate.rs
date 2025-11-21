use anyhow::{Context, Result};
use dicom::object::open_file;
use dicom::core::Tag;
use std::path::Path;

/// Validates if a file can be parsed as DICOM and prints a detailed summary.
pub fn check_file(path: &Path) -> Result<()> {
    println!("Validating: {:?}", path);
    let obj = open_file(path).context("Failed to open/parse DICOM file")?;
    let meta = obj.meta();

    println!("[OK] File Structure Parsed");
    println!("[OK] Transfer Syntax: {}", meta.transfer_syntax());
    println!("[OK] Media Storage SOP Class: {}", meta.media_storage_sop_class_uid);

    // Critical Attributes to check
    let required_tags = vec![
        (Tag(0x0008, 0x0016), "SOP Class UID"),
        (Tag(0x0008, 0x0018), "SOP Instance UID"),
        (Tag(0x0010, 0x0010), "Patient Name"),
        (Tag(0x0010, 0x0020), "Patient ID"),
        (Tag(0x0008, 0x0020), "Study Date"),
        (Tag(0x0008, 0x0060), "Modality"),
    ];

    let mut missing_count = 0;

    for (tag, name) in required_tags {
        match obj.element(tag) {
            Ok(_) => println!("[OK] Found {}: {}", name, tag),
            Err(_) => {
                println!("[MISSING] Critical Attribute {}: {}", name, tag);
                missing_count += 1;
            }
        }
    }

    // Check for Pixel Data (7FE0, 0010)
    let pixel_data_tag = Tag(0x7fe0, 0x0010);
    if obj.element(pixel_data_tag).is_ok() {
        println!("[OK] Pixel Data present");
    } else {
        println!("[WARN] No Pixel Data found (valid for SR/Structured Reports, but verify if unexpected)");
    }

    if missing_count == 0 {
        println!("\nResult: VALID (All critical attributes found)");
    } else {
        println!("\nResult: INVALID ({} critical attributes missing)", missing_count);
    }

    Ok(())
}
