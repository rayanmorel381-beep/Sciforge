use sciforge::hub::prelude::*;

fn run_bio(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biology, name);
    for (k, v) in params {
        exp = exp.param(k, v);
    }
    ExperimentRunner::new()
        .run(&exp)
        .unwrap_or_else(|_| panic!("dispatch '{name}' failed"))
}

fn scalar(o: RunOutput) -> f64 {
    match o {
        RunOutput::Scalar(v) => v,
        _ => panic!("expected Scalar, got {o:?}"),
    }
}

#[test]
fn gc_content() {
    let v = scalar(run_bio(
        "gc_content",
        vec![("sequence", ParameterValue::Text("ATGC".into()))],
    ));
    assert!((v - 0.5).abs() < 1e-8, "ATGC has 50% GC, got {v}");
}

#[test]
fn gene_density() {
    let v = scalar(run_bio(
        "gene_density",
        vec![
            ("genes", ParameterValue::Integer(200)),
            ("region_length_mb", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.0, "gene density should be positive, got {v}");
}

#[test]
fn snp_allele_frequency() {
    let v = scalar(run_bio(
        "snp_allele_frequency",
        vec![
            ("alt_count", ParameterValue::Integer(30)),
            ("total_alleles", ParameterValue::Integer(100)),
        ],
    ));
    assert!((v - 0.3).abs() < 1e-8, "30/100=0.3, got {v}");
}

#[test]
fn codon_adaptation_index() {
    let v = scalar(run_bio(
        "codon_adaptation_index",
        vec![(
            "codon_weights",
            ParameterValue::Vector(vec![0.8, 0.6, 0.9, 0.7, 0.5]),
        )],
    ));
    assert!((0.0..=1.0).contains(&v), "CAI in [0,1], got {v}");
}
