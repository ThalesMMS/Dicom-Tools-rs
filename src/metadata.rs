use dicom::object::open_file;
use dicom::core::Tag;
use anyhow::{Result, Context};
use std::path::Path;

pub fn print_info(path: &Path, verbose: bool) -> Result<()> {
    let obj = open_file(path).context("Falha ao abrir arquivo DICOM")?;

    println!("{}", "=".repeat(80));
    println!("DICOM File Information: {:?}", path.file_name().unwrap());
    println!("{}", "=".repeat(80));

    // Helper to safely extract string values
    let get_str = |group, elem| {
        obj.element(Tag(group, elem))
            .ok()
            .and_then(|e| e.to_str().ok())
            .unwrap_or("N/A".into())
    };

    println!("PATIENT");
    println!("  Name: {}", get_str(0x0010, 0x0010));
    println!("  ID:   {}", get_str(0x0010, 0x0020));
    
    println!("\nSTUDY");
    println!("  Date: {}", get_str(0x0008, 0x0020));
    println!("  Desc: {}", get_str(0x0008, 0x1030));

    println!("\nIMAGE");
    println!("  Modality: {}", get_str(0x0008, 0x0060));
    println!("  Rows: {}", get_str(0x0028, 0x0010));
    println!("  Cols: {}", get_str(0x0028, 0x0011));

    if verbose {
        println!("\nALL TAGS (Verbose):");
        for element in obj.iter() {
            println!("  {} : {:?}", element.header().tag, element.value());
        }
    }

    Ok(())
}
