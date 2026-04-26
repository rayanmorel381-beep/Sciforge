use sciforge::hub::prelude::*;

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
fn shannon_diversity() {
    let v = scalar(run_bio(
        "shannon_diversity",
        vec![(
            "abundances",
            ParameterValue::Vector(vec![0.25, 0.25, 0.25, 0.25]),
        )],
    ));
    let expected = (4.0_f64).ln();
    assert!(
        (v - expected).abs() < 1e-8,
        "H = ln(4) for uniform dist, got {v}"
    );
}

#[test]
fn simpson_diversity() {
    let v = scalar(run_bio(
        "simpson_diversity",
        vec![("abundances", ParameterValue::Vector(vec![0.5, 0.5]))],
    ));
    assert!((v - 0.5).abs() < 1e-10, "D = 1 - Σpi² = 0.5, got {v}");
}

#[test]
fn species_richness() {
    let out = run_bio(
        "species_richness",
        vec![(
            "abundances",
            ParameterValue::Vector(vec![10.0, 5.0, 3.0, 0.0, 7.0]),
        )],
    );
    match out {
        RunOutput::Integer(v) => assert!(v == 4, "4 non-zero species, got {v}"),
        RunOutput::Scalar(v) => assert!((v - 4.0).abs() < 1e-10, "4 non-zero species, got {v}"),
        _ => panic!("unexpected output: {out:?}"),
    }
}

#[test]
fn inverse_simpson() {
    let v = scalar(run_bio(
        "inverse_simpson",
        vec![(
            "abundances",
            ParameterValue::Vector(vec![0.25, 0.25, 0.25, 0.25]),
        )],
    ));
    assert!((v - 4.0).abs() < 1e-8, "1/Σpi² = 4 for uniform, got {v}");
}
