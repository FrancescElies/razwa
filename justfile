set shell := ["nu", "-c"]

bootstrap:
    rustup override set 1.78
    cargo b --all

repos:
    cargo run -- repo-list

pr-status pr_id:
    cargo run -- pr-status {{pr_id}} | parse "{pull_request_id}\|{title}\|{status}\|{merge_status}\|{created_by}"

pr-build-list pr_id:
    cargo run -- pr-build-list {{pr_id}} | lines | parse "{build}|{branch}|{reason}|{status}|{result}|{finished}"

work-items-list:
    cargo run -- work-items-list

prs:
    cargo run -- pr-list | lines | parse "{pull_request_id}\|{title}\|{status}\|{merge_status}\|{created_by}" | sort-by created_by

user-prs email:
    cargo run -- pr-list | lines | parse "{pull_request_id}\|{title}\|{status}\|{merge_status}\|{created_by}" | where created_by == {{email}}

distributed-tasks:
    cargo run -- distributed-tasks
