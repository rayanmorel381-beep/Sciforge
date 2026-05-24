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
fn cell_doubling_time() {
    let v = scalar(run_bio(
        "cell_doubling_time",
        vec![("growth_rate", ParameterValue::Scalar(0.5))],
    ));
    let expected = (2.0_f64).ln() / 0.5;
    assert!((v - expected).abs() < 1e-6, "Td=ln2/r, got {v}");
}

#[test]
fn ligand_receptor_binding() {
    let v = scalar(run_bio(
        "ligand_receptor_binding",
        vec![
            ("l", ParameterValue::Scalar(10.0)),
            ("r_total", ParameterValue::Scalar(100.0)),
            ("kd", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-6, "at L=Kd, 50% bound, got {v}");
}

#[test]
fn osmotic_pressure() {
    let v = scalar(run_bio(
        "osmotic_pressure",
        vec![
            ("c", ParameterValue::Scalar(0.1)),
            ("t", ParameterValue::Scalar(310.0)),
        ],
    ));
    assert!(
        v > 0.0 && v.is_finite(),
        "osmotic pressure should be positive, got {v}"
    );
}

#[test]
fn fick_first_law() {
    let v = scalar(run_bio(
        "fick_first_law",
        vec![
            ("d", ParameterValue::Scalar(1e-9)),
            ("dc_dx", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(v.is_finite(), "flux should be finite, got {v}");
}
