use std::error::Error;
use std::fs;

mod seq;
pub mod cli;

pub use seq::{kmer, fasta};

pub fn run(args: cli::Opts) -> Result<(), Box<dyn Error>> {
    let assembly = fs::read_to_string(&args.assembly())
        .expect("Could not read assembly file");
    let contigs: Vec<&str> = assembly.split(">")
        .collect::<Vec<&str>>()[1..]
        .to_vec();
    for entry in contigs {
        let mut lines = entry.split_whitespace();
        let header = lines.next()
            .expect("Assembly contained header line with no sequence name. (i.e., just '>')");

        // let header = lines.pop();
        let seq = lines.collect::<Vec<&str>>()
            .join("");
        println!("{:?}", header);
        println!("{:?}", seq);
    }
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