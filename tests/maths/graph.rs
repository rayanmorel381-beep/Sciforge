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

#[test]
fn connected_components() {
    let o = run_maths(
        "connected_components",
        vec![(
            "adj",
            ParameterValue::IntMatrix(vec![vec![1], vec![0], vec![3], vec![2]]),
        )],
    );
    match o {
        RunOutput::Vector(v) => {
            assert_eq!(v.len(), 4, "should have label for each node");
            assert!(
                v[0] != v[2] || v[0] != v[3],
                "disconnected nodes should differ"
            );
        }
        other => panic!("expected Vector, got {other:?}"),
    }
}

#[test]
fn dijkstra() {
    let o = run_maths(
        "dijkstra",
        vec![
            (
                "adj",
                ParameterValue::Matrix(vec![vec![1.0, 1.0], vec![0.0, 1.0, 2.0, 2.0], vec![]]),
            ),
            ("start", ParameterValue::Integer(0)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "dijkstra should return distances"),
        other => panic!("expected Vector, got {other:?}"),
    }
}

#[test]
fn bfs() {
    let o = run_maths(
        "bfs",
        vec![
            (
                "adj",
                ParameterValue::IntMatrix(vec![vec![1], vec![0, 2], vec![1]]),
            ),
            ("start", ParameterValue::Integer(0)),
        ],
    );
    match o {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "BFS should return visited nodes"),
        other => panic!("expected Vector, got {other:?}"),
    }
}
