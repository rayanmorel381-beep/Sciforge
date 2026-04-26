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
fn tensor_reshape() {
    let out = run_maths(
        "tensor_reshape",
        vec![
            (
                "t",
                ParameterValue::Tensor {
                    data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                    shape: vec![6],
                },
            ),
            ("new_shape", ParameterValue::IntVector(vec![2, 3])),
        ],
    );
    match out {
        RunOutput::TensorOut { data, shape } => {
            assert_eq!(data.len(), 6, "reshaped tensor same element count");
            assert_eq!(shape, vec![2, 3]);
        }
        other => panic!("expected TensorOut, got {other:?}"),
    }
}

#[test]
fn tensor_contract() {
    let out = run_maths(
        "tensor_contract",
        vec![
            (
                "a",
                ParameterValue::Tensor {
                    data: vec![1.0, 2.0, 3.0],
                    shape: vec![3],
                },
            ),
            (
                "b",
                ParameterValue::Tensor {
                    data: vec![4.0, 5.0, 6.0],
                    shape: vec![3],
                },
            ),
            ("axis_a", ParameterValue::Integer(0)),
            ("axis_b", ParameterValue::Integer(0)),
        ],
    );
    match out {
        RunOutput::TensorOut { data, .. } => {
            assert!(!data.is_empty(), "contraction should produce data");
        }
        RunOutput::Scalar(v) => assert!((v - 32.0).abs() < 1e-8, "dot=32, got {v}"),
        other => panic!("expected TensorOut or Scalar, got {other:?}"),
    }
}

#[test]
fn tensor_determinant() {
    let v = scalar(run_maths(
        "tensor_determinant",
        vec![(
            "t",
            ParameterValue::Tensor {
                data: vec![1.0, 0.0, 0.0, 1.0],
                shape: vec![2, 2],
            },
        )],
    ));
    assert!((v - 1.0).abs() < 1e-8, "det(I)=1, got {v}");
}
