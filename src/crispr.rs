use std::f32::MIN;

use crate::fasta::{Seq, Fasta};
use crate::kmer::{KmerTable, KmerLocs};


// Settings for determining valid arrays
const MIN_REPS: u32 = 3; // minimum number of repeats to call an array
const MIN_REP_SIZE: u32 = 20;
const MAX_REP_SIZE: u32 = 70;
const MIN_SPACER_SIZE: u32 = 20;
const MAX_SPACER_SIZE: u32 = 70;
const MAX_REP_LEN_DEV: f32 = 0.1; // maximum proportion difference in repeat lengths in array
const MAX_SPACER_LEN_DEV: f32 = 0.1; // maximum proportion difference in spacer lengths in array


pub struct CRISPRArray {
    repeats: Vec<Seq>,
    spacers: Vec<Seq>,
    source_name: String,
    location: [usize; 2],
}

// methods
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

    pub fn to_table(&self) -> String {
        let mut seq_rows = Vec::<String>::with_capacity(self.repeats.len());
        for (rep, sp) in self.repeats.iter()
            .zip(self.spacers.iter()) {
                seq_rows.push(format!("{}\t{}\n", rep.to_string(), sp.to_string()));
        }
        seq_rows.push(
            format!("{}\n", self.repeats
                .last()
                .expect("There should always be one more repeat than spacer in an array")
                .to_string()
                )
            );
        seq_rows.join("")
    }
}


pub fn find_crisprs(source_seq: &Seq, source_name: &str, k: usize) -> Option<u32> { // Option<CRISPRArray> {
    let kt = KmerTable::from_seq(source_seq, k);
    let candidate_kmers = match get_candidate_kmers(kt) {
        Some(thing) => thing,
        _ => return None,
    };

    // replace with CRISPRArray return
    Some(5)
    // for (k, locs) in kmers.iter() {
    //     if locs.len() >= 9 {
    //         println!("{}: {:?}", k, locs);
    //     }
    // }
}

fn get_candidate_kmers(kt: KmerTable) -> Option<Vec<KmerLocs>> {
    let mut clusters: Vec<KmerLocs> = Vec::new();
    for (k, locs) in kt.iter() {
        let clus = match find_kmer_loc_clusters(locs) {
            Some(thing) => thing,
            _ => continue,
        };
        for c in clus {
            clusters.push(KmerLocs::new(k.clone(),  c));
        }
    }


    // To del
    let kl = KmerLocs::new( Seq::from_dna("ATCG".to_string()).unwrap(), vec![0usize]);
    match kl.k().len() {
        0 => None,
        _ => Some(vec![kl]),
    }
}

fn find_kmer_loc_clusters(locs: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
    // initial checks to determine whether to proceed
    if locs.len() < MIN_REPS as usize {
        return None
    }
    // Set max interval between kmers that could be array
    let min_max_size = (MIN_REPS * (MAX_REP_SIZE + MAX_SPACER_SIZE)) as usize;
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    let mut this_clus = Vec::<usize>::new();
    let mut in_cluster: bool = false; // keep track of whether to start or add to cluster
    for i in MIN_REPS as usize .. locs.len() {
        if locs[i] - locs[i - (MIN_REPS - 1) as usize] < min_max_size {
            if in_cluster {
                this_clus.push(locs[i])
            } else {
                let array_start = i - (MIN_REPS - 1) as usize;
                this_clus.extend(&locs[array_start ..= i]);
                in_cluster = true;
            }
        } else {
            if this_clus.is_empty() { continue }

            in_cluster = false;
            clusters.push(this_clus.clone());
            this_clus.clear();
        }
    }
    if !this_clus.is_empty() { // Add final cluster
        clusters.push(this_clus.clone());
    }

    if clusters.is_empty() { // No clusters found
        return None
    }
    Some(clusters)
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
