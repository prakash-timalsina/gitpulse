




use clap::Parser;
use git2::Repository;

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

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Not a git repository: {}", e);
            std::process::exit(1);
        }
    };

    if cli.verbose {
        println!("Verbose mode is on.");
        println!("Opened repo at: {:?}", repo.path());
    }

    println!("gitpulse is alive.");
}