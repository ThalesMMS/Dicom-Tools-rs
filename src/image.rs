use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn convert(input: &Path, output: Option<PathBuf>, format: &str) -> Result<()> {
    let obj = open_file(input)?;
    
    // Decode pixel data (handles compression when features are enabled)
    let decoded_image = obj.decode_pixel_data()?;
    
    // Convert into a DynamicImage from the image crate
    let dynamic_image = decoded_image.to_dynamic_image(0)?; // Frame 0

    let output_path = output.unwrap_or_else(|| {
        let mut p = input.to_path_buf();
        p.set_extension(format);
        p
    });

    dynamic_image.save(&output_path)?;
    println!("Imagem salva em: {:?}", output_path);

    Ok(())
}
