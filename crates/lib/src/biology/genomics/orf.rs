pub fn find_orfs(sequence: &str, min_length: usize) -> Vec<(usize, usize, String)> {
    let seq: Vec<u8> = sequence.as_bytes().to_vec();
    let n = seq.len();
    let mut orfs = Vec::new();
    for frame in 0..3 {
        let mut i = frame;
        while i + 2 < n {
            if &seq[i..i + 3] == b"ATG" {
                let start = i;
                let mut j = i + 3;
                let mut found_stop = false;
                while j + 2 < n {
                    let codon = &seq[j..j + 3];
                    if codon == b"TAA" || codon == b"TAG" || codon == b"TGA" {
                        let length = j + 3 - start;
                        if length >= min_length {
                            let orf_seq = String::from_utf8_lossy(&seq[start..j + 3]).to_string();
                            orfs.push((start, j + 3, orf_seq));
                        }
                        i = j + 3;
                        found_stop = true;
                        break;
                    }
                    j += 3;
                }
                if !found_stop {
                    i += 3;
                }
            } else {
                i += 3;
            }
        }
    }
    orfs
}

pub fn codon_usage(sequence: &str) -> Vec<(String, usize)> {
    let seq = sequence.as_bytes();
    let mut counts: Vec<(String, usize)> = Vec::new();
    let mut i = 0;
    while i + 2 < seq.len() {
        let codon = String::from_utf8_lossy(&seq[i..i + 3]).to_string();
        let mut found = false;
        for entry in counts.iter_mut() {
            if entry.0 == codon {
                entry.1 += 1;
                found = true;
                break;
            }
        }
        if !found {
            counts.push((codon, 1));
        }
        i += 3;
    }
    counts
}

pub fn reading_frame_proteins(sequence: &str, frame: usize) -> Vec<String> {
    let seq = sequence.as_bytes();
    let mut proteins = Vec::new();
    let mut i = frame;
    let mut current_protein = String::new();
    let mut in_protein = false;
    while i + 2 < seq.len() {
        let codon = &seq[i..i + 3];
        let aa = match codon {
            b"ATG" => {
                in_protein = true;
                Some('M')
            }
            b"TAA" | b"TAG" | b"TGA" => {
                if in_protein && !current_protein.is_empty() {
                    proteins.push(current_protein.clone());
                    current_protein.clear();
                }
                in_protein = false;
                None
            }
            _ if in_protein => Some(codon_to_aa(codon)),
            _ => None,
        };
        if let Some(a) = aa {
            current_protein.push(a);
        }
        i += 3;
    }
    proteins
}

fn codon_to_aa(codon: &[u8]) -> char {
    match codon {
        b"GCT" | b"GCC" | b"GCA" | b"GCG" => 'A',
        b"TGT" | b"TGC" => 'C',
        b"GAT" | b"GAC" => 'D',
        b"GAA" | b"GAG" => 'E',
        b"TTT" | b"TTC" => 'F',
        b"GGT" | b"GGC" | b"GGA" | b"GGG" => 'G',
        b"CAT" | b"CAC" => 'H',
        b"ATT" | b"ATC" | b"ATA" => 'I',
        b"AAA" | b"AAG" => 'K',
        b"TTA" | b"TTG" | b"CTT" | b"CTC" | b"CTA" | b"CTG" => 'L',
        b"ATG" => 'M',
        b"AAT" | b"AAC" => 'N',
        b"CCT" | b"CCC" | b"CCA" | b"CCG" => 'P',
        b"CAA" | b"CAG" => 'Q',
        b"CGT" | b"CGC" | b"CGA" | b"CGG" | b"AGA" | b"AGG" => 'R',
        b"TCT" | b"TCC" | b"TCA" | b"TCG" | b"AGT" | b"AGC" => 'S',
        b"ACT" | b"ACC" | b"ACA" | b"ACG" => 'T',
        b"GTT" | b"GTC" | b"GTA" | b"GTG" => 'V',
        b"TGG" => 'W',
        b"TAT" | b"TAC" => 'Y',
        _ => 'X',
    }
}

