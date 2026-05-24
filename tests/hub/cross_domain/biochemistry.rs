use sciforge_hub::prelude::*;

fn run(name: &str, params: Vec<(&str, ParameterValue)>) -> RunOutput {
    let mut exp = Experiment::new(DomainType::Biochemistry, name);
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
fn michaelis_menten_rate() {
    let v = scalar(run(
        "michaelis_menten_rate",
        vec![
            ("vmax", ParameterValue::Scalar(100.0)),
            ("substrate", ParameterValue::Scalar(50.0)),
            ("km", ParameterValue::Scalar(50.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-6, "V=Vmax*S/(Km+S)=50, got {v}");
}

#[test]
fn henderson_hasselbalch() {
    let v = scalar(run(
        "henderson_hasselbalch",
        vec![
            ("pka", ParameterValue::Scalar(4.75)),
            ("base_conc", ParameterValue::Scalar(1.0)),
            ("acid_conc", ParameterValue::Scalar(1.0)),
        ],
    ));
    assert!((v - 4.75).abs() < 1e-6, "pH=pKa when [A-]=[HA], got {v}");
}

#[test]
fn gibbs_free_energy() {
    let v = scalar(run(
        "gibbs_free_energy",
        vec![
            ("delta_h", ParameterValue::Scalar(-100.0)),
            ("delta_s", ParameterValue::Scalar(0.0)),
            ("temperature", ParameterValue::Scalar(298.0)),
        ],
    ));
    assert!((v - (-100.0)).abs() < 1e-6, "G=H-TS=-100 when S=0, got {v}");
}

#[test]
fn ph_from_concentration() {
    let v = scalar(run(
        "ph_from_concentration",
        vec![("h_concentration", ParameterValue::Scalar(1e-7))],
    ));
    assert!((v - 7.0).abs() < 1e-6, "pH=-log10(1e-7)=7, got {v}");
}

#[test]
fn enzyme_turnover_number() {
    let v = scalar(run(
        "enzyme_turnover_number",
        vec![
            ("vmax", ParameterValue::Scalar(500.0)),
            ("enzyme_conc", ParameterValue::Scalar(10.0)),
        ],
    ));
    assert!((v - 50.0).abs() < 1e-6, "kcat=Vmax/[E]=50, got {v}");
}
