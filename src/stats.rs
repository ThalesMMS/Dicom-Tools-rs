use std::path::Path;

use anyhow::{Context, Result};
use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use dicom_pixeldata::{DecodedPixelData, PixelRepresentation};
use ndarray::ArrayD;

use crate::models::PixelStatistics;

/// Calculate and print basic statistics of the pixel data.
pub fn stats(input: &Path) -> Result<()> {
    let stats = pixel_statistics_for_file(input)?;

    println!("Statistics for {:?}", input);
    println!("  Shape: {:?}", stats.shape);
    println!("  Min:   {:.2}", stats.min);
    println!("  Max:   {:.2}", stats.max);
    println!("  Mean:  {:.2}", stats.mean);
    if let Some(median) = stats.median {
        println!("  Median:{:.2}", median);
    }
    println!("  StdDv: {:.2}", stats.std_dev);
    println!("  Total Pixels: {}", stats.total_pixels);

    Ok(())
}

pub fn pixel_statistics_for_file(input: &Path) -> Result<PixelStatistics> {
    let obj = open_file(input).context("Failed to open DICOM file")?;
    let decoded = obj
        .decode_pixel_data()
        .context("Failed to decode pixel data")?;

    pixel_statistics_from_decoded(&decoded)
}

pub fn pixel_statistics_from_decoded(decoded: &DecodedPixelData) -> Result<PixelStatistics> {
    let bits_allocated = decoded.bits_allocated();
    let pixel_representation = decoded.pixel_representation();

    let array = if pixel_representation == PixelRepresentation::Unsigned {
        if bits_allocated <= 8 {
            decoded.to_ndarray::<u8>()?.mapv(|v| v as f32).into_dyn()
        } else if bits_allocated <= 16 {
            decoded.to_ndarray::<u16>()?.mapv(|v| v as f32).into_dyn()
        } else {
            decoded.to_ndarray::<u32>()?.mapv(|v| v as f32).into_dyn()
        }
    } else if bits_allocated <= 8 {
        decoded.to_ndarray::<i8>()?.mapv(|v| v as f32).into_dyn()
    } else if bits_allocated <= 16 {
        decoded.to_ndarray::<i16>()?.mapv(|v| v as f32).into_dyn()
    } else {
        decoded.to_ndarray::<i32>()?.mapv(|v| v as f32).into_dyn()
    };

    Ok(compute_stats(array))
}

fn compute_stats(array: ArrayD<f32>) -> PixelStatistics {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let mut sum = 0f64;
    let mut values = Vec::with_capacity(array.len());

    for &v in array.iter() {
        let v_f = v as f64;
        min = min.min(v);
        max = max.max(v);
        sum += v_f;
        values.push(v);
    }

    let total_pixels = values.len();
    if total_pixels == 0 {
        return PixelStatistics {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            median: None,
            std_dev: 0.0,
            total_pixels: 0,
            shape: array.shape().to_vec(),
        };
    }

    let mean = (sum / total_pixels as f64) as f32;

    let mut variance_sum = 0f64;
    for v in &values {
        let diff = *v as f64 - mean as f64;
        variance_sum += diff * diff;
    }
    let std_dev = (variance_sum / total_pixels as f64).sqrt() as f32;

    let median = {
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            Some((sorted[mid - 1] + sorted[mid]) / 2.0)
        } else {
            Some(sorted[mid])
        }
    };

    PixelStatistics {
        min,
        max,
        mean,
        median,
        std_dev,
        total_pixels,
        shape: array.shape().to_vec(),
    }
}
