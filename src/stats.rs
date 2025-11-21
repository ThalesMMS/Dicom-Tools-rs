use anyhow::{Context, Result};
use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use std::path::Path;
use ndarray::ArrayD;
use dicom_pixeldata::PixelRepresentation;

/// Calculate and print basic statistics of the pixel data.
pub fn stats(input: &Path) -> Result<()> {
    let obj = open_file(input).context("Failed to open DICOM file")?;
    let decoded = obj.decode_pixel_data().context("Failed to decode pixel data")?;

    let bits_allocated = decoded.bits_allocated();
    let pixel_representation = decoded.pixel_representation(); 

    let (min, max, mean, std_dev, shape) = if pixel_representation == PixelRepresentation::Unsigned {
        // Unsigned
        if bits_allocated <= 8 {
            let arr = decoded.to_ndarray::<u8>().context("Failed to convert to u8 ndarray")?;
            compute_stats(&arr.mapv(|v| v as f32).into_dyn())
        } else if bits_allocated <= 16 {
            let arr = decoded.to_ndarray::<u16>().context("Failed to convert to u16 ndarray")?;
            compute_stats(&arr.mapv(|v| v as f32).into_dyn())
        } else {
             let arr = decoded.to_ndarray::<u32>().context("Failed to convert to u32 ndarray")?;
             compute_stats(&arr.mapv(|v| v as f32).into_dyn())
        }
    } else {
        // Signed
        if bits_allocated <= 8 {
             let arr = decoded.to_ndarray::<i8>().context("Failed to convert to i8 ndarray")?;
             compute_stats(&arr.mapv(|v| v as f32).into_dyn())
        } else if bits_allocated <= 16 {
             let arr = decoded.to_ndarray::<i16>().context("Failed to convert to i16 ndarray")?;
             compute_stats(&arr.mapv(|v| v as f32).into_dyn())
        } else {
             let arr = decoded.to_ndarray::<i32>().context("Failed to convert to i32 ndarray")?;
             compute_stats(&arr.mapv(|v| v as f32).into_dyn())
        }
    };

    println!("Statistics for {:?}", input);
    println!("  Shape: {:?}", shape);
    println!("  Min:   {:.2}", min);
    println!("  Max:   {:.2}", max);
    println!("  Mean:  {:.2}", mean);
    println!("  StdDv: {:.2}", std_dev);

    Ok(())
}

fn compute_stats(array: &ArrayD<f32>) -> (f32, f32, f32, f32, Vec<usize>) {
    let min = array.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = array.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let sum: f32 = array.iter().sum();
    let count = array.len() as f32;
    let mean = sum / count;
    
    let variance = array.iter().map(|x| {
        let diff = mean - x;
        diff * diff
    }).sum::<f32>() / count;
    let std_dev = variance.sqrt();
    
    (min, max, mean, std_dev, array.shape().to_vec())
}
