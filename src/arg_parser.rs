use clap::Parser;

/// Simple tournament manager
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    /// Name of the tournament
    #[arg(short, long)]
    pub tournament_name: String,


}