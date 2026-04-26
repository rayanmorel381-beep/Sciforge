use sciforge::hub::prelude::*;

fn run_phys(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Physics, name);
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
fn diffusion_coefficient() {
    let v = scalar(run_phys(
        "diffusion_coefficient",
        vec![
            ("d0", ParameterValue::Scalar(1e-4)),
            ("q", ParameterValue::Scalar(50000.0)),
            ("t", ParameterValue::Scalar(300.0)),
        ],
    ));
    assert!(v > 0.0, "D should be positive, got {v}");
}

#[test]
fn vacancy_concentration() {
    let v = scalar(run_phys(
        "vacancy_concentration",
        vec![
            ("ev", ParameterValue::Scalar(1.602e-19)),
            ("t", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(v > 0.0 && v < 1.0, "vacancy fraction in (0,1), got {v}");
}

#[test]
fn fick_first_law() {
    let v = scalar(run_phys(
        "fick_first_law",
        vec![
            ("d", ParameterValue::Scalar(1e-9)),
            ("dc_dx", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(v.is_finite(), "flux should be finite, got {v}");
}
