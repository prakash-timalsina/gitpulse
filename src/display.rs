use colored::*;
use crate::git_ops::{FileStatusCounts, BranchInfo};

pub fn print_header(branch: &str) {
    println!("{} {}", "Branch:".bold(), branch.cyan());
}

pub fn print_changes(counts: &FileStatusCounts) {
    let total = counts.modified + counts.staged + counts.untracked;

    let line = format!(
        "{} modified, {} staged, {} untracked",
        counts.modified, counts.staged, counts.untracked
    );

    if total == 0 {
        println!("{} {}", "Changes:".bold(), line.green());
    } else if total <= 3 {
        println!("{} {}", "Changes:".bold(), line.yellow());
    } else {
        println!("{} {}", "Changes:".bold(), line.red());
    }
}

pub fn print_last_commit(staleness_days: i64, staleness_text: &str) {
    let label = "Last commit:".bold();

    if staleness_days <= 2 {
        println!("{} {}", label, staleness_text.green());
    } else if staleness_days <= 7 {
        println!("{} {}", label, staleness_text.yellow());
    } else {
        println!("{} {}", label, staleness_text.red());
    }
}

pub fn print_other_branches(branches: &[BranchInfo], humanize: impl Fn(chrono::DateTime<chrono::Utc>) -> String) {
    if branches.is_empty() {
        return;
    }

    println!("{}", "Other branches:".bold());

    for b in branches {
        let (staleness_text, days) = match b.last_commit {
            Some(t) => {
                let days = chrono::Utc::now().signed_duration_since(t).num_days();
                (humanize(t), days)
            }
            None => ("unknown".to_string(), 0),
        };

        let line = format!(
            "  {} — {}, {} ahead, {} behind main",
            b.name, staleness_text, b.ahead, b.behind
        );

        if days > 7 || b.behind > 5 {
            println!("{}", line.red());
        } else if days > 2 || b.behind > 0 {
            println!("{}", line.yellow());
        } else {
            println!("{}", line.green());
        }
    }
}