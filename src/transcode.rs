use anyhow::{Context, Result};
use dicom::core::{DataElement, PrimitiveValue, Tag, VR};
use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use dicom::transfer_syntax::entries::EXPLICIT_VR_LITTLE_ENDIAN;
use std::borrow::Cow;
use std::path::Path;

/// Transcode a DICOM file to Explicit VR Little Endian (Uncompressed).
/// Currently only supports transcoding TO uncompressed syntaxes.
pub fn transcode(input: &Path, output: &Path) -> Result<()> {
    let obj = open_file(input).context("Failed to open DICOM file")?;

    // 1. Decode Pixel Data
    let decoded = obj
        .decode_pixel_data()
        .context("Failed to decode pixel data")?;

    // 2. Get raw bytes (native)
    let pixel_bytes = decoded
        .to_vec()
        .context("Failed to convert decoded pixels to vector")?;

    let bits_allocated = decoded.bits_allocated();

    // Release borrow on obj so we can consume it
    drop(decoded);

    // 3. Reconstruct object
    let mut new_obj = obj.into_inner(); // Unwrap the FileDicomObject to get InMemDicomObject

    // Update Pixel Data Element
    // Tag: 7FE0,0010
    let pixel_data_tag = Tag(0x7FE0, 0x0010);

    let vr = if bits_allocated > 8 { VR::OW } else { VR::OB };

    new_obj.put(DataElement::new(
        pixel_data_tag,
        vr,
        PrimitiveValue::from(pixel_bytes),
    ));

    // 4. Save with new Transfer Syntax

    use dicom::object::FileDicomObject;
    use dicom::object::FileMetaTableBuilder;

    let sop_class_uid = new_obj
        .element(Tag(0x0008, 0x0016))
        .ok()
        .and_then(|e| e.to_str().ok())
        .unwrap_or(Cow::Borrowed("1.2.840.10008.5.1.4.1.1.7"));

    let sop_instance_uid = new_obj
        .element(Tag(0x0008, 0x0018))
        .ok()
        .and_then(|e| e.to_str().ok())
        .unwrap_or(Cow::Borrowed("1.2.3.4.5"));

    let file_meta = FileMetaTableBuilder::new()
        .transfer_syntax(EXPLICIT_VR_LITTLE_ENDIAN.uid())
        .media_storage_sop_class_uid(sop_class_uid.as_ref())
        .media_storage_sop_instance_uid(sop_instance_uid.as_ref())
        .build()?;

    let mut file_obj = FileDicomObject::new_empty_with_dict_and_meta(
        dicom::dictionary_std::StandardDataDictionary,
        file_meta,
    );

    for elem in new_obj {
        file_obj.put(elem);
    }

    file_obj
        .write_to_file(output)
        .context("Failed to write output file")?;
    println!("Transcoded to Explicit VR Little Endian: {:?}", output);

    Ok(())
}
