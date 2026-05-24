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
fn lotka_volterra_competition() {
    let out = run_bio(
        "lotka_volterra_competition",
        vec![
            ("n1", ParameterValue::Scalar(50.0)),
            ("n2", ParameterValue::Scalar(20.0)),
            ("r1", ParameterValue::Scalar(0.5)),
            ("r2", ParameterValue::Scalar(0.3)),
            ("k1", ParameterValue::Scalar(100.0)),
            ("k2", ParameterValue::Scalar(80.0)),
            ("alpha12", ParameterValue::Scalar(0.5)),
            ("alpha21", ParameterValue::Scalar(0.4)),
        ],
    );
    match out {
        RunOutput::Pair(dn1, dn2) => {
            assert!(dn1.is_finite() && dn2.is_finite(), "dN/dt should be finite");
        }
        _ => panic!("expected Pair, got {out:?}"),
    }
}

#[test]
fn lotka_volterra_predator_prey() {
    let out = run_bio(
        "lotka_volterra_predator_prey",
        vec![
            ("prey", ParameterValue::Scalar(100.0)),
            ("predator", ParameterValue::Scalar(10.0)),
            ("r", ParameterValue::Scalar(0.1)),
            ("a", ParameterValue::Scalar(0.02)),
            ("b", ParameterValue::Scalar(0.01)),
            ("m", ParameterValue::Scalar(0.3)),
        ],
    );
    match out {
        RunOutput::Pair(dp, dd) => {
            assert!(dp.is_finite() && dd.is_finite());
        }
        _ => panic!("expected Pair, got {out:?}"),
    }
}

#[test]
fn carrying_capacity_from_resources() {
    let v = scalar(run_bio(
        "carrying_capacity_from_resources",
        vec![
            ("resource", ParameterValue::Scalar(1000.0)),
            ("consumption_per_capita", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-8, "K = R/c = 100, got {v}");
}

#[test]
fn cell_growth_logistic() {
    let out = run_bio(
        "cell_growth_logistic",
        vec![
            ("n", ParameterValue::Scalar(50.0)),
            ("r", ParameterValue::Scalar(0.1)),
            ("k", ParameterValue::Scalar(1000.0)),
            ("dt", ParameterValue::Scalar(1.0)),
            ("steps", ParameterValue::Integer(10)),
        ],
    );
    match out {
        RunOutput::Vector(v) => {
            assert!(!v.is_empty(), "should return growth trajectory");
            assert!(
                *v.last().unwrap() > 50.0,
                "population should grow when n < k"
            );
        }
        _ => panic!("expected Vector, got {out:?}"),
    }
}
