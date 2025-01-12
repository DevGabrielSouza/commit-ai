use git2::{DiffFormat, Repository, Status, StatusOptions};
use std::fs;
use std::path::Path;

pub fn get_git_changes() -> Result<String, git2::Error> {
    let repo = Repository::discover(".")?;
    let mut changes = String::new();
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true).renames(true);

    let statuses = repo.statuses(Some(&mut status_options))?;

    for entry in statuses.iter() {
        let status = entry.status();
        let path = match entry.path() {
            Some(p) => p,
            None => {
                changes.push_str("\nUnknown path");
                continue;
            }
        };

        let handler: Box<dyn ChangeHandler> = if status.is_index_new() {
            Box::new(NewFileHandler)
        } else if status.is_index_modified() {
            Box::new(ModifiedFileHandler)
        } else if status.is_index_deleted() {
            Box::new(DeletedFileHandler)
        } else if status.is_index_renamed() {
            Box::new(RenamedFileHandler)
        } else {
            continue; 
        };
        changes.push_str(&handler.handle(path, &repo));
    }

    Ok(changes)
}
