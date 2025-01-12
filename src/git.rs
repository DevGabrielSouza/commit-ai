use git2::{DiffFormat, DiffOptions, Repository, StatusOptions};
use std::fs;
use std::path::Path;

pub fn get_git_changes() -> Result<String, git2::Error> {
    let repo = Repository::discover(".")?;
    let mut changes = String::new();

    // Configuração para capturar todos os status
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

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

        if status.is_index_new() || status.is_wt_new() {
            // Arquivo novo
            match fs::read_to_string(Path::new(path)) {
                Ok(content) => changes.push_str(&format!("\nNew file: {}\n{}", path, content)),
                Err(_) => changes.push_str(&format!("\nNew file (binary or unreadable): {}", path)),
            }
        } else if status.is_index_modified() || status.is_wt_modified() {
            // Arquivo modificado
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
        } else if status.is_index_deleted() || status.is_wt_deleted() {
            // Arquivo deletado
            changes.push_str(&format!("\nDeleted file: {}", path));
        } else if status.is_index_renamed() || status.is_wt_renamed() {
            // Arquivo renomeado
            changes.push_str(&format!("\nRenamed file: {}", path));
        }
    }

    Ok(changes)
}
