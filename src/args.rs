use clap::{Parser,Subcommand};
pub mod project_args;
mod language_args;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action
}

#[derive(Debug,Subcommand)]
pub enum Action {
    New(project_args::ProjectArgs),
    Create(language_args::LanguageArgs)
}