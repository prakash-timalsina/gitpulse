use git2::Repository;

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