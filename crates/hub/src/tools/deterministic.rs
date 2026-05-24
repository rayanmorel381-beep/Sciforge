//! Deterministic computation tools: reproducible RNG (xoshiro256**),
//! numeric fingerprinting (FNV-1a), Kahan summation, and auditable context.

/// Pseudo-random number generator (xoshiro256**) with fixed seed.
#[derive(Debug, Clone)]
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// Creates an RNG from `seed` via SplitMix64 mixing.
    pub fn new(seed: u64) -> Self {
        let mut z = seed;
        let mut s = [0u64; 4];
        for si in &mut s {
            z = z.wrapping_add(0x9e3779b97f4a7c15);
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            *si = z ^ (z >> 31);
        }
        Self { s }
    }

    /// Generates the next 64-bit integer.
    pub fn next_u64(&mut self) -> u64 {
        let result = (self.s[1].wrapping_mul(5)).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }

    /// Generates a uniform float in `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Generates a uniform float in `[lo, hi)`.
    pub fn uniform(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.next_f64() * (hi - lo)
    }

    /// Generates a uniform index in `[0, n)`.
    pub fn next_usize(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    /// Generates a standard normal sample (Box-Muller).
    pub fn normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-300);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }

    /// Generates a normal sample with parameters `(mu, sigma)`.
    pub fn normal_params(&mut self, mu: f64, sigma: f64) -> f64 {
        mu + sigma * self.normal()
    }

    /// Fills `buf` with uniform values in `[0, 1)`.
    pub fn fill_uniform(&mut self, buf: &mut [f64]) {
        for v in buf.iter_mut() {
            *v = self.next_f64();
        }
    }

    /// Fills `buf` with standard normal values.
    pub fn fill_normal(&mut self, buf: &mut [f64]) {
        for v in buf.iter_mut() {
            *v = self.normal();
        }
    }

    /// Shuffles `data` in place (Fisher-Yates).
    pub fn shuffle<T>(&mut self, data: &mut [T]) {
        let n = data.len();
        for i in (1..n).rev() {
            let j = self.next_usize(i + 1);
            data.swap(i, j);
        }
    }

    /// Generates `n` uniform samples in `[lo, hi)`.
    pub fn sample_uniform(&mut self, n: usize, lo: f64, hi: f64) -> Vec<f64> {
        (0..n).map(|_| self.uniform(lo, hi)).collect()
    }

    /// Generates `n` normal samples with parameters `(mu, sigma)`.
    pub fn sample_normal(&mut self, n: usize, mu: f64, sigma: f64) -> Vec<f64> {
        (0..n).map(|_| self.normal_params(mu, sigma)).collect()
    }
}

/// Computes the FNV-1a fingerprint of a `f64` slice.
pub fn fingerprint(data: &[f64]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &v in data {
        let bytes = v.to_le_bytes();
        for &b in &bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

/// Computes the fingerprint of a single scalar.
pub fn fingerprint_scalar(v: f64) -> u64 {
    fingerprint(&[v])
}

/// Checks whether two fingerprints are equal.
pub fn fingerprints_match(a: u64, b: u64) -> bool {
    a == b
}

/// Reproducible context: fixed-seed RNG with audit trail.
#[derive(Debug, Clone)]
pub struct ReproducibleContext {
    /// Initial seed used to initialize the RNG.
    pub seed: u64,
    /// Pseudo-random number generator.
    pub rng: Rng,
    /// Log of operations performed.
    pub audit_trail: Vec<String>,
}

impl ReproducibleContext {
    /// Creates a new reproducible context with the given seed.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            rng: Rng::new(seed),
            audit_trail: Vec::new(),
        }
    }

    /// Appends an entry to the audit trail.
    pub fn log(&mut self, entry: &str) {
        self.audit_trail.push(entry.to_string());
    }

    /// Resets the RNG and clears the audit trail.
    pub fn reset(&mut self) {
        self.rng = Rng::new(self.seed);
        self.audit_trail.clear();
    }

    /// Creates a child context derived from the current seed and `sub_seed`.
    pub fn fork(&self, sub_seed: u64) -> Self {
        let combined = self
            .seed
            .wrapping_mul(0x9e3779b97f4a7c15)
            .wrapping_add(sub_seed);
        Self::new(combined)
    }

    /// Returns a human-readable summary of the audit trail.
    pub fn audit_summary(&self) -> String {
        let mut out = format!("Seed: {}\nSteps: {}\n", self.seed, self.audit_trail.len());
        for (i, entry) in self.audit_trail.iter().enumerate() {
            out.push_str(&format!("  [{}] {}\n", i + 1, entry));
        }
        out
    }
}

/// Checks that `computed` is close to `expected` within relative tolerance `tol`.
pub fn assert_reproducible(computed: f64, expected: f64, tol: f64) -> bool {
    if expected == 0.0 {
        computed.abs() <= tol
    } else {
        ((computed - expected) / expected).abs() <= tol
    }
}

/// Kahan compensated summation to reduce floating-point rounding errors.
pub fn kahan_sum(data: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    for &x in data {
        let y = x - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

/// Kahan compensated dot product.
pub fn kahan_dot(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    let n = a.len().min(b.len());
    for i in 0..n {
        let y = a[i] * b[i] - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}
