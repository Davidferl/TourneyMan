use clap::Parser;
use crate::domain::tournament::Tournament;

mod arg_parser;
mod domain;


fn main() {
    let args = arg_parser::Args::parse();
    let tournament = Tournament::new(args.tournament_name, 4);

    println!("Welcome to {}", tournament.get_name());
}
