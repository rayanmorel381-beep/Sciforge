use sciforge_hub::prelude::*;

fn run_phys(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Physics, name);
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
fn snell() {
    let v = scalar(run_phys(
        "optics::refraction::snell",
        vec![
            ("n1", ParameterValue::Scalar(1.0)),
            ("n2", ParameterValue::Scalar(1.5)),
            (
                "theta1",
                ParameterValue::Scalar(std::f64::consts::FRAC_PI_6),
            ),
        ],
    ));
    assert!(
        v > 0.0 && v < std::f64::consts::FRAC_PI_6,
        "refracted angle smaller in denser medium, got {v}"
    );
}

#[test]
fn thin_lens_equation() {
    let v = scalar(run_phys(
        "thin_lens_equation",
        vec![
            ("focal", ParameterValue::Scalar(0.1)),
            ("object_dist", ParameterValue::Scalar(0.2)),
        ],
    ));
    assert!((v - 0.2).abs() < 1e-8, "2f object => 2f image, got {v}");
}

#[test]
fn brewster_angle() {
    let v = scalar(run_phys(
        "optics::refraction::brewster_angle",
        vec![
            ("n1", ParameterValue::Scalar(1.0)),
            ("n2", ParameterValue::Scalar(1.5)),
        ],
    ));
    assert!(
        v > 0.5 && v < 1.2,
        "Brewster angle for glass ~0.98 rad, got {v}"
    );
}

#[test]
fn rayleigh_criterion() {
    let v = scalar(run_phys(
        "rayleigh_criterion",
        vec![
            ("wavelength", ParameterValue::Scalar(500e-9)),
            ("aperture", ParameterValue::Scalar(0.01)),
        ],
    ));
    assert!(v > 0.0, "angular resolution should be positive, got {v}");
}
