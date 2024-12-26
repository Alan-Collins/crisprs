#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct KmerTable {
    seq_id: String,
    kmers: HashMap<String, Vec<u32>>,
}

impl KmerTable {
    pub fn new(seq_id: &str) -> Self{
        Self {
            seq_id: seq_id.to_owned(),
            kmers: HashMap::new(),
        }
    }

    pub fn view(&self) {
        println!("{:#?}", self);
    }

    pub fn add(&mut self, seq: &str, loc: u32) {
        self.kmers.entry(seq.to_string())
            .or_default().push(loc);
    }

    pub fn get(&self, seq: &str) -> Vec<u32> {
        match self.kmers.get(seq) {
            Some(kmer_locs) => kmer_locs.clone(),
            _ => Vec::<u32>::new() // return empty vector
        }
    }
}
