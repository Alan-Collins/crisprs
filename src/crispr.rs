use crate::fasta::{Seq, Fasta};
use crate::kmer::KmerTable;

struct CRISPRArray {
    repeats: Vec<Seq>,
    spacers: Vec<Seq>,
    source_name: String,
    location: [usize; 2],
}

impl CRISPRArray {
    pub fn to_fasta(&self, header: &str) -> String {
        let mut seq_list = Vec::<String>::with_capacity(self.repeats.len()*2);

        seq_list.push(format!(">{header}\n"));
        for (rep, sp) in self.repeats.iter()
            .zip(self.spacers.iter()) {
                seq_list.push(rep.to_string());
                seq_list.push(sp.to_string());
        }
        seq_list.push(self.repeats
            .last()
            .expect("There should always be one more repeat than spacer in an array")
            .to_string());
        seq_list.join("")
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn crispr_to_fasta_works() {
        let rep = Seq::from_dna("ATCG".to_string()).unwrap();
        let spacer = Seq::from_dna("AAAA".to_string()).unwrap();
        let cr = CRISPRArray {
            repeats: vec![rep.clone(), rep.clone()],
            spacers: vec![spacer.clone()],
            source_name: "test".to_string(),
            location: [5usize, 10usize]
        };
        let expected = ">test\nATCGAAAAATCG".to_string();
        let result = cr.to_fasta("test");
        assert_eq!(result, expected);
    }
}
