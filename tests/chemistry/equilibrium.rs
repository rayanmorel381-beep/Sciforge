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
fn equilibrium_constant_from_gibbs() {
    let v = scalar(run_chem(
        "equilibrium_constant_from_gibbs",
        vec![
            ("delta_g", ParameterValue::Scalar(0.0)),
            ("t", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "dG=0 => K=1, got {v}");
}

#[test]
fn reaction_quotient() {
    let v = scalar(run_chem(
        "reaction_quotient",
        vec![
            ("products", ParameterValue::Matrix(vec![vec![2.0, 1.0]])),
            ("reactants", ParameterValue::Matrix(vec![vec![1.0, 1.0]])),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-8, "Q=(2^1)/(1^1)=2, got {v}");
}

#[test]
fn vant_hoff() {
    let v = scalar(run_chem(
        "vant_hoff",
        vec![
            ("k1", ParameterValue::Scalar(1.0)),
            ("t1", ParameterValue::Scalar(298.0)),
            ("t2", ParameterValue::Scalar(298.0)),
            ("delta_h", ParameterValue::Scalar(50000.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "same T => K2=K1, got {v}");
}
