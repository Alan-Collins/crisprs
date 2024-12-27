use std::process;

use crisprs::{
    cli,
    run
};

fn main() {
    let args = cli::Opts::parse_args();
    if let Err(e) = run(args) {
        println!("crisprs failed with the following error: {e}");
        process::exit(1);
    }
}
