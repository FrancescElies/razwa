// https://github.com/microsoft/azure-devops-rust-api/blob/main/azure_devops_rust_api/examples/work.rs
use anyhow::{Context, Ok, Result};
use azure_devops_rust_api::{work, Credential};

pub async fn list(
    credential: Credential,
    organization: &str,
    project: &str,
    team: &str,
) -> Result<()> {
    // Create a work client
    let work_client = work::ClientBuilder::new(credential).build();

    // Get team settings
    let team_settings = work_client
        .teamsettings_client()
        .get(organization, project, team)
        .await?;
    println!("Team settings:\n{:#?}", team_settings);

    // Get all iterations
    let iterations = work_client
        .iterations_client()
        .list(organization, project, team)
        .await?;
    println!("\nIterations:\n{:#?}", iterations);

    // Get current iteration
    let current_iteration = work_client
        .iterations_client()
        .list(organization, project, team)
        .timeframe("current")
        .await?;
    println!("\nCurrent iteration:\n{:#?}", current_iteration);

    // Get current iteration id
    let current_iteration_id = current_iteration
        .value
        .first()
        .context("No current iteration")?
        .id
        .as_ref()
        .context("Current iteration has no id")?;
    println!("\ncurrent_iteration_id: {current_iteration_id}");

    // Get current iteration work items
    let current_iteration_workitems = work_client
        .iterations_client()
        .get_iteration_work_items(organization, project, current_iteration_id, team)
        .await?;
    println!(
        "\nCurrent iteration workitems:\n{:#?}",
        current_iteration_workitems
    );

    println!(
        "\nCurrent iteration work_item_relations length: {}",
        current_iteration_workitems.work_item_relations.len()
    );

    Ok(())
}
