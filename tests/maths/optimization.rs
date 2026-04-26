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
fn quadratic_objective() {
    let v = scalar(run_maths(
        "quadratic_objective",
        vec![
            (
                "h",
                ParameterValue::Matrix(vec![vec![2.0, 0.0], vec![0.0, 2.0]]),
            ),
            ("c", ParameterValue::Vector(vec![0.0, 0.0])),
            ("x", ParameterValue::Vector(vec![0.0, 0.0])),
        ],
    ));
    assert!(v.abs() < 1e-8, "f([0,0])=0 at minimum, got {v}");
}

#[test]
fn project_box() {
    let o = run_maths(
        "project_box",
        vec![
            ("x", ParameterValue::Vector(vec![5.0])),
            ("lower", ParameterValue::Vector(vec![0.0])),
            ("upper", ParameterValue::Vector(vec![3.0])),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(
            (v[0] - 3.0).abs() < 1e-8,
            "project(5, [0,3])=3, got {}",
            v[0]
        ),
        other => panic!("expected Vector, got {other:?}"),
    }
}

#[test]
fn lagrangian() {
    let v = scalar(run_maths(
        "lagrangian",
        vec![
            ("f", ParameterValue::Scalar(10.0)),
            ("constraints", ParameterValue::Vector(vec![2.0])),
            ("lambdas", ParameterValue::Vector(vec![3.0])),
        ],
    ));
    assert!((v - 16.0).abs() < 1e-8, "L=f+lambda*g=10+3*2=16, got {v}");
}
