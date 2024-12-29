#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

use std::collections::{HashMap, VecDeque};

use crate::seq::fasta::{Fasta, Seq};

#[derive(Debug, PartialEq)]
pub struct KmerTable {
    kmers: HashMap<Seq, Vec<usize>>,
}

// methods
impl KmerTable {
    pub fn add(&mut self, seq: Seq, loc: usize) {
        self.kmers.entry(seq)
            .or_default().push(loc);
    }

    pub fn get(&self, seq: &Seq) -> Option<&Vec<usize>> {
        self.kmers.get(seq)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Seq, Vec<usize>>  {
        self.kmers.iter()
    }
}

// constructors
impl KmerTable {
    pub fn new() -> Self{
        Self {
            kmers: HashMap::new(),
        }
    }

    pub fn from_seq(s: &Seq, k: usize) -> Self {
        let mut kt = Self::new();
        let mut vd = VecDeque::with_capacity(k);
        // Initial kmer
        for i in 0..k {
            vd.push_back(s.get_base(i));
        };
        let seq = Seq::from_dna(
                vd.iter().map(|x| x.as_ref().expect("Seq should only yield valid seq").to_string())
                .collect()
            ).expect("Combining bases from Seq should yield valid Seq");
        let mut k_start_pos = 0usize;
        kt.add(seq, k_start_pos);
        for new_base_pos in k..s.len() {
            k_start_pos += 1;
            vd.pop_front();
            vd.push_back(s.get_base(new_base_pos));
            let seq = Seq::from_dna(
                    vd.iter().map(|x| x.as_ref().expect("Seq should only yield valid seq").to_string())
                    .collect()
                ).expect("Combining bases from Seq should yield valid Seq");
            kt.add(seq, k_start_pos);
        }
        kt
    }
}


impl Default for KmerTable {
    fn default() -> Self {
        Self::new()
    }
}


pub struct KmerLocs {
    k: Seq,
    locs: Vec<usize>,
}


// methods
impl KmerLocs {
    pub fn k(&self) -> Seq {
        self.k.clone()
    }

    pub fn locs(&self) -> Vec<usize> {
        self.locs.clone()
    }
}

// constructors
impl KmerLocs {
    pub fn new(k: Seq, locs: Vec<usize>) -> Self {
        Self {
            k,
            locs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kmer_table_empty_get_returns_nothing() {
        let k_table = KmerTable::new();
        let k = Seq::from_dna("ATCG".to_string()).unwrap();
        let result = k_table.get(&k);
        assert!(result.is_none());
    }

    #[test]
    fn kmer_table_get_returns_locs() {
        let mut k_table = KmerTable::new();
        let k = Seq::from_dna("ATCG".to_string()).unwrap();
        k_table.add(k.clone(), 5usize);
        let result = k_table.get(&k).unwrap();
        let expected = Vec::<usize>::from([5]);
        assert_eq!(result, &expected);
    }

    #[test]
    fn kmer_from_seq_works() {
        let s = Seq::from_dna("ATCGATCG".to_string()).unwrap();
        let result = KmerTable::from_seq(&s, 4usize);
        let expected = KmerTable{
            kmers: HashMap::from([
                (Seq::from_dna("ATCG".to_string()).unwrap(), vec![0usize, 4usize]),
                (Seq::from_dna("TCGA".to_string()).unwrap(), vec![1usize]),
                (Seq::from_dna("CGAT".to_string()).unwrap(), vec![2usize]),
                (Seq::from_dna("GATC".to_string()).unwrap(), vec![3usize]),
            ])
        };
        assert_eq!(result, expected)

    }
}
