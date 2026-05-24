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
fn limiting_reagent() {
    let o = run_chem(
        "limiting_reagent",
        vec![
            ("moles", ParameterValue::Vector(vec![2.0, 3.0])),
            ("coefficients", ParameterValue::Vector(vec![1.0, 2.0])),
        ],
    );
    match o {
        RunOutput::Integer(v) => assert!(v >= 0, "limiting reagent index >= 0, got {v}"),
        other => panic!("expected Integer, got {other:?}"),
    }
}

#[test]
fn theoretical_yield() {
    let v = scalar(run_chem(
        "theoretical_yield",
        vec![
            ("moles_limiting", ParameterValue::Scalar(2.0)),
            ("coeff_limiting", ParameterValue::Scalar(1.0)),
            ("coeff_product", ParameterValue::Scalar(1.0)),
            ("mw_product", ParameterValue::Scalar(44.0)),
        ],
    ));
    assert!((v - 88.0).abs() < 1e-6, "yield=2*1*44=88, got {v}");
}

#[test]
fn percent_yield() {
    let v = scalar(run_chem(
        "percent_yield",
        vec![
            ("actual", ParameterValue::Scalar(80.0)),
            ("theoretical", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 80.0).abs() < 1e-8, "yield=80%, got {v}");
}

#[test]
fn atom_economy() {
    let v = scalar(run_chem(
        "atom_economy",
        vec![
            ("mw_desired_product", ParameterValue::Scalar(100.0)),
            ("mw_all_products", ParameterValue::Vector(vec![200.0])),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-6, "AE=100/200*100=50%, got {v}");
}
