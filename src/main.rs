// use clap::Parser;
// use git2::Repository;

// mod git_ops;

// #[derive(Parser)]
// #[command(name = "gitpulse", version, about)]
// struct Cli {
//     #[arg(short, long)]
//     verbose: bool,
// }

// fn main() {
//     let cli = Cli::parse();

//     let repo = match Repository::open(".") {
//         Ok(repo) => repo,
//         Err(e) => {
//             eprintln!("Not a git repository: {}", e);
//             std::process::exit(1);
//         }
//     };

//     let branch = git_ops::current_branch(&repo);
//     let counts = git_ops::file_status_counts(&repo);

//     if cli.verbose {
//         println!("Opened repo at: {:?}", repo.path());
//     }

//     println!("Branch: {}", branch);
//     println!(
//         "Changes: {} modified, {} staged, {} untracked",
//         counts.modified, counts.staged, counts.untracked
//     );

//     match git_ops::last_commit_time(&repo) {
//         Some(time) => println!("Last commit: {}", git_ops::humanize_duration(time)),
//         None => println!("Last commit: unknown"),
//     }


//     let others = git_ops::list_other_branches(&repo);
//     if !others.is_empty() {
//         println!("Other branches:");
//         for b in others {
//             let staleness = match b.last_commit {
//                 Some(t) => git_ops::humanize_duration(t),
//                 None => "unknown".to_string(),
//             };
//             println!(
//                 "  {} — {}, {} ahead, {} behind main",
//                 b.name, staleness, b.ahead, b.behind
//             );
//         }
//     }
// }


use clap::Parser;
use git2::Repository;
use chrono::Utc;

mod git_ops;
mod display;

#[derive(Parser)]
#[command(name = "gitpulse", version, about)]
struct Cli {
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
        println!("Opened repo at: {:?}", repo.path());
    }

    let branch = git_ops::current_branch(&repo);
    display::print_header(&branch);

    let counts = git_ops::file_status_counts(&repo);
    display::print_changes(&counts);

    match git_ops::last_commit_time(&repo) {
        Some(time) => {
            let days = Utc::now().signed_duration_since(time).num_days();
            let text = git_ops::humanize_duration(time);
            display::print_last_commit(days, &text);
        }
        None => println!("Last commit: unknown"),
    }

    let others = git_ops::list_other_branches(&repo);
    display::print_other_branches(&others, git_ops::humanize_duration);
}