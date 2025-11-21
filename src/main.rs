use std::process::exit;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

use crate::commands::list;
use crate::commands::record;
use crate::commands::replay;
use crate::storage::Storage;
use crate::storage::json::JsonFileStorage;

mod commands;
mod storage;
mod structure;

const FISH_SCRIPT: &str = include_str!("../scripts/parrot.fish");

/// 🦜 Record & replay shell commands 🦜
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: ParrotCommand,
}

#[derive(Debug, Clone, ValueEnum)]
enum Shell {
    Bash,
    Fish,
}

#[derive(Subcommand, Debug)]
enum ParrotCommand {
    /// Record commands
    #[clap(subcommand)]
    Record(RecordAction),

    /// List recorded commands
    List,

    /// Replay recorded commands
    Replay {
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Initialize shell integration
    #[clap(value_enum)]
    Init {
        /// Shell type to initialize
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
enum RecordAction {
    /// Start interactive recording in your terminal text editor
    Interactive {
        /// Name or tag of the recording
        #[clap(short, long)]
        tag: String,
    },

    /// Start recording
    Start {
        /// Name or tag of the recording
        #[clap(short, long)]
        tag: String,
    },

    /// Record a command
    Add {
        /// Command to record
        command: String,
    },

    /// Stop recording
    Stop,

    /// Abort recording
    Abort,
}

#[derive(thiserror::Error, Debug)]
enum ParrotError {
    #[error("Storage error: {0}")]
    StorageError(#[from] storage::StorageError),

    #[error("Record error: {0}")]
    RecordError(#[from] record::RecordError),

    #[error("Replay error: {0}")]
    ReplayError(#[from] replay::ReplayError),
}

fn main() {
    let args = Args::parse();
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| ".".into())
        .join("parrot.json");
    let storage = JsonFileStorage::new(&config_dir);
    if let Err(e) = run_command(args, storage) {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn run_command(args: Args, mut storage: JsonFileStorage) -> Result<(), ParrotError> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let mut data = storage.load()?;
    let needs_save = matches!(args.command, ParrotCommand::Record(_));

    match args.command {
        ParrotCommand::Init { shell } => match shell {
            Shell::Bash => {
                todo!("Bash initialization script is not implemented yet");
            }
            Shell::Fish => {
                println!("{}", FISH_SCRIPT);
            }
        },
        ParrotCommand::List => list::run(&mut data),
        ParrotCommand::Record(action) => match action {
            RecordAction::Interactive { tag } => record::interactive(tag, &mut data, &editor)?,
            RecordAction::Start { tag } => record::start(tag, &mut data),
            RecordAction::Add { command } => record::add(command, &mut data),
            RecordAction::Stop => record::stop(&mut data),
            RecordAction::Abort => record::abort(&mut data),
        },
        ParrotCommand::Replay { tag } => replay::run(tag, &data)?,
    }

    if needs_save {
        storage.save(&data)?;
    }

    Ok(())
}
