use std::collections::HashMap;
use std::fs;

use anyhow::{anyhow, Context, Result, Error};

#[derive(Debug)]
pub struct Fasta {
    seqs: HashMap<String, String>

}

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
        let mut seqs: HashMap<String, String> = HashMap::new();
        let seq_entries: Vec<&str> = fasta.split(">")
            .collect::<Vec<&str>>()[1..]
            .to_vec();

        // Check file has contents
        if seq_entries.is_empty() {
            return Err(anyhow!("Sequence is empty"))
        };

        for entry in seq_entries {
            // Check header is valid
            match entry.chars().next() {
                Some('\n') => return Err(anyhow!("Sequence contained header line with no sequence name. (i.e., just '>')")),
                Some('>') => (),
                Some(_) => return Err(anyhow!("fasta sequence does not begin with header line")),
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
            seqs.insert(header, seq);
        }
    Ok(Self{seqs})
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
}
