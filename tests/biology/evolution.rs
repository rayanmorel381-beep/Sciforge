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
fn adaptation_rate() {
    let v = scalar(run_bio(
        "adaptation_rate",
        vec![
            ("mutation_rate", ParameterValue::Scalar(1e-6)),
            ("population_size", ParameterValue::Scalar(1e6)),
            ("selection_coefficient", ParameterValue::Scalar(0.01)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "adaptation rate should be positive, got {v}"
    );
}

#[test]
fn fixation_probability_neutral() {
    let v = scalar(run_bio(
        "fixation_probability_neutral",
        vec![("ne", ParameterValue::Scalar(100.0))],
    ));
    assert!((v - 0.005).abs() < 1e-8, "1/(2*Ne)=0.005, got {v}");
}

#[test]
fn substitution_rate() {
    let v = scalar(run_bio(
        "substitution_rate",
        vec![
            ("mu", ParameterValue::Scalar(1e-8)),
            ("ne", ParameterValue::Scalar(1e4)),
            ("s", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "substitution rate should be positive, got {v}"
    );
}

#[test]
fn speciation_rate_bdi() {
    let v = scalar(run_bio(
        "speciation_rate_bdi",
        vec![
            ("lambda", ParameterValue::Scalar(0.1)),
            ("mu", ParameterValue::Scalar(0.05)),
            ("t", ParameterValue::Scalar(1.0)),
            ("n0", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "species count should be positive, got {v}"
    );
}
