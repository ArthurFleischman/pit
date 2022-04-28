use clap::Parser;
mod args;
use args::Action;
fn main() {
    let args = args::Args::parse();

    match args.action {
        Action::New(values) => println!("{:?}",values),
        Action::Create(values) => println!("{:?}",values)
    }
}
