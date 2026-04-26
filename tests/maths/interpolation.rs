use sciforge::hub::prelude::*;

fn run_maths(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Maths, name);
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
fn linear_interpolate() {
    let v = scalar(run_maths(
        "linear_interpolate",
        vec![
            ("x", ParameterValue::Scalar(0.5)),
            ("xs", ParameterValue::Vector(vec![0.0, 1.0])),
            ("ys", ParameterValue::Vector(vec![0.0, 10.0])),
        ],
    ));
    assert!((v - 5.0).abs() < 1e-8, "linear interp at 0.5 => 5, got {v}");
}

#[test]
fn lagrange_interpolate() {
    let v = scalar(run_maths(
        "lagrange_interpolate",
        vec![
            (
                "points",
                ParameterValue::Matrix(vec![vec![1.0, 1.0], vec![2.0, 4.0], vec![3.0, 9.0]]),
            ),
            ("x", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!((v - 4.0).abs() < 1e-8, "L(2)=4 on [1,4,9], got {v}");
}

#[test]
fn hermite_interpolate() {
    let v = scalar(run_maths(
        "hermite_interpolate",
        vec![
            ("x0", ParameterValue::Scalar(0.0)),
            ("y0", ParameterValue::Scalar(0.0)),
            ("m0", ParameterValue::Scalar(1.0)),
            ("x1", ParameterValue::Scalar(1.0)),
            ("y1", ParameterValue::Scalar(1.0)),
            ("m1", ParameterValue::Scalar(1.0)),
            ("x", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(v.abs() < 1e-8, "H(0)=0, got {v}");
}
