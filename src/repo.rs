use anyhow::{Ok, Result};
use azure_devops_rust_api::{git, Credential};

pub async fn list(credential: Credential, organization: &str, project: &str) -> Result<()> {
    // // Create a git client
    let git_client = git::ClientBuilder::new(credential).build();

    // Get all repositories in the specified organization/project
    let repos = git_client
        .repositories_client()
        .list(organization, project)
        .await?
        .value;

    let repo_names = repos
        .iter()
        .map(|x| x.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    println!("repos {}: {}", repos.len(), repo_names);
    Ok(())
}
