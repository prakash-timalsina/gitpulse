use clap::Parser;

/// gitpulse — check the pulse of your repo
#[derive(Parser)]
#[command(name = "gitpulse", version, about)]
struct Cli {
    /// Show extra detail in the output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode is on.");
    }

    println!("gitpulse is alive.");
}