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
fn radio_linear_quadratic_survival() {
    let v = scalar(run_bio(
        "radio_linear_quadratic_survival",
        vec![
            ("dose", ParameterValue::Scalar(0.0)),
            ("alpha", ParameterValue::Scalar(0.3)),
            ("beta", ParameterValue::Scalar(0.03)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-8, "dose=0 => survival=1, got {v}");
}

#[test]
fn radio_biologically_effective_dose() {
    let v = scalar(run_bio(
        "radio_biologically_effective_dose",
        vec![
            ("n", ParameterValue::Scalar(30.0)),
            ("d", ParameterValue::Scalar(2.0)),
            ("alpha_beta", ParameterValue::Scalar(10.0)),
        ],
    ));
    let expected = 30.0 * 2.0 * (1.0 + 2.0 / 10.0);
    assert!(
        (v - expected).abs() < 1e-6,
        "BED=nD(1+d/ab)={expected}, got {v}"
    );
}

#[test]
fn oxygen_enhancement_ratio() {
    let v = scalar(run_bio(
        "oxygen_enhancement_ratio",
        vec![
            ("dose_hypoxic", ParameterValue::Scalar(30.0)),
            ("dose_oxic", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 3.0).abs() < 1e-8, "OER=30/10=3, got {v}");
}

#[test]
fn tcp() {
    let v = scalar(run_bio(
        "tcp",
        vec![
            ("n_cells", ParameterValue::Scalar(1e9)),
            ("survival_fraction", ParameterValue::Scalar(1e-10)),
        ],
    ));
    assert!((0.0..=1.0).contains(&v), "TCP in [0,1], got {v}");
}
