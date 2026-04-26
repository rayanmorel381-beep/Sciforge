use sciforge::hub::prelude::*;

#[test]
fn first_derivative_x_squared() {
    let exp = Experiment::new(DomainType::Maths, "first_derivative")
        .param("f", ParameterValue::Vector(vec![0.0, 1.0, 4.0, 9.0, 16.0]))
        .param("dx", ParameterValue::Scalar(1.0));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Vector(v) = out {
        assert!(v.len() == 5);
        assert!((v[2] - 4.0).abs() < 1e-10, "d/dx(x²) at x=2 ≈ 4");
    } else {
        panic!("expected Vector, got {:?}", out);
    }
}

#[test]
fn second_derivative_x_squared() {
    let exp = Experiment::new(DomainType::Maths, "second_derivative")
        .param(
            "f",
            ParameterValue::Vector(vec![0.0, 1.0, 4.0, 9.0, 16.0, 25.0]),
        )
        .param("dx", ParameterValue::Scalar(1.0));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Vector(v) = out {
        for &val in &v[1..v.len() - 1] {
            assert!((val - 2.0).abs() < 1e-10, "d²/dx²(x²) = 2");
        }
    } else {
        panic!("expected Vector, got {:?}", out);
    }
}

#[test]
fn stat_binomial_pmf() {
    let exp = Experiment::new(DomainType::Maths, "stat_binomial_pmf")
        .param("k", ParameterValue::Scalar(3.0))
        .param("n", ParameterValue::Scalar(5.0))
        .param("p", ParameterValue::Scalar(0.5));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        let expected = 10.0 * 0.5_f64.powi(5);
        assert!((v - expected).abs() < 1e-10);
    } else {
        panic!("expected Scalar, got {:?}", out);
    }
}

#[test]
fn rising_factorial() {
    let exp = Experiment::new(DomainType::Maths, "rising_factorial")
        .param("x", ParameterValue::Scalar(3.0))
        .param("n", ParameterValue::Scalar(4.0));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!((v - 360.0).abs() < 1e-8, "3*4*5*6 = 360, got {v}");
    } else {
        panic!("expected Scalar, got {:?}", out);
    }
}

#[test]
fn falling_factorial() {
    let exp = Experiment::new(DomainType::Maths, "falling_factorial")
        .param("x", ParameterValue::Scalar(5.0))
        .param("n", ParameterValue::Scalar(3.0));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!((v - 60.0).abs() < 1e-8, "5*4*3 = 60, got {v}");
    } else {
        panic!("expected Scalar, got {:?}", out);
    }
}
