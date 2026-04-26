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
fn polydispersity_index() {
    let v = scalar(run_chem(
        "polydispersity_index",
        vec![
            ("mw", ParameterValue::Scalar(100000.0)),
            ("mn", ParameterValue::Scalar(50000.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-8, "PDI=Mw/Mn=2, got {v}");
}

#[test]
fn carothers_equation() {
    let v = scalar(run_chem(
        "carothers_equation",
        vec![
            ("p", ParameterValue::Scalar(0.99)),
            ("f_avg", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 1e-6, "Xn=1/(1-p)=100, got {v}");
}

#[test]
fn flory_huggins_free_energy() {
    let v = scalar(run_chem(
        "flory_huggins_free_energy",
        vec![
            ("phi", ParameterValue::Scalar(0.5)),
            ("n1", ParameterValue::Scalar(100.0)),
            ("n2", ParameterValue::Scalar(100.0)),
            ("chi", ParameterValue::Scalar(0.4)),
            ("temperature", ParameterValue::Scalar(298.15)),
        ],
    ));
    assert!(v.is_finite(), "free energy should be finite, got {v}");
}
