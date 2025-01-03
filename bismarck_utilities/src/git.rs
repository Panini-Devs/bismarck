use git2::{ErrorCode, Repository};

/// Retrieves the current git branch in a given git repository.
pub fn get_current_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return format!("An error occured: {e:?}"),
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    head.unwrap().to_string()
}

/// Retrieves the latest HEAD revision for the given git repository.
pub fn get_head_revision(repo: &Repository) -> String {
    let revspec = repo.revparse("HEAD").unwrap();
    let revision = revspec.from().unwrap();
    revision.short_id().unwrap().as_str().unwrap().to_string()
}

/// Retrieves the absolute path for a variable
pub fn get_absolute_path() -> String {
    let dir = std::path::PathBuf::from("./");

    std::fs::canonicalize(dir).unwrap().to_str().unwrap().to_string()
}