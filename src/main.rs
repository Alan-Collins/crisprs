#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use crisprs::{
    cli,
    run
};

fn main() {
    let args = cli::Opts::parse_args();
    let res = run(args);
    
}
