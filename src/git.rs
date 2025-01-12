use git2::{DiffFormat, DiffOptions, Repository, StatusOptions};
use std::fs;
use std::path::{Path};

trait ChangeHandler {
    fn can_handle(&self, status: git2::Status) -> bool;
    fn handle(&self, path: &str, repo: &Repository, changes: &mut String);
}

struct NewFileHandler;

impl ChangeHandler for NewFileHandler {
    fn can_handle(&self, status: git2::Status) -> bool {
        status.is_index_new() || status.is_wt_new()
    }

    fn handle(&self, path: &str, _repo: &Repository, changes: &mut String) {
        match fs::read_to_string(Path::new(path)) {
            Ok(content) => changes.push_str(&format!("\nNew file: {}\n{}", path, content)),
            Err(_) => changes.push_str(&format!("\nNew file (binary or unreadable): {}", path)),
        }
    }
}

struct ModifiedFileHandler;

impl ChangeHandler for ModifiedFileHandler {
    fn can_handle(&self, status: git2::Status) -> bool {
        status.is_index_modified() || status.is_wt_modified()
    }

    fn handle(&self, path: &str, repo: &Repository, changes: &mut String) {
        let mut diff_options = DiffOptions::new();
        diff_options.include_untracked(true);

        if let Ok(diff) = repo.diff_index_to_workdir(None, Some(&mut diff_options)) {
            diff.print(DiffFormat::Patch, |delta, _, line| {
                if let Some(old_path) = delta.old_file().path() {
                    if old_path == Path::new(path) {
                        changes.push_str(std::str::from_utf8(line.content()).unwrap_or_default());
                    }
                }
                true
            })
            .unwrap_or(());
        }
    }
}

struct DeletedFileHandler;

impl ChangeHandler for DeletedFileHandler {
    fn can_handle(&self, status: git2::Status) -> bool {
        status.is_index_deleted() || status.is_wt_deleted()
    }

    fn handle(&self, path: &str, _repo: &Repository, changes: &mut String) {
        changes.push_str(&format!("\nDeleted file: {}", path));
    }
}

struct RenamedFileHandler;

impl ChangeHandler for RenamedFileHandler {
    fn can_handle(&self, status: git2::Status) -> bool {
        status.is_index_renamed() || status.is_wt_renamed()
    }

    fn handle(&self, path: &str, _repo: &Repository, changes: &mut String) {
        changes.push_str(&format!("\nRenamed file: {}", path));
    }
}

pub fn get_git_changes() -> Result<String, git2::Error> {
    let repo = Repository::discover(".")?;
    let mut changes = String::new();

    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    let statuses = repo.statuses(Some(&mut status_options))?;

    let handlers: Vec<Box<dyn ChangeHandler>> = vec![
        Box::new(NewFileHandler),
        Box::new(ModifiedFileHandler),
        Box::new(DeletedFileHandler),
        Box::new(RenamedFileHandler),
    ];

    for entry in statuses.iter() {
        let status = entry.status();
        let path = match entry.path() {
            Some(p) => p,
            None => {
                changes.push_str("\nUnknown path");
                continue;
            }
        };

        for handler in &handlers {
            if handler.can_handle(status) {
                handler.handle(path, &repo, &mut changes);
                break;
            }
        }
    }

    Ok(changes)
}
