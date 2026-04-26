use sciforge::hub::prelude::*;

#[test]
fn mean_basic() {
    let exp = Experiment::new(DomainType::Maths, "mean")
        .param("data", ParameterValue::Vector(vec![2.0, 4.0, 6.0, 8.0]));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!((v - 5.0).abs() < 1e-12);
    } else {
        panic!("expected Scalar, got {:?}", out);
    }
}

#[test]
fn std_dev_basic() {
    let exp = Experiment::new(DomainType::Maths, "std_dev").param(
        "data",
        ParameterValue::Vector(vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]),
    );
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!(v > 1.5 && v < 3.0);
    } else {
        panic!("expected Scalar");
    }
}

#[test]
fn weighted_mean() {
    let exp = Experiment::new(DomainType::Maths, "weighted_mean")
        .param("data", ParameterValue::Vector(vec![10.0, 20.0, 30.0]))
        .param("weights", ParameterValue::Vector(vec![1.0, 1.0, 1.0]));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!((v - 20.0).abs() < 1e-12);
    } else {
        panic!("expected Scalar");
    }
}

#[test]
fn geometric_mean() {
    let exp = Experiment::new(DomainType::Maths, "geometric_mean")
        .param("data", ParameterValue::Vector(vec![4.0, 9.0]));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!((v - 6.0).abs() < 1e-10);
    } else {
        panic!("expected Scalar");
    }
}

#[test]
fn harmonic_mean() {
    let exp = Experiment::new(DomainType::Maths, "harmonic_mean")
        .param("data", ParameterValue::Vector(vec![1.0, 2.0, 4.0]));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        let expected = 3.0 / (1.0 + 0.5 + 0.25);
        assert!((v - expected).abs() < 1e-10);
    } else {
        panic!("expected Scalar");
    }
}

#[test]
fn erf_zero() {
    let exp = Experiment::new(DomainType::Maths, "erf").param("x", ParameterValue::Scalar(0.0));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Scalar(v) = out {
        assert!(v.abs() < 1e-6, "erf(0) ≈ 0, got {v}");
    } else {
        panic!("expected Scalar");
    }
}

#[test]
fn linear_regression_perfect() {
    let exp = Experiment::new(DomainType::Maths, "linear_regression")
        .param("x", ParameterValue::Vector(vec![1.0, 2.0, 3.0, 4.0]))
        .param("y", ParameterValue::Vector(vec![2.0, 4.0, 6.0, 8.0]));
    let out = ExperimentRunner::new().run(&exp).unwrap();
    if let RunOutput::Pair(slope, intercept) = out {
        assert!((slope - 2.0).abs() < 1e-10);
        assert!(intercept.abs() < 1e-10);
    } else {
        panic!("expected Pair(slope, intercept), got {:?}", out);
    }
}

#[test]
fn mean_permutation_invariant() {
    let a = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    let b = vec![9.0, 1.0, 7.0, 3.0, 5.0];
    let run = |data: Vec<f64>| {
        let exp =
            Experiment::new(DomainType::Maths, "mean").param("data", ParameterValue::Vector(data));
        match ExperimentRunner::new().run(&exp).unwrap() {
            RunOutput::Scalar(v) => v,
            out => panic!("expected Scalar, got {out:?}"),
        }
    };
    let ma = run(a);
    let mb = run(b);
    assert!((ma - mb).abs() < 1e-12);
}

#[test]
fn weighted_mean_scale_invariant_on_weights() {
    let data = vec![5.0, 15.0, 40.0];
    let run = |weights: Vec<f64>| {
        let exp = Experiment::new(DomainType::Maths, "weighted_mean")
            .param("data", ParameterValue::Vector(data.clone()))
            .param("weights", ParameterValue::Vector(weights));
        match ExperimentRunner::new().run(&exp).unwrap() {
            RunOutput::Scalar(v) => v,
            out => panic!("expected Scalar, got {out:?}"),
        }
    };
    let m1 = run(vec![1.0, 2.0, 3.0]);
    let m2 = run(vec![10.0, 20.0, 30.0]);
    assert!((m1 - m2).abs() < 1e-12);
}
