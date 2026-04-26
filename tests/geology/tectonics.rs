use sciforge::hub::prelude::*;

fn run_geo(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Geology, name);
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
fn plate_velocity() {
    let v = scalar(run_geo(
        "plate_velocity",
        vec![
            ("distance", ParameterValue::Scalar(1000e3)),
            ("time", ParameterValue::Scalar(1e6)),
        ],
    ));
    assert!((v - 1.0).abs() < 1e-6, "1000 km / 1 Myr = 1 m/yr?");
}

#[test]
fn geothermal_gradient() {
    let v = scalar(run_geo(
        "geothermal_gradient",
        vec![
            ("surface_temp", ParameterValue::Scalar(15.0)),
            ("depth", ParameterValue::Scalar(1000.0)),
            ("gradient", ParameterValue::Scalar(0.03)),
        ],
    ));
    assert!((v - 45.0).abs() < 1.0, "T = 15 + 0.03*1000 = 45, got {v}");
}

#[test]
fn heat_flow() {
    let v = scalar(run_geo(
        "heat_flow",
        vec![
            ("k", ParameterValue::Scalar(3.0)),
            ("dt_dz", ParameterValue::Scalar(0.03)),
        ],
    ));
    assert!(
        (v - (-0.09)).abs() < 1e-6,
        "q = -k·dT/dz = -3·0.03 = -0.09, got {v}"
    );
}

#[test]
fn mg_number() {
    let v = scalar(run_geo(
        "mg_number",
        vec![
            ("mgo", ParameterValue::Scalar(40.0)),
            ("feo", ParameterValue::Scalar(10.0)),
        ],
    ));
    let mg = 40.0 / 40.3;
    let fe = 10.0 / 71.85;
    let expected = mg / (mg + fe) * 100.0;
    assert!((v - expected).abs() < 0.1, "Mg# ≈ {expected}, got {v}");
}

#[test]
fn isostatic_equilibrium() {
    let v = scalar(run_geo(
        "isostatic_equilibrium",
        vec![
            ("rho_crust", ParameterValue::Scalar(2700.0)),
            ("rho_mantle", ParameterValue::Scalar(3300.0)),
            ("thickness", ParameterValue::Scalar(35e3)),
        ],
    ));
    assert!(v > 0.0, "compensation depth should be positive");
}

#[test]
fn viscosity_arrhenius() {
    let v1 = scalar(run_geo(
        "viscosity_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e10)),
            ("ea", ParameterValue::Scalar(300e3)),
            ("t", ParameterValue::Scalar(1500.0)),
        ],
    ));
    let v2 = scalar(run_geo(
        "viscosity_arrhenius",
        vec![
            ("a", ParameterValue::Scalar(1e10)),
            ("ea", ParameterValue::Scalar(300e3)),
            ("t", ParameterValue::Scalar(2000.0)),
        ],
    ));
    assert!(v2 < v1, "viscosity should decrease with temperature");
}
