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
fn territory_size() {
    let v = scalar(run_bio(
        "territory_size",
        vec![
            ("body_mass", ParameterValue::Scalar(10.0)),
            ("scaling_exponent", ParameterValue::Scalar(1.0)),
            ("constant", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 10.0).abs() < 1e-6, "territory = c*M^b = 10, got {v}");
}

#[test]
fn hamilton_relatedness_benefit() {
    let o = run_bio(
        "hamilton_relatedness_benefit",
        vec![
            ("relatedness", ParameterValue::Scalar(0.5)),
            ("benefit", ParameterValue::Scalar(10.0)),
            ("cost", ParameterValue::Scalar(3.0)),
        ],
    );
    match o {
        RunOutput::Boolean(v) => assert!(v, "r*B > C => altruism favored"),
        other => panic!("expected Boolean, got {other:?}"),
    }
}

#[test]
fn ideal_free_distribution() {
    let o = run_bio(
        "ideal_free_distribution",
        vec![
            ("resource", ParameterValue::Vector(vec![100.0, 50.0])),
            ("total_individuals", ParameterValue::Scalar(30.0)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "should distribute individuals"),
        other => panic!("expected Vector, got {other:?}"),
    }
}

#[test]
fn hawk_dove_payoff() {
    let o = run_bio(
        "hawk_dove_payoff",
        vec![
            ("v", ParameterValue::Scalar(10.0)),
            ("c", ParameterValue::Scalar(20.0)),
            ("hawk_freq", ParameterValue::Scalar(0.5)),
        ],
    );
    match o {
        RunOutput::Pair(a, b) => {
            assert!(a.is_finite() && b.is_finite(), "payoffs should be finite");
        }
        other => panic!("expected Pair, got {other:?}"),
    }
}
