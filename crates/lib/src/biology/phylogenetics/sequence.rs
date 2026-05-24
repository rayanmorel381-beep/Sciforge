pub fn gc_content(seq: &[u8]) -> f64 {
    let gc = seq
        .iter()
        .filter(|&&b| b == b'G' || b == b'C' || b == b'g' || b == b'c')
        .count();
    gc as f64 / seq.len() as f64
}

pub fn complement(base: u8) -> u8 {
    match base {
        b'A' => b'T',
        b'T' => b'A',
        b'G' => b'C',
        b'C' => b'G',
        b'a' => b't',
        b't' => b'a',
        b'g' => b'c',
        b'c' => b'g',
        other => other,
    }
}

pub fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    seq.iter().rev().map(|&b| complement(b)).collect()
}

pub fn translate_codon(codon: &[u8]) -> u8 {
    if codon.len() < 3 {
        return b'X';
    }
    match (codon[0], codon[1], codon[2]) {
        (b'T', b'T', b'T') | (b'T', b'T', b'C') => b'F',
        (b'T', b'T', b'A') | (b'T', b'T', b'G') => b'L',
        (b'C', b'T', _) => b'L',
        (b'A', b'T', b'T') | (b'A', b'T', b'C') | (b'A', b'T', b'A') => b'I',
        (b'A', b'T', b'G') => b'M',
        (b'G', b'T', _) => b'V',
        (b'T', b'C', _) => b'S',
        (b'C', b'C', _) => b'P',
        (b'A', b'C', _) => b'T',
        (b'G', b'C', _) => b'A',
        (b'T', b'A', b'T') | (b'T', b'A', b'C') => b'Y',
        (b'T', b'A', b'A') | (b'T', b'A', b'G') => b'*',
        (b'C', b'A', b'T') | (b'C', b'A', b'C') => b'H',
        (b'C', b'A', b'A') | (b'C', b'A', b'G') => b'Q',
        (b'A', b'A', b'T') | (b'A', b'A', b'C') => b'N',
        (b'A', b'A', b'A') | (b'A', b'A', b'G') => b'K',
        (b'G', b'A', b'T') | (b'G', b'A', b'C') => b'D',
        (b'G', b'A', b'A') | (b'G', b'A', b'G') => b'E',
        (b'T', b'G', b'T') | (b'T', b'G', b'C') => b'C',
        (b'T', b'G', b'A') => b'*',
        (b'T', b'G', b'G') => b'W',
        (b'C', b'G', _) => b'R',
        (b'A', b'G', b'T') | (b'A', b'G', b'C') => b'S',
        (b'A', b'G', b'A') | (b'A', b'G', b'G') => b'R',
        (b'G', b'G', _) => b'G',
        _ => b'X',
    }
}

pub fn translate(dna: &[u8]) -> Vec<u8> {
    (0..dna.len() / 3)
        .map(|i| translate_codon(&dna[i * 3..i * 3 + 3]))
        .collect()
}

pub fn transcribe(dna: &[u8]) -> Vec<u8> {
    dna.iter()
        .map(|&b| match b {
            b'T' => b'U',
            b't' => b'u',
            other => other,
        })
        .collect()
}

pub fn nucleotide_frequencies(seq: &[u8]) -> [f64; 4] {
    let mut counts = [0usize; 4];
    for &b in seq {
        match b {
            b'A' | b'a' => counts[0] += 1,
            b'C' | b'c' => counts[1] += 1,
            b'G' | b'g' => counts[2] += 1,
            b'T' | b't' | b'U' | b'u' => counts[3] += 1,
            _ => {}
        }
    }
    let total = counts.iter().sum::<usize>() as f64;
    if total < 1.0 {
        return [0.0; 4];
    }
    [
        counts[0] as f64 / total,
        counts[1] as f64 / total,
        counts[2] as f64 / total,
        counts[3] as f64 / total,
    ]
}

pub fn molecular_weight_dna(seq: &[u8]) -> f64 {
    let mut weight = 0.0;
    for &b in seq {
        weight += match b {
            b'A' | b'a' => 331.2,
            b'T' | b't' => 322.2,
            b'G' | b'g' => 347.2,
            b'C' | b'c' => 307.2,
            _ => 0.0,
        };
    }
    weight - 61.96
}

pub fn melting_temperature_basic(seq: &[u8]) -> f64 {
    let mut at = 0;
    let mut gc = 0;
    for &b in seq {
        match b {
            b'A' | b'a' | b'T' | b't' => at += 1,
            b'G' | b'g' | b'C' | b'c' => gc += 1,
            _ => {}
        }
    }
    if seq.len() < 14 {
        2.0 * at as f64 + 4.0 * gc as f64
    } else {
        64.9 + 41.0 * (gc as f64 - 16.4) / (at + gc) as f64
    }
}
