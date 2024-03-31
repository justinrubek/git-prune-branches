use crate::commands::Args;
use clap::Parser;
use gix::Reference;
use std::{borrow::Cow, error::Error};
use tracing::{debug, info};

mod commands;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let repository = gix::open(args.path)?;
    let references = repository.references()?;

    let local_branches = local_branches_from_remote(&repository, &references, &args.remote)?;
    debug!(?local_branches, "remote: {}", args.remote);

    let remote_branches = references
        .remote_branches()?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    debug!(?remote_branches);

    let to_delete = local_branches.iter().filter(|branch| {
        // compare the short name of the branch to the list of remote branches
        let branch = branch.name().shorten();
        let remote_branches = remote_branches.iter().map(|branch| branch.name().shorten());
        !remote_branches
            .clone()
            .any(|remote_branch| remote_branch.ends_with(branch))
    });

    if to_delete.clone().count() == 0 {
        info!("no branches to delete. run `git fetch -p` to prune remote branches if you are expecting some.");
        return Ok(());
    }
    if !args.dry_run {
        to_delete.for_each(|reference| {
            reference.delete().expect("failed to delete reference");
        });
    } else {
        let to_delete = to_delete
            .map(|branch| branch.name().shorten())
            .collect::<Vec<_>>();
        info!(?to_delete, "dry run");
    }

    Ok(())
}

/// Retrieves the local branches that track the given remote.
fn local_branches_from_remote<'a>(
    repository: &'a gix::Repository,
    references: &'a gix::reference::iter::Platform<'a>,
    remote: &str,
) -> Result<Vec<Reference<'a>>, Box<dyn Error>> {
    let remote_name = gix::remote::Name::Symbol(Cow::from(remote));
    let local_branch_names = references
        .local_branches()?
        .filter_map(Result::ok)
        .filter_map(|branch| {
            let name = branch.name().shorten();
            let branch_remote = repository.branch_remote_name(name, gix::remote::Direction::Fetch);
            if let Some(branch_remote) = branch_remote {
                if branch_remote == remote_name {
                    Some(branch.to_owned())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(local_branch_names)
}
