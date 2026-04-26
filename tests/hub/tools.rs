use sciforge::hub::prelude::*;

#[test]
fn rng_deterministic() {
    let mut r1 = Rng::new(42);
    let mut r2 = Rng::new(42);
    for _ in 0..100 {
        assert_eq!(r1.next_u64(), r2.next_u64());
    }
}

#[test]
fn rng_different_seeds_differ() {
    let mut r1 = Rng::new(42);
    let mut r2 = Rng::new(43);
    let mut same = 0;
    for _ in 0..100 {
        if r1.next_u64() == r2.next_u64() {
            same += 1;
        }
    }
    assert!(
        same < 5,
        "different seeds should produce different sequences"
    );
}

#[test]
fn rng_uniform_range() {
    let mut r = Rng::new(123);
    for _ in 0..1000 {
        let v = r.uniform(10.0, 20.0);
        assert!((10.0..=20.0).contains(&v), "out of range: {v}");
    }
}

#[test]
fn rng_normal_distribution() {
    let mut r = Rng::new(7);
    let samples: Vec<f64> = (0..10_000).map(|_| r.normal()).collect();
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / samples.len() as f64;
    assert!(mean.abs() < 0.05, "normal mean ≈ 0, got {mean}");
    assert!((var - 1.0).abs() < 0.1, "normal variance ≈ 1, got {var}");
}

#[test]
fn fingerprint_deterministic() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let fp1 = fingerprint(&data);
    let fp2 = fingerprint(&data);
    assert_eq!(fp1, fp2);
}

#[test]
fn fingerprint_changes_with_data() {
    let fp1 = fingerprint(&[1.0, 2.0, 3.0]);
    let fp2 = fingerprint(&[1.0, 2.0, 3.0001]);
    assert_ne!(
        fp1, fp2,
        "different data should give different fingerprints"
    );
}

#[test]
fn kahan_sum_precision() {
    let mut data = vec![1.0];
    data.extend(std::iter::repeat_n(1e-16, 100_000));
    let naive: f64 = data.iter().sum();
    let kahan = kahan_sum(&data);
    let exact = 1.0 + 100_000.0 * 1e-16;
    assert!(
        (kahan - exact).abs() <= (naive - exact).abs(),
        "Kahan should be at least as good as naive sum"
    );
}

#[test]
fn kahan_dot_basic() {
    let a = [1.0, 2.0, 3.0];
    let b = [4.0, 5.0, 6.0];
    let result = kahan_dot(&a, &b);
    assert!((result - 32.0).abs() < 1e-10);
}

#[test]
fn reproducible_context_fork() {
    let ctx = ReproducibleContext::new(42);
    let fork1 = ctx.fork(1);
    let fork2 = ctx.fork(2);
    assert_ne!(
        fork1.seed, fork2.seed,
        "forks with different sub-seeds should differ"
    );
}

#[test]
fn arena_alloc_and_read() {
    let arena = Arena::new(100);
    let slice = arena.alloc_slice(5).unwrap();
    arena.write(&slice, &[10.0, 20.0, 30.0, 40.0, 50.0]);
    let data = arena.read(&slice);
    assert_eq!(data, &[10.0, 20.0, 30.0, 40.0, 50.0]);
}

#[test]
fn arena_capacity_limit() {
    let arena = Arena::new(10);
    assert!(arena.alloc_slice(10).is_some());
    assert!(arena.alloc_slice(1).is_none(), "should fail when full");
}

#[test]
fn arena_reset() {
    let arena = Arena::new(10);
    arena.alloc_slice(10).unwrap();
    arena.reset();
    assert!(
        arena.alloc_slice(10).is_some(),
        "should succeed after reset"
    );
}

#[test]
fn arena_matrix() {
    let arena = Arena::new(100);
    let mat = ArenaMatrix::new(&arena, 3, 4).unwrap();
    mat.set(&arena, 1, 2, 42.0);
    assert!((mat.get(&arena, 1, 2).unwrap() - 42.0).abs() < 1e-12);
}

#[test]
fn scratch_pool_independent() {
    let pool = ScratchPool::new(10, 3);
    pool.write(0, &[1.0; 10]);
    pool.write(1, &[2.0; 10]);
    let d0 = pool.read(0);
    let d1 = pool.read(1);
    assert!((d0[0] - 1.0).abs() < 1e-12);
    assert!((d1[0] - 2.0).abs() < 1e-12);
}

#[test]
fn format_scientific_output() {
    let s = format_scientific(6.674e-11, 4);
    assert!(s.contains("6.674"), "should contain 6.674, got {s}");
}

#[test]
fn linspace_basic() {
    let v = linspace(0.0, 1.0, 5);
    assert_eq!(v.len(), 5);
    assert!((v[0] - 0.0).abs() < 1e-12);
    assert!((v[4] - 1.0).abs() < 1e-12);
    assert!((v[2] - 0.5).abs() < 1e-12);
}

#[test]
fn logspace_basic() {
    let v = logspace(0.0, 3.0, 4);
    assert_eq!(v.len(), 4);
    assert!((v[0] - 1.0).abs() < 1e-10);
    assert!((v[3] - 1000.0).abs() < 1e-6);
}

#[test]
fn approx_equal_works() {
    assert!(approx_equal(1.0, 1.0 + 1e-13, 1e-12));
    assert!(!approx_equal(1.0, 1.1, 1e-3));
}
