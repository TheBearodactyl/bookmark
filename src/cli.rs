use crate::data::{Bookmark, BookmarkData};
use clap::{Parser, Subcommand};
use prettytable::{Table, format, row};
use std::fs::read_to_string;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    command: Cmds,
}

#[derive(Subcommand)]
enum Cmds {
    /// Initialize the config file
    Init,
    /// List the current bookmarks and their data
    List,
    /// Open a specific bookmark's path with your system default
    Open {
        /// The name of the bookmark
        name: String,
    },
    /// Add a bookmark
    Add {
        /// The name of the bookmark
        #[arg(short = 'n', long)]
        name: String,
        
        /// The path to the bookmarked file/folder
        #[arg(short = 'p', long)]
        path: String,
        
        /// The description for the bookmark
        #[arg(short = 'd', long)]
        description: String,
    },
    /// Remove a bookmark
    Remove {
        /// The name of the bookmark
        name: String,

    },
}

pub fn cli() -> anyhow::Result<()> {
    let argv = Cli::parse();

    init_cfg_file()?;
    
    match argv.command {
        Cmds::Init {} => {
            init_cfg_file()?;
        }
        Cmds::List => {
            let local_appdata = std::env::var("LOCALAPPDATA")?;
            let local_path = std::path::Path::new(&local_appdata).join("bookmarks.json");

            let cfg_str = read_to_string(local_path)?;
            let config = serde_json::from_str::<BookmarkData>(&cfg_str)?;

            if config.bookmarks.is_empty() {
                println!("No bookmarks found.");
                return Ok(());
            }

            // Create a pretty table
            let mut table = Table::new();
            table.set_format(
                format::FormatBuilder::new()
                    .column_separator('│')
                    .borders('│')
                    .separators(
                        &[format::LinePosition::Top],
                        format::LineSeparator::new('─', '┬', '┌', '┐'),
                    )
                    .separators(
                        &[format::LinePosition::Intern],
                        format::LineSeparator::new('─', '┼', '├', '┤'),
                    )
                    .separators(
                        &[format::LinePosition::Bottom],
                        format::LineSeparator::new('─', '┴', '└', '┘'),
                    )
                    .padding(1, 1)
                    .build(),
            );

            // Add header row
            table.add_row(row![bF=> "NAME", "PATH", "DESCRIPTION"]);

            // Add bookmark rows
            for bookmark in config.bookmarks {
                table.add_row(row![bookmark.name, bookmark.path, bookmark.desc,]);
            }

            table.printstd();
        }
        Cmds::Add {
            name,
            path,
            description,
        } => {
            add_bookmark(Bookmark {
                name,
                path,
                desc: description,
            })?;
        }
        Cmds::Open { name } => {
            let local_appdata = std::env::var("LOCALAPPDATA")?;
            let local_path = std::path::Path::new(&local_appdata).join("bookmarks.json");

            let cfg_str = read_to_string(local_path)?;
            let config = serde_json::from_str::<BookmarkData>(&cfg_str)?;

            if let Some(bookmark) = config.bookmarks.iter().find(|b| b.name == name) {
                opener::open(&bookmark.path)?;
                println!(
                    "-- Opened bookmark '{}' at path: {}",
                    bookmark.name, bookmark.path
                );
            } else {
                anyhow::bail!("-- Bookmark '{}' not found", name);
            }
        }
        Cmds::Remove { name} => {
            remove_bookmark(&name)?;
            println!("Removed bookmark '{}'", name);
        }
    }

    Ok(())
}

pub fn init_cfg_file() -> anyhow::Result<()> {
    let local_appdata = std::env::var("LOCALAPPDATA")?;
    let local_path = std::path::Path::new(&local_appdata).join("bookmarks.json");

    if !local_path.exists() {
        let empty_cfg = serde_json::to_string_pretty(&BookmarkData::default());
        std::fs::write(local_path.clone(), empty_cfg?)?;

        println!("-- Created configuration file at {}", local_path.display());
    }

    Ok(())
}

pub fn add_bookmark(bookmark: Bookmark) -> anyhow::Result<()> {
    let local_appdata = std::env::var("LOCALAPPDATA")?;
    let local_path = std::path::Path::new(&local_appdata).join("bookmarks.json");

    if !local_path.exists() {
        init_cfg_file()?;
    } else {
        let cfg_str = read_to_string(local_path.clone())?;
        let mut config = serde_json::from_str::<BookmarkData>(cfg_str.as_str())?;

        config.bookmarks.push(bookmark);

        std::fs::write(
            local_path.clone(),
            serde_json::to_string_pretty(&config).expect("Failed to serialize"),
        )
        .expect("Failed to save config");
    }

    Ok(())
}

pub fn remove_bookmark(name: &str) -> anyhow::Result<()> {
    let local_appdata = std::env::var("LOCALAPPDATA")?;
    let local_path = std::path::Path::new(&local_appdata).join("bookmarks.json");

    if !local_path.exists() {
        anyhow::bail!("Configuration file not found");
    }

    let cfg_str = read_to_string(local_path.clone())?;
    let mut config = serde_json::from_str::<BookmarkData>(&cfg_str)?;

    let initial_len = config.bookmarks.len();
    config.bookmarks.retain(|b| b.name != name);

    if config.bookmarks.len() == initial_len {
        anyhow::bail!("Bookmark with name '{}' not found", name);
    }

    std::fs::write(
        local_path,
        serde_json::to_string_pretty(&config).expect("Failed to serialize"),
    )?;

    Ok(())
}
