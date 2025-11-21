use anyhow::Result;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

use crate::{anonymize, cli::BatchOperation, validate};

pub fn process_directory(dir: &Path, operation: BatchOperation) -> Result<()> {
    println!(
        "Processando diretório: {:?} | Operação: {:?}",
        dir, operation
    );

    let files: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "dcm"))
        .collect();

    println!("Encontrados {} arquivos.", files.len());

    files.par_iter().for_each(|entry| {
        let path = entry.path();
        let res = match operation {
            BatchOperation::Anonymize => anonymize::process_file(path, None),
            BatchOperation::Validate => validate::check_file(path),
        };

        if let Err(e) = res {
            eprintln!("Erro em {:?}: {}", path, e);
        } else {
            println!("Sucesso: {:?}", path.file_name().unwrap());
        }
    });

    Ok(())
}
