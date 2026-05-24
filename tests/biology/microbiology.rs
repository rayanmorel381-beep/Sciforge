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
fn monod_growth() {
    let v = scalar(run_bio(
        "monod_growth",
        vec![
            ("mu_max", ParameterValue::Scalar(1.0)),
            ("s", ParameterValue::Scalar(10.0)),
            ("ks", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 0.5).abs() < 1e-8, "at S=Ks, mu=mu_max/2=0.5, got {v}");
}

#[test]
fn generation_time_bacteria() {
    let v = scalar(run_bio(
        "generation_time_bacteria",
        vec![("mu", ParameterValue::Scalar(0.5))],
    ));
    let expected = (2.0_f64).ln() / 0.5;
    assert!((v - expected).abs() < 1e-6, "g=ln2/mu, got {v}");
}

#[test]
fn chemostat_steady_state_biomass() {
    let v = scalar(run_bio(
        "chemostat_steady_state_biomass",
        vec![
            ("y", ParameterValue::Scalar(0.5)),
            ("s_in", ParameterValue::Scalar(10.0)),
            ("ks", ParameterValue::Scalar(1.0)),
            ("mu_max", ParameterValue::Scalar(1.0)),
            ("d", ParameterValue::Scalar(0.2)),
        ],
    ));
    assert!(
        v > 0.0,
        "biomass should be positive at steady state, got {v}"
    );
}

#[test]
fn biofilm_formation() {
    let o = run_bio(
        "biofilm_formation",
        vec![
            ("planktonic", ParameterValue::Scalar(100.0)),
            ("attachment_rate", ParameterValue::Scalar(0.1)),
            ("detachment_rate", ParameterValue::Scalar(0.05)),
            ("biofilm", ParameterValue::Scalar(1.0)),
            ("carrying_capacity", ParameterValue::Scalar(1000.0)),
        ],
    );
    match o {
        RunOutput::Pair(a, b) => {
            assert!(a.is_finite() && b.is_finite(), "both values finite");
        }
        other => panic!("expected Pair, got {other:?}"),
    }
}
