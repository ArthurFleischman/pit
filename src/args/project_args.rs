use clap::Parser;

#[derive(Parser, Debug)]
pub struct ProjectArgs{
    #[clap(short='n', long)]
    pub name: String,

    #[clap(short='p', long, default_value = "$PROJECTS_ENV")]
    pub path: String,
    
    #[clap(short='l', long)]
    pub lang: String,
    
    #[clap(short='t', long)]
    pub typ: String
}