# git-prune-branches

This is a utility that you can use to delete all local git branches that have been deleted from a remote.
The tool will look at the local branches that track a remote branch and delete any that are no longer present on the remote.

## usage

- to delete local branches that track `origin`: `git prune-branches`
- to perform a dry run: `git prune-branches --dry-run`
- help page: `git prune-branches -h`
- to target a different remote: `git prune-branches --remote <remote-name>`
- to target a repository in another directory: `git prune-branches --path <path-to-repo>`


## why use this

There are many different ways to delete local branches that have been deleted from a remote. 
However, remembering them and using them correctly can be non-trivial.
Here is one example of how you can do this using the command line:
`git branch -r | awk '{print $1}' | egrep -v -f /dev/fd/0 <(git branch -vv | grep origin) | awk '{print $1}' | xargs git branch -D`.
This command is not easy to understand or remember.
Additionally, cross-platform compatibility can be an issue.
