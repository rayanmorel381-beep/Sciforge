use sciforge_hub::prelude::*;

#[test]
fn pipeline_normalize_and_filter() {
    let data = vec![10.0, -5.0, 20.0, -3.0, 15.0];
    let pipe = Pipeline::new()
        .add_stage("normalize", normalize_stage())
        .add_stage("filter", filter_positive_stage());
    let result = pipe.execute(data).unwrap();
    for &v in &result {
        assert!(v >= 0.0, "all values should be non-negative after filter");
    }
    assert!(!result.is_empty());
}

#[test]
fn pipeline_scale() {
    let data = vec![1.0, 2.0, 3.0];
    let pipe = Pipeline::new().add_stage("scale", scale_stage(10.0));
    let result = pipe.execute(data).unwrap();
    assert!((result[0] - 10.0).abs() < 1e-12);
    assert!((result[1] - 20.0).abs() < 1e-12);
    assert!((result[2] - 30.0).abs() < 1e-12);
}

#[test]
fn pipeline_empty_input() {
    let data: Vec<f64> = vec![];
    let pipe = Pipeline::new().add_stage("normalize", normalize_stage());
    let result = pipe.execute(data);
    assert!(result.is_err(), "empty input should produce error");
}

#[test]
fn pipeline_chained_stages() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let pipe = Pipeline::new()
        .add_stage("normalize", normalize_stage())
        .add_stage("scale", scale_stage(100.0));
    let result = pipe.execute(data).unwrap();
    let max = result.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    assert!(
        (max - 100.0).abs() < 1e-8,
        "max should be 100 after normalize+scale"
    );
}
