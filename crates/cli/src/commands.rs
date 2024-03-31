#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long, default_value = ".")]
    /// the path to the repository
    pub path: std::path::PathBuf,
    #[arg(short, long, default_value = "origin")]
    /// the name of the remote to check against
    pub remote: String,
    #[arg(short, long, default_value = "false")]
    /// when set, the program will not make any changes to the repository
    pub dry_run: bool,
}
