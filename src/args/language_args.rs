use clap::Parser;

#[derive(Parser, Debug)]
pub struct LanguageArgs{
    #[clap(short='H', long)]
    pub handlers: Vec<String>,
    #[clap(short='m', long)]
    pub modules: Vec<String>
}