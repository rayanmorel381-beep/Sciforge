use sciforge_hub::prelude::*;

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
fn cpg_methylation_level() {
    let v = scalar(run_bio(
        "cpg_methylation_level",
        vec![
            ("methylated", ParameterValue::Integer(80)),
            ("total_cpg", ParameterValue::Integer(100)),
        ],
    ));
    assert!((v - 0.8).abs() < 1e-8, "80/100=0.8, got {v}");
}

#[test]
fn nucleosome_occupancy() {
    let v = scalar(run_bio(
        "nucleosome_occupancy",
        vec![
            ("dna_affinity", ParameterValue::Scalar(1.0)),
            ("histone_conc", ParameterValue::Scalar(1.0)),
            ("competitor_conc", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((0.0..=1.0).contains(&v), "occupancy in [0,1], got {v}");
}

#[test]
fn epigenetic_inheritance_probability() {
    let v = scalar(run_bio(
        "epigenetic_inheritance_probability",
        vec![
            ("maintenance", ParameterValue::Scalar(0.95)),
            ("generations", ParameterValue::Integer(0)),
        ],
    ));
    assert!(
        (v - 1.0).abs() < 1e-6,
        "at 0 generations, probability~1, got {v}"
    );
}

#[test]
fn rnai_knockdown_efficiency() {
    let v = scalar(run_bio(
        "rnai_knockdown_efficiency",
        vec![
            ("sirna_conc", ParameterValue::Scalar(50.0)),
            ("target_mrna", ParameterValue::Scalar(100.0)),
            ("risc_activity", ParameterValue::Scalar(0.9)),
            ("kd", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!(v > 0.0, "knockdown should be positive, got {v}");
}
