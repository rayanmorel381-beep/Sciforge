use sciforge_hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::MathematicalPhysics, name);
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
fn de_broglie_wavelength() {
    let v = scalar(run(
        "de_broglie_wavelength",
        vec![("momentum", ParameterValue::Scalar(1e-24))],
    ));
    assert!(v > 0.0, "wavelength should be positive, got {v}");
}

#[test]
fn heisenberg_uncertainty_position() {
    let v = scalar(run(
        "heisenberg_uncertainty_position",
        vec![("delta_p", ParameterValue::Scalar(1e-24))],
    ));
    assert!(v > 0.0, "Δx should be positive, got {v}");
}

#[test]
fn fermi_dirac_distribution() {
    let v = scalar(run(
        "fermi_dirac_distribution",
        vec![
            ("energy", ParameterValue::Scalar(1.0)),
            ("chemical_potential", ParameterValue::Scalar(1.0)),
            ("temperature", ParameterValue::Scalar(300.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-6, "f(E=μ)=0.5, got {v}");
}

#[test]
fn relativistic_energy() {
    let v = scalar(run(
        "relativistic_energy",
        vec![
            ("rest_mass", ParameterValue::Scalar(9.109e-31)),
            ("momentum", ParameterValue::Scalar(0.0)),
        ],
    ));
    let c = 2.998e8;
    let expected = 9.109e-31 * c * c;
    assert!(
        (v - expected).abs() / expected < 1e-3,
        "E(p=0)=mc², got {v}"
    );
}
