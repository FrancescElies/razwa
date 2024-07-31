# razwa (Rust AZure devops WAtch)

Some personal helper funcitons to do common azure devops tasks.

Start by running `just bootstrap`

## Notes

Most of this code comes from [azure_devops_rust_api/examples](https://github.com/microsoft/azure-devops-rust-api/tree/main/azure_devops_rust_api/examples)

This cli needs the following environment variables:
- `ADO_ORGANIZATION`: go to [dev.azure.com](https://dev.azure.com) and copy the name of your organization from the left side bar.
- `ADO_PROJECT`: inside that organization copy the project name
- `ADO_TEAM`: see available teams by running `az devops team list | from json | select name id description`

