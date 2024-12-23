use clap::Parser;

/// crisprs (CRISPR in-silico prediction with Rust)
/// Predict CRISPR arrays in assemblies
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Alan Collins <https://github.com/Alan-Collins>")]
pub struct Opts {
    /// Assembly file
    #[clap(short, long)]
    assembly: String,
    /// outprefix
    #[clap(short, long)]
    outprefix: String,
}

impl Opts {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
