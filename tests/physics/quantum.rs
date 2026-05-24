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
fn hydrogen_energy() {
    let v = scalar(run_phys(
        "hydrogen_energy",
        vec![("n", ParameterValue::Scalar(1.0))],
    ));
    assert!((v - (-2.179e-18)).abs() < 1e-20, "E1=-2.179e-18 J, got {v}");
}

#[test]
fn infinite_well_energy() {
    let v = scalar(run_phys(
        "infinite_well_energy",
        vec![
            ("n", ParameterValue::Scalar(1.0)),
            ("length", ParameterValue::Scalar(1e-9)),
            ("mass", ParameterValue::Scalar(9.109e-31)),
        ],
    ));
    assert!(v > 0.0, "ground state energy should be positive, got {v}");
}

#[test]
fn tunneling_coefficient() {
    let v = scalar(run_phys(
        "tunneling_coefficient",
        vec![
            ("energy", ParameterValue::Scalar(1.0)),
            ("v0", ParameterValue::Scalar(2.0)),
            ("width", ParameterValue::Scalar(1e-10)),
            ("mass", ParameterValue::Scalar(9.109e-31)),
        ],
    ));
    assert!((0.0..=1.0).contains(&v), "transmission in [0,1], got {v}");
}

#[test]
fn harmonic_oscillator_energy() {
    let v = scalar(run_phys(
        "harmonic_oscillator_energy",
        vec![
            ("n", ParameterValue::Scalar(0.0)),
            ("omega", ParameterValue::Scalar(1e14)),
        ],
    ));
    assert!(v > 0.0, "zero-point energy should be positive, got {v}");
}
