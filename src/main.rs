mod anonymize;
mod batch;
mod image;
mod json;
mod metadata;
mod scu;
mod stats;
mod transcode;
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
    },
    /// Perform a DICOM C-ECHO (Ping)
    Echo {
        addr: String,
    },
    /// Perform a DICOM C-STORE (Push)
    Push {
        addr: String,
        file: PathBuf,
    },
    /// Convert DICOM to JSON
    ToJson {
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert JSON to DICOM
    FromJson {
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Transcode a DICOM file (Decompress)
    Transcode {
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Calculate Pixel Statistics
    Stats {
        file: PathBuf,
    },
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
        Commands::Echo { addr } => scu::echo(&addr)?,
        Commands::Push { addr, file } => scu::push(&addr, &file)?,
        Commands::ToJson { file, output } => json::to_json(&file, output.as_deref())?,
        Commands::FromJson { input, output } => json::from_json(&input, &output)?,
        Commands::Transcode { input, output } => transcode::transcode(&input, &output)?,
        Commands::Stats { file } => stats::stats(&file)?,
    }

    Ok(())
}
