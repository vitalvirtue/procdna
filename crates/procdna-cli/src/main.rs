use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "procdna")]
#[command(about = "AI-native process passport layer for Linux")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Ps,
    Explain {
        pid: Option<u32>,
    },
    Graph {
        pid: Option<u32>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ps => {
            println!("ProcDNA ps: process passport list will be implemented.");
        }
        Commands::Explain { pid } => {
            println!("ProcDNA explain: pid={pid:?}");
        }
        Commands::Graph { pid } => {
            println!("ProcDNA graph: pid={pid:?}");
        }
    }

    Ok(())
}
