use sciforge::hub::prelude::*;

fn run_chem(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Chemistry, name);
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
fn fick_first_law() {
    let v = scalar(run_chem(
        "fick_first_law",
        vec![
            ("d", ParameterValue::Scalar(1e-9)),
            ("dc_dx", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(v.is_finite(), "flux should be finite, got {v}");
}

#[test]
fn diffusion_coefficient_stokes_einstein() {
    let v = scalar(run_chem(
        "diffusion_coefficient_stokes_einstein",
        vec![
            ("t", ParameterValue::Scalar(298.0)),
            ("viscosity", ParameterValue::Scalar(1e-3)),
            ("r", ParameterValue::Scalar(1e-9)),
        ],
    ));
    assert!(v > 0.0, "diffusion coefficient should be positive, got {v}");
}

#[test]
fn sherwood_number() {
    let v = scalar(run_chem(
        "sherwood_number",
        vec![
            ("k", ParameterValue::Scalar(0.01)),
            ("l", ParameterValue::Scalar(0.1)),
            ("d", ParameterValue::Scalar(1e-9)),
        ],
    ));
    assert!(v > 0.0, "Sh should be positive, got {v}");
}
