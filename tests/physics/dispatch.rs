use sciforge_hub::prelude::*;

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
fn drift_velocity() {
    let v = scalar(run_phys(
        "drift_velocity",
        vec![
            ("mu", ParameterValue::Scalar(0.15)),
            ("e_field", ParameterValue::Scalar(100.0)),
        ],
    ));
    let expected = 0.15 * 100.0;
    assert!((v - expected).abs() < 1e-6, "v_d = mu * E, got {v}");
}

#[test]
fn hall_coefficient() {
    let v = scalar(run_phys(
        "hall_coefficient",
        vec![
            ("n", ParameterValue::Scalar(8.5e28)),
            ("p", ParameterValue::Scalar(0.0)),
            ("mu_e", ParameterValue::Scalar(0.15)),
            ("mu_h", ParameterValue::Scalar(0.05)),
        ],
    ));
    assert!(v.abs() > 0.0, "Hall coefficient should be nonzero");
}

#[test]
fn maxwell_speed_distribution_peak() {
    let v_low = scalar(run_phys(
        "maxwell_speed_distribution",
        vec![
            ("v", ParameterValue::Scalar(1.0)),
            ("mass", ParameterValue::Scalar(4.65e-26)),
            ("temperature", ParameterValue::Scalar(300.0)),
        ],
    ));
    let v_peak = scalar(run_phys(
        "maxwell_speed_distribution",
        vec![
            ("v", ParameterValue::Scalar(400.0)),
            ("mass", ParameterValue::Scalar(4.65e-26)),
            ("temperature", ParameterValue::Scalar(300.0)),
        ],
    ));
    assert!(v_peak > v_low, "peak should be higher than tail");
}

#[test]
fn diffraction_grating_maxima() {
    let v = scalar(run_phys(
        "diffraction_grating_maxima",
        vec![
            ("d", ParameterValue::Scalar(1e-6)),
            ("wavelength", ParameterValue::Scalar(500e-9)),
            ("order", ParameterValue::Scalar(1.0)),
        ],
    ));
    let expected = (500e-9 / 1e-6_f64).asin();
    assert!((v - expected).abs() < 1e-6);
}

#[test]
fn dispatch_unknown_function_returns_error() {
    let exp = Experiment::new(DomainType::Physics, "unknown_physics_function");
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}

#[test]
fn dispatch_missing_parameter_returns_error() {
    let exp = Experiment::new(DomainType::Physics, "drift_velocity")
        .param("mu", ParameterValue::Scalar(0.15));
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}
