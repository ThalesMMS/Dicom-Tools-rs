use anyhow::{Context, Result};
use dicom::object::{open_file, InMemDicomObject};
// Re-export StandardDataDictionary from dicom crate (v0.7 uses dicom_dictionary_std v0.7 internally)
// We can access it via dicom::dictionary_std or similar if exposed, 
// or just rely on generic inference if possible.
// In dicom 0.7, `dicom::object::StandardDataDictionary` might be available.
use dicom::object::StandardDataDictionary; 
use dicom_json::{from_value, DicomJson};
use serde_json::Value;
use std::fs::File;
use std::path::Path;
use dicom::object::FileMetaTableBuilder;

/// Convert a DICOM file to JSON and print it to stdout.
pub fn to_json(input: &Path, output: Option<&Path>) -> Result<()> {
    let obj = open_file(input).context("Failed to open DICOM file")?;
    
    // Now that versions align, &obj (FileDicomObject) should convert to DicomJson via From/Into if supported,
    // or we use the inner object.
    
    let inner_obj: &InMemDicomObject<StandardDataDictionary> = &*obj;
    // In dicom-json 0.7, DicomJson::from might expect specific types.
    // If `DicomJson::from(&InMemDicomObject)` exists, we use it.
    let json_obj = DicomJson::from(inner_obj);
    
    let json_string = serde_json::to_string_pretty(&json_obj)
        .context("Failed to serialize to JSON")?;

    match output {
        Some(path) => {
            std::fs::write(path, json_string).context("Failed to write JSON to file")?;
            println!("JSON saved to {:?}", path);
        }
        None => {
            println!("{}", json_string);
        }
    }

    Ok(())
}

/// Create a DICOM file from a JSON source.
pub fn from_json(input: &Path, output: &Path) -> Result<()> {
    let file = File::open(input).context("Failed to open JSON file")?;
    let json_val: Value = serde_json::from_reader(file).context("Failed to parse JSON")?;

    let obj: InMemDicomObject<StandardDataDictionary> = from_value(json_val).context("Failed to convert JSON to DICOM object")?;

    use dicom::object::FileDicomObject;
    
    let file_meta = FileMetaTableBuilder::new()
        .transfer_syntax(dicom::transfer_syntax::entries::EXPLICIT_VR_LITTLE_ENDIAN.uid())
        .build()?;
        
    let mut file_obj = FileDicomObject::new_empty_with_dict_and_meta(StandardDataDictionary::default(), file_meta);
    
    for elem in obj {
        file_obj.put(elem);
    }

    file_obj.write_to_file(output).context("Failed to write DICOM file")?;
    
    println!("DICOM saved to {:?}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom::object::InMemDicomObject;
    use dicom::core::{DataElement, Tag, VR, PrimitiveValue};

    #[test]
    fn test_json_roundtrip() {
        let mut obj = InMemDicomObject::new_empty();
        obj.put(DataElement::new(Tag(0x0010, 0x0010), VR::PN, PrimitiveValue::from("Test^Patient")));
        obj.put(DataElement::new(Tag(0x0010, 0x0020), VR::LO, PrimitiveValue::from("12345")));

        let json_obj = DicomJson::from(&obj);
        let json_val = serde_json::to_value(&json_obj).unwrap();

        let restored: InMemDicomObject<StandardDataDictionary> = from_value(json_val).expect("Deserialization failed");

        let name = restored.element(Tag(0x0010, 0x0010)).unwrap().to_str().unwrap();
        assert_eq!(name, "Test^Patient");
    }
}