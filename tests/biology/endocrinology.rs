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
fn homa_ir() {
    let v = scalar(run_bio(
        "homa_ir",
        vec![
            ("fasting_glucose_mmol", ParameterValue::Scalar(5.0)),
            ("fasting_insulin_mu_per_ml", ParameterValue::Scalar(10.0)),
        ],
    ));
    let expected = 5.0 * 10.0 / 22.5;
    assert!((v - expected).abs() < 1e-6, "HOMA-IR=G*I/22.5, got {v}");
}

#[test]
fn hormone_half_life_clearance() {
    let v = scalar(run_bio(
        "hormone_half_life_clearance",
        vec![
            ("concentration", ParameterValue::Scalar(100.0)),
            ("half_life", ParameterValue::Scalar(10.0)),
            ("t", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 0.1, "at t=t1/2, C~C0/2~50, got {v}");
}

#[test]
fn receptor_saturation() {
    let v = scalar(run_bio(
        "receptor_saturation",
        vec![
            ("hormone", ParameterValue::Scalar(10.0)),
            ("kd", ParameterValue::Scalar(10.0)),
            ("receptor_total", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1.0, "at H=Kd, ~50% sat, got {v}");
}

#[test]
fn insulin_sensitivity_index() {
    let v = scalar(run_bio(
        "insulin_sensitivity_index",
        vec![
            ("glucose", ParameterValue::Scalar(10.0)),
            ("insulin", ParameterValue::Scalar(50.0)),
        ],
    ));
    let expected = 1.0 / (10.0 * 50.0);
    assert!(
        (v - expected).abs() < 1e-8,
        "ISI=1/(G*I)={expected}, got {v}"
    );
}
