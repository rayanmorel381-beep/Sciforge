use sciforge::hub::prelude::*;

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
fn gibbs_free_energy() {
    let v = scalar(run_chem(
        "gibbs_free_energy",
        vec![
            ("delta_h", ParameterValue::Scalar(-100000.0)),
            ("delta_s", ParameterValue::Scalar(-200.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(v.is_finite(), "Gibbs free energy should be finite, got {v}");
}

#[test]
fn clausius_clapeyron() {
    let v = scalar(run_chem(
        "clausius_clapeyron",
        vec![
            ("p1", ParameterValue::Scalar(1.0)),
            ("t1", ParameterValue::Scalar(373.15)),
            ("t2", ParameterValue::Scalar(373.15)),
            ("delta_h_vap", ParameterValue::Scalar(40700.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "same T => same P, got {v}");
}

#[test]
fn entropy_change() {
    let v = scalar(run_chem(
        "entropy_change",
        vec![
            ("q_rev", ParameterValue::Scalar(1000.0)),
            ("t", ParameterValue::Scalar(500.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-8, "dS=Q/T=1000/500=2, got {v}");
}

#[test]
fn hess_law() {
    let v = scalar(run_chem(
        "hess_law",
        vec![
            (
                "enthalpies",
                ParameterValue::Vector(vec![-100.0, 50.0, -200.0]),
            ),
            ("coefficients", ParameterValue::Vector(vec![1.0, 1.0, 1.0])),
        ],
    ));
    assert!((v - (-250.0)).abs() < 1e-8, "sum=-250, got {v}");
}
