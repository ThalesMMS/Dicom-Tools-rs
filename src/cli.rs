use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

use crate::{anonymize, batch, image, json, metadata, scu, stats, transcode, validate, web};

#[derive(Parser)]
#[command(name = "dicom-tools")]
#[command(about = "Ferramentas DICOM em Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
    Validate { file: PathBuf },
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
        #[arg(short, long, value_enum)]
        operation: BatchOperation,
    },
    /// Perform a DICOM C-ECHO (Ping)
    Echo { addr: String },
    /// Perform a DICOM C-STORE (Push)
    Push { addr: String, file: PathBuf },
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
        #[arg(
            long,
            value_enum,
            default_value_t = TransferSyntax::ExplicitVrLittleEndian,
            help = "Target transfer syntax (uncompressed only)"
        )]
        transfer_syntax: TransferSyntax,
    },
    /// Calculate Pixel Statistics
    Stats { file: PathBuf },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum BatchOperation {
    Anonymize,
    Validate,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum TransferSyntax {
    ExplicitVrLittleEndian,
    ImplicitVrLittleEndian,
}

impl From<TransferSyntax> for transcode::UncompressedTransferSyntax {
    fn from(value: TransferSyntax) -> Self {
        match value {
            TransferSyntax::ExplicitVrLittleEndian => {
                transcode::UncompressedTransferSyntax::ExplicitVRLittleEndian
            }
            TransferSyntax::ImplicitVrLittleEndian => {
                transcode::UncompressedTransferSyntax::ImplicitVRLittleEndian
            }
        }
    }
}

pub async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { file, verbose } => metadata::print_info(&file, verbose)?,
        Commands::Anonymize { input, output } => anonymize::process_file(&input, output)?,
        Commands::ToImage {
            input,
            output,
            format,
        } => image::convert(&input, output, &format)?,
        Commands::Validate { file } => validate::check_file(&file)?,
        Commands::Web { host, port } => web::start_server(&host, port).await?,
        Commands::Batch {
            directory,
            operation,
        } => batch::process_directory(&directory, operation)?,
        Commands::Echo { addr } => scu::echo(&addr)?,
        Commands::Push { addr, file } => scu::push(&addr, &file)?,
        Commands::ToJson { file, output } => json::to_json(&file, output.as_deref())?,
        Commands::FromJson { input, output } => json::from_json(&input, &output)?,
        Commands::Transcode {
            input,
            output,
            transfer_syntax,
        } => transcode::transcode(&input, &output, transfer_syntax.into())?,
        Commands::Stats { file } => stats::stats(&file)?,
    }

    Ok(())
}
