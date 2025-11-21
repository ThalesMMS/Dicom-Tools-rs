use anyhow::{Context, Result};
use dicom::object::{open_file, InMemDicomObject};
use dicom::core::Tag;
use std::path::Path;

pub struct ValidationReport {
    pub is_valid: bool,
    pub missing_tags: Vec<String>,
    pub has_pixel_data: bool,
}

/// Validates a DICOM object in memory.
pub fn validate_obj(obj: &InMemDicomObject) -> ValidationReport {
    // Critical Attributes to check
    let required_tags = vec![
        (Tag(0x0008, 0x0016), "SOP Class UID"),
        (Tag(0x0008, 0x0018), "SOP Instance UID"),
        (Tag(0x0010, 0x0010), "Patient Name"),
        (Tag(0x0010, 0x0020), "Patient ID"),
        (Tag(0x0008, 0x0020), "Study Date"),
        (Tag(0x0008, 0x0060), "Modality"),
    ];

    let mut missing_tags = Vec::new();

    for (tag, name) in required_tags {
        if obj.element(tag).is_err() {
            missing_tags.push(format!("{} ({})", name, tag));
        }
    }

    // Check for Pixel Data (7FE0, 0010)
    let pixel_data_tag = Tag(0x7fe0, 0x0010);
    let has_pixel_data = obj.element(pixel_data_tag).is_ok();

    ValidationReport {
        is_valid: missing_tags.is_empty(),
        missing_tags,
        has_pixel_data,
    }
}

/// Validates if a file can be parsed as DICOM and prints a detailed summary.
pub fn check_file(path: &Path) -> Result<()> {
    println!("Validating: {:?}", path);
    let obj = open_file(path).context("Failed to open/parse DICOM file")?;
    let meta = obj.meta();

    println!("[OK] File Structure Parsed");
    println!("[OK] Transfer Syntax: {}", meta.transfer_syntax());
    println!("[OK] Media Storage SOP Class: {}", meta.media_storage_sop_class_uid);

    let report = validate_obj(&obj);

    if report.has_pixel_data {
        println!("[OK] Pixel Data present");
    } else {
        println!("[WARN] No Pixel Data found");
    }

    if report.is_valid {
        println!("\nResult: VALID (All critical attributes found)");
    } else {
        println!("\nResult: INVALID ({} critical attributes missing)", report.missing_tags.len());
        for missing in report.missing_tags {
             println!("[MISSING] {}", missing);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom::object::InMemDicomObject;
    use dicom::core::{DataElement, VR, PrimitiveValue};

    #[test]
    fn test_validate_empty_object() {
        let obj = InMemDicomObject::new_empty();
        let report = validate_obj(&obj);
        assert!(!report.is_valid);
        assert!(report.missing_tags.len() >= 6); // At least the 6 we check
        assert!(!report.has_pixel_data);
    }

    #[test]
    fn test_validate_valid_object() {
        let mut obj = InMemDicomObject::new_empty();
        
        // Add required tags
        obj.put(DataElement::new(Tag(0x0008, 0x0016), VR::UI, PrimitiveValue::from("1.2.840.10008.5.1.4.1.1.2"))); // SOP Class
        obj.put(DataElement::new(Tag(0x0008, 0x0018), VR::UI, PrimitiveValue::from("1.2.3.4.5"))); // SOP Instance
        obj.put(DataElement::new(Tag(0x0010, 0x0010), VR::PN, PrimitiveValue::from("Doe^John"))); // Name
        obj.put(DataElement::new(Tag(0x0010, 0x0020), VR::LO, PrimitiveValue::from("12345"))); // ID
        obj.put(DataElement::new(Tag(0x0008, 0x0020), VR::DA, PrimitiveValue::from("20230101"))); // Date
        obj.put(DataElement::new(Tag(0x0008, 0x0060), VR::CS, PrimitiveValue::from("CT"))); // Modality

        let report = validate_obj(&obj);
        assert!(report.is_valid, "Object should be valid, missing: {:?}", report.missing_tags);
        assert!(!report.has_pixel_data); // We didn't add pixel data, but it's still "valid" structure-wise for our basic check
    }
}
