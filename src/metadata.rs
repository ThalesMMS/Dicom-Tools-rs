use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use dicom::core::Tag;
use dicom::object::{open_file, DefaultDicomObject};

use crate::dicom_access::ElementAccess;
use crate::models::{BasicMetadata, DetailedMetadata};

fn text_for_tag<T: ElementAccess>(obj: &T, tag: Tag) -> Option<String> {
    obj.element_str(tag)
}

fn insert_if(map: &mut BTreeMap<String, String>, label: &str, value: Option<String>) {
    if let Some(value) = value {
        map.insert(label.to_string(), value);
    }
}

pub fn extract_basic_metadata<T: ElementAccess>(obj: &T) -> BasicMetadata {
    let patient_name = text_for_tag(obj, Tag(0x0010, 0x0010));
    let patient_id = text_for_tag(obj, Tag(0x0010, 0x0020));
    let study_date = text_for_tag(obj, Tag(0x0008, 0x0020));
    let modality = text_for_tag(obj, Tag(0x0008, 0x0060));
    let sop_class_uid = text_for_tag(obj, Tag(0x0008, 0x0016));
    let has_pixel_data = obj.has_element(Tag(0x7fe0, 0x0010));

    BasicMetadata {
        patient_name,
        patient_id,
        study_date,
        modality,
        sop_class_uid,
        has_pixel_data,
    }
}

pub fn extract_detailed_metadata<T: ElementAccess>(obj: &T) -> DetailedMetadata {
    let mut patient = BTreeMap::new();
    insert_if(&mut patient, "Name", text_for_tag(obj, Tag(0x0010, 0x0010)));
    insert_if(&mut patient, "ID", text_for_tag(obj, Tag(0x0010, 0x0020)));
    insert_if(
        &mut patient,
        "Birth Date",
        text_for_tag(obj, Tag(0x0010, 0x0030)),
    );
    insert_if(&mut patient, "Sex", text_for_tag(obj, Tag(0x0010, 0x0040)));

    let mut study = BTreeMap::new();
    insert_if(&mut study, "Date", text_for_tag(obj, Tag(0x0008, 0x0020)));
    insert_if(&mut study, "Time", text_for_tag(obj, Tag(0x0008, 0x0030)));
    insert_if(
        &mut study,
        "Description",
        text_for_tag(obj, Tag(0x0008, 0x1030)),
    );
    insert_if(
        &mut study,
        "Accession Number",
        text_for_tag(obj, Tag(0x0008, 0x0050)),
    );

    let mut image = BTreeMap::new();
    insert_if(
        &mut image,
        "Modality",
        text_for_tag(obj, Tag(0x0008, 0x0060)),
    );
    insert_if(&mut image, "Rows", text_for_tag(obj, Tag(0x0028, 0x0010)));
    insert_if(
        &mut image,
        "Columns",
        text_for_tag(obj, Tag(0x0028, 0x0011)),
    );
    insert_if(
        &mut image,
        "Pixel Representation",
        text_for_tag(obj, Tag(0x0028, 0x0103)),
    );
    insert_if(
        &mut image,
        "Photometric Interpretation",
        text_for_tag(obj, Tag(0x0028, 0x0004)),
    );

    let mut misc = BTreeMap::new();
    insert_if(
        &mut misc,
        "SOP Class UID",
        text_for_tag(obj, Tag(0x0008, 0x0016)),
    );
    insert_if(
        &mut misc,
        "SOP Instance UID",
        text_for_tag(obj, Tag(0x0008, 0x0018)),
    );
    if let Some(ts) = obj.transfer_syntax() {
        insert_if(&mut misc, "Transfer Syntax", Some(ts));
    }

    DetailedMetadata {
        patient,
        study,
        image,
        misc,
    }
}

pub fn read_basic_metadata(path: &Path) -> Result<BasicMetadata> {
    let obj: DefaultDicomObject = open_file(path).context("Falha ao abrir arquivo DICOM")?;
    Ok(extract_basic_metadata(&obj))
}

pub fn read_detailed_metadata(path: &Path) -> Result<DetailedMetadata> {
    let obj: DefaultDicomObject = open_file(path).context("Falha ao abrir arquivo DICOM")?;
    Ok(extract_detailed_metadata(&obj))
}

pub fn print_info(path: &Path, verbose: bool) -> Result<()> {
    let obj: DefaultDicomObject = open_file(path).context("Falha ao abrir arquivo DICOM")?;
    let basic = extract_basic_metadata(&obj);

    println!("{}", "=".repeat(80));
    println!("DICOM File Information: {:?}", path.file_name().unwrap());
    println!("{}", "=".repeat(80));

    println!("PATIENT");
    println!("  Name: {}", basic.patient_name.as_deref().unwrap_or("N/A"));
    println!("  ID:   {}", basic.patient_id.as_deref().unwrap_or("N/A"));

    println!("\nSTUDY");
    println!("  Date: {}", basic.study_date.as_deref().unwrap_or("N/A"));
    println!(
        "  SOP Class: {}",
        basic.sop_class_uid.as_deref().unwrap_or("N/A")
    );

    println!("\nIMAGE");
    println!("  Modality: {}", basic.modality.as_deref().unwrap_or("N/A"));
    println!(
        "  Pixel Data: {}",
        if basic.has_pixel_data {
            "present"
        } else {
            "absent"
        }
    );

    if verbose {
        println!("\nALL TAGS (Verbose):");
        for element in obj.iter() {
            println!("  {} : {:?}", element.header().tag, element.value());
        }
    }

    Ok(())
}
