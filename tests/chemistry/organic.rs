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
fn degree_of_unsaturation() {
    let v = scalar(run_chem(
        "degree_of_unsaturation",
        vec![
            ("c", ParameterValue::Scalar(6.0)),
            ("h", ParameterValue::Scalar(6.0)),
            ("n", ParameterValue::Scalar(0.0)),
            ("halogens", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 4.0).abs() < 1e-8, "benzene DoU=4, got {v}");
}

#[test]
fn hammett_equation() {
    let v = scalar(run_chem(
        "hammett_equation",
        vec![
            ("rho", ParameterValue::Scalar(1.0)),
            ("sigma", ParameterValue::Scalar(0.0)),
            ("log_k0", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-8, "sigma=0 => k=k0, got {v}");
}

#[test]
fn partition_coefficient_log_p() {
    let v = scalar(run_chem(
        "partition_coefficient_log_p",
        vec![("fragments", ParameterValue::Vector(vec![2.0]))],
    ));
    assert!((v - 2.0).abs() < 1e-8, "logP=sum(fragments)=2, got {v}");
}
