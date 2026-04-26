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
fn cable_equation_steady() {
    let v = scalar(run_bio(
        "cable_equation_steady",
        vec![
            ("v0", ParameterValue::Scalar(10.0)),
            ("x", ParameterValue::Scalar(0.0)),
            ("lambda", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 10.0).abs() < 1e-8, "V(0)=V0, got {v}");
}

#[test]
fn electrotonic_length() {
    let v = scalar(run_bio(
        "electrotonic_length",
        vec![
            ("physical_length", ParameterValue::Scalar(2.0)),
            ("space_constant", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-8, "L/lambda=2, got {v}");
}

#[test]
fn strength_duration_weiss() {
    let v = scalar(run_bio(
        "strength_duration_weiss",
        vec![
            ("rheobase", ParameterValue::Scalar(1.0)),
            ("chronaxie", ParameterValue::Scalar(0.5)),
            ("duration", ParameterValue::Scalar(0.5)),
        ],
    ));
    assert!((v - 2.0).abs() < 1e-8, "I=Irh*(1+c/d)=2, got {v}");
}

#[test]
fn impedance_tissue() {
    let v = scalar(run_bio(
        "impedance_tissue",
        vec![
            ("resistance", ParameterValue::Scalar(1000.0)),
            ("capacitance", ParameterValue::Scalar(1e-6)),
            ("frequency", ParameterValue::Scalar(1000.0)),
        ],
    ));
    assert!(
        v.is_finite() && v > 0.0,
        "impedance should be finite and positive, got {v}"
    );
}
