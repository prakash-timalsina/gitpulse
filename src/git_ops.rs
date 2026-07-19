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


pub struct BranchInfo {
    pub name: String,
    pub last_commit: Option<DateTime<Utc>>,
    pub ahead: usize,
    pub behind: usize,
}

pub fn list_other_branches(repo: &Repository) -> Vec<BranchInfo> {
    let current = current_branch(repo);
    let mut result = Vec::new();

    let main_oid = repo
        .find_branch("main", git2::BranchType::Local)
        .ok()
        .and_then(|b| b.get().target());

    let branches = match repo.branches(Some(git2::BranchType::Local)) {
        Ok(b) => b,
        Err(_) => return result,
    };

    for branch_result in branches {
        let Ok((branch, _)) = branch_result else { continue };
        let Some(name) = branch.name().ok().flatten() else { continue };

        if name == current || name == "main" {
            continue;
        }

        let Some(branch_oid) = branch.get().target() else { continue };

        let last_commit = repo
            .find_commit(branch_oid)
            .ok()
            .and_then(|c| DateTime::from_timestamp(c.time().seconds(), 0));

        let (ahead, behind) = match main_oid {
            Some(main_oid) => repo
                .graph_ahead_behind(branch_oid, main_oid)
                .unwrap_or((0, 0)),
            None => (0, 0),
        };

        result.push(BranchInfo {
            name: name.to_string(),
            last_commit,
            ahead,
            behind,
        });
    }

    result
}