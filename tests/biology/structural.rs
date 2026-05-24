use sciforge_hub::prelude::*;

fn run_bio(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biology, name);
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
fn rmsd() {
    let v = scalar(run_bio(
        "rmsd",
        vec![
            (
                "coords_a",
                ParameterValue::Matrix(vec![vec![0.0, 0.0, 0.0]]),
            ),
            (
                "coords_b",
                ParameterValue::Matrix(vec![vec![1.0, 0.0, 0.0]]),
            ),
        ],
    ));
    assert!(
        (v - 1.0).abs() < 1e-8,
        "RMSD for one point offset by 1 = 1.0, got {v}"
    );
}

#[test]
fn radius_of_gyration() {
    let v = scalar(run_bio(
        "struct_radius_of_gyration",
        vec![
            (
                "coords",
                ParameterValue::Matrix(vec![
                    vec![0.0, 0.0, 0.0],
                    vec![1.0, 0.0, 0.0],
                    vec![0.0, 1.0, 0.0],
                    vec![0.0, 0.0, 1.0],
                ]),
            ),
            ("masses", ParameterValue::Vector(vec![1.0, 1.0, 1.0, 1.0])),
        ],
    ));
    assert!(v > 0.0, "Rg should be positive, got {v}");
}

#[test]
fn alpha_helix_propensity() {
    let v = scalar(run_bio(
        "alpha_helix_propensity",
        vec![(
            "residue_propensities",
            ParameterValue::Vector(vec![1.42, 1.42, 1.42, 1.42, 1.42]),
        )],
    ));
    assert!(
        v > 0.0,
        "alanine-rich should have high helix propensity, got {v}"
    );
}

#[test]
fn binding_free_energy() {
    let v = scalar(run_bio(
        "binding_free_energy",
        vec![
            ("van_der_waals", ParameterValue::Scalar(-5.0)),
            ("electrostatic", ParameterValue::Scalar(-3.0)),
            ("desolvation", ParameterValue::Scalar(2.0)),
            ("entropy_penalty", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(
        v < 0.0,
        "binding should be favorable (negative dG), got {v}"
    );
}
