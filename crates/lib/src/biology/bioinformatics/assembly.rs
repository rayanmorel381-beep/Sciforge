pub fn assembly_n50(contig_lengths: &[usize]) -> usize {
    if contig_lengths.is_empty() {
        return 0;
    }
    let mut sorted = contig_lengths.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let total: usize = sorted.iter().sum();
    let half = total / 2;
    let mut cumulative = 0;
    for &len in &sorted {
        cumulative += len;
        if cumulative >= half {
            return len;
        }
    }
    0
}

pub fn n_metric(contig_lengths: &[usize], fraction: f64) -> usize {
    if contig_lengths.is_empty() {
        return 0;
    }
    let mut sorted = contig_lengths.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let total: usize = sorted.iter().sum();
    let target = (total as f64 * fraction) as usize;
    let mut cumulative = 0;
    for &len in &sorted {
        cumulative += len;
        if cumulative >= target {
            return len;
        }
    }
    0
}

pub fn l50(contig_lengths: &[usize]) -> usize {
    if contig_lengths.is_empty() {
        return 0;
    }
    let mut sorted = contig_lengths.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let total: usize = sorted.iter().sum();
    let half = total / 2;
    let mut cumulative = 0;
    for (i, &len) in sorted.iter().enumerate() {
        cumulative += len;
        if cumulative >= half {
            return i + 1;
        }
    }
    sorted.len()
}

pub fn genome_coverage(total_bases_sequenced: usize, genome_size: usize) -> f64 {
    total_bases_sequenced as f64 / genome_size.max(1) as f64
}

pub fn lander_waterman(coverage: f64) -> f64 {
    1.0 - (-coverage).exp()
}

pub fn expected_contigs(
    n_reads: usize,
    read_length: usize,
    genome_size: usize,
    overlap: usize,
) -> f64 {
    let effective_length = read_length.saturating_sub(overlap);
    let coverage = (n_reads * read_length) as f64 / genome_size.max(1) as f64;
    let sigma = effective_length as f64 / genome_size.max(1) as f64;
    genome_size as f64 * (-coverage * sigma).exp() / read_length.max(1) as f64
}

pub fn assembly_completeness(aligned_bases: usize, reference_size: usize) -> f64 {
    aligned_bases as f64 / reference_size.max(1) as f64
}

pub fn gc_content_reads(reads: &[&[u8]]) -> f64 {
    let mut gc = 0usize;
    let mut total = 0usize;
    for read in reads {
        for &b in *read {
            match b {
                b'G' | b'g' | b'C' | b'c' => gc += 1,
                _ => {}
            }
            total += 1;
        }
    }
    if total == 0 {
        return 0.0;
    }
    gc as f64 / total as f64
}

pub fn ng50(contig_lengths: &[usize], genome_size: usize) -> usize {
    if contig_lengths.is_empty() {
        return 0;
    }
    let mut sorted = contig_lengths.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let half = genome_size / 2;
    let mut cumulative = 0;
    for &len in &sorted {
        cumulative += len;
        if cumulative >= half {
            return len;
        }
    }
    0
}

pub fn aunga(contig_lengths: &[usize], genome_size: usize) -> f64 {
    let mut sorted = contig_lengths.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let mut area = 0.0;
    let mut cumul = 0;
    for &len in &sorted {
        let prev = cumul as f64 / genome_size.max(1) as f64;
        cumul += len;
        let curr = cumul as f64 / genome_size.max(1) as f64;
        area += (curr - prev) * len as f64;
    }
    area
}

pub fn misassembly_rate(misassemblies: usize, total_contigs: usize) -> f64 {
    misassemblies as f64 / total_contigs.max(1) as f64
}

pub fn chimeric_contig_fraction(chimeric: usize, total: usize) -> f64 {
    chimeric as f64 / total.max(1) as f64
}

pub fn contig_size_distribution(contig_lengths: &[usize]) -> (f64, f64, usize, usize) {
    if contig_lengths.is_empty() {
        return (0.0, 0.0, 0, 0);
    }
    let n = contig_lengths.len() as f64;
    let mean = contig_lengths.iter().sum::<usize>() as f64 / n;
    let var = contig_lengths
        .iter()
        .map(|&l| (l as f64 - mean).powi(2))
        .sum::<f64>()
        / n;
    let max = *contig_lengths.iter().max().unwrap_or(&0);
    let min = *contig_lengths.iter().min().unwrap_or(&0);
    (mean, var.sqrt(), max, min)
}

pub fn expected_gap_count(coverage: f64, genome_size: usize, read_length: usize) -> f64 {
    genome_size as f64 * (-coverage * read_length as f64 / genome_size.max(1) as f64).exp()
}
