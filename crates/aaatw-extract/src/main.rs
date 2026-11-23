use std::fs;
use std::path::PathBuf;

use aaatw_assets::ResourceDll;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "aaatw-extract",
    version,
    about = "Inspect and extract Amazing Adventures Around the World resources"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List all resource names.
    List {
        /// Path to Resource.dll.
        resource_dll: PathBuf,
    },
    /// Print resource metadata.
    Info {
        /// Path to Resource.dll.
        resource_dll: PathBuf,
    },
    /// Extract one resource to a file.
    Extract {
        /// Path to Resource.dll.
        resource_dll: PathBuf,
        /// Resource name, for example SPLASHSCREENS.XUI.
        name: String,
        /// Output file.
        output: PathBuf,
    },
    /// Extract all resources into a directory.
    ExtractAll {
        /// Path to Resource.dll.
        resource_dll: PathBuf,
        /// Output directory.
        output_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::List { resource_dll } => {
            let dll = ResourceDll::open(&resource_dll)?;
            println!("{} resources", dll.len());
            for name in dll.list() {
                println!("{name}");
            }
        }
        Command::Info { resource_dll } => {
            let dll = ResourceDll::open(&resource_dll)?;
            println!("{} resources", dll.len());
            println!("Use `aaatw-extract list` to print the names, or `extract-all` to dump them.");
        }
        Command::Extract {
            resource_dll,
            name,
            output,
        } => {
            let dll = ResourceDll::open(&resource_dll)?;
            let data = dll
                .get(&name)
                .with_context(|| format!("resource not found: {name}"))?;
            fs::write(&output, data)?;
            println!("wrote {} bytes to {}", data.len(), output.display());
        }
        Command::ExtractAll {
            resource_dll,
            output_dir,
        } => {
            let dll = ResourceDll::open(&resource_dll)?;
            fs::create_dir_all(&output_dir)?;
            let mut count = 0usize;
            for name in dll.list() {
                if let Some(data) = dll.get(name) {
                    let safe = sanitize(name);
                    fs::write(output_dir.join(safe), data)?;
                    count += 1;
                }
            }
            println!("extracted {count} resources to {}", output_dir.display());
        }
    }

    Ok(())
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => ch,
        })
        .collect()
}
