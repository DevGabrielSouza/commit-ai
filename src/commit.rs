use git2::{IndexAddOption, Repository};
use std::error::Error;

pub fn create_commit(message: &str) -> Result<(), Box<dyn Error>> {
    let repo = Repository::discover(".")?;
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let oid = index.write_tree()?;
    let signature = repo.signature()?;
    let parent_commit = repo.head()?.peel_to_commit()?;
    let tree = repo.find_tree(oid)?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;

    println!("Commit created successfully!");
    Ok(())
}
