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
fn ld50_probit() {
    let v = scalar(run_bio(
        "ld50_probit",
        vec![
            (
                "doses",
                ParameterValue::Vector(vec![10.0, 30.0, 50.0, 70.0, 90.0]),
            ),
            (
                "responses",
                ParameterValue::Vector(vec![1.0, 3.0, 5.0, 7.0, 9.0]),
            ),
            (
                "totals",
                ParameterValue::Vector(vec![10.0, 10.0, 10.0, 10.0, 10.0]),
            ),
        ],
    ));
    assert!(v > 0.0 && v.is_finite(), "LD50 should be positive, got {v}");
}

#[test]
fn therapeutic_window() {
    let v = scalar(run_bio(
        "therapeutic_window",
        vec![
            ("ec50", ParameterValue::Scalar(10.0)),
            ("td50", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 10.0).abs() < 1e-6, "TW=TD50/EC50=10, got {v}");
}

#[test]
fn bcf_ratio() {
    let v = scalar(run_bio(
        "bcf_ratio",
        vec![
            ("c_organism", ParameterValue::Scalar(500.0)),
            ("c_water", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-8, "BCF=500/10=50, got {v}");
}

#[test]
fn risk_quotient() {
    let v = scalar(run_bio(
        "risk_quotient",
        vec![
            (
                "predicted_environmental_concentration",
                ParameterValue::Scalar(5.0),
            ),
            (
                "predicted_no_effect_concentration",
                ParameterValue::Scalar(10.0),
            ),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "RQ=5/10=0.5, got {v}");
}
