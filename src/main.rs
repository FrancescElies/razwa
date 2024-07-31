use azure_devops_rust_api::{git::models::git_pull_request::MergeStatus, Credential};
use clap::Parser;

use anyhow::{Ok, Result};
use std::env;
use utils::get_auth_credential;

mod build;
mod distributed_tasks;
mod pr;
mod repo;
mod utils;
mod workitems;

use clap::Subcommand;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "watchingaz")]
#[command(about = "Interact with azure devops", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// show list of builds
    // #[command(arg_required_else_help = true)]
    PrBuildList {
        pr_id: i32,
    },
    RepoList,
    PrList {
        email: Option<String>,
    },
    PrStatus {
        pr_id: i32,
    },
    WorkItemsList,
    DistributedTasks,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    let credential = get_auth_credential();

    // Get ADO server configuration via environment variables
    let organization = env::var("ADO_ORGANIZATION").expect("Must define ADO_ORGANIZATION");
    let project = env::var("ADO_PROJECT").expect("Must define ADO_PROJECT");

    let args = Cli::parse();
    match args.command {
        Commands::PrBuildList { pr_id } => {
            pr_build_list(&credential, &organization, &project, pr_id).await?;
        }
        Commands::RepoList => {
            repo::list(credential.clone(), &organization, &project).await?;
        }
        Commands::PrList { email } => {
            let prs = pr::list(credential.clone(), &organization, &project, email).await?;
            prs.iter().for_each(|pr| {
                let pr = pr.clone();
                let merge_status = pr.merge_status.unwrap_or(MergeStatus::NotSet);
                let title = pr.title.unwrap_or("title-missing".to_owned());
                let status = pr.status;
                let created_by = pr.created_by.unique_name.unwrap_or("unkown".to_owned());
                let pull_request_id = pr.pull_request_id;
                // git_client.pull_request_statuses_client();
                println!("{pull_request_id}|{title}|{status:?}|{merge_status:?}|{created_by}");
            });
        }
        Commands::PrStatus { pr_id } => {
            pr::status(credential.clone(), &organization, &project, pr_id).await?;
        }
        Commands::WorkItemsList => {
            let team = env::var("ADO_TEAM").unwrap_or("Must define ADO_TEAM".to_owned());
            workitems::list(credential.clone(), &organization, &project, &team).await?;
        }
        Commands::DistributedTasks => {
            distributed_tasks::list(credential.clone(), &organization, &project).await?;
        }
    };

    Ok(())
}

async fn pr_build_list(
    credential: &Credential,
    organization: &str,
    project: &str,
    pr_id: i32,
) -> Result<()> {
    let inprogress = build::list(
        credential.clone(),
        organization,
        project,
        &format!("refs/pull/{pr_id}/merge"),
        5,
        "inprogress",
    )
    .await?;
    let completed = build::list(
        credential.clone(),
        organization,
        project,
        &format!("refs/pull/{pr_id}/merge"),
        5,
        "completed",
    )
    .await?;
    inprogress.into_iter().chain(completed).for_each(|b| {
        println!(
            "{} | {} | {:?} | {:?} | {:?} | {}",
            b.id,
            b.source_branch.clone().unwrap_or("".to_owned()),
            b.reason,
            b.status
                .clone()
                .unwrap_or(azure_devops_rust_api::build::models::build::Status::None),
            b.result
                .clone()
                .unwrap_or(azure_devops_rust_api::build::models::build::Result::None),
            b.finish_time
                .map(|x| format!("{x}"))
                .unwrap_or("not-finished".to_owned())
        )
    });
    Ok(())
}
