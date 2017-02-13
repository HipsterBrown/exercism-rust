use std::collections::HashMap;

pub fn nucleotide_counts(strand: &str) -> Result<HashMap<char, usize>, ()> {
    if !strand.chars().all(|c| valid_nuc(&c)) {
        return Err(());
    }
    let hm = ['A', 'C', 'G', 'T'].iter().fold(HashMap::new(), |mut acc, nuc| {
        acc.insert(*nuc, count(*nuc, strand).unwrap() as usize);
        acc
    });
    Ok(hm)
}

pub fn count(nuc: char, strand: &str) -> Result<u32, &str> {
    if !valid_nuc(&nuc) || !strand.chars().all(|c| valid_nuc(&c)) {
        return Err("Invalid nucleotide.");
    }
    let count: u32 = strand.chars().fold(0, |mut acc, c| {
        if c == nuc { acc += 1; }
        acc
    });
    Ok(count)
}

fn valid_nuc(nuc: &char) -> bool {
    match nuc {
        &'A' | &'C' | &'G' | &'T' => true,
        _ => false,
    }
}
