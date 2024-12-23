

mod cli;

fn main() {
    let args = cli::Opts::parse_args();
    println!("{:#?}", args);
}
