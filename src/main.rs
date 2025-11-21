use dicom_tools::cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
