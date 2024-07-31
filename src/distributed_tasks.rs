use anyhow::{Ok, Result};
use azure_devops_rust_api::{distributed_task, Credential};

pub async fn list(credential: Credential, organization: &str, project: &str) -> Result<()> {
    // Create distributed task client
    let distributed_task_client = distributed_task::ClientBuilder::new(credential).build();

    //  Get a list of agent pools for the org
    println!("Agents pools for the org are:");
    let distributed_task_agents_pools = distributed_task_client
        .pools_client()
        .get_agent_pools(organization)
        .await?
        .value;
    println!("{:#?}", distributed_task_agents_pools);

    //  Get a list of agent queues for the project
    println!("Agents queues for the project are:");
    let distributed_task_agent_queues = distributed_task_client
        .queues_client()
        .get_agent_queues(organization, project)
        .await?
        .value;
    println!("{:#?}", distributed_task_agent_queues);

    // Get all variable groups for the project
    println!("Variable groups for the project are:");
    let distributed_task_variable_groups = distributed_task_client
        .variablegroups_client()
        .get_variable_groups(organization, project)
        .await?
        .value;
    println!("{:#?}", distributed_task_variable_groups);

    Ok(())
}
