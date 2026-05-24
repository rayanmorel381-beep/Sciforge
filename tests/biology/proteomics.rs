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
fn mass_accuracy_ppm() {
    let v = scalar(run_bio(
        "mass_accuracy_ppm",
        vec![
            ("observed", ParameterValue::Scalar(500.001)),
            ("theoretical", ParameterValue::Scalar(500.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 0.1, "ppm=|dM/M|*1e6~2, got {v}");
}

#[test]
fn protein_coverage() {
    let v = scalar(run_bio(
        "protein_coverage",
        vec![
            ("identified_residues", ParameterValue::Scalar(150.0)),
            ("total_residues", ParameterValue::Scalar(300.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "coverage=50%, got {v}");
}

#[test]
fn mz_ratio() {
    let v = scalar(run_bio(
        "mz_ratio",
        vec![
            ("mass", ParameterValue::Scalar(1000.0)),
            ("charge", ParameterValue::Integer(2)),
        ],
    ));
    assert!(
        (v - 501.00728).abs() < 0.01,
        "m/z=(M+z*1.00728)/z~501.007, got {v}"
    );
}

#[test]
fn gravy_index() {
    let v = scalar(run_bio(
        "gravy_index",
        vec![("sequence", ParameterValue::Text("AAAAAA".into()))],
    ));
    assert!(v.is_finite(), "GRAVY should be finite, got {v}");
}
