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
fn dispatch_complex_sqrt() {
    let out = run_maths(
        "complex_sqrt",
        vec![("z", ParameterValue::Complex(0.0, 4.0))],
    );
    if let RunOutput::Complex(re, im) = out {
        assert!((re * re - im * im).abs() < 1e-8);
        assert!((2.0 * re * im - 4.0).abs() < 1e-8);
    } else {
        panic!("expected Complex");
    }
}

#[test]
fn dispatch_vec_midpoint() {
    let out = run_maths(
        "vec_midpoint",
        vec![
            ("a", ParameterValue::Vector(vec![0.0, 0.0, 0.0])),
            ("b", ParameterValue::Vector(vec![4.0, 6.0, 2.0])),
        ],
    );
    if let RunOutput::Triple(x, y, z) = out {
        assert!((x - 2.0).abs() < 1e-12);
        assert!((y - 3.0).abs() < 1e-12);
        assert!((z - 1.0).abs() < 1e-12);
    } else {
        panic!("expected Triple, got {out:?}");
    }
}

#[test]
fn dispatch_gaussian_kernel_smooth() {
    let out = run_maths(
        "gaussian_kernel_smooth",
        vec![
            (
                "data",
                ParameterValue::Vector(vec![1.0, 2.0, 10.0, 2.0, 1.0]),
            ),
            ("xs", ParameterValue::Vector(vec![0.0, 1.0, 2.0, 3.0, 4.0])),
            ("bandwidth", ParameterValue::Scalar(1.0)),
        ],
    );
    if let RunOutput::Vector(v) = out {
        assert_eq!(v.len(), 5);
        assert!(v[2] < 10.0, "smoothing should reduce peak");
    } else {
        panic!("expected Vector");
    }
}

#[test]
fn dispatch_wave_packet_gaussian() {
    let v = scalar(run_maths(
        "wave_packet_gaussian",
        vec![
            ("x", ParameterValue::Scalar(0.0)),
            ("t", ParameterValue::Scalar(0.0)),
            ("k0", ParameterValue::Scalar(0.0)),
            ("sigma", ParameterValue::Scalar(1.0)),
            ("omega", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!(v > 0.0, "gaussian peak should be positive");
}

#[test]
fn dispatch_unknown_function_returns_error() {
    let exp = Experiment::new(DomainType::Maths, "unknown_maths_functions");
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}

#[test]
fn dispatch_missing_parameter_returns_error() {
    let exp = Experiment::new(DomainType::Maths, "mean")
        .param("weights", ParameterValue::Vector(vec![1.0, 1.0]));
    let result = ExperimentRunner::new().run(&exp);
    assert!(result.is_err());
}
