# DICOM Tools (Rust)

Command-line and web helpers for inspecting, anonymizing, validating, and converting DICOM files. Built with `dicom-rs`, Axum, and Tokio for fast local workflows and lightweight demos.

## Project Layout
- `src/main.rs`: CLI entry using Clap.
- `src/anonymize.rs`, `src/metadata.rs`, `src/image.rs`, `src/validate.rs`: Core DICOM operations.
- `src/batch.rs`: Parallel directory processing (WalkDir + Rayon).
- `src/web.rs` + `src/templates/index.html`: Simple Axum server and static UI.
- `Cargo.toml`: Dependencies and crate metadata.

## Prerequisites
- Rust toolchain (stable 1.75+ recommended) with `cargo`.
- Sample DICOM files for testing (keep them anonymized).

## Build and Run
Install deps and check the project:
```bash
cargo check
cargo fmt --all
cargo clippy --all-targets --all-features
```

Run commands locally:
```bash
# Extract metadata (verbose prints all tags)
cargo run -- info path/to/file.dcm --verbose

# Anonymize into <name>_anon.dcm unless --output is provided
cargo run -- anonymize path/to/file.dcm --output out/clean.dcm

# Convert to image (default png)
cargo run -- to-image path/to/file.dcm --format jpg

# Validate basic integrity
cargo run -- validate path/to/file.dcm

# Batch over a directory (operation: anonymize|validate)
cargo run -- batch --directory ./cases --operation anonymize
```

Start the web demo:
```bash
cargo run -- web --host 0.0.0.0 --port 3000
```
Then open `http://localhost:3000` and use the upload UI.

## Coding Style and Quality
- Format with `cargo fmt --all`.
- Lint with `cargo clippy --all-targets --all-features`; address warnings or justify them in reviews.
- Add `#[cfg(test)]` unit tests next to implementations; keep fixtures under `tests/data` if needed.

## Contributing
- Follow the guidelines in `AGENTS.md` (project-specific instructions, commands, and review expectations).
- Use imperative, concise commit messages and document any manual test commands in pull requests.
- Do not commit real PHI; only use synthetic or anonymized DICOM samples.
