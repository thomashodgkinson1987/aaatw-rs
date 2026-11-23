#![cfg_attr(windows, windows_subsystem = "windows")]

use std::path::{Path, PathBuf};

use aaatw_assets::ResourceDll;
use aaatw_core::{AssetProvider, Config};
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "aaatw",
    version,
    about = "Open-source Amazing Adventures Around the World runtime"
)]
struct Cli {
    /// Directory containing Resource.dll and the other game assets.
    #[arg(long, value_name = "PATH")]
    data_dir: Option<PathBuf>,

    /// Explicit path to Resource.dll.
    #[arg(long, value_name = "PATH")]
    resource_dll: Option<PathBuf>,

    /// Skip the splash sequence and go straight to the title screen.
    #[arg(long)]
    skip_splash: bool,
}

struct GameAssets {
    dll: ResourceDll,
    data_dir: PathBuf,
}

impl AssetProvider for GameAssets {
    fn get(&self, uri: &str) -> Option<Vec<u8>> {
        if let Some(path) = uri.strip_prefix("std:") {
            return self.read_loose(path);
        }

        self.dll.get(uri).map(|data| data.to_vec())
    }
}

impl GameAssets {
    fn read_loose(&self, name: &str) -> Option<Vec<u8>> {
        for candidate in [
            name.to_string(),
            name.to_ascii_uppercase(),
            name.to_ascii_lowercase(),
        ] {
            if let Ok(data) = std::fs::read(self.data_dir.join(candidate)) {
                return Some(data);
            }
        }
        None
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut data_dir = cli.data_dir.clone().or_else(default_data_dir);
    let resource_path = cli
        .resource_dll
        .clone()
        .or_else(|| find_resource_dll(data_dir.as_deref()))
        .context("could not find Resource.dll; pass --resource-dll or --data-dir")?;

    if cli.data_dir.is_none()
        && let Some(parent) = resource_path.parent()
    {
        data_dir = Some(if parent.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            parent.to_path_buf()
        });
    }

    let data_dir = data_dir.unwrap_or_else(|| PathBuf::from("."));
    let dll = ResourceDll::open(&resource_path)
        .with_context(|| format!("failed to open {}", resource_path.display()))?;
    let assets = GameAssets {
        dll,
        data_dir: data_dir.clone(),
    };

    let config = Config {
        data_dir: Some(data_dir),
        resource_dll: Some(resource_path),
        skip_splash: cli.skip_splash,
    };

    let mut app = aaatw_core::App::new(&assets, config)?;
    let sdl_app = aaatw_sdl2::Sdl2App::new("Amazing Adventures Around the World", 800, 600)?;
    Ok(sdl_app.run(&mut app)?)
}

fn default_data_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
}

fn find_resource_dll(data_dir: Option<&Path>) -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(data_dir) = data_dir {
        candidates.push(data_dir.join("Resource.dll"));
    }

    if let Ok(executable) = std::env::current_exe()
        && let Some(parent) = executable.parent()
    {
        candidates.push(parent.join("Resource.dll"));
    }

    candidates.push(PathBuf::from("Resource.dll"));
    candidates.into_iter().find(|path| path.is_file())
}
