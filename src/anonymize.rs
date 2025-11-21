use dicom::object::open_file;
use dicom::core::{DataElement, Tag, VR};
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
    let mut obj = open_file(input)?; // Keep metadata available so we can save later

    // 1. Get original ID to derive a hash
    let patient_id_tag = Tag(0x0010, 0x0020);
    let original_id = obj.element(patient_id_tag)
        .ok()
        .and_then(|e| e.to_str().ok())
        .unwrap_or("UNKNOWN".into());
    
    let anon_id = format!("ANON_{}", generate_hash(&original_id));

    // 2. Replace fields with anonymized values
    let replacements = vec![
        (Tag(0x0010, 0x0010), "ANONYMOUS^PATIENT"), // PatientName
        (Tag(0x0010, 0x0020), &anon_id),            // PatientID
        (Tag(0x0008, 0x0080), "ANONYMIZED"),        // InstitutionName
        (Tag(0x0008, 0x0090), "ANONYMIZED"),        // ReferringPhysicianName
    ];

    for (tag, val) in replacements {
        obj.put(DataElement::new(tag, VR::LO, PrimitiveValue::from(val)));
    }

    // 3. Remove sensitive fields (e.g., birth date)
    obj.remove_element(Tag(0x0010, 0x0030)); // PatientBirthDate

    // 4. Save file to disk, defaulting to *_anon.dcm
    let output_path = output.unwrap_or_else(|| {
        let mut p = input.to_path_buf();
        let stem = p.file_stem().unwrap().to_str().unwrap();
        p.set_file_name(format!("{}_anon.dcm", stem));
        p
    });

    obj.write_to_file(&output_path)?;
    println!("Arquivo anonimizado salvo em: {:?}", output_path);

    Ok(())
}
