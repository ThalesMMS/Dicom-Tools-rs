use walkdir::WalkDir;
use rayon::prelude::*;
use std::path::Path;
use anyhow::Result;
use crate::{anonymize, validate};

pub fn process_directory(dir: &Path, operation: &str) -> Result<()> {
    println!("Processando diretório: {:?} | Operação: {}", dir, operation);

    // Collect all .dcm files
    let files: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "dcm"))
        .collect();

    println!("Encontrados {} arquivos.", files.len());

    // Parallel processing
    files.par_iter().for_each(|entry| {
        let path = entry.path();
        let res = match operation {
            "anonymize" => anonymize::process_file(path, None),
            "validate" => validate::check_file(path),
            _ => Ok(()),
        };

        if let Err(e) = res {
            eprintln!("Erro em {:?}: {}", path, e);
        } else {
            println!("Sucesso: {:?}", path.file_name().unwrap());
        }
    });

    Ok(())
}
