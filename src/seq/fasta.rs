use std::{any, collections::HashMap, collections::HashSet};
use std::fs;

use anyhow::{anyhow, Context, Result, Error};

// Private constants

const DNA_BASES: &str = "ATCGN";

#[derive(Debug)]
pub struct Fasta {
    seqs: HashMap<String, Seq>
}

// Constructors
impl Fasta {
    pub fn from_file(file: &str) -> Result<Self, Error> {
        let fasta_string = fs::read_to_string(file)
            .context("Could not read Fasta file")?;
        match Self::from_string(fasta_string) {
            Ok(instance) => Ok(instance),
            Err(e) => Err(anyhow!("Issue processing fasta file: {e:?}")),
        }
    }

    pub fn from_string(fasta: String) -> Result<Self, Error> {
        // check input starts with valid characters
        match &fasta.trim().chars().next() {
            Some('>') => (),
            Some(_) => return Err(anyhow!("fasta sequence does not begin with header line")),
            _ => return Err(anyhow!("Sequence is empty")),
        }

        let mut seqs: HashMap<String, Seq> = HashMap::new();
        let seq_entries: Vec<&str> = fasta.split(">")
            .collect::<Vec<&str>>()[1..]
            .to_vec();

        for entry in seq_entries {
            // Check header is valid
            match entry.chars().next() {
                Some('\n') => return Err(anyhow!("Sequence contained header line with no sequence name. (i.e., just '>')")),
                Some(_) => (),
                _ => return Err(anyhow!("fasta sequence is just '>'"))
            }


            if let Some('\n') = entry.chars().next() {
                return Err(anyhow!("Sequence contained header line with no sequence name. (i.e., just '>')"))
                };

            let mut lines = entry.lines();
            let header = match lines.next() {
                Some(line) => line.to_string(),
                _ => return Err(anyhow!("Invalid format sequence found")),
            };

            let seq = lines.collect::<Vec<&str>>()
                .join("");
            let dna = match Seq::from_dna(seq) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            seqs.insert(header, dna);
        }
    Ok(Self{seqs})
    }
}

// Methods
impl Fasta {
    pub fn get_seq(&self, name: &str) -> Result<&Seq> {
        match self.seqs.get(name) {
            Some(s) => Ok(s),
            _ => Err(anyhow!("Sequence name {name:?} not found"))
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Seq>  {
        self.seqs.iter()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Seq {
    seq: String
}

// Constructors
impl Seq {
    pub fn from_dna(bases: String) -> Result<Self> {
        let bases = match bases.contains("\n") {
            true => bases.replace('\n', "").to_uppercase(),
            false => bases.to_uppercase(),
        };
        if !bases.chars().all(|b| DNA_BASES.contains(b)) {
            return Err(anyhow!("non-ATCGN base found in DNA sequence"))
        }
        Ok(Self {seq: bases})
    }
}

impl Seq {
    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn get_range(&self, start: usize, stop: usize) -> Result<Self> {
        if start >= self.seq.len() {return Err(anyhow!("start index must be less than sequence length"))}
        if stop > self.seq.len() {return Err(anyhow!("stop index must be less than or equal to sequence length"))}
        let slice = self.seq[start..stop].to_string();
        let new_instance = Self::from_dna(slice)
            .expect("A Slice of an existing Seq should not throw any errors");
        Ok(new_instance)
    }

    pub fn get_base(&self, index: usize) -> Result<Self> {
        if index > self.seq.len() {return Err(anyhow!("index must be less than sequence length"))}
        let base = self.seq[index..index+1].to_string();
        let new_instance = Self::from_dna(base)
            .expect("A Slice of an existing Seq should not throw any errors");
        Ok(new_instance)
    }

    pub fn rev_comp(&self) -> Seq {
        let rc = Self::from_dna(
            self.seq.chars().rev()
                .map(|b| match b {
                    'A' => 'T',
                    'T' => 'A',
                    'C' => 'G',
                    'G' => 'C',
                    'N' => 'N',
                    _ => panic!("A non-DNA base was found in an existing Seq instance: {b}"),
                })
                .collect::<String>()
        ).expect("Self should only contain legal bases.");
        rc
    }
}

impl std::fmt::Display for Seq {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&self.seq)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fasta_from_string_blank_header_is_err() {
        let fasta_string = ">\nATCG\n>2\nATCG".to_string();
        let result = Fasta::from_string(fasta_string);
        assert!(result.is_err());
    }

    #[test]
    fn fasta_from_string_blank_is_err() {
        let fasta_string = "".to_string();
        let result = Fasta::from_string(fasta_string);
        assert!(result.is_err());
    }

    #[test]
    fn fasta_from_string_newline_is_err() {
        let fasta_string = "\n".to_string();
        let result = Fasta::from_string(fasta_string);
        assert!(result.is_err());
    }

    #[test]
    fn fasta_from_string_missing_header_is_err() {
        let fasta_string = "ATCG\n>2\nATCG".to_string();
        let result = Fasta::from_string(fasta_string);
        assert!(result.is_err());
    }

    #[test]
    fn fasta_from_string_just_gt_is_err() {
        let fasta_string = ">".to_string();
        let result = Fasta::from_string(fasta_string);
        assert!(result.is_err());
    }

    #[test]
    fn seq_from_dna_strips_newline() {
        let result = Seq::from_dna("ATCG\nATCG".to_string()).unwrap();
        let expected = Seq::from_dna("ATCGATCG".to_string()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn seq_from_dna_non_atcgn() {
        let result = Seq::from_dna("ATCGXATCG".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn seq_from_dna_lowercase_converted() {
        let result = Seq::from_dna("atcgn".to_string()).unwrap();
        let expected = Seq::from_dna("ATCGN".to_string()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn seq_rev_comp_works() {
        let result = Seq::from_dna("ATCGN".to_string()).unwrap()
            .rev_comp();
        let expected = Seq::from_dna("NCGAT".to_string()).unwrap();
    }

    #[test]
    fn seq_slicing_works() {
        let result = Seq::from_dna("ATCGN".to_string()).unwrap()
            .get_range(2, 4).unwrap();
        let expected = Seq::from_dna("CG".to_string()).unwrap();
    }

    #[test]
    fn seq_indexing_works() {
        let result = Seq::from_dna("ATCGN".to_string()).unwrap()
            .get_base(1).unwrap();
        let expected = Seq::from_dna("T".to_string()).unwrap();
    }

}
