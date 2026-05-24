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
fn fft_basic() {
    let out = run_maths(
        "fft",
        vec![(
            "input",
            ParameterValue::ComplexVector(vec![(1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (0.0, 0.0)]),
        )],
    );
    match out {
        RunOutput::ComplexVector(v) => assert!(!v.is_empty(), "FFT should return non-empty vector"),
        _ => panic!("expected ComplexVector output"),
    }
}

#[test]
fn power_spectrum() {
    let out = run_maths(
        "power_spectrum",
        vec![("input", ParameterValue::Vector(vec![1.0, 0.0, -1.0, 0.0]))],
    );
    match out {
        RunOutput::Vector(v) => assert!(!v.is_empty(), "power spectrum should be non-empty"),
        _ => panic!("expected Vector output"),
    }
}

#[test]
fn hann_window() {
    let out = run_maths("hann_window", vec![("n", ParameterValue::Integer(4))]);
    match out {
        RunOutput::Vector(v) => {
            assert_eq!(v.len(), 4, "window length should match n");
            assert!(v[0].abs() < 1e-8, "first sample ~0, got {}", v[0]);
        }
        _ => panic!("expected Vector output"),
    }
}
