use sciforge_hub::prelude::*;

fn run_bio(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biology, name);
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
fn gait_stride_length() {
    let v = scalar(run_bio(
        "gait_stride_length",
        vec![
            ("velocity", ParameterValue::Scalar(1.4)),
            ("cadence", ParameterValue::Scalar(120.0)),
        ],
    ));
    assert!(
        (v - 0.7).abs() < 1e-8,
        "v*60/cadence=1.4*60/120=0.7, got {v}"
    );
}

#[test]
fn ground_reaction_force() {
    let v = scalar(run_bio(
        "ground_reaction_force",
        vec![
            ("mass", ParameterValue::Scalar(70.0)),
            ("acceleration", ParameterValue::Scalar(0.0)),
        ],
    ));
    assert!((v - 686.7).abs() < 1.0, "F=m*(g+a)=70*9.81=686.7, got {v}");
}

#[test]
fn joint_moment() {
    let v = scalar(run_bio(
        "joint_moment",
        vec![
            ("force", ParameterValue::Scalar(100.0)),
            ("moment_arm", ParameterValue::Scalar(0.05)),
        ],
    ));
    assert!((v - 5.0).abs() < 1e-8, "M=F*d=5, got {v}");
}

#[test]
fn biomech_cardiac_output() {
    let v = scalar(run_bio(
        "biomech_cardiac_output",
        vec![
            ("stroke_volume", ParameterValue::Scalar(0.070)),
            ("heart_rate", ParameterValue::Scalar(70.0)),
        ],
    ));
    assert!((v - 4.9).abs() < 0.1, "CO=SV*HR=4.9, got {v}");
}