pub fn gc_content(sequence: &str) -> f64 {
    let total = sequence.len() as f64;
    if total == 0.0 {
        return 0.0;
    }
    let gc = sequence
        .bytes()
        .filter(|&b| b == b'G' || b == b'g' || b == b'C' || b == b'c')
        .count() as f64;
    gc / total
}

pub fn gc3_content(sequence: &str) -> f64 {
    let seq = sequence.as_bytes();
    let mut gc3 = 0usize;
    let mut total = 0usize;
    let mut i = 0;
    while i + 2 < seq.len() {
        let third = seq[i + 2];
        if third == b'G' || third == b'g' || third == b'C' || third == b'c' {
            gc3 += 1;
        }
        total += 1;
        i += 3;
    }
    if total == 0 {
        0.0
    } else {
        gc3 as f64 / total as f64
    }
}

pub fn effective_number_of_codons(codon_counts: &[(String, usize)]) -> f64 {
    let aa_families: [&[&str]; 9] = [
        &["TTT", "TTC"],
        &["TTA", "TTG", "CTT", "CTC", "CTA", "CTG"],
        &["ATT", "ATC", "ATA"],
        &["GTT", "GTC", "GTA", "GTG"],
        &["TCT", "TCC", "TCA", "TCG"],
        &["CCT", "CCC", "CCA", "CCG"],
        &["ACT", "ACC", "ACA", "ACG"],
        &["GCT", "GCC", "GCA", "GCG"],
        &["GGT", "GGC", "GGA", "GGG"],
    ];
    let mut f_sum = 0.0;
    let mut k = 0;
    for family in &aa_families {
        let counts: Vec<f64> = family
            .iter()
            .map(|cod| {
                codon_counts
                    .iter()
                    .find(|(c, _)| c == cod)
                    .map_or(0.0, |(_, n)| *n as f64)
            })
            .collect();
        let total: f64 = counts.iter().sum();
        if total <= 1.0 {
            continue;
        }
        let pi_sum: f64 = counts.iter().map(|&c| (c / total) * (c / total)).sum();
        f_sum += pi_sum;
        k += 1;
    }
    if k == 0 {
        return 61.0;
    }
    let avg_f = f_sum / k as f64;
    if avg_f <= 0.0 {
        61.0
    } else {
        (1.0 / avg_f).min(61.0)
    }
}

pub fn longest_orf_length(sequence: &str) -> usize {
    let orfs = super::orf::find_orfs(sequence, 3);
    orfs.iter()
        .map(|(start, end, _)| end - start)
        .max()
        .unwrap_or(0)
}

pub fn nucleotide_frequency(sequence: &str) -> [f64; 4] {
    let n = sequence.len() as f64;
    if n == 0.0 {
        return [0.0; 4];
    }
    let mut counts = [0usize; 4];
    for b in sequence.bytes() {
        match b {
            b'A' | b'a' => counts[0] += 1,
            b'T' | b't' => counts[1] += 1,
            b'G' | b'g' => counts[2] += 1,
            b'C' | b'c' => counts[3] += 1,
            _ => {}
        }
    }
    [
        counts[0] as f64 / n,
        counts[1] as f64 / n,
        counts[2] as f64 / n,
        counts[3] as f64 / n,
    ]
}

pub fn translate(sequence: &str) -> String {
    let seq = sequence.as_bytes();
    let mut protein = String::new();
    let mut i = 0;
    while i + 2 < seq.len() {
        let codon = &seq[i..i + 3];
        match codon {
            b"TAA" | b"TAG" | b"TGA" => break,
            _ => protein.push(codon_to_aa(codon)),
        }
        i += 3;
    }
    protein
}

pub fn reverse_complement(sequence: &str) -> String {
    sequence
        .bytes()
        .rev()
        .map(|b| match b {
            b'A' | b'a' => 'T',
            b'T' | b't' => 'A',
            b'G' | b'g' => 'C',
            b'C' | b'c' => 'G',
            c => c as char,
        })
        .collect()
}
