use std::path::PathBuf;

use dicom::core::{DataElement, PrimitiveValue, Tag, VR};
use dicom::dictionary_std::StandardDataDictionary;
use dicom::object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom::transfer_syntax::entries::EXPLICIT_VR_LITTLE_ENDIAN;
use dicom_tools::{anonymize, image, metadata, stats, transcode, validate};
use tempfile::{tempdir, TempDir};

fn build_test_dicom() -> (TempDir, PathBuf) {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("sample.dcm");

    let mut obj = InMemDicomObject::new_empty_with_dict(StandardDataDictionary);
    obj.put(DataElement::new(
        Tag(0x0010, 0x0010),
        VR::PN,
        PrimitiveValue::from("Test^Patient"),
    ));
    obj.put(DataElement::new(
        Tag(0x0010, 0x0020),
        VR::LO,
        PrimitiveValue::from("PAT123"),
    ));
    obj.put(DataElement::new(
        Tag(0x0008, 0x0060),
        VR::CS,
        PrimitiveValue::from("OT"),
    ));
    obj.put(DataElement::new(
        Tag(0x0008, 0x0020),
        VR::DA,
        PrimitiveValue::from("20240101"),
    ));
    obj.put(DataElement::new(
        Tag(0x0008, 0x0016),
        VR::UI,
        PrimitiveValue::from("1.2.840.10008.5.1.4.1.1.7"),
    ));
    obj.put(DataElement::new(
        Tag(0x0008, 0x0018),
        VR::UI,
        PrimitiveValue::from("1.2.826.0.1.3680043.2.1125.1"),
    ));

    obj.put(DataElement::new(
        Tag(0x0028, 0x0010),
        VR::US,
        PrimitiveValue::from(2_u16),
    )); // Rows
    obj.put(DataElement::new(
        Tag(0x0028, 0x0011),
        VR::US,
        PrimitiveValue::from(2_u16),
    )); // Columns
    obj.put(DataElement::new(
        Tag(0x0028, 0x0002),
        VR::US,
        PrimitiveValue::from(1_u16),
    )); // Samples per pixel
    obj.put(DataElement::new(
        Tag(0x0028, 0x0100),
        VR::US,
        PrimitiveValue::from(8_u16),
    )); // Bits Allocated
    obj.put(DataElement::new(
        Tag(0x0028, 0x0101),
        VR::US,
        PrimitiveValue::from(8_u16),
    )); // Bits Stored
    obj.put(DataElement::new(
        Tag(0x0028, 0x0102),
        VR::US,
        PrimitiveValue::from(7_u16),
    )); // High Bit
    obj.put(DataElement::new(
        Tag(0x0028, 0x0103),
        VR::US,
        PrimitiveValue::from(0_u16),
    )); // Pixel Representation
    obj.put(DataElement::new(
        Tag(0x0028, 0x0004),
        VR::CS,
        PrimitiveValue::from("MONOCHROME2"),
    ));

    obj.put(DataElement::new(
        Tag(0x7fe0, 0x0010),
        VR::OB,
        PrimitiveValue::from(vec![0, 64, 128, 255]),
    ));

    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(EXPLICIT_VR_LITTLE_ENDIAN.uid())
        .media_storage_sop_class_uid("1.2.840.10008.5.1.4.1.1.7")
        .media_storage_sop_instance_uid("1.2.826.0.1.3680043.2.1125.1")
        .build()
        .expect("meta");

    let mut file_obj = FileDicomObject::new_empty_with_dict_and_meta(StandardDataDictionary, meta);
    for elem in obj {
        file_obj.put(elem);
    }
    file_obj.write_to_file(&path).expect("write test dicom");

    (dir, path)
}

#[test]
fn metadata_and_validation_cover_required_tags() {
    let (_dir, path) = build_test_dicom();

    let basic = metadata::read_basic_metadata(&path).expect("basic metadata");
    assert_eq!(basic.patient_name.as_deref(), Some("Test^Patient"));
    assert_eq!(basic.modality.as_deref(), Some("OT"));
    assert!(basic.has_pixel_data);

    let obj = dicom::object::open_file(&path).expect("open file");
    let report = validate::validate_obj(&obj);
    assert!(report.valid);
    assert!(report.has_pixel_data);
}

#[test]
fn pixel_stats_and_image_preview_work() {
    let (_dir, path) = build_test_dicom();

    let stats = stats::pixel_statistics_for_file(&path).expect("stats");
    assert_eq!(stats.total_pixels, 4);
    assert!((stats.min - 0.0).abs() < f32::EPSILON);
    assert!((stats.max - 255.0).abs() < f32::EPSILON);
    assert!((stats.mean - 111.75).abs() < 0.1);
    assert!(stats.median.unwrap_or_default() > 90.0 && stats.median.unwrap_or_default() < 100.0);

    let png = image::first_frame_png_bytes(&path).expect("render png");
    assert!(png.starts_with(&[0x89, b'P', b'N', b'G']));
}

#[test]
fn anonymization_creates_clean_copy() {
    let (_dir, path) = build_test_dicom();
    let output = path.with_file_name("sample_anon.dcm");

    anonymize::process_file(&path, Some(output.clone())).expect("anonymize");
    let anon = dicom::object::open_file(&output).expect("open anon");

    let patient_name = anon
        .element(Tag(0x0010, 0x0010))
        .expect("name")
        .to_str()
        .unwrap();
    assert_eq!(patient_name, "ANONYMOUS^PATIENT");
}

#[test]
fn transcode_keeps_pixel_data_intact() {
    let (_dir, path) = build_test_dicom();
    let output = path.with_file_name("sample_transcoded.dcm");

    transcode::transcode(&path, &output).expect("transcode");

    let baseline = stats::pixel_statistics_for_file(&path).expect("baseline stats");
    let transcoded = stats::pixel_statistics_for_file(&output).expect("transcoded stats");

    assert_eq!(baseline.total_pixels, transcoded.total_pixels);
    assert!((baseline.min - transcoded.min).abs() < f32::EPSILON);
    assert!((baseline.max - transcoded.max).abs() < f32::EPSILON);
}
