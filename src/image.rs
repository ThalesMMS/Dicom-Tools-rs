use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn convert(input: &Path, output: Option<PathBuf>, format: &str) -> Result<()> {
    let obj = open_file(input)?;
    
    // Decode pixel data (handles compression when features are enabled)
    let decoded_image = obj.decode_pixel_data()?;
    let num_frames = decoded_image.number_of_frames();

    let base_output = output.unwrap_or_else(|| {
        let mut p = input.to_path_buf();
        p.set_extension(format);
        p
    });

    if num_frames <= 1 {
        // Convert into a DynamicImage from the image crate
        let dynamic_image = decoded_image.to_dynamic_image(0)?; // Frame 0
        dynamic_image.save(&base_output)?;
        println!("Image saved to: {:?}", base_output);
    } else {
        println!("Multi-frame DICOM detected: {} frames.", num_frames);
        let parent = base_output.parent().unwrap_or_else(|| Path::new("."));
        let stem = base_output.file_stem().unwrap().to_string_lossy();

        for i in 0..num_frames {
            let dynamic_image = decoded_image.to_dynamic_image(i)?;
            let frame_name = format!("{}_frame{:03}.{}", stem, i, format);
            let frame_path = parent.join(frame_name);
            
            dynamic_image.save(&frame_path)?;
            println!("Saved frame {} to {:?}", i, frame_path);
        }
    }

    Ok(())
}
