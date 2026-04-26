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
fn ode_euler() {
    let out = run_maths(
        "euler_step",
        vec![
            ("pos", ParameterValue::Vector(vec![0.0, 0.0, 0.0])),
            ("vel", ParameterValue::Vector(vec![1.0, 0.0, 0.0])),
            ("accel", ParameterValue::Vector(vec![0.0, 0.0, -9.81])),
            ("dt", ParameterValue::Scalar(0.01)),
        ],
    );
    match out {
        RunOutput::Matrix(m) => {
            assert_eq!(m.len(), 2, "euler_step returns [pos, vel] rows");
            assert_eq!(m[0].len(), 3);
        }
        other => panic!("expected Matrix, got {other:?}"),
    }
}
