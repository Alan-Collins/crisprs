#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use std::error::Error;

use anyhow::{anyhow, Result, Context};


pub mod seq;
pub mod crispr;
pub mod cli;
pub use seq::{kmer, fasta};

mod clusters;

pub fn run(args: cli::Opts) -> Result<(), Box<dyn Error>> {
    let contigs = fasta::Fasta::from_file(args.assembly())
        .unwrap_or_else(|error| {
            panic!("Issue loading assembly: {error:?}");
        }
    );
    for (name, seq) in contigs.iter() {
        let crs = crispr::find_crisprs(seq, name, 11usize);
    };

    Ok(())
}


    // let seq_id = String::from("contig");
    // let seq = String::from("CTTCGCCGTCGCCGGGAG"); // TGGTGCGCATTATAGGGAGATAGAAACTGGCGTCAACACTTA");
    // let kmer_size: usize = 11;
    // let mut kmers = kmer::KmerTable::new(&seq_id);
    // for i in 0..(seq.len() - kmer_size){
    //     kmers.add(&seq[i..(i + kmer_size)], i as u32)
    // }
    // kmers.view();
