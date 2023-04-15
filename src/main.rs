use crate::engine::FunctionCallMatchingEngine;
use clap::{Parser, Subcommand};
use eyre::Result;
use solidity::ast::*;
use std::fs::File;
use std::io::BufReader;
mod dsl;
mod engine;
use dsl::Rule;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[clap(name = "pretty-print")]
    PrettyPrint {
        #[clap(help = "The path to a file with Solidity's AST representation in JSON format")]
        path: String,
    },
    #[clap(name = "search")]
    Search {
        #[clap(help = "The path to a file with Solidity's AST represenation in JSON format")]
        path_ast: String,
        #[clap(help = "The path to a rule written in YAML")]
        path_rule: String,
    },
}

fn main() -> Result<()> {
    let opts = Cli::parse();
    match opts.sub {
        Subcommands::PrettyPrint { path } => pretty_print(path)?,
        Subcommands::Search {
            path_ast,
            path_rule,
        } => search(path_ast, path_rule)?,
    }

    Ok(())
}

fn search(path_ast: String, path_rule: String) -> Result<()> {
    let rule: Rule = serde_yaml::from_reader(File::open(path_rule)?)?;
    let source_unit: SourceUnit = serde_json::from_reader(BufReader::new(File::open(path_ast)?))?;
    let function_call_matching_engine = FunctionCallMatchingEngine::new(rule);
    function_call_matching_engine.match_source_unit(source_unit)?;
    Ok(())
}

// TODO move into a separate
fn pretty_print(path: String) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let deserialized_data: SourceUnit = serde_json::from_reader(reader)?;
    println!("{deserialized_data:#?}");
    Ok(())
}
