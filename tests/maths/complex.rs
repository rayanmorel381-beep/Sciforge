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
fn complex_exp() {
    let o = run_maths(
        "complex_exp",
        vec![("z", ParameterValue::Complex(0.0, 0.0))],
    );
    match o {
        RunOutput::Complex(re, _im) => assert!((re - 1.0).abs() < 1e-8, "exp(0)=1, got {re}"),
        other => panic!("expected Complex, got {other:?}"),
    }
}

#[test]
fn complex_sqrt() {
    let o = run_maths(
        "complex_sqrt",
        vec![("z", ParameterValue::Complex(4.0, 0.0))],
    );
    match o {
        RunOutput::Complex(re, im) => {
            assert!((re - 2.0).abs() < 1e-8, "sqrt(4).re=2, got {re}");
            assert!(im.abs() < 1e-8, "sqrt(4).im=0, got {im}");
        }
        other => panic!("expected Complex, got {other:?}"),
    }
}

#[test]
fn mandelbrot_iterate() {
    let o = run_maths(
        "mandelbrot_iterate",
        vec![
            ("c", ParameterValue::Complex(0.0, 0.0)),
            ("max_iter", ParameterValue::Integer(100)),
        ],
    );
    match o {
        RunOutput::Integer(v) => assert!(v >= 0, "iteration count >= 0, got {v}"),
        other => panic!("expected Integer, got {other:?}"),
    }
}
