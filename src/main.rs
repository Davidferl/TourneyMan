use clap::Parser;
/// Simple tournament manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Name of the tournament
    #[arg(short, long)]
    tournament_name: String,
}

fn main() {
    let args = Args::parse();

    println!("Welcome to {}", args.tournament_name)
}
