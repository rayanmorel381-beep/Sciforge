use sciforge::hub::prelude::*;

#[test]
fn matrix_det_2x2() {
    let exp = Experiment::new(DomainType::Maths, "tensor_determinant").param(
        "t",
        ParameterValue::Tensor {
            data: vec![1.0, 2.0, 3.0, 4.0],
            shape: vec![2, 2],
        },
    );
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!((v - (-2.0)).abs() < 1e-10);
    } else {
        panic!("expected Scalar, got {:?}", out);
    }
}

#[test]
fn tensor_trace_identity() {
    let exp = Experiment::new(DomainType::Maths, "tensor_trace").param(
        "t",
        ParameterValue::Tensor {
            data: vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            shape: vec![3, 3],
        },
    );
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!((v - 3.0).abs() < 1e-12);
    } else {
        panic!("expected Scalar, got {:?}", out);
    }
}

#[test]
fn complex_exp_zero() {
    let exp = Experiment::new(DomainType::Maths, "complex_exp")
        .param("z", ParameterValue::Complex(0.0, 0.0));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Complex(re, im) = out {
        assert!((re - 1.0).abs() < 1e-12);
        assert!(im.abs() < 1e-12);
    } else {
        panic!("expected Complex, got {:?}", out);
    }
}

#[test]
fn complex_exp_i_pi() {
    let exp = Experiment::new(DomainType::Maths, "complex_exp")
        .param("z", ParameterValue::Complex(0.0, std::f64::consts::PI));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Complex(re, im) = out {
        assert!(
            (re + 1.0).abs() < 1e-10,
            "e^(iπ) real part should be -1, got {re}"
        );
        assert!(im.abs() < 1e-10, "e^(iπ) imag part should be 0, got {im}");
    } else {
        panic!("expected Complex");
    }
}

#[test]
fn levi_civita_antisymmetry() {
    let exp =
        Experiment::new(DomainType::Maths, "levi_civita").param("n", ParameterValue::Scalar(3.0));
    let runner = ExperimentRunner::new();
    let out = runner.run(&exp).unwrap();
    if let RunOutput::TensorOut { data, shape } = out {
        assert_eq!(shape, vec![3, 3, 3]);
        assert!((data[5] - 1.0).abs() < 1e-12, "ε_012 = 1");
        assert!((data[7] + 1.0).abs() < 1e-12, "ε_021 = -1");
    } else {
        panic!("expected TensorOut, got {out:?}");
    }
}
