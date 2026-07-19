use git2::Repository;
use chrono::{DateTime, Utc};

pub fn current_branch(repo: &Repository) -> String {
    match repo.head() {
        Ok(head) => head.shorthand().unwrap_or("HEAD (detached)").to_string(),
        Err(_) => "HEAD (unborn)".to_string(),
    }
}

pub struct FileStatusCounts {
    pub modified: usize,
    pub staged: usize,
    pub untracked: usize,
}

pub fn file_status_counts(repo: &Repository) -> FileStatusCounts {
    let mut modified = 0;
    let mut staged = 0;
    let mut untracked = 0;

    if let Ok(statuses) = repo.statuses(None) {
        for entry in statuses.iter() {
            let status = entry.status();

            if status.is_wt_new() {
                untracked += 1;
            } else if status.is_wt_modified() || status.is_wt_deleted() {
                modified += 1;
            }

            if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
                staged += 1;
            }
        }
    }

    FileStatusCounts { modified, staged, untracked }
}




pub fn last_commit_time(repo: &Repository) -> Option<DateTime<Utc>> {
    let head = repo.head().ok()?;
    let commit = head.peel_to_commit().ok()?;
    let timestamp = commit.time().seconds();
    DateTime::from_timestamp(timestamp, 0)
}

pub fn humanize_duration(then: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(then);

    let days = diff.num_days();
    let hours = diff.num_hours();
    let minutes = diff.num_minutes();

    if days > 0 {
        format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
    } else if hours > 0 {
        format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
    } else if minutes > 0 {
        format!("{} minute{} ago", minutes, if minutes == 1 { "" } else { "s" })
    } else {
        "just now".to_string()
    }
}