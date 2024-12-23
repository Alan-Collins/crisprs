
mod cli;
mod kmer;

fn main() {
    let args = cli::Opts::parse_args();
    let seq_id = String::from("contig");
    let seq = String::from("CTTCGCCGTCGCCGGGAGTGGTGCGCATTATAGGGAGATAGAAACTGGCGTCAACACTTA");
    let kmer_size: u32 = 3;
    let mut kmers = kmer::KmerTable::new(&seq_id);
    for i in 0..(seq.len() - kmer_size as usize){
        kmers.add(&seq[i..(i + kmer_size as usize)], i as u32)
    }
    kmers.view();

}
