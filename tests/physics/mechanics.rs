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
fn hooke_stress() {
    let v = scalar(run_phys(
        "hooke_stress",
        vec![
            ("e", ParameterValue::Scalar(200e9)),
            ("strain", ParameterValue::Scalar(0.001)),
        ],
    ));
    assert!((v - 200e6).abs() < 1e3, "σ = E·ε = 200 MPa");
}

#[test]
fn hooke_strain() {
    let v = scalar(run_phys(
        "hooke_strain",
        vec![
            ("stress", ParameterValue::Scalar(100e6)),
            ("e", ParameterValue::Scalar(200e9)),
        ],
    ));
    assert!((v - 0.0005).abs() < 1e-8, "ε = σ/E = 0.0005");
}

#[test]
fn drag_force() {
    let v = scalar(run_phys(
        "drag_force",
        vec![
            ("cd", ParameterValue::Scalar(0.47)),
            ("rho", ParameterValue::Scalar(1.225)),
            ("a", ParameterValue::Scalar(0.01)),
            ("v", ParameterValue::Scalar(10.0)),
        ],
    ));
    let expected = 0.5 * 0.47 * 1.225 * 0.01 * 100.0;
    assert!((v - expected).abs() / expected < 1e-6);
}

#[test]
fn mach_number() {
    let v = scalar(run_phys(
        "mach_number",
        vec![
            ("v", ParameterValue::Scalar(680.0)),
            ("c", ParameterValue::Scalar(340.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-12);
}

#[test]
fn bernoulli_pressure() {
    let v = scalar(run_phys(
        "bernoulli_pressure",
        vec![
            ("p1", ParameterValue::Scalar(101325.0)),
            ("rho", ParameterValue::Scalar(1.225)),
            ("v1", ParameterValue::Scalar(0.0)),
            ("v2", ParameterValue::Scalar(50.0)),
        ],
    ));
    let expected = 101325.0 - 0.5 * 1.225 * 2500.0;
    assert!((v - expected).abs() < 1.0);
}

#[test]
fn beat_frequency() {
    let v = scalar(run_phys(
        "beat_frequency",
        vec![
            ("f1", ParameterValue::Scalar(440.0)),
            ("f2", ParameterValue::Scalar(444.0)),
        ],
    ));
    assert!((v - 4.0).abs() < 1e-10);
}

#[test]
fn doppler_shift_wavelength() {
    let v = scalar(run_phys(
        "doppler_shift_wavelength",
        vec![
            ("lambda0", ParameterValue::Scalar(500e-9)),
            ("v", ParameterValue::Scalar(0.0)),
            ("c", ParameterValue::Scalar(343.0)),
        ],
    ));
    assert!((v - 500e-9).abs() < 1e-15, "no shift at v=0");
}
