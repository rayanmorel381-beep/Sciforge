use sciforge::hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Astrophysics, name);
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
fn schwarzschild_radius() {
    let v = scalar(run(
        "schwarzschild_radius",
        vec![("mass", ParameterValue::Scalar(1.989e30))],
    ));
    assert!((v - 2953.0).abs() < 10.0, "Rs(Sun) ≈ 2953 m, got {v}");
}

#[test]
fn eddington_luminosity() {
    let v = scalar(run(
        "eddington_luminosity",
        vec![("mass", ParameterValue::Scalar(1.989e30))],
    ));
    assert!(v > 1e30, "Eddington L for 1 M☉ should be huge, got {v}");
}

#[test]
fn hawking_temperature() {
    let v = scalar(run(
        "hawking_temperature",
        vec![("mass", ParameterValue::Scalar(1.989e30))],
    ));
    assert!(v > 0.0, "Hawking T should be positive, got {v}");
}

#[test]
fn compton_wavelength() {
    let v = scalar(run(
        "compton_wavelength",
        vec![("mass", ParameterValue::Scalar(9.109e-31))],
    ));
    assert!(
        (v - 2.426e-12).abs() < 1e-14,
        "electron Compton ≈ 2.426e-12 m, got {v}"
    );
}
