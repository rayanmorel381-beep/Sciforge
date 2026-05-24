use sciforge_hub::benchmark::decode::decode;
use sciforge_hub::benchmark::encode::{encode, encoded_size};
use sciforge_hub::benchmark::engine::{BenchmarkEncodeError, bench};

#[test]
fn encode_decode_roundtrip() {
    let m = bench("roundtrip", "f64", 50, || 3 * 7);
    let size = encoded_size(&m).unwrap();
    let mut buf = vec![0u8; size];
    let written = encode(&m, &mut buf).unwrap();
    assert_eq!(written, size);
    let view = decode(&buf).unwrap();
    assert_eq!(view.experiment_name, "roundtrip");
    assert_eq!(view.precision, "f64");
    assert_eq!(view.iterations, 50);
}

#[test]
fn encode_buffer_too_small() {
    let m = bench("small", "f64", 10, || {});
    let mut buf = vec![0u8; 10];
    assert_eq!(
        encode(&m, &mut buf).unwrap_err(),
        BenchmarkEncodeError::BufferTooSmall
    );
}

#[test]
fn decode_empty() {
    assert_eq!(
        decode(&[]).unwrap_err(),
        BenchmarkEncodeError::InvalidFormat
    );
}

#[test]
fn decode_bad_magic() {
    let mut buf = vec![0u8; 200];
    buf[0..4].copy_from_slice(b"XXXX");
    assert_eq!(
        decode(&buf).unwrap_err(),
        BenchmarkEncodeError::InvalidFormat
    );
}

#[test]
fn encoded_size_matches_written() {
    let m = bench("sz", "f32", 20, || 1.0_f32.sqrt());
    let size = encoded_size(&m).unwrap();
    let mut buf = vec![0u8; size + 100];
    assert_eq!(encode(&m, &mut buf).unwrap(), size);
}

#[test]
fn roundtrip_preserves_floats() {
    let m = bench("floats", "f64", 200, || {});
    let size = encoded_size(&m).unwrap();
    let mut buf = vec![0u8; size];
    encode(&m, &mut buf).unwrap();
    let view = decode(&buf).unwrap();
    assert!((view.avg_time_ns - m.avg_time_ns).abs() < 1.0);
    assert!((view.min_time_ns - m.min_time_ns).abs() < 1.0);
    assert!((view.max_time_ns - m.max_time_ns).abs() < 1.0);
}
