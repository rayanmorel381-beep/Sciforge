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
fn scaffold_porosity() {
    let v = scalar(run_bio(
        "scaffold_porosity",
        vec![
            ("void_volume", ParameterValue::Scalar(70.0)),
            ("total_volume", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 0.7).abs() < 1e-8, "porosity=70/100=0.7, got {v}");
}

#[test]
fn degradation_rate_first_order() {
    let v = scalar(run_bio(
        "degradation_rate_first_order",
        vec![
            ("mass", ParameterValue::Scalar(100.0)),
            ("k_deg", ParameterValue::Scalar(0.1)),
            ("time", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-6, "at t=0, M=M0, got {v}");
}

#[test]
fn cell_proliferation_on_scaffold() {
    let v = scalar(run_bio(
        "cell_proliferation_on_scaffold",
        vec![
            ("n0", ParameterValue::Scalar(1000.0)),
            ("doubling_time", ParameterValue::Scalar(24.0)),
            ("elapsed", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 1000.0).abs() < 1e-6, "at t=0, N=N0, got {v}");
}

#[test]
fn oxygen_transfer_rate() {
    let v = scalar(run_bio(
        "oxygen_transfer_rate",
        vec![
            ("kla", ParameterValue::Scalar(10.0)),
            ("c_star", ParameterValue::Scalar(8.0)),
            ("c_bulk", ParameterValue::Scalar(4.0)),
        ],
    ));
    assert!((v - 40.0).abs() < 1e-6, "OTR=kLa*(C*-C)=40, got {v}");
}
