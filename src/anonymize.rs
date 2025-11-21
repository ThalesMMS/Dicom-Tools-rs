use dicom::object::open_file;
use dicom::core::{DataElement, Tag, VR};
use dicom::core::header::Header;
use dicom::core::value::PrimitiveValue;
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use anyhow::Result;

fn generate_hash(original: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(original.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result)[..16].to_uppercase()
}

pub fn process_file(input: &Path, output: Option<PathBuf>) -> Result<()> {
    let mut obj = open_file(input)?; 

    // 1. Get original ID to derive a hash
    let patient_id_tag = Tag(0x0010, 0x0020);
    let original_id = obj.element(patient_id_tag)
        .ok()
        .and_then(|e| e.to_str().ok())
        .unwrap_or("UNKNOWN".into());
    
    let anon_id = format!("ANON_{}", generate_hash(&original_id));

    // 2. Collect tags that need replacement based on VR
    let mut replacements = Vec::new();

    for elem in obj.iter() {
        let tag = elem.tag();
        let vr = elem.vr();

        // Skip PatientID (handled explicitly)
        if tag == patient_id_tag {
            continue;
        }

        match vr {
            VR::PN => {
                if tag == Tag(0x0010, 0x0010) {
                    replacements.push((tag, vr, "ANONYMOUS^PATIENT".to_string()));
                } else {
                    replacements.push((tag, vr, "ANONYMIZED".to_string()));
                }
            }
            VR::DA => {
                replacements.push((tag, vr, "19010101".to_string()));
            }
            VR::TM => {
                replacements.push((tag, vr, "000000".to_string()));
            }
            VR::DT => {
                replacements.push((tag, vr, "19010101000000".to_string()));
            }
            _ => {}
        }
    }

    // 3. Apply generic replacements
    for (tag, vr, val) in replacements {
        // Use PrimitiveValue::from for strings which works for LO, PN, DA, TM, DT (mostly)
        // For VR::DA/TM/DT, they are effectively strings in DICOM standard (mostly).
        obj.put(DataElement::new(tag, vr, PrimitiveValue::from(val)));
    }

    // 4. Apply specific PatientID override
    obj.put(DataElement::new(patient_id_tag, VR::LO, PrimitiveValue::from(anon_id)));

    // 5. Save file
    let output_path = output.unwrap_or_else(|| {
        let mut p = input.to_path_buf();
        let stem = p.file_stem().unwrap().to_str().unwrap();
        p.set_file_name(format!("{}_anon.dcm", stem));
        p
    });

    obj.write_to_file(&output_path)?;
    println!("Anonymized file saved to: {:?}", output_path);

    Ok(())
}
