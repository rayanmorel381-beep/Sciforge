use sciforge_hub::prelude::*;

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
fn pauling_electronegativity_difference() {
    let v = scalar(run_chem(
        "pauling_electronegativity_difference",
        vec![
            ("en_a", ParameterValue::Scalar(3.44)),
            ("en_b", ParameterValue::Scalar(0.93)),
        ],
    ));
    assert!((v - 2.51).abs() < 1e-6, "dEN=3.44-0.93=2.51, got {v}");
}

#[test]
fn magnetic_moment_spin_only() {
    let v = scalar(run_chem(
        "magnetic_moment_spin_only",
        vec![("unpaired", ParameterValue::Integer(4))],
    ));
    let expected = (4.0_f64 * (4.0 + 2.0)).sqrt();
    assert!(
        (v - expected).abs() < 1e-6,
        "mu=sqrt(n*(n+2))={expected}, got {v}"
    );
}

#[test]
fn lattice_energy_born_lande() {
    let v = scalar(run_chem(
        "lattice_energy_born_lande",
        vec![
            ("madelung", ParameterValue::Scalar(1.748)),
            ("z_plus", ParameterValue::Scalar(1.0)),
            ("z_minus", ParameterValue::Scalar(1.0)),
            ("r0", ParameterValue::Scalar(2.81e-10)),
            ("born_exponent", ParameterValue::Scalar(8.0)),
        ],
    ));
    assert!(
        v < 0.0,
        "lattice energy should be negative (exothermic), got {v}"
    );
}
