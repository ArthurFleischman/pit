use clap::Parser;
mod args;
use args::Action;
mod handler;


fn main() {
    let args = args::Args::parse();
    handler::handle_data(args.action);
}
