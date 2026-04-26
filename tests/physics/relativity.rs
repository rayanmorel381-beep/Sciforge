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
fn time_dilation() {
    let v = scalar(run_phys(
        "time_dilation",
        vec![
            ("proper_time", ParameterValue::Scalar(1.0)),
            ("v", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-12, "no dilation at v=0");
}

#[test]
fn time_dilation_high_v() {
    let v = scalar(run_phys(
        "time_dilation",
        vec![
            ("proper_time", ParameterValue::Scalar(1.0)),
            ("v", ParameterValue::Scalar(0.866 * 299_792_458.0)),
        ],
    ));
    assert!(v > 1.9 && v < 2.1, "γ ≈ 2 at v ≈ 0.866c, got {v}");
}

#[test]
fn length_contraction() {
    let v = scalar(run_phys(
        "length_contraction",
        vec![
            ("proper_length", ParameterValue::Scalar(10.0)),
            ("v", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 10.0).abs() < 1e-12, "no contraction at v=0");
}

#[test]
fn gamma_factor_at_rest() {
    let v = scalar(run_phys(
        "gamma_factor",
        vec![("v", ParameterValue::Scalar(0.0))],
    ));
    assert!((v - 1.0).abs() < 1e-12, "γ=1 at rest");
}

#[test]
fn gamma_factor_increases_with_velocity() {
    let g1 = scalar(run_phys(
        "gamma_factor",
        vec![("v", ParameterValue::Scalar(0.5 * 299_792_458.0))],
    ));
    let g2 = scalar(run_phys(
        "gamma_factor",
        vec![("v", ParameterValue::Scalar(0.9 * 299_792_458.0))],
    ));
    assert!(g2 > g1, "γ should increase with v");
    assert!(g1 > 1.0);
}

#[test]
fn energy_momentum_relation() {
    let v = scalar(run_phys(
        "energy_momentum_relation",
        vec![
            ("mass", ParameterValue::Scalar(1.0)),
            ("momentum", ParameterValue::Scalar(0.0)),
        ],
    ));
    let c = 299_792_458.0;
    let expected = 1.0 * c * c;
    assert!((v - expected).abs() / expected < 1e-6, "E = mc² at rest");
}

#[test]
fn compton_wavelength_shift() {
    let v = scalar(run_phys(
        "compton_wavelength_shift",
        vec![("angle", ParameterValue::Scalar(std::f64::consts::PI))],
    ));
    let expected = 2.0 * 2.426_310_238_67e-12;
    assert!(
        (v - expected).abs() / expected < 1e-4,
        "max Compton shift at 180°"
    );
}

#[test]
fn planck_radiation_positive() {
    let v = scalar(run_phys(
        "planck_radiation",
        vec![
            ("frequency", ParameterValue::Scalar(1e14)),
            ("temperature", ParameterValue::Scalar(5000.0)),
        ],
    ));
    assert!(v > 0.0, "Planck radiation should be positive");
}
