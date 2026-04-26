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
fn planck_energy() {
    let v = scalar(run_phys("planck_energy", vec![]));
    assert!(v > 0.0, "Planck energy should be positive, got {v}");
}

#[test]
fn photon_energy() {
    let v = scalar(run_phys(
        "photon_energy",
        vec![("frequency", ParameterValue::Scalar(6e14))],
    ));
    assert!(v > 0.0, "photon energy should be positive, got {v}");
}

#[test]
fn fine_structure_constant() {
    let v = scalar(run_phys("fine_structure_constant", vec![]));
    assert!((v - 7.297e-3).abs() < 1e-4, "alpha≈1/137≈7.3e-3, got {v}");
}

#[test]
fn compton_time() {
    let v = scalar(run_phys(
        "compton_time",
        vec![("mass", ParameterValue::Scalar(9.109e-31))],
    ));
    assert!(v > 0.0, "Compton time should be positive, got {v}");
}
