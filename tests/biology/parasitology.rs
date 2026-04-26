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
fn parasite_r0() {
    let v = scalar(run_bio(
        "parasite_r0",
        vec![
            ("beta", ParameterValue::Scalar(0.5)),
            ("host_density", ParameterValue::Scalar(100.0)),
            ("recovery_rate", ParameterValue::Scalar(0.1)),
            ("mortality_rate", ParameterValue::Scalar(0.01)),
        ],
    ));
    assert!(v > 0.0 && v.is_finite(), "R0 should be positive, got {v}");
}

#[test]
fn vector_borne_r0() {
    let v = scalar(run_bio(
        "vector_borne_r0",
        vec![
            ("mosquito_density", ParameterValue::Scalar(10.0)),
            ("biting_rate", ParameterValue::Scalar(0.5)),
            ("prob_m_to_h", ParameterValue::Scalar(0.3)),
            ("prob_h_to_m", ParameterValue::Scalar(0.2)),
            ("mosquito_mortality", ParameterValue::Scalar(0.1)),
            ("extrinsic_incubation", ParameterValue::Scalar(10.0)),
            ("recovery", ParameterValue::Scalar(0.05)),
        ],
    ));
    assert!(v > 0.0 && v.is_finite(), "R0 should be positive, got {v}");
}

#[test]
fn force_of_infection() {
    let v = scalar(run_bio(
        "cercarial_force_of_infection",
        vec![
            ("cercarial_density", ParameterValue::Scalar(100.0)),
            ("contact_rate", ParameterValue::Scalar(0.3)),
            ("penetration_prob", ParameterValue::Scalar(0.1)),
        ],
    ));
    assert!(v > 0.0 && v.is_finite(), "FOI should be positive, got {v}");
}
