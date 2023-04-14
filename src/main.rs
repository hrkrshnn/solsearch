use clap::{Parser, Subcommand};
use eyre::Result;
use solidity::ast::SourceUnit;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[clap(name = "pretty-print")]
    PrettyPrint {
        #[clap(help = "A JSON file to solc's AST")]
        path: String,
    },
}

fn main() -> Result<()> {
    let opts = Cli::parse();
    println!("{opts:?}");

    match opts.sub {
        Subcommands::PrettyPrint { path } => pretty_print(path)?,
    }

    Ok(())
}

// TODO move into a separate file later
fn pretty_print(path: String) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let deserialized_data: SourceUnit = serde_json::from_reader(reader)?;
    println!("{deserialized_data:#?}");
    Ok(())
}
