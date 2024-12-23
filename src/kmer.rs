use std::collections::HashMap;

#[derive(Debug)]
pub struct KmerTable {
    seq_id: String,
    kmers: HashMap<String, Vec<u32>>,
}

impl KmerTable {
    pub fn new(seq_id: &String) -> Self{
        Self {
            seq_id: seq_id.clone(),
            kmers: HashMap::new(),
        }
    }

    pub fn view(&self) {
        println!("{:#?}", self);
    }

    pub fn add(&mut self, seq: &str, loc: u32) {
        self.kmers.entry(seq.to_string())
            .or_insert(Vec::<u32>::new()).push(loc);
    }

    pub fn get(&self, seq: &str) -> Vec<u32> {
        match self.kmers.get(seq) {
            Some(&ref kmer_locs) => kmer_locs.clone(),
            _ => Vec::<u32>::new() // return empty vector
        }
    }
}
