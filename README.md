# DICOM Tools (Rust)

A robust command-line interface (CLI) and web utility for processing DICOM medical imaging files. Built with Rust, it leverages the `dicom-rs` ecosystem for core operations, offering high performance, type safety, and modern tooling integration.

## Project Overview

This project provides a suite of tools to:
- **Inspect:** Extract and view DICOM metadata tags in detail.
- **Anonymize:** Smart redaction of Patient Information (PII) based on Value Representations (VR). Automatically masks Names (`PN`), Dates (`DA`), and Times (`TM`) while hashing `PatientID`.
- **Convert:** Transform DICOM pixel data into standard image formats (PNG/JPG). Fully supports **multi-frame** images (videos/volumes) by extracting all frames.
- **JSON:** Bi-directional conversion between DICOM files and DICOM JSON representations for interoperability.
- **Validate:** Deep inspection of DICOM files, checking for critical attributes (SOP Class, Patient Info, Pixel Data) and standard compliance.
- **Network (Experimental):** Basic DICOM SCU capabilities (`echo`, `push`) to interact with PACS (currently in early development).
- **Serve:** A lightweight web server (`Axum`) for demonstrating these capabilities via a browser.

### Key Technologies
- **Language:** Rust (Edition 2021)
- **CLI:** `clap` (v4)
- **DICOM:** `dicom-rs` ecosystem (`dicom-core`, `dicom-object`, `dicom-pixeldata`, `dicom-ul`, `dicom-json`)
- **Web:** `axum` (v0.7), `tokio` (v1)
- **Concurrency:** `rayon` (v1.8)

## Architecture

The project is structured as a single binary with modularized functionality:

- **`src/main.rs`**: Application entry point and CLI dispatch.
- **`src/anonymize.rs`**: Generic VR-based anonymization logic.
- **`src/image.rs`**: Pixel data extraction and multi-frame image conversion.
- **`src/json.rs`**: DICOM <-> JSON conversion utilities.
- **`src/validate.rs`**: Deep validation of DICOM attributes and structure.
- **`src/scu.rs`**: Experimental DICOM networking (C-ECHO, C-STORE).
- **`src/web.rs`**: Axum web server implementation.
- **`src/batch.rs`**: Parallel directory processing.
- **`src/metadata.rs`**: Metadata extraction utilities.

## Building and Running

### Prerequisites
- **Rust Toolchain:** Stable Rust version installed (1.75+ recommended).

### Development Commands

| Action | Command | Description |
| :--- | :--- | :--- |
| **Build** | `cargo build` | Compiles the project in debug mode. |
| **Check** | `cargo check` | Fast compilation check. |
| **Test** | `cargo test` | Runs unit and integration tests. |
| **Format** | `cargo fmt --all` | Formats code to Rust standards. |
| **Lint** | `cargo clippy --all-targets --all-features` | Runs the linter. |

### Usage Examples

**CLI Mode:**

```bash
# Extract metadata
cargo run -- info path/to/image.dcm --verbose

# Anonymize a file (Smart VR-based)
cargo run -- anonymize path/to/image.dcm --output output/clean.dcm

# Convert to PNG (Extracts all frames for multi-frame files)
cargo run -- to-image path/to/image.dcm --format png

# Convert to JSON
cargo run -- to-json path/to/image.dcm --output metadata.json

# Create DICOM from JSON
cargo run -- from-json metadata.json --output restored.dcm

# Validate a file (Deep check)
cargo run -- validate path/to/image.dcm

# Network Echo (Experimental)
cargo run -- echo 127.0.0.1:104

# Batch anonymize a directory
cargo run -- batch --directory ./data/patients --operation anonymize
```

**Web Mode:**

```bash
# Start the server on localhost:3000
cargo run -- web --host 127.0.0.1 --port 3000
```

## Development Conventions

- **Code Style:** Adhere strictly to `rustfmt` and `clippy` defaults.
- **Error Handling:** Use `anyhow` for top-level error reporting.
- **Safety:** Do not commit real Protected Health Information (PHI). Use synthetic or anonymized DICOM data for testing.