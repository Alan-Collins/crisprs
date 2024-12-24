#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]


mod cli;

use crisprs::kmer;

fn main() {
    let args = cli::Opts::parse_args();
    let seq_id = String::from("contig");
    let seq = String::from("CTTCGCCGTCGCCGGGAG"); // TGGTGCGCATTATAGGGAGATAGAAACTGGCGTCAACACTTA");
    let kmer_size: usize = 11;
    let mut kmers = kmer::KmerTable::new(&seq_id);
    for i in 0..(seq.len() - kmer_size){
        kmers.add(&seq[i..(i + kmer_size)], i as u32)
    }
    kmers.view();

}
