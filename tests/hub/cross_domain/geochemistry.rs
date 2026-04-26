use sciforge::hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Geochemistry, name);
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
fn partition_coefficient() {
    let v = scalar(run(
        "partition_coefficient",
        vec![
            ("c_solid", ParameterValue::Scalar(10.0)),
            ("c_liquid", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-6, "Kd=Cs/Cl=2, got {v}");
}

#[test]
fn delta_notation() {
    let v = scalar(run(
        "delta_notation",
        vec![
            ("r_sample", ParameterValue::Scalar(0.0112372)),
            ("r_standard", ParameterValue::Scalar(0.0112372)),
        ],
    ));
    assert!(v.abs() < 1e-6, "same ratio => delta=0, got {v}");
}

#[test]
fn mixing_two_component() {
    let v = scalar(run(
        "mixing_two_component",
        vec![
            ("c1", ParameterValue::Scalar(10.0)),
            ("c2", ParameterValue::Scalar(20.0)),
            ("fraction_1", ParameterValue::Scalar(0.5)),
        ],
    ));
    assert!((v - 15.0).abs() < 1e-6, "mix(10,20,0.5)=15, got {v}");
}

#[test]
fn distribution_coefficient() {
    let v = scalar(run(
        "distribution_coefficient",
        vec![
            ("c_adsorbed", ParameterValue::Scalar(100.0)),
            ("c_solution", ParameterValue::Scalar(50.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-6, "Kd=100/50=2, got {v}");
}
