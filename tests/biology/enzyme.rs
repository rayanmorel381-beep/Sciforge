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
fn michaelis_menten_half_vmax() {
    let v = scalar(run_bio(
        "michaelis_menten",
        vec![
            ("s", ParameterValue::Scalar(5.0)),
            ("vmax", ParameterValue::Scalar(100.0)),
            ("km", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-8, "v = Vmax/2 when [S]=Km, got {v}");
}

#[test]
fn michaelis_menten_saturation() {
    let v = scalar(run_bio(
        "michaelis_menten",
        vec![
            ("s", ParameterValue::Scalar(1e6)),
            ("vmax", ParameterValue::Scalar(100.0)),
            ("km", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!((v - 100.0).abs() < 0.01, "v ≈ Vmax at [S] >> Km, got {v}");
}

#[test]
fn hill_equation() {
    let v = scalar(run_bio(
        "hill_equation",
        vec![
            ("s", ParameterValue::Scalar(10.0)),
            ("vmax", ParameterValue::Scalar(100.0)),
            ("k", ParameterValue::Scalar(10.0)),
            ("n", ParameterValue::Scalar(2.0)),
        ],
    ));
    assert!(
        (v - 50.0).abs() < 1e-8,
        "v = Vmax/2 when [S]=K for Hill, got {v}"
    );
}

#[test]
fn competitive_inhibition_increases_apparent_km() {
    let v_no_inh = scalar(run_bio(
        "michaelis_menten",
        vec![
            ("s", ParameterValue::Scalar(5.0)),
            ("vmax", ParameterValue::Scalar(100.0)),
            ("km", ParameterValue::Scalar(5.0)),
        ],
    ));
    let v_inh = scalar(run_bio(
        "michaelis_menten_competitive",
        vec![
            ("s", ParameterValue::Scalar(5.0)),
            ("vmax", ParameterValue::Scalar(100.0)),
            ("km", ParameterValue::Scalar(5.0)),
            ("inhibitor", ParameterValue::Scalar(10.0)),
            ("ki", ParameterValue::Scalar(5.0)),
        ],
    ));
    assert!(
        v_inh < v_no_inh,
        "competitive inhibition should reduce rate"
    );
}

#[test]
fn lineweaver_burk() {
    let out = run_bio(
        "lineweaver_burk",
        vec![
            ("s", ParameterValue::Vector(vec![1.0, 2.0, 5.0, 10.0, 20.0])),
            (
                "v",
                ParameterValue::Vector(vec![16.67, 28.57, 50.0, 66.67, 80.0]),
            ),
        ],
    );
    match out {
        RunOutput::Pair(slope, intercept) => {
            assert!(slope > 0.0 && intercept > 0.0);
        }
        _ => panic!("expected Pair, got {out:?}"),
    }
}

#[test]
fn michaelis_menten_monotonic_with_substrate() {
    let run = |s: f64| {
        scalar(run_bio(
            "michaelis_menten",
            vec![
                ("s", ParameterValue::Scalar(s)),
                ("vmax", ParameterValue::Scalar(120.0)),
                ("km", ParameterValue::Scalar(8.0)),
            ],
        ))
    };
    let v1 = run(1.0);
    let v2 = run(8.0);
    let v3 = run(64.0);
    assert!(v1 < v2 && v2 < v3);
}

#[test]
fn hill_higher_cooperativity_reduces_rate_below_k() {
    let v_n1 = scalar(run_bio(
        "hill_equation",
        vec![
            ("s", ParameterValue::Scalar(5.0)),
            ("vmax", ParameterValue::Scalar(100.0)),
            ("k", ParameterValue::Scalar(10.0)),
            ("n", ParameterValue::Scalar(1.0)),
        ],
    ));
    let v_n4 = scalar(run_bio(
        "hill_equation",
        vec![
            ("s", ParameterValue::Scalar(5.0)),
            ("vmax", ParameterValue::Scalar(100.0)),
            ("k", ParameterValue::Scalar(10.0)),
            ("n", ParameterValue::Scalar(4.0)),
        ],
    ));
    assert!(v_n4 < v_n1);
}
