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
fn body_mass_index() {
    let v = scalar(run_bio(
        "body_mass_index",
        vec![
            ("weight_kg", ParameterValue::Scalar(80.0)),
            ("height_m", ParameterValue::Scalar(1.80)),
        ],
    ));
    let expected = 80.0 / (1.80 * 1.80);
    assert!((v - expected).abs() < 1e-4, "BMI=w/h²={expected}, got {v}");
}

#[test]
fn basal_metabolic_rate_mifflin() {
    let v = scalar(run_bio(
        "basal_metabolic_rate_mifflin",
        vec![
            ("weight_kg", ParameterValue::Scalar(70.0)),
            ("height_cm", ParameterValue::Scalar(175.0)),
            ("age", ParameterValue::Scalar(30.0)),
            ("is_male", ParameterValue::Boolean(true)),
        ],
    ));
    assert!(
        v > 1000.0 && v < 3000.0,
        "BMR should be reasonable, got {v}"
    );
}

#[test]
fn nitrogen_balance() {
    let v = scalar(run_bio(
        "nitrogen_balance",
        vec![
            ("protein_intake_g", ParameterValue::Scalar(100.0)),
            ("urinary_n", ParameterValue::Scalar(10.0)),
            ("fecal_n", ParameterValue::Scalar(3.0)),
            ("sweat_n", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v.is_finite(), "nitrogen balance should be finite, got {v}");
}

#[test]
fn respiratory_exchange_ratio() {
    let v = scalar(run_bio(
        "respiratory_exchange_ratio",
        vec![
            ("co2_produced", ParameterValue::Scalar(200.0)),
            ("o2_consumed", ParameterValue::Scalar(250.0)),
        ],
    ));
    assert!((v - 0.8).abs() < 1e-8, "RER=200/250=0.8, got {v}");
}
