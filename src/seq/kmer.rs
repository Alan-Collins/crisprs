#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use std::collections::HashMap;

use crate::seq::fasta;

#[derive(Debug)]
pub struct KmerTable {
    seq_id: String,
    kmers: HashMap<fasta::Seq, Vec<u32>>,
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

    pub fn add(&mut self, seq: fasta::Seq, loc: u32) {
        self.kmers.entry(seq)
            .or_default().push(loc);
    }

    pub fn get(&self, seq: &fasta::Seq) -> Option<&Vec<u32>> {
        self.kmers.get(seq)
    }
}


#[cfg(test)]
mod tests {
    use fasta::Fasta;

    use super::*;

    #[test]
    fn kmer_table_empty_get_returns_nothing() {
        let k_table = KmerTable::new("test");
        let k = fasta::Seq::from_dna("ATCG".to_string()).unwrap();
        let result = k_table.get(&k);
        assert!(result.is_none());
    }

    fn kmer_table_get_returns_locs() {
        let mut k_table = KmerTable::new("test");
        let k = fasta::Seq::from_dna("ATCG".to_string()).unwrap();
        k_table.add(k.clone(), 5);
        let result = k_table.get(&k).unwrap();
        let expected = Vec::<u32>::from([5]);
        assert_eq!(result, &expected);
    }

}
