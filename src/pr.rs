use anyhow::{Ok, Result};
use azure_devops_rust_api::{
    git::{self, models::GitPullRequest},
    Credential,
};

pub async fn list(
    credential: Credential,
    organization: &str,
    project: &str,
    email: Option<String>,
) -> Result<Vec<GitPullRequest>> {
    let git_client = git::ClientBuilder::new(credential).build();
    let pr_client = git_client.pull_requests_client();
    let prs = pr_client
        .get_pull_requests_by_project(organization, project)
        .await?;
    let prs = prs.value.into_iter();

    Ok(if email.is_some() {
        prs.filter(|x| x.created_by.unique_name == email).collect()
    } else {
        prs.collect()
    })
}

pub async fn status(
    credential: Credential,
    organization: &str,
    project: &str,
    pull_request_id: i32,
) -> Result<()> {
    let git_client = git::ClientBuilder::new(credential).build();
    let pr_client = git_client.pull_requests_client();

    let pr = pr_client
        .get_pull_request_by_id(organization, pull_request_id, project)
        .await?;
    let merge_status = pr.merge_status.clone();
    let title = pr.title.clone().unwrap_or("title-missing".to_owned());
    let status = pr.status.clone();
    let created_by = pr.created_by.unique_name.clone().unwrap_or("".to_owned());
    println!("{pull_request_id}|{title}|{status:?}|{merge_status:?}|{created_by}",);

    Ok(())
}
