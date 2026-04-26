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
fn huckel_total_pi_energy() {
    let v = scalar(run_chem(
        "huckel_total_pi_energy",
        vec![
            (
                "eigenvalues",
                ParameterValue::Vector(vec![-2.0, -1.0, 1.0, 2.0]),
            ),
            (
                "occupations",
                ParameterValue::Vector(vec![2.0, 2.0, 0.0, 0.0]),
            ),
        ],
    ));
    assert!((v - (-6.0)).abs() < 1e-8, "Epi=2*(-2)+2*(-1)=-6, got {v}");
}

#[test]
fn lcao_bonding_energy() {
    let v = scalar(run_chem(
        "lcao_bonding_energy",
        vec![
            ("alpha", ParameterValue::Scalar(-11.4)),
            ("beta", ParameterValue::Scalar(-1.2)),
            ("overlap", ParameterValue::Scalar(0.0)),
        ],
    ));
    let expected = -11.4 + -1.2;
    assert!((v - expected).abs() < 1e-6, "E_bonding=alpha+beta, got {v}");
}

#[test]
fn mulliken_population() {
    let o = run_chem(
        "mulliken_population",
        vec![
            (
                "density",
                ParameterValue::Matrix(vec![vec![1.0, 0.3], vec![0.3, 1.0]]),
            ),
            (
                "overlap",
                ParameterValue::Matrix(vec![vec![1.0, 0.5], vec![0.5, 1.0]]),
            ),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "Mulliken population should not be empty"),
        other => panic!("expected Vector, got {other:?}"),
    }
}
