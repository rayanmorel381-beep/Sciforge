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
fn conjugate_gradient() {
    let o = run_maths(
        "conjugate_gradient",
        vec![
            (
                "a",
                ParameterValue::Sparse {
                    rows: 2,
                    cols: 2,
                    row_ptr: vec![0, 2, 4],
                    col_idx: vec![0, 1, 0, 1],
                    values: vec![4.0, 1.0, 1.0, 3.0],
                },
            ),
            ("b", ParameterValue::Vector(vec![1.0, 2.0])),
            ("tol", ParameterValue::Scalar(1e-10)),
            ("max_iter", ParameterValue::Integer(100)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert_eq!(v.len(), 2, "CG should return solution vector"),
        other => panic!("expected Vector, got {other:?}"),
    }
}

#[test]
fn jacobi_iterate() {
    let o = run_maths(
        "jacobi_iterate",
        vec![
            (
                "a",
                ParameterValue::Sparse {
                    rows: 2,
                    cols: 2,
                    row_ptr: vec![0, 2, 4],
                    col_idx: vec![0, 1, 0, 1],
                    values: vec![4.0, 1.0, 1.0, 3.0],
                },
            ),
            ("b", ParameterValue::Vector(vec![1.0, 2.0])),
            ("tol", ParameterValue::Scalar(1e-8)),
            ("max_iter", ParameterValue::Integer(200)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert_eq!(v.len(), 2, "Jacobi should return solution vector"),
        other => panic!("expected Vector, got {other:?}"),
    }
}

#[test]
fn sparse_add() {
    let o = run_maths(
        "sparse_add",
        vec![
            (
                "a",
                ParameterValue::Sparse {
                    rows: 2,
                    cols: 2,
                    row_ptr: vec![0, 1, 2],
                    col_idx: vec![0, 1],
                    values: vec![1.0, 2.0],
                },
            ),
            (
                "b",
                ParameterValue::Sparse {
                    rows: 2,
                    cols: 2,
                    row_ptr: vec![0, 1, 2],
                    col_idx: vec![0, 1],
                    values: vec![3.0, 4.0],
                },
            ),
        ],
    );
    match o {
        RunOutput::SparseOut { values, .. } => {
            assert!(!values.is_empty(), "sparse add should produce values");
        }
        other => panic!("expected SparseOut, got {other:?}"),
    }
}
