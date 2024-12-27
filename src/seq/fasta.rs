use std::collections::HashMap;
use std::fs;

use anyhow::{anyhow, Result, Context};




pub fn read_fasta(fasta_file: &str) -> Result<HashMap<String, String>> {
    let mut contigs: HashMap<String, String> = HashMap::new();

    let fasta = fs::read_to_string(fasta_file)
        .context("Could not read Fasta file")?;
    let seq_entries: Vec<&str> = fasta.split(">")
        .collect::<Vec<&str>>()[1..]
        .to_vec();

    // Check file has contents
    if seq_entries.is_empty() {
        return Err(anyhow!("Fasta file {fasta_file} is empty"))
    };

    for entry in seq_entries {
        // Check header is valid
        if let Some('\n') = entry.chars().next() {
            return Err(anyhow!("Fasta contained header line with no sequence name. (i.e., just '>')"))
            }

        let mut lines = entry.lines();
        let header = match lines.next() {
            Some(line) => line.to_string(),
            _ => return Err(anyhow!("No lines found in Fasta file")),
        };

        let seq = lines.collect::<Vec<&str>>()
            .join("");
        contigs.insert(header, seq);
    }
    Ok(contigs)
}
