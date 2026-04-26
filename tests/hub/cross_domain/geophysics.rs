use sciforge::hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Geophysics, name);
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
fn bouguer_anomaly() {
    let v = scalar(run(
        "bouguer_anomaly",
        vec![
            ("observed_gravity", ParameterValue::Scalar(9.81)),
            ("reference_gravity", ParameterValue::Scalar(9.80)),
            ("elevation", ParameterValue::Scalar(100.0)),
            ("slab_density", ParameterValue::Scalar(2670.0)),
        ],
    ));
    assert!(v.is_finite(), "Bouguer anomaly should be finite, got {v}");
}

#[test]
fn seismic_impedance_reflection() {
    let v = scalar(run(
        "seismic_impedance_reflection",
        vec![
            ("rho1", ParameterValue::Scalar(2500.0)),
            ("v1", ParameterValue::Scalar(3000.0)),
            ("rho2", ParameterValue::Scalar(2500.0)),
            ("v2", ParameterValue::Scalar(3000.0)),
        ],
    ));
    assert!(v.abs() < 1e-10, "same impedance => R=0, got {v}");
}

#[test]
fn seismic_wave_attenuation() {
    let v = scalar(run(
        "seismic_wave_attenuation",
        vec![
            ("amplitude_0", ParameterValue::Scalar(1.0)),
            ("frequency", ParameterValue::Scalar(1.0)),
            ("travel_time", ParameterValue::Scalar(0.0)),
            ("quality_factor", ParameterValue::Scalar(100.0)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "A(t=0)=A0=1, got {v}");
}

#[test]
fn electromagnetic_skin_depth() {
    let v = scalar(run(
        "electromagnetic_skin_depth",
        vec![
            ("resistivity", ParameterValue::Scalar(100.0)),
            ("frequency", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!(v > 0.0, "skin depth should be positive, got {v}");
}
