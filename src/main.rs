mod anonymize;
mod batch;
mod image;
mod metadata;
mod validate;
mod web;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dicom-tools")]
#[command(about = "Ferramentas DICOM em Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract metadata (analogue to extract_metadata.py / dicom_info.py)
    Info {
        file: PathBuf,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Anonymize a DICOM file
    Anonymize {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert to an image (similar to convert_to_image.py)
    ToImage {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "png")]
        format: String,
    },
    /// Validate file integrity
    Validate {
        file: PathBuf,
    },
    /// Start the web server
    Web {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Batch processing over a directory
    Batch {
        #[arg(short, long)]
        directory: PathBuf,
        #[arg(short, long)]
        operation: String, // choices: anonymize, convert, validate
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { file, verbose } => metadata::print_info(&file, verbose)?,
        Commands::Anonymize { input, output } => anonymize::process_file(&input, output)?,
        Commands::ToImage { input, output, format } => image::convert(&input, output, &format)?,
        Commands::Validate { file } => validate::check_file(&file)?,
        Commands::Web { host, port } => web::start_server(&host, port).await?,
        Commands::Batch { directory, operation } => batch::process_directory(&directory, &operation)?,
    }

    Ok(())
}
