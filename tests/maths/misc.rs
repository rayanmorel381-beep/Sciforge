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

#[test]
fn quaternion_pure() {
    let out = run_maths(
        "quaternion_pure",
        vec![
            ("x", ParameterValue::Scalar(1.0)),
            ("y", ParameterValue::Scalar(2.0)),
            ("z", ParameterValue::Scalar(3.0)),
        ],
    );
    match out {
        RunOutput::Vector(v) => {
            assert_eq!(v.len(), 4);
            assert!((v[0]).abs() < 1e-8, "w=0 for pure quaternion");
        }
        _ => panic!("expected Vector, got {out:?}"),
    }
}

#[test]
fn poly_zero() {
    let out = run_maths("poly_zero", vec![]);
    match out {
        RunOutput::PolynomialOut(c) => assert!(c.iter().all(|&x| x.abs() < 1e-15)),
        _ => panic!("expected PolynomialOut, got {out:?}"),
    }
}

#[test]
fn poly_one() {
    let out = run_maths("poly_one", vec![]);
    match out {
        RunOutput::PolynomialOut(c) => {
            assert!(!c.is_empty());
            assert!((c[0] - 1.0).abs() < 1e-15);
        }
        _ => panic!("expected PolynomialOut, got {out:?}"),
    }
}

#[test]
fn tensor_zeros() {
    let out = run_maths(
        "tensor_zeros",
        vec![("shape", ParameterValue::IntVector(vec![2, 3]))],
    );
    match out {
        RunOutput::TensorOut { data, shape } => {
            assert_eq!(shape, vec![2, 3]);
            assert_eq!(data.len(), 6);
            assert!(data.iter().all(|&x| x.abs() < 1e-15));
        }
        _ => panic!("expected TensorOut, got {out:?}"),
    }
}
