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
fn p_o_ratio() {
    let v = scalar(run_bio(
        "p_o_ratio",
        vec![
            ("atp_produced", ParameterValue::Scalar(30.0)),
            ("oxygen_consumed", ParameterValue::Scalar(12.0)),
        ],
    ));
    assert!((v - 2.5).abs() < 1e-8, "30/12=2.5, got {v}");
}

#[test]
fn respiratory_control_index() {
    let v = scalar(run_bio(
        "respiratory_control_index",
        vec![
            ("state3_rate", ParameterValue::Scalar(100.0)),
            ("state4_rate", ParameterValue::Scalar(20.0)),
        ],
    ));
    assert!((v - 5.0).abs() < 1e-8, "100/20=5, got {v}");
}

#[test]
fn adenylate_energy_charge() {
    let v = scalar(run_bio(
        "adenylate_energy_charge",
        vec![
            ("atp", ParameterValue::Scalar(3.0)),
            ("adp", ParameterValue::Scalar(1.0)),
            ("amp", ParameterValue::Scalar(0.5)),
        ],
    ));
    let expected = (3.0 + 0.5) / (3.0 + 1.0 + 0.5);
    assert!(
        (v - expected).abs() < 1e-8,
        "EC=(ATP+0.5ADP)/(ATP+ADP+AMP), got {v}"
    );
}

#[test]
fn glycolysis_net_atp() {
    let v = scalar(run_bio(
        "glycolysis_net_atp",
        vec![("glucose", ParameterValue::Scalar(1.0))],
    ));
    assert!((v - 2.0).abs() < 1e-6, "net ATP per glucose = 2, got {v}");
}
