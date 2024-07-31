use anyhow::{Ok, Result};
use azure_devops_rust_api::{
    build::{self, models::Build},
    Credential,
};

pub async fn list(
    credential: Credential,
    organization: &str,
    project: &str,
    branch: &str,
    top: i32,
    status: &str, // e.g. inprogress
) -> Result<Vec<Build>> {
    // Create a build client
    let build_client = build::ClientBuilder::new(credential).build();

    // Get all builds in the specified organization/project in the past hour
    let builds = build_client
        .builds_client()
        .list(organization, project)
        .branch_name(branch)
        .status_filter(status)
        .top(top) // this is the max value the api allows
        .await?
        .value;

    Ok(builds)
}
