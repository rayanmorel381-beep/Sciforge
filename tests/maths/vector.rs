use sciforge_hub::prelude::*;

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
fn vec_lerp() {
    let o = run_maths(
        "vec_lerp",
        vec![
            ("a", ParameterValue::Vector(vec![0.0, 0.0, 0.0])),
            ("b", ParameterValue::Vector(vec![10.0, 10.0, 0.0])),
            ("t", ParameterValue::Scalar(0.5)),
        ],
    );
    match o {
        RunOutput::Triple(x, _y, _z) => assert!(
            (x - 5.0).abs() < 1e-8,
            "lerp(0.5) first component=5, got {x}"
        ),
        other => panic!("expected Triple, got {other:?}"),
    }
}

#[test]
fn vec_project() {
    let o = run_maths(
        "vec_project",
        vec![
            ("v", ParameterValue::Vector(vec![3.0, 4.0, 0.0])),
            ("onto", ParameterValue::Vector(vec![1.0, 0.0, 0.0])),
        ],
    );
    match o {
        RunOutput::Triple(x, _y, _z) => assert!(
            (x - 3.0).abs() < 1e-8,
            "proj of (3,4,0) on (1,0,0) x=3, got {x}"
        ),
        other => panic!("expected Triple, got {other:?}"),
    }
}

#[test]
fn vec_distance() {
    let v = scalar(run_maths(
        "vec_distance",
        vec![
            ("a", ParameterValue::Vector(vec![0.0, 0.0, 0.0])),
            ("b", ParameterValue::Vector(vec![3.0, 4.0, 0.0])),
        ],
    ));
    assert!(
        (v - 5.0).abs() < 1e-8,
        "distance (0,0,0)-(3,4,0)=5, got {v}"
    );
}
