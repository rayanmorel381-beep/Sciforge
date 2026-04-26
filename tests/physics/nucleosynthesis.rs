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
fn nuclear_radius_fm() {
    let v = scalar(run_phys(
        "nuclear_radius_fm",
        vec![("a", ParameterValue::Scalar(56.0))],
    ));
    assert!(v > 0.0, "nuclear radius should be positive, got {v}");
}

#[test]
fn binding_energy_per_nucleon_semf() {
    let v = scalar(run_phys(
        "binding_energy_per_nucleon_semf",
        vec![
            ("z", ParameterValue::Scalar(26.0)),
            ("a", ParameterValue::Scalar(56.0)),
        ],
    ));
    assert!(v > 7.0 && v < 9.0, "BE/A for Fe-56 ~8.8 MeV, got {v}");
}

#[test]
fn coulomb_barrier() {
    let v = scalar(run_phys(
        "coulomb_barrier",
        vec![
            ("z1", ParameterValue::Integer(1)),
            ("z2", ParameterValue::Integer(1)),
            ("a1", ParameterValue::Integer(1)),
            ("a2", ParameterValue::Integer(1)),
        ],
    ));
    assert!(v > 0.0, "Coulomb barrier should be positive, got {v}");
}

#[test]
fn half_life_from_constant() {
    let v = scalar(run_phys(
        "half_life_from_constant",
        vec![("decay_constant", ParameterValue::Scalar(0.1))],
    ));
    let expected = (2.0_f64).ln() / 0.1;
    assert!((v - expected).abs() < 1e-6, "t1/2=ln2/lambda, got {v}");
}
