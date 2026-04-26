# Mathematics — ChangeLog

## v0.0.3

| Category | Detail |
|---|---|
| Files modified | 8 |
| New functions | 10 (`integration/quadrature.rs` +8, `probability/distributions.rs` +2) |

### New functions

| Function | Signature | Description | Module |
|---|---|---|---|
| `clenshaw_curtis` | `fn clenshaw_curtis(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Clenshaw-Curtis quadrature | `integration::quadrature` |
| `tanh_sinh` | `fn tanh_sinh(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Tanh-sinh quadrature | `integration::quadrature` |
| `lobatto_quadrature` | `fn lobatto_quadrature(f: impl Fn(f64) → f64, a: f64, b: f64) → f64` | Lobatto quadrature with 5 points | `integration::quadrature` |
| `open_newton_cotes_4` | `fn open_newton_cotes_4(f: impl Fn(f64) → f64, a: f64, b: f64) → f64` | Open Newton-Cotes 4th order | `integration::quadrature` |
| `gauss_kronrod_15` | `fn gauss_kronrod_15(f: impl Fn(f64) → f64, a: f64, b: f64) → (f64, f64)` | 15-point Gauss-Kronrod with error estimate | `integration::quadrature` |
| `richardson_extrapolation` | `fn richardson_extrapolation(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize, order: usize) → f64` | Richardson extrapolation | `integration::quadrature` |
| `filon_sin` | `fn filon_sin(f: impl Fn(f64) → f64, a: f64, b: f64, omega: f64, n: usize) → f64` | Filon quadrature for oscillatory sine integrands | `integration::quadrature` |
| `filon_cos` | `fn filon_cos(f: impl Fn(f64) → f64, a: f64, b: f64, omega: f64, n: usize) → f64` | Filon quadrature for oscillatory cosine integrands | `integration::quadrature` |
| `entropy_discrete` | `fn entropy_discrete(probs: &[f64]) → f64` | Information entropy of discrete distribution | `probability::distributions` |
| `kl_divergence` | `fn kl_divergence(p: &[f64], q: &[f64]) → f64` | Kullback-Leibler divergence | `probability::distributions` |

No new functions in other submodules — see `testing.md` for test details.

---

## v0.0.2

| Category | Detail |
|---|---|
| Submodules | 17 (all rewritten) |

All 17 submodules rewritten from scratch.

---

## v0.0.1

Module: `src/maths/` — 17 submodules — 977 pub items

### 1️⃣ Complex Algebra — maths::complex — 69 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `Complex` | `struct { re: f64, im: f64 }` | Complex number representation | `complex::types` |
| `Complex::new` | `fn new(re: f64, im: f64) → Self` | Construct complex number from real and imaginary parts | `complex::types` |
| `Complex::from_polar` | `fn from_polar(r: f64, theta: f64) → Self` | Construct from polar form | `complex::types` |
| `Complex::i` | `fn i() → Self` | Return imaginary unit i | `complex::types` |
| `Complex::zero` | `fn zero() → Self` | Return complex zero | `complex::types` |
| `Complex::one` | `fn one() → Self` | Return complex one | `complex::types` |
| `Complex::conj` | `fn conj(&self) → Self` | Complex conjugate | `complex::types` |
| `Complex::norm_sq` | `fn norm_sq(&self) → f64` | Squared magnitude | `complex::types` |
| `Complex::norm` | `fn norm(&self) → f64` | Magnitude | `complex::types` |
| `Complex::arg` | `fn arg(&self) → f64` | Argument angle | `complex::types` |
| `Complex::inv` | `fn inv(&self) → Self` | Multiplicative inverse | `complex::types` |
| `Complex::exp` | `fn exp(&self) → Self` | Complex exponential | `complex::types` |
| `Complex::ln` | `fn ln(&self) → Self` | Complex natural logarithm | `complex::types` |
| `Complex::pow` | `fn pow(&self, n: Self) → Self` | Complex power to complex exponent | `complex::types` |
| `Complex::pow_f64` | `fn pow_f64(&self, n: f64) → Self` | Complex power to real exponent | `complex::types` |
| `Complex::sqrt` | `fn sqrt(&self) → Self` | Complex square root | `complex::types` |
| `Complex::sin` | `fn sin(&self) → Self` | Complex sine | `complex::types` |
| `Complex::cos` | `fn cos(&self) → Self` | Complex cosine | `complex::types` |
| `Complex::tan` | `fn tan(&self) → Self` | Complex tangent | `complex::types` |
| `Complex::sinh` | `fn sinh(&self) → Self` | Complex hyperbolic sine | `complex::types` |
| `Complex::cosh` | `fn cosh(&self) → Self` | Complex hyperbolic cosine | `complex::types` |
| `Complex::tanh` | `fn tanh(&self) → Self` | Complex hyperbolic tangent | `complex::types` |
| `Complex::scale` | `fn scale(&self, s: f64) → Self` | Scale by real scalar | `complex::types` |
| `Quaternion` | `struct { w: f64, x: f64, y: f64, z: f64 }` | Quaternion for 3D rotations | `complex::quaternion` |
| `Quaternion::new` | `fn new(w: f64, x: f64, y: f64, z: f64) → Self` | Construct quaternion | `complex::quaternion` |
| `Quaternion::identity` | `fn identity() → Self` | Identity quaternion | `complex::quaternion` |
| `Quaternion::zero` | `fn zero() → Self` | Zero quaternion | `complex::quaternion` |
| `Quaternion::pure` | `fn pure(x: f64, y: f64, z: f64) → Self` | Pure imaginary quaternion | `complex::quaternion` |
| `Quaternion::from_axis_angle` | `fn from_axis_angle(axis: [f64; 3], angle: f64) → Self` | From axis-angle representation | `complex::quaternion` |
| `Quaternion::from_euler` | `fn from_euler(roll: f64, pitch: f64, yaw: f64) → Self` | From Euler angles | `complex::quaternion` |
| `Quaternion::to_axis_angle` | `fn to_axis_angle(&self) → ([f64; 3], f64)` | Convert to axis-angle | `complex::quaternion` |
| `Quaternion::to_rotation_matrix` | `fn to_rotation_matrix(&self) → [[f64; 3]; 3]` | Convert to 3×3 rotation matrix | `complex::quaternion` |
| `Quaternion::conj` | `fn conj(&self) → Self` | Quaternion conjugate | `complex::quaternion` |
| `Quaternion::norm_sq` | `fn norm_sq(&self) → f64` | Squared norm | `complex::quaternion` |
| `Quaternion::norm` | `fn norm(&self) → f64` | Norm | `complex::quaternion` |
| `Quaternion::normalize` | `fn normalize(&self) → Self` | Normalize to unit length | `complex::quaternion` |
| `Quaternion::inv` | `fn inv(&self) → Self` | Multiplicative inverse | `complex::quaternion` |
| `Quaternion::rotate_vector` | `fn rotate_vector(&self, v: [f64; 3]) → [f64; 3]` | Rotate 3D vector | `complex::quaternion` |
| `Quaternion::dot` | `fn dot(&self, other: &Self) → f64` | Dot product | `complex::quaternion` |
| `Quaternion::slerp` | `fn slerp(&self, other: &Self, t: f64) → Self` | Spherical linear interpolation | `complex::quaternion` |
| `Quaternion::scale` | `fn scale(&self, s: f64) → Self` | Scale by real scalar | `complex::quaternion` |
| `Quaternion::exp` | `fn exp(&self) → Self` | Quaternion exponential | `complex::quaternion` |
| `Quaternion::ln` | `fn ln(&self) → Self` | Quaternion natural logarithm | `complex::quaternion` |
| `roots_of_unity` | `fn roots_of_unity(n: usize) → Vec<Complex>` | Nth roots of unity | `complex::ops` |
| `complex_polynomial_eval` | `fn complex_polynomial_eval(coeffs: &[Complex], z: Complex) → Complex` | Evaluate complex polynomial | `complex::ops` |
| `mandelbrot_iterate` | `fn mandelbrot_iterate(c: Complex, max_iter: u32) → u32` | Mandelbrot iteration count | `complex::ops` |
| `julia_iterate` | `fn julia_iterate(z0: Complex, c: Complex, max_iter: u32) → u32` | Julia set iteration count | `complex::ops` |
| `newton_fractal_step` | `fn newton_fractal_step(z: Complex, coeffs: &[Complex], deriv_coeffs: &[Complex]) → Complex` | Newton fractal step | `complex::ops` |
| `complex_matrix_mul` | `fn complex_matrix_mul(a: &[Vec<Complex>], b: &[Vec<Complex>]) → Vec<Vec<Complex>>` | Complex matrix multiplication | `complex::ops` |
| `complex_matrix_det` | `fn complex_matrix_det(m: &[Vec<Complex>]) → Complex` | Complex matrix determinant | `complex::ops` |
| `complex_exp` | `fn complex_exp(z: Complex) → Complex` | Complex exponential via Euler formula | `complex::ops` |
| `complex_log` | `fn complex_log(z: Complex) → Complex` | Complex logarithm | `complex::ops` |
| `complex_sin` | `fn complex_sin(z: Complex) → Complex` | Complex sine | `complex::ops` |
| `complex_cos` | `fn complex_cos(z: Complex) → Complex` | Complex cosine | `complex::ops` |
| `complex_tan` | `fn complex_tan(z: Complex) → Complex` | Complex tangent | `complex::ops` |
| `complex_sinh` | `fn complex_sinh(z: Complex) → Complex` | Complex hyperbolic sine | `complex::ops` |
| `complex_cosh` | `fn complex_cosh(z: Complex) → Complex` | Complex hyperbolic cosine | `complex::ops` |
| `complex_sqrt` | `fn complex_sqrt(z: Complex) → Complex` | Complex square root (principal branch) | `complex::ops` |
| `complex_power` | `fn complex_power(z: Complex, w: Complex) → Complex` | Complex to complex power | `complex::ops` |
| `complex_power_real` | `fn complex_power_real(z: Complex, n: f64) → Complex` | Complex to real power | `complex::ops` |
| `mobius_transform` | `fn mobius_transform(z: Complex, a: Complex, b: Complex, c: Complex, d: Complex) → Complex` | Möbius transformation | `complex::ops` |
| `bilinear_transform` | `fn bilinear_transform(s: Complex, t_sample: f64) → Complex` | Bilinear transform for filter design | `complex::ops` |
| `complex_contour_integral` | `fn complex_contour_integral(f: impl Fn(Complex) → Complex, path: &[Complex]) → Complex` | Numerical contour integral | `complex::ops` |
| `complex_conjugate_transpose` | `fn complex_conjugate_transpose(m: &[Vec<Complex>]) → Vec<Vec<Complex>>` | Conjugate transpose of complex matrix | `complex::ops` |
| `complex_matrix_trace` | `fn complex_matrix_trace(m: &[Vec<Complex>]) → Complex` | Trace of complex matrix | `complex::ops` |
| `complex_matrix_add` | `fn complex_matrix_add(a: &[Vec<Complex>], b: &[Vec<Complex>]) → Vec<Vec<Complex>>` | Element-wise complex matrix addition | `complex::ops` |
| `complex_matrix_scale` | `fn complex_matrix_scale(m: &[Vec<Complex>], s: Complex) → Vec<Vec<Complex>>` | Scale complex matrix | `complex::ops` |
| `complex_dft` | `fn complex_dft(input: &[Complex]) → Vec<Complex>` | Discrete Fourier transform | `complex::ops` |
| `complex_idft` | `fn complex_idft(input: &[Complex]) → Vec<Complex>` | Inverse discrete Fourier transform | `complex::ops` |

### 2️⃣ Tensor Operations — maths::tensor — 47 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `Tensor` | `struct { data: Vec<f64>, shape: Vec<usize>, strides: Vec<usize> }` | N-dimensional array with flexible indexing | `tensor::storage` |
| `Tensor::zeros` | `fn zeros(shape: &[usize]) → Self` | Tensor filled with zeros | `tensor::storage` |
| `Tensor::ones` | `fn ones(shape: &[usize]) → Self` | Tensor filled with ones | `tensor::storage` |
| `Tensor::from_vec` | `fn from_vec(shape: &[usize], data: Vec<f64>) → Self` | Tensor from flat vector and shape | `tensor::storage` |
| `Tensor::from_fn` | `fn from_fn(shape: &[usize], f: impl Fn(&[usize]) → f64) → Self` | Tensor from index function | `tensor::storage` |
| `Tensor::scalar` | `fn scalar(val: f64) → Self` | Rank-0 scalar tensor | `tensor::storage` |
| `Tensor::identity` | `fn identity(n: usize) → Self` | n×n identity matrix | `tensor::storage` |
| `Tensor::shape` | `fn shape(&self) → &[usize]` | Shape dimensions | `tensor::storage` |
| `Tensor::rank` | `fn rank(&self) → usize` | Number of dimensions | `tensor::storage` |
| `Tensor::size` | `fn size(&self) → usize` | Total element count | `tensor::storage` |
| `Tensor::data` | `fn data(&self) → &[f64]` | Underlying data reference | `tensor::storage` |
| `Tensor::get` | `fn get(&self, indices: &[usize]) → f64` | Element at multi-dimensional indices | `tensor::storage` |
| `Tensor::set` | `fn set(&mut self, indices: &[usize], value: f64)` | Set element at indices | `tensor::storage` |
| `Tensor::scale` | `fn scale(&self, s: f64) → Self` | Scale all elements | `tensor::storage` |
| `Tensor::map` | `fn map(&self, f: impl Fn(f64) → f64) → Self` | Apply function to each element | `tensor::storage` |
| `Tensor::elementwise` | `fn elementwise(&self, other: &Tensor, f: impl Fn(f64, f64) → f64) → Self` | Element-wise binary operation | `tensor::storage` |
| `Tensor::sum` | `fn sum(&self) → f64` | Sum all elements | `tensor::storage` |
| `Tensor::max` | `fn max(&self) → f64` | Maximum element | `tensor::storage` |
| `Tensor::min` | `fn min(&self) → f64` | Minimum element | `tensor::storage` |
| `Tensor::frobenius_norm` | `fn frobenius_norm(&self) → f64` | Frobenius norm | `tensor::storage` |
| `reshape` | `fn reshape(t: &Tensor, new_shape: &[usize]) → Tensor` | Reshape without copying data | `tensor::ops` |
| `transpose` | `fn transpose(t: &Tensor, axes: &[usize]) → Tensor` | Permute dimensions | `tensor::ops` |
| `contract` | `fn contract(a: &Tensor, b: &Tensor, axis_a: usize, axis_b: usize) → Tensor` | Contract along specified axes | `tensor::ops` |
| `outer` | `fn outer(a: &Tensor, b: &Tensor) → Tensor` | Outer product | `tensor::ops` |
| `kronecker` | `fn kronecker(a: &Tensor, b: &Tensor) → Tensor` | Kronecker product | `tensor::ops` |
| `levi_civita` | `fn levi_civita(n: usize) → Tensor` | Levi-Civita antisymmetric tensor | `tensor::ops` |
| `metric_raise` | `fn metric_raise(t: &Tensor, metric_inv: &Tensor, index: usize) → Tensor` | Raise index using inverse metric | `tensor::ops` |
| `metric_lower` | `fn metric_lower(t: &Tensor, metric: &Tensor, index: usize) → Tensor` | Lower index using metric | `tensor::ops` |
| `trace` | `fn trace(t: &Tensor) → f64` | Trace of square matrix | `tensor::linalg` |
| `matmul` | `fn matmul(a: &Tensor, b: &Tensor) → Tensor` | Matrix multiplication | `tensor::linalg` |
| `minor` | `fn minor(t: &Tensor, row: usize, col: usize) → Tensor` | Minor by removing row and column | `tensor::linalg` |
| `determinant` | `fn determinant(t: &Tensor) → f64` | Determinant via cofactor expansion | `tensor::linalg` |
| `inverse` | `fn inverse(t: &Tensor) → Option<Tensor>` | Matrix inverse via cofactor method | `tensor::linalg` |
| `eigenvalues_2x2` | `fn eigenvalues_2x2(t: &Tensor) → (f64, f64)` | 2×2 matrix eigenvalues | `tensor::linalg` |
| `lu_decompose` | `fn lu_decompose(t: &Tensor) → (Tensor, Tensor)` | LU decomposition | `tensor::linalg` |
| `solve` | `fn solve(a: &Tensor, b: &[f64]) → Vec<f64>` | Solve Ax=b via LU | `tensor::linalg` |
| `qr_decompose` | `fn qr_decompose(a: &Tensor) → (Tensor, Tensor)` | QR via Gram-Schmidt | `tensor::decompose` |
| `cholesky` | `fn cholesky(a: &Tensor) → Option<Tensor>` | Cholesky decomposition | `tensor::decompose` |
| `eigenvalues_qr` | `fn eigenvalues_qr(a: &Tensor, max_iter: usize, tol: f64) → Vec<f64>` | Eigenvalues via QR algorithm | `tensor::decompose` |
| `eigenvectors_qr` | `fn eigenvectors_qr(a: &Tensor, max_iter: usize, tol: f64) → (Vec<f64>, Tensor)` | Eigenvalues and eigenvectors via QR | `tensor::decompose` |
| `svd` | `fn svd(a: &Tensor) → (Tensor, Vec<f64>, Tensor)` | Singular value decomposition | `tensor::decompose` |
| `pseudoinverse` | `fn pseudoinverse(a: &Tensor) → Tensor` | Moore-Penrose pseudoinverse | `tensor::decompose` |
| `condition_number` | `fn condition_number(a: &Tensor) → f64` | Condition number from singular values | `tensor::decompose` |
| `matrix_exp` | `fn matrix_exp(a: &Tensor, terms: usize) → Tensor` | Matrix exponential via Taylor series | `tensor::decompose` |
| `matrix_norm_1` | `fn matrix_norm_1(a: &Tensor) → f64` | Matrix 1-norm (max column sum) | `tensor::decompose` |
| `matrix_norm_inf` | `fn matrix_norm_inf(a: &Tensor) → f64` | Matrix ∞-norm (max row sum) | `tensor::decompose` |
| `power_iteration` | `fn power_iteration(a: &Tensor, max_iter: usize, tol: f64) → (f64, Vec<f64>)` | Dominant eigenvalue and eigenvector | `tensor::decompose` |

### 3️⃣ Vector Mathematics — maths::vector — 71 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `Vec3` | `struct { x: f64, y: f64, z: f64 }` | Three-dimensional vector | `vector::types` |
| `Vec3::new` | `fn new(x: f64, y: f64, z: f64) → Self` | Construct 3D vector | `vector::types` |
| `Vec3::zero` | `fn zero() → Self` | Zero vector | `vector::types` |
| `Vec3::magnitude` | `fn magnitude(&self) → f64` | Vector length | `vector::types` |
| `Vec3::dot` | `fn dot(&self, rhs: &Self) → f64` | Dot product | `vector::types` |
| `Vec3::cross` | `fn cross(&self, rhs: &Self) → Self` | Cross product | `vector::types` |
| `Vec3::normalize` | `fn normalize(&self) → Self` | Unit vector | `vector::types` |
| `Vec3::scale` | `fn scale(&self, s: f64) → Self` | Scale by scalar | `vector::types` |
| `VecN` | `struct { components: Vec<f64> }` | N-dimensional vector | `vector::types` |
| `VecN::new` | `fn new(components: Vec<f64>) → Self` | Construct n-dimensional vector | `vector::types` |
| `VecN::zeros` | `fn zeros(n: usize) → Self` | Zero vector of dimension n | `vector::types` |
| `VecN::dim` | `fn dim(&self) → usize` | Vector dimension | `vector::types` |
| `VecN::dot` | `fn dot(&self, rhs: &Self) → f64` | Dot product | `vector::types` |
| `VecN::magnitude` | `fn magnitude(&self) → f64` | Vector length | `vector::types` |
| `VecN::normalize` | `fn normalize(&self) → Self` | Unit vector | `vector::types` |
| `VecN::scale` | `fn scale(&self, s: f64) → Self` | Scale by scalar | `vector::types` |
| `VecN::add_vec` | `fn add_vec(&self, rhs: &Self) → Self` | Vector addition | `vector::types` |
| `VecN::sub_vec` | `fn sub_vec(&self, rhs: &Self) → Self` | Vector subtraction | `vector::types` |
| `Particle` | `struct { position: Vec3, velocity: Vec3, mass: f64, charge: f64 }` | Point particle with kinematic properties | `vector::types` |
| `Particle::new` | `fn new(position: Vec3, velocity: Vec3, mass: f64, charge: f64) → Self` | Construct particle | `vector::types` |
| `Particle::kinetic_energy` | `fn kinetic_energy(&self) → f64` | Kinetic energy ½mv² | `vector::types` |
| `Particle::momentum` | `fn momentum(&self) → Vec3` | Linear momentum mv | `vector::types` |
| `lerp` | `fn lerp(a: &Vec3, b: &Vec3, t: f64) → Vec3` | Linear interpolation | `vector::ops` |
| `angle_between` | `fn angle_between(a: &Vec3, b: &Vec3) → f64` | Angle between vectors | `vector::ops` |
| `project` | `fn project(v: &Vec3, onto: &Vec3) → Vec3` | Project onto vector | `vector::ops` |
| `reject` | `fn reject(v: &Vec3, from: &Vec3) → Vec3` | Perpendicular component | `vector::ops` |
| `reflect` | `fn reflect(v: &Vec3, normal: &Vec3) → Vec3` | Reflect across normal | `vector::ops` |
| `triple_scalar` | `fn triple_scalar(a: &Vec3, b: &Vec3, c: &Vec3) → f64` | Scalar triple product | `vector::ops` |
| `triple_vector` | `fn triple_vector(a: &Vec3, b: &Vec3, c: &Vec3) → Vec3` | Vector triple product | `vector::ops` |
| `slerp` | `fn slerp(a: &Vec3, b: &Vec3, t: f64) → Vec3` | Spherical linear interpolation | `vector::ops` |
| `distance` | `fn distance(a: &Vec3, b: &Vec3) → f64` | Euclidean distance | `vector::ops` |
| `midpoint` | `fn midpoint(a: &Vec3, b: &Vec3) → Vec3` | Midpoint between points | `vector::ops` |
| `centroid` | `fn centroid(points: &[Vec3]) → Vec3` | Centroid of point cloud | `vector::ops` |
| `area_triangle` | `fn area_triangle(a: &Vec3, b: &Vec3, c: &Vec3) → f64` | Triangle area via cross product | `vector::ops` |
| `normal_triangle` | `fn normal_triangle(a: &Vec3, b: &Vec3, c: &Vec3) → Vec3` | Unit normal to triangle | `vector::ops` |
| `decompose_parallel_perpendicular` | `fn decompose_parallel_perpendicular(v: &Vec3, direction: &Vec3) → (Vec3, Vec3)` | Parallel and perpendicular decomposition | `vector::ops` |
| `rotate_around_axis` | `fn rotate_around_axis(v: &Vec3, axis: &Vec3, angle: f64) → Vec3` | Rodrigues rotation | `vector::ops` |
| `spherical_to_cartesian` | `fn spherical_to_cartesian(r: f64, theta: f64, phi: f64) → Vec3` | Spherical to Cartesian conversion | `vector::ops` |
| `cartesian_to_spherical` | `fn cartesian_to_spherical(v: &Vec3) → (f64, f64, f64)` | Cartesian to spherical conversion | `vector::ops` |
| `cylindrical_to_cartesian` | `fn cylindrical_to_cartesian(rho: f64, phi: f64, z: f64) → Vec3` | Cylindrical to Cartesian conversion | `vector::ops` |
| `cartesian_to_cylindrical` | `fn cartesian_to_cylindrical(v: &Vec3) → (f64, f64, f64)` | Cartesian to cylindrical conversion | `vector::ops` |
| `gravitational_force` | `fn gravitational_force(p1: &Particle, p2: &Particle) → Vec3` | Gravitational force between particles | `vector::fields` |
| `coulomb_force` | `fn coulomb_force(p1: &Particle, p2: &Particle) → Vec3` | Coulomb electrostatic force | `vector::fields` |
| `lorentz_force` | `fn lorentz_force(charge: f64, velocity: &Vec3, e_field: &Vec3, b_field: &Vec3) → Vec3` | Lorentz force on charged particle | `vector::fields` |
| `spring_force` | `fn spring_force(k: f64, rest_length: f64, p1: &Vec3, p2: &Vec3) → Vec3` | Hooke's law spring force | `vector::fields` |
| `drag_force` | `fn drag_force(cd: f64, rho: f64, area: f64, velocity: &Vec3) → Vec3` | Aerodynamic drag force | `vector::fields` |
| `dipole_field` | `fn dipole_field(moment: &Vec3, position: &Vec3) → Vec3` | Electric dipole field | `vector::fields` |
| `gradient_scalar_field` | `fn gradient_scalar_field(f: impl Fn(f64, f64, f64) → f64, x: f64, y: f64, z: f64, h: f64) → Vec3` | Gradient via finite differences | `vector::fields` |
| `divergence` | `fn divergence(fx: impl Fn(f64, f64, f64) → f64, fy: impl Fn(f64, f64, f64) → f64, fz: impl Fn(f64, f64, f64) → f64, x: f64, y: f64, z: f64, h: f64) → f64` | Divergence via finite differences | `vector::fields` |
| `curl` | `fn curl(fx: impl Fn(f64, f64, f64) → f64, fy: impl Fn(f64, f64, f64) → f64, fz: impl Fn(f64, f64, f64) → f64, x: f64, y: f64, z: f64, h: f64) → Vec3` | Curl via finite differences | `vector::fields` |
| `laplacian_scalar` | `fn laplacian_scalar(f: impl Fn(f64, f64, f64) → f64, x: f64, y: f64, z: f64, h: f64) → f64` | Laplacian via finite differences | `vector::fields` |
| `potential_energy_field` | `fn potential_energy_field(force: impl Fn(&Vec3) → Vec3, path_start: &Vec3, path_end: &Vec3, steps: usize) → f64` | Work done by force field along path | `vector::fields` |
| `streamline` | `fn streamline(vx: impl Fn(f64, f64, f64) → f64, vy: impl Fn(f64, f64, f64) → f64, vz: impl Fn(f64, f64, f64) → f64, start: &Vec3, dt: f64, steps: usize) → Vec<Vec3>` | Trace field streamline | `vector::fields` |
| `euler_step` | `fn euler_step(pos: &Vec3, vel: &Vec3, accel: &Vec3, dt: f64) → (Vec3, Vec3)` | Euler integration step | `vector::integrators` |
| `verlet_step` | `fn verlet_step(pos: &Vec3, vel: &Vec3, accel_fn: impl Fn(&Vec3) → Vec3, dt: f64) → (Vec3, Vec3)` | Verlet integration step | `vector::integrators` |
| `rk4_step` | `fn rk4_step(y: &Vec3, dydt: impl Fn(&Vec3) → Vec3, dt: f64) → Vec3` | 4th-order Runge-Kutta step | `vector::integrators` |
| `leapfrog_step` | `fn leapfrog_step(pos: &Vec3, vel_half: &Vec3, accel_fn: impl Fn(&Vec3) → Vec3, dt: f64) → (Vec3, Vec3)` | Leapfrog integration step | `vector::integrators` |
| `symplectic_euler_step` | `fn symplectic_euler_step(pos: &Vec3, vel: &Vec3, accel_fn: impl Fn(&Vec3) → Vec3, dt: f64) → (Vec3, Vec3)` | Symplectic Euler step | `vector::integrators` |
| `yoshida4_step` | `fn yoshida4_step(pos: &Vec3, vel: &Vec3, accel_fn: impl Fn(&Vec3) → Vec3, dt: f64) → (Vec3, Vec3)` | 4th-order Yoshida symplectic step | `vector::integrators` |
| `rk4_vec3_step` | `fn rk4_vec3_step(pos: &Vec3, vel: &Vec3, accel_fn: impl Fn(&Vec3, &Vec3) → Vec3, dt: f64) → (Vec3, Vec3)` | RK4 for position-velocity pairs | `vector::integrators` |
| `stormer_verlet_step` | `fn stormer_verlet_step(pos: &Vec3, pos_prev: &Vec3, accel_fn: impl Fn(&Vec3) → Vec3, dt: f64) → Vec3` | Störmer-Verlet step | `vector::integrators` |
| `forest_ruth_step` | `fn forest_ruth_step(pos: &Vec3, vel: &Vec3, accel_fn: impl Fn(&Vec3) → Vec3, dt: f64) → (Vec3, Vec3)` | Forest-Ruth 4th-order symplectic step | `vector::integrators` |
| `adaptive_verlet` | `fn adaptive_verlet(pos: &Vec3, vel: &Vec3, accel_fn: impl Fn(&Vec3) → Vec3, dt: f64, tol: f64) → (Vec3, Vec3, f64)` | Adaptive Verlet with error control | `vector::integrators` |
| `VectorFieldSim` | `struct { particles: Vec<Particle>, dt: f64 }` | N-body particle system simulator | `vector::sim` |
| `VectorFieldSim::new` | `fn new(dt: f64) → Self` | Create particle system | `vector::sim` |
| `VectorFieldSim::add_particle` | `fn add_particle(&mut self, p: Particle)` | Add particle to system | `vector::sim` |
| `VectorFieldSim::step_gravity` | `fn step_gravity(&mut self)` | Advance with gravity | `vector::sim` |
| `VectorFieldSim::step_em` | `fn step_em(&mut self, e_field: &Vec3, b_field: &Vec3)` | Advance with electromagnetic forces | `vector::sim` |
| `VectorFieldSim::total_kinetic_energy` | `fn total_kinetic_energy(&self) → f64` | Total kinetic energy | `vector::sim` |
| `VectorFieldSim::total_momentum` | `fn total_momentum(&self) → Vec3` | Total linear momentum | `vector::sim` |
| `VectorFieldSim::center_of_mass` | `fn center_of_mass(&self) → Vec3` | Center of mass | `vector::sim` |

### 4️⃣ Polynomial Mathematics — maths::polynomial — 44 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `Polynomial` | `struct { coeffs: Vec<f64> }` | Univariate polynomial in monomial basis | `polynomial::poly` |
| `Polynomial::new` | `fn new(coeffs: Vec<f64>) → Self` | Construct from coefficients | `polynomial::poly` |
| `Polynomial::zero` | `fn zero() → Self` | Zero polynomial | `polynomial::poly` |
| `Polynomial::one` | `fn one() → Self` | Constant polynomial 1 | `polynomial::poly` |
| `Polynomial::monomial` | `fn monomial(degree: usize, coeff: f64) → Self` | Single monomial term | `polynomial::poly` |
| `Polynomial::degree` | `fn degree(&self) → usize` | Polynomial degree | `polynomial::poly` |
| `Polynomial::eval` | `fn eval(&self, x: f64) → f64` | Evaluate via Horner's method | `polynomial::poly` |
| `Polynomial::derivative` | `fn derivative(&self) → Self` | Formal derivative | `polynomial::poly` |
| `Polynomial::integral` | `fn integral(&self, constant: f64) → Self` | Indefinite integral | `polynomial::poly` |
| `Polynomial::definite_integral` | `fn definite_integral(&self, a: f64, b: f64) → f64` | Definite integral over interval | `polynomial::poly` |
| `Polynomial::scale` | `fn scale(&self, s: f64) → Self` | Scale coefficients | `polynomial::poly` |
| `Polynomial::compose` | `fn compose(&self, other: &Self) → Self` | Compose two polynomials | `polynomial::poly` |
| `Polynomial::div_rem` | `fn div_rem(&self, divisor: &Self) → (Self, Self)` | Division with remainder | `polynomial::poly` |
| `Polynomial::gcd` | `fn gcd(&self, other: &Self) → Self` | Greatest common divisor | `polynomial::poly` |
| `Polynomial::from_roots` | `fn from_roots(roots: &[f64]) → Self` | Construct from roots | `polynomial::poly` |
| `newton_raphson` | `fn newton_raphson(f: impl Fn(f64) → f64, df: impl Fn(f64) → f64, x0: f64, tol: f64, max_iter: usize) → f64` | Newton-Raphson root finding | `polynomial::roots` |
| `bisection` | `fn bisection(f: impl Fn(f64) → f64, mut a: f64, mut b: f64, tol: f64, max_iter: usize) → f64` | Bisection method | `polynomial::roots` |
| `secant_method` | `fn secant_method(f: impl Fn(f64) → f64, mut x0: f64, mut x1: f64, tol: f64, max_iter: usize) → f64` | Secant method | `polynomial::roots` |
| `brent_method` | `fn brent_method(f: impl Fn(f64) → f64, mut a: f64, mut b: f64, tol: f64, max_iter: usize) → f64` | Brent's method | `polynomial::roots` |
| `polynomial_roots_real` | `fn polynomial_roots_real(poly: &Polynomial, search_range: (f64, f64), subdivisions: usize, tol: f64) → Vec<f64>` | All real roots by subdivision | `polynomial::roots` |
| `durand_kerner` | `fn durand_kerner(poly: &Polynomial, max_iter: usize, tol: f64) → Vec<(f64, f64)>` | All complex roots | `polynomial::roots` |
| `ridder_method` | `fn ridder_method(f: impl Fn(f64) → f64, mut a: f64, mut b: f64, tol: f64, max_iter: usize) → f64` | Ridder's method | `polynomial::roots` |
| `illinois_method` | `fn illinois_method(f: impl Fn(f64) → f64, mut a: f64, mut b: f64, tol: f64, max_iter: usize) → f64` | Illinois method (regula falsi variant) | `polynomial::roots` |
| `muller_method` | `fn muller_method(f: impl Fn(f64) → f64, x0: f64, x1: f64, x2: f64, tol: f64, max_iter: usize) → f64` | Müller's method | `polynomial::roots` |
| `fixed_point_iteration` | `fn fixed_point_iteration(g: impl Fn(f64) → f64, x0: f64, tol: f64, max_iter: usize) → f64` | Fixed-point iteration | `polynomial::roots` |
| `steffensen_method` | `fn steffensen_method(f: impl Fn(f64) → f64, x0: f64, tol: f64, max_iter: usize) → f64` | Steffensen with Aitken acceleration | `polynomial::roots` |
| `halley_method` | `fn halley_method(f: impl Fn(f64) → f64, df: impl Fn(f64) → f64, ddf: impl Fn(f64) → f64, x0: f64, tol: f64, max_iter: usize) → f64` | Halley's 3rd-order method | `polynomial::roots` |
| `laguerre_root` | `fn laguerre_root(poly: &Polynomial, x0: f64, tol: f64, max_iter: usize) → f64` | Laguerre method for polynomial roots | `polynomial::roots` |
| `polynomial_deflate` | `fn polynomial_deflate(poly: &Polynomial, root: f64) → Polynomial` | Divide by (x − root) | `polynomial::roots` |
| `find_all_roots_real` | `fn find_all_roots_real(poly: &Polynomial, tol: f64, max_iter: usize) → Vec<f64>` | All real roots via deflation | `polynomial::roots` |
| `legendre` | `fn legendre(n: usize) → Polynomial` | Nth Legendre polynomial | `polynomial::special` |
| `chebyshev_t` | `fn chebyshev_t(n: usize) → Polynomial` | Nth Chebyshev polynomial (first kind) | `polynomial::special` |
| `hermite` | `fn hermite(n: usize) → Polynomial` | Nth Hermite polynomial | `polynomial::special` |
| `laguerre` | `fn laguerre(n: usize) → Polynomial` | Nth Laguerre polynomial | `polynomial::special` |
| `bernstein_basis` | `fn bernstein_basis(n: usize, k: usize, t: f64) → f64` | Bernstein basis polynomial | `polynomial::special` |
| `chebyshev_u` | `fn chebyshev_u(n: usize) → Polynomial` | Nth Chebyshev polynomial (second kind) | `polynomial::special` |
| `gegenbauer` | `fn gegenbauer(n: usize, alpha: f64) → Polynomial` | Nth Gegenbauer polynomial | `polynomial::special` |
| `jacobi` | `fn jacobi(n: usize, alpha: f64, beta: f64) → Polynomial` | Nth Jacobi polynomial | `polynomial::special` |
| `associated_laguerre` | `fn associated_laguerre(n: usize, k: usize) → Polynomial` | Associated Laguerre polynomial | `polynomial::special` |
| `rising_factorial` | `fn rising_factorial(x: f64, n: usize) → f64` | Pochhammer symbol | `polynomial::special` |
| `falling_factorial` | `fn falling_factorial(x: f64, n: usize) → f64` | Falling factorial | `polynomial::special` |
| `bernoulli_polynomial` | `fn bernoulli_polynomial(n: usize) → Polynomial` | Nth Bernoulli polynomial | `polynomial::special` |
| `euler_polynomial` | `fn euler_polynomial(n: usize) → Polynomial` | Nth Euler polynomial | `polynomial::special` |
| `abel_polynomial` | `fn abel_polynomial(n: usize, a: f64) → Polynomial` | Nth Abel polynomial | `polynomial::special` |

### 5️⃣ Linear Algebra — maths::linalg — 57 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `lu_decomposition` | `fn lu_decomposition(a: &[Vec<f64>]) → (Vec<Vec<f64>>, Vec<Vec<f64>>)` | LU decomposition | `linalg::decomposition` |
| `lu_solve` | `fn lu_solve(l: &[Vec<f64>], u: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Solve via LU factors | `linalg::decomposition` |
| `qr_decomposition` | `fn qr_decomposition(a: &[Vec<f64>]) → (Vec<Vec<f64>>, Vec<Vec<f64>>)` | QR via Gram-Schmidt | `linalg::decomposition` |
| `cholesky_decomposition` | `fn cholesky_decomposition(a: &[Vec<f64>]) → Option<Vec<Vec<f64>>>` | Cholesky decomposition | `linalg::decomposition` |
| `svd_decomposition` | `fn svd_decomposition(a: &[Vec<f64>]) → (Vec<Vec<f64>>, Vec<f64>, Vec<Vec<f64>>)` | Singular value decomposition | `linalg::decomposition` |
| `matrix_rank` | `fn matrix_rank(a: &[Vec<f64>], tol: f64) → usize` | Rank via SVD | `linalg::decomposition` |
| `pseudoinverse` | `fn pseudoinverse(a: &[Vec<f64>]) → Vec<Vec<f64>>` | Moore-Penrose pseudoinverse | `linalg::decomposition` |
| `condition_number` | `fn condition_number(a: &[Vec<f64>]) → f64` | Condition number from SVD | `linalg::decomposition` |
| `determinant` | `fn determinant(a: &[Vec<f64>]) → f64` | Determinant via LU | `linalg::decomposition` |
| `inverse` | `fn inverse(a: &[Vec<f64>]) → Option<Vec<Vec<f64>>>` | Matrix inverse | `linalg::decomposition` |
| `rref` | `fn rref(a: &[Vec<f64>]) → Vec<Vec<f64>>` | Reduced row echelon form | `linalg::decomposition` |
| `null_space` | `fn null_space(a: &[Vec<f64>], tol: f64) → Vec<Vec<f64>>` | Null space basis | `linalg::decomposition` |
| `column_space` | `fn column_space(a: &[Vec<f64>], tol: f64) → Vec<Vec<f64>>` | Column space basis | `linalg::decomposition` |
| `schur_decomposition` | `fn schur_decomposition(a: &[Vec<f64>], max_iter: usize, tol: f64) → (Vec<Vec<f64>>, Vec<Vec<f64>>)` | Real Schur decomposition | `linalg::decomposition` |
| `hessenberg` | `fn hessenberg(a: &[Vec<f64>]) → Vec<Vec<f64>>` | Upper Hessenberg form | `linalg::decomposition` |
| `eigenvalues` | `fn eigenvalues(a: &[Vec<f64>], max_iter: usize, tol: f64) → Vec<f64>` | Eigenvalues via QR iteration | `linalg::eigenvalue` |
| `eigenvectors` | `fn eigenvectors(a: &[Vec<f64>], max_iter: usize, tol: f64) → (Vec<f64>, Vec<Vec<f64>>)` | Eigenvalues and eigenvectors | `linalg::eigenvalue` |
| `power_method` | `fn power_method(a: &[Vec<f64>], max_iter: usize, tol: f64) → (f64, Vec<f64>)` | Dominant eigenvalue/vector | `linalg::eigenvalue` |
| `inverse_power_method` | `fn inverse_power_method(a: &[Vec<f64>], sigma: f64, max_iter: usize, tol: f64) → (f64, Vec<f64>)` | Nearest eigenvalue to shift | `linalg::eigenvalue` |
| `rayleigh_quotient` | `fn rayleigh_quotient(a: &[Vec<f64>], x: &[f64]) → f64` | Rayleigh quotient | `linalg::eigenvalue` |
| `rayleigh_quotient_iteration` | `fn rayleigh_quotient_iteration(a: &[Vec<f64>], x0: &[f64], max_iter: usize, tol: f64) → (f64, Vec<f64>)` | Rayleigh quotient iteration | `linalg::eigenvalue` |
| `spectral_radius` | `fn spectral_radius(a: &[Vec<f64>], max_iter: usize, tol: f64) → f64` | Spectral radius | `linalg::eigenvalue` |
| `gershgorin_disks` | `fn gershgorin_disks(a: &[Vec<f64>]) → Vec<(f64, f64)>` | Gershgorin disk centers and radii | `linalg::eigenvalue` |
| `is_positive_definite` | `fn is_positive_definite(a: &[Vec<f64>]) → bool` | Positive definiteness check | `linalg::eigenvalue` |
| `is_symmetric` | `fn is_symmetric(a: &[Vec<f64>], tol: f64) → bool` | Symmetry check | `linalg::eigenvalue` |
| `matrix_norm_1` | `fn matrix_norm_1(a: &[Vec<f64>]) → f64` | Matrix 1-norm | `linalg::eigenvalue` |
| `matrix_norm_inf` | `fn matrix_norm_inf(a: &[Vec<f64>]) → f64` | Matrix infinity norm | `linalg::eigenvalue` |
| `matrix_norm_frobenius` | `fn matrix_norm_frobenius(a: &[Vec<f64>]) → f64` | Frobenius norm | `linalg::eigenvalue` |
| `solve_triangular_lower` | `fn solve_triangular_lower(l: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Forward substitution | `linalg::factorization` |
| `solve_triangular_upper` | `fn solve_triangular_upper(u: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Back substitution | `linalg::factorization` |
| `ldl_decomposition` | `fn ldl_decomposition(a: &[Vec<f64>]) → (Vec<Vec<f64>>, Vec<f64>)` | LDL^T decomposition | `linalg::factorization` |
| `tridiagonal_solve` | `fn tridiagonal_solve(lower: &[f64], diag: &[f64], upper: &[f64], rhs: &[f64]) → Vec<f64>` | Thomas algorithm for tridiagonal systems | `linalg::factorization` |
| `gauss_jordan` | `fn gauss_jordan(a: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Gauss-Jordan elimination | `linalg::factorization` |
| `cramer_rule` | `fn cramer_rule(a: &[Vec<f64>], b: &[f64]) → Option<Vec<f64>>` | Cramer's rule | `linalg::factorization` |
| `block_lu` | `fn block_lu(a: &[Vec<f64>], block_size: usize) → (Vec<Vec<f64>>, Vec<Vec<f64>>)` | Block LU factorization | `linalg::factorization` |
| `pivoted_lu` | `fn pivoted_lu(a: &[Vec<f64>]) → (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<usize>)` | Partial pivoting LU | `linalg::factorization` |
| `incomplete_cholesky` | `fn incomplete_cholesky(a: &[Vec<f64>]) → Vec<Vec<f64>>` | Incomplete Cholesky preconditioner | `linalg::factorization` |
| `band_solve` | `fn band_solve(bands: &[Vec<f64>], b: &[f64], lower_bw: usize, upper_bw: usize) → Vec<f64>` | Banded system solver | `linalg::factorization` |
| `householder_reflection` | `fn householder_reflection(x: &[f64]) → Vec<Vec<f64>>` | Householder reflector matrix | `linalg::factorization` |
| `givens_rotation` | `fn givens_rotation(a: f64, b: f64) → (f64, f64)` | Givens rotation parameters | `linalg::factorization` |
| `gram_schmidt` | `fn gram_schmidt(vectors: &[Vec<f64>]) → Vec<Vec<f64>>` | Gram-Schmidt orthogonalization | `linalg::factorization` |
| `modified_gram_schmidt` | `fn modified_gram_schmidt(vectors: &[Vec<f64>]) → Vec<Vec<f64>>` | Modified Gram-Schmidt (numerically stable) | `linalg::factorization` |
| `arnoldi_iteration` | `fn arnoldi_iteration(a: &[Vec<f64>], q1: &[f64], k: usize) → (Vec<Vec<f64>>, Vec<Vec<f64>>)` | Arnoldi iteration for Krylov subspace | `linalg::factorization` |
| `lanczos_iteration` | `fn lanczos_iteration(a: &[Vec<f64>], q1: &[f64], k: usize) → (Vec<f64>, Vec<f64>)` | Lanczos for symmetric eigenproblems | `linalg::factorization` |
| `matrix_exponential` | `fn matrix_exponential(a: &[Vec<f64>], terms: usize) → Vec<Vec<f64>>` | Matrix exponential via Taylor series | `linalg::factorization` |
| `least_squares` | `fn least_squares(a: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Ordinary least squares via normal equations | `linalg::least_squares` |
| `least_squares_qr` | `fn least_squares_qr(a: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Least squares via QR | `linalg::least_squares` |
| `least_squares_svd` | `fn least_squares_svd(a: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Least squares via SVD | `linalg::least_squares` |
| `weighted_least_squares` | `fn weighted_least_squares(a: &[Vec<f64>], b: &[f64], w: &[f64]) → Vec<f64>` | Weighted least squares | `linalg::least_squares` |
| `ridge_regression` | `fn ridge_regression(a: &[Vec<f64>], b: &[f64], lambda: f64) → Vec<f64>` | Ridge regression (L2 regularization) | `linalg::least_squares` |
| `tikhonov` | `fn tikhonov(a: &[Vec<f64>], b: &[f64], lambda: f64) → Vec<f64>` | Tikhonov regularization | `linalg::least_squares` |
| `total_least_squares` | `fn total_least_squares(a: &[Vec<f64>], b: &[f64]) → Vec<f64>` | Total least squares via SVD | `linalg::least_squares` |
| `iteratively_reweighted` | `fn iteratively_reweighted(a: &[Vec<f64>], b: &[f64], max_iter: usize, tol: f64) → Vec<f64>` | Iteratively reweighted least squares | `linalg::least_squares` |
| `constrained_least_squares` | `fn constrained_least_squares(a: &[Vec<f64>], b: &[f64], c: &[Vec<f64>], d: &[f64]) → Vec<f64>` | Equality-constrained least squares | `linalg::least_squares` |
| `truncated_svd_solve` | `fn truncated_svd_solve(a: &[Vec<f64>], b: &[f64], k: usize) → Vec<f64>` | Truncated SVD for rank-deficient systems | `linalg::least_squares` |
| `leverage_scores` | `fn leverage_scores(a: &[Vec<f64>]) → Vec<f64>` | Statistical leverage (hat matrix diagonal) | `linalg::least_squares` |
| `residual_analysis` | `fn residual_analysis(a: &[Vec<f64>], b: &[f64], x: &[f64]) → (Vec<f64>, f64, f64)` | Residual vector, norm, and R² | `linalg::least_squares` |

### 6️⃣ Sparse Matrices — maths::sparse — 38 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `SparseMatrix` | `struct { rows: usize, cols: usize, row_ptr: Vec<usize>, col_idx: Vec<usize>, values: Vec<f64> }` | Compressed sparse row matrix | `sparse::csr` |
| `SparseMatrix::new` | `fn new(rows: usize, cols: usize, row_ptr: Vec<usize>, col_idx: Vec<usize>, values: Vec<f64>) → Self` | Construct from CSR components | `sparse::csr` |
| `SparseMatrix::from_triplets` | `fn from_triplets(rows: usize, cols: usize, triplets: &[(usize, usize, f64)]) → Self` | Construct from (row, col, val) triplets | `sparse::csr` |
| `SparseMatrix::identity` | `fn identity(n: usize) → Self` | Sparse identity matrix | `sparse::csr` |
| `SparseMatrix::nnz` | `fn nnz(&self) → usize` | Number of stored nonzeros | `sparse::csr` |
| `SparseMatrix::get` | `fn get(&self, row: usize, col: usize) → f64` | Element access | `sparse::csr` |
| `SparseMatrix::mul_vec` | `fn mul_vec(&self, x: &[f64]) → Vec<f64>` | Sparse matrix-vector product | `sparse::csr` |
| `SparseMatrix::transpose` | `fn transpose(&self) → Self` | Sparse transpose | `sparse::csr` |
| `SparseMatrix::add` | `fn add(&self, other: &Self) → Self` | Sparse matrix addition | `sparse::csr` |
| `SparseMatrix::scale` | `fn scale(&self, s: f64) → Self` | Scale all values | `sparse::csr` |
| `SparseMatrix::matmul` | `fn matmul(&self, other: &Self) → Self` | Sparse matrix-matrix product | `sparse::csr` |
| `SparseMatrix::diagonal` | `fn diagonal(&self) → Vec<f64>` | Extract diagonal | `sparse::csr` |
| `SparseMatrix::frobenius_norm` | `fn frobenius_norm(&self) → f64` | Frobenius norm | `sparse::csr` |
| `SparseMatrix::to_dense` | `fn to_dense(&self) → Vec<Vec<f64>>` | Convert to dense matrix | `sparse::csr` |
| `sparse_kronecker` | `fn sparse_kronecker(a: &SparseMatrix, b: &SparseMatrix) → SparseMatrix` | Kronecker product | `sparse::ops` |
| `sparse_from_diagonals` | `fn sparse_from_diagonals(n: usize, offsets: &[i32], diags: &[Vec<f64>]) → SparseMatrix` | Construct from diagonals | `sparse::ops` |
| `sparse_add` | `fn sparse_add(a: &SparseMatrix, b: &SparseMatrix) → SparseMatrix` | Sparse addition | `sparse::ops` |
| `sparse_scale` | `fn sparse_scale(a: &SparseMatrix, s: f64) → SparseMatrix` | Sparse scaling | `sparse::ops` |
| `sparse_mat_mul` | `fn sparse_mat_mul(a: &SparseMatrix, b: &SparseMatrix) → SparseMatrix` | Sparse multiplication | `sparse::ops` |
| `sparse_trace` | `fn sparse_trace(a: &SparseMatrix) → f64` | Trace of sparse matrix | `sparse::ops` |
| `sparse_frobenius_norm` | `fn sparse_frobenius_norm(a: &SparseMatrix) → f64` | Frobenius norm | `sparse::ops` |
| `sparse_infinity_norm` | `fn sparse_infinity_norm(a: &SparseMatrix) → f64` | Infinity norm | `sparse::ops` |
| `sparse_one_norm` | `fn sparse_one_norm(a: &SparseMatrix) → f64` | 1-norm | `sparse::ops` |
| `sparse_diagonal` | `fn sparse_diagonal(a: &SparseMatrix) → Vec<f64>` | Extract diagonal | `sparse::ops` |
| `sparse_lower_triangular` | `fn sparse_lower_triangular(a: &SparseMatrix) → SparseMatrix` | Lower triangular part | `sparse::ops` |
| `sparse_upper_triangular` | `fn sparse_upper_triangular(a: &SparseMatrix) → SparseMatrix` | Upper triangular part | `sparse::ops` |
| `sparse_row_sum` | `fn sparse_row_sum(a: &SparseMatrix) → Vec<f64>` | Row sums | `sparse::ops` |
| `sparse_col_sum` | `fn sparse_col_sum(a: &SparseMatrix) → Vec<f64>` | Column sums | `sparse::ops` |
| `sparse_is_symmetric` | `fn sparse_is_symmetric(a: &SparseMatrix, tol: f64) → bool` | Symmetry check | `sparse::ops` |
| `conjugate_gradient` | `fn conjugate_gradient(a: &SparseMatrix, b: &[f64], x0: &[f64], tol: f64, max_iter: usize) → Vec<f64>` | CG for symmetric positive definite systems | `sparse::solvers` |
| `jacobi_iterate` | `fn jacobi_iterate(a: &SparseMatrix, b: &[f64], x: &[f64]) → Vec<f64>` | One Jacobi iteration | `sparse::solvers` |
| `gauss_seidel` | `fn gauss_seidel(a: &SparseMatrix, b: &[f64], x: &[f64]) → Vec<f64>` | One Gauss-Seidel iteration | `sparse::solvers` |
| `sparse_lu_solve` | `fn sparse_lu_solve(a: &SparseMatrix, b: &[f64]) → Vec<f64>` | Direct LU solve for sparse | `sparse::solvers` |
| `sor_iterate` | `fn sor_iterate(a: &SparseMatrix, b: &[f64], x: &[f64], omega: f64) → Vec<f64>` | Successive over-relaxation | `sparse::solvers` |
| `steepest_descent` | `fn steepest_descent(a: &SparseMatrix, b: &[f64], x0: &[f64], tol: f64, max_iter: usize) → Vec<f64>` | Steepest descent method | `sparse::solvers` |
| `bicgstab` | `fn bicgstab(a: &SparseMatrix, b: &[f64], x0: &[f64], tol: f64, max_iter: usize) → Vec<f64>` | BiCGSTAB for non-symmetric systems | `sparse::solvers` |
| `preconditioned_cg` | `fn preconditioned_cg(a: &SparseMatrix, b: &[f64], x0: &[f64], precond: &SparseMatrix, tol: f64, max_iter: usize) → Vec<f64>` | Preconditioned conjugate gradient | `sparse::solvers` |
| `gmres` | `fn gmres(a: &SparseMatrix, b: &[f64], x0: &[f64], tol: f64, max_iter: usize, restart: usize) → Vec<f64>` | GMRES with restart | `sparse::solvers` |

### 7️⃣ Integration — maths::integration — 48 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `adaptive_simpson` | `fn adaptive_simpson(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Adaptive Simpson's rule | `integration::adaptive` |
| `adaptive_trapezoid` | `fn adaptive_trapezoid(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Adaptive trapezoidal rule | `integration::adaptive` |
| `improper_integral_transform` | `fn improper_integral_transform(f: impl Fn(f64) → f64, a: f64, tol: f64) → f64` | Semi-infinite via substitution | `integration::adaptive` |
| `adaptive_gauss_kronrod` | `fn adaptive_gauss_kronrod(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Adaptive Gauss-Kronrod | `integration::adaptive` |
| `adaptive_midpoint` | `fn adaptive_midpoint(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Adaptive midpoint rule | `integration::adaptive` |
| `adaptive_boole` | `fn adaptive_boole(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Adaptive Boole's rule | `integration::adaptive` |
| `double_exponential` | `fn double_exponential(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Double exponential (tanh-sinh) | `integration::adaptive` |
| `cauchy_principal_value` | `fn cauchy_principal_value(f: impl Fn(f64) → f64, a: f64, b: f64, c: f64, n: usize) → f64` | Cauchy principal value integral | `integration::adaptive` |
| `improper_both_infinite` | `fn improper_both_infinite(f: impl Fn(f64) → f64, n: usize) → f64` | Doubly improper integral (−∞, +∞) | `integration::adaptive` |
| `improper_left_infinite` | `fn improper_left_infinite(f: impl Fn(f64) → f64, b: f64, n: usize) → f64` | Left-infinite integral (−∞, b] | `integration::adaptive` |
| `numerical_derivative_central` | `fn numerical_derivative_central(f: impl Fn(f64) → f64, x: f64, h: f64) → f64` | Central difference derivative | `integration::adaptive` |
| `numerical_second_derivative` | `fn numerical_second_derivative(f: impl Fn(f64) → f64, x: f64, h: f64) → f64` | Second derivative via central differences | `integration::adaptive` |
| `numerical_derivative_5point` | `fn numerical_derivative_5point(f: impl Fn(f64) → f64, x: f64, h: f64) → f64` | 5-point stencil derivative | `integration::adaptive` |
| `numerical_integral_cumulative` | `fn numerical_integral_cumulative(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → Vec<(f64, f64)>` | Cumulative integral | `integration::adaptive` |
| `monte_carlo_integrate` | `fn monte_carlo_integrate(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n: usize) → f64` | Monte Carlo integration | `integration::multidim` |
| `double_integral` | `fn double_integral(f: impl Fn(f64, f64) → f64, x_range: (f64, f64), y_range: impl Fn(f64) → (f64, f64), nx: usize, ny: usize) → f64` | Double integral with variable limits | `integration::multidim` |
| `triple_integral` | `fn triple_integral(f: impl Fn(f64, f64, f64) → f64, x_range: (f64, f64), y_range: impl Fn(f64) → (f64, f64), z_range: impl Fn(f64, f64) → (f64, f64), n: usize) → f64` | Triple integral with variable limits | `integration::multidim` |
| `stratified_monte_carlo` | `fn stratified_monte_carlo(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n_per_dim: usize) → f64` | Stratified Monte Carlo | `integration::multidim` |
| `halton_sequence` | `fn halton_sequence(index: usize, base: usize) → f64` | Halton quasi-random sequence | `integration::multidim` |
| `quasi_monte_carlo_halton` | `fn quasi_monte_carlo_halton(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n: usize) → f64` | Quasi-Monte Carlo via Halton | `integration::multidim` |
| `polar_integral` | `fn polar_integral(f: impl Fn(f64, f64) → f64, r_range: (f64, f64), theta_range: (f64, f64), nr: usize, ntheta: usize) → f64` | Polar coordinate integral | `integration::multidim` |
| `spherical_integral` | `fn spherical_integral(f: impl Fn(f64, f64, f64) → f64, r_range: (f64, f64), nr: usize, ntheta: usize, nphi: usize) → f64` | Spherical coordinate integral | `integration::multidim` |
| `cylindrical_integral` | `fn cylindrical_integral(f: impl Fn(f64, f64, f64) → f64, rho_range: (f64, f64), z_range: (f64, f64), n: usize) → f64` | Cylindrical coordinate integral | `integration::multidim` |
| `line_integral` | `fn line_integral(f: impl Fn(f64, f64) → f64, path_x: impl Fn(f64) → f64, path_y: impl Fn(f64) → f64, t_range: (f64, f64), n: usize) → f64` | Line integral along parametric path | `integration::multidim` |
| `surface_integral_parametric` | `fn surface_integral_parametric(f: impl Fn(f64, f64, f64) → f64, x: impl Fn(f64, f64) → f64, y: impl Fn(f64, f64) → f64, z: impl Fn(f64, f64) → f64, u_range: (f64, f64), v_range: (f64, f64), nu: usize, nv: usize) → f64` | Surface integral over parametric surface | `integration::multidim` |
| `LcgRng` | `struct { state: u64, a: u64, c: u64, m: u64 }` | Linear congruential generator | `integration::multidim` |
| `importance_sampling` | `fn importance_sampling(f: impl Fn(f64) → f64, g: impl Fn(f64) → f64, sample_g: impl Fn() → f64, n: usize) → f64` | Importance sampling integration | `integration::multidim` |
| `trapezoid` | `fn trapezoid(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Trapezoidal rule | `integration::quadrature` |
| `simpson` | `fn simpson(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Simpson's rule | `integration::quadrature` |
| `simpson_38` | `fn simpson_38(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Simpson's 3/8 rule | `integration::quadrature` |
| `boole` | `fn boole(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Boole's rule | `integration::quadrature` |
| `midpoint` | `fn midpoint(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Midpoint rule | `integration::quadrature` |
| `romberg` | `fn romberg(f: impl Fn(f64) → f64, a: f64, b: f64, max_iter: usize, tol: f64) → f64` | Romberg integration | `integration::quadrature` |
| `gauss_legendre_5` | `fn gauss_legendre_5(f: impl Fn(f64) → f64, a: f64, b: f64) → f64` | 5-point Gauss-Legendre | `integration::quadrature` |
| `gauss_laguerre_5` | `fn gauss_laguerre_5(f: impl Fn(f64) → f64) → f64` | 5-point Gauss-Laguerre | `integration::quadrature` |
| `gauss_hermite_5` | `fn gauss_hermite_5(f: impl Fn(f64) → f64) → f64` | 5-point Gauss-Hermite | `integration::quadrature` |
| `gauss_chebyshev_5` | `fn gauss_chebyshev_5(f: impl Fn(f64) → f64) → f64` | 5-point Gauss-Chebyshev | `integration::quadrature` |
| `composite_simpson` | `fn composite_simpson(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Composite Simpson's rule | `integration::quadrature` |
| `composite_trapezoid` | `fn composite_trapezoid(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Composite trapezoidal rule | `integration::quadrature` |
| `weddle` | `fn weddle(f: impl Fn(f64) → f64, a: f64, b: f64) → f64` | Weddle's rule (6-point Newton-Cotes) | `integration::quadrature` |
| `clenshaw_curtis` | `fn clenshaw_curtis(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Clenshaw-Curtis quadrature | `integration::quadrature` |
| `tanh_sinh` | `fn tanh_sinh(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Tanh-sinh quadrature | `integration::quadrature` |
| `lobatto_quadrature` | `fn lobatto_quadrature(f: impl Fn(f64) → f64, a: f64, b: f64) → f64` | Lobatto quadrature with 5 points | `integration::quadrature` |
| `open_newton_cotes_4` | `fn open_newton_cotes_4(f: impl Fn(f64) → f64, a: f64, b: f64) → f64` | Open Newton-Cotes 4th order | `integration::quadrature` |
| `gauss_kronrod_15` | `fn gauss_kronrod_15(f: impl Fn(f64) → f64, a: f64, b: f64) → (f64, f64)` | 15-point Gauss-Kronrod with error estimate | `integration::quadrature` |
| `richardson_extrapolation` | `fn richardson_extrapolation(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize, order: usize) → f64` | Richardson extrapolation | `integration::quadrature` |
| `filon_sin` | `fn filon_sin(f: impl Fn(f64) → f64, a: f64, b: f64, omega: f64, n: usize) → f64` | Filon quadrature (sine oscillatory) | `integration::quadrature` |
| `filon_cos` | `fn filon_cos(f: impl Fn(f64) → f64, a: f64, b: f64, omega: f64, n: usize) → f64` | Filon quadrature (cosine oscillatory) | `integration::quadrature` |

### 8️⃣ Interpolation — maths::interpolation — 39 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `lagrange_interpolate` | `fn lagrange_interpolate(points: &[(f64, f64)], x: f64) → f64` | Lagrange interpolation | `interpolation::lagrange` |
| `lagrange_barycentric` | `fn lagrange_barycentric(points: &[(f64, f64)], x: f64) → f64` | Barycentric Lagrange interpolation | `interpolation::lagrange` |
| `neville` | `fn neville(points: &[(f64, f64)], x: f64) → f64` | Neville's algorithm | `interpolation::lagrange` |
| `divided_differences` | `fn divided_differences(points: &[(f64, f64)]) → Vec<f64>` | Newton divided differences | `interpolation::lagrange` |
| `newton_interpolate` | `fn newton_interpolate(points: &[(f64, f64)], x: f64) → f64` | Newton interpolation | `interpolation::lagrange` |
| `lagrange_derivative` | `fn lagrange_derivative(points: &[(f64, f64)], x: f64) → f64` | Derivative via Lagrange differentiation | `interpolation::lagrange` |
| `chebyshev_nodes` | `fn chebyshev_nodes(n: usize, a: f64, b: f64) → Vec<f64>` | Chebyshev interpolation nodes | `interpolation::lagrange` |
| `chebyshev_interpolate` | `fn chebyshev_interpolate(f: impl Fn(f64) → f64, n: usize, a: f64, b: f64, x: f64) → f64` | Chebyshev interpolation | `interpolation::lagrange` |
| `rational_interpolate` | `fn rational_interpolate(points: &[(f64, f64)], x: f64) → f64` | Rational interpolation | `interpolation::lagrange` |
| `newton_forward_difference` | `fn newton_forward_difference(y: &[f64], h: f64, x0: f64, x: f64) → f64` | Newton forward difference formula | `interpolation::lagrange` |
| `interpolation_error_bound` | `fn interpolation_error_bound(points: &[(f64, f64)], x: f64) → f64` | Error bound estimate | `interpolation::lagrange` |
| `lebesgue_constant` | `fn lebesgue_constant(nodes: &[f64], n_eval: usize) → f64` | Lebesgue constant for node set | `interpolation::lagrange` |
| `least_squares_fit` | `fn least_squares_fit(points: &[(f64, f64)], degree: usize) → Vec<f64>` | Polynomial least squares fit | `interpolation::methods` |
| `moving_average` | `fn moving_average(data: &[f64], window: usize) → Vec<f64>` | Simple moving average | `interpolation::methods` |
| `savitzky_golay_5` | `fn savitzky_golay_5(data: &[f64]) → Vec<f64>` | 5-point Savitzky-Golay filter | `interpolation::methods` |
| `exponential_moving_average` | `fn exponential_moving_average(data: &[f64], alpha: f64) → Vec<f64>` | Exponential moving average | `interpolation::methods` |
| `weighted_moving_average` | `fn weighted_moving_average(data: &[f64], weights: &[f64]) → Vec<f64>` | Weighted moving average | `interpolation::methods` |
| `gaussian_kernel_smooth` | `fn gaussian_kernel_smooth(data: &[f64], sigma: f64) → Vec<f64>` | Gaussian kernel smoothing | `interpolation::methods` |
| `loess_smooth` | `fn loess_smooth(x: &[f64], y: &[f64], span: f64, eval_x: &[f64]) → Vec<f64>` | LOESS local regression | `interpolation::methods` |
| `savitzky_golay_7` | `fn savitzky_golay_7(data: &[f64]) → Vec<f64>` | 7-point Savitzky-Golay filter | `interpolation::methods` |
| `median_filter` | `fn median_filter(data: &[f64], window: usize) → Vec<f64>` | Median filter | `interpolation::methods` |
| `whittaker_smooth` | `fn whittaker_smooth(data: &[f64], lambda: f64) → Vec<f64>` | Whittaker smoother | `interpolation::methods` |
| `total_variation_denoise` | `fn total_variation_denoise(data: &[f64], lambda: f64, max_iter: usize) → Vec<f64>` | Total variation denoising | `interpolation::methods` |
| `CubicSpline` | `struct { x: Vec<f64>, a: Vec<f64>, b: Vec<f64>, c: Vec<f64>, d: Vec<f64> }` | Natural cubic spline | `interpolation::spline` |
| `CubicSpline::natural` | `fn natural(points: &[(f64, f64)]) → Self` | Construct natural spline | `interpolation::spline` |
| `CubicSpline::eval` | `fn eval(&self, x: f64) → f64` | Evaluate spline at point | `interpolation::spline` |
| `CubicSpline::integrate` | `fn integrate(&self, a: f64, b: f64) → f64` | Integrate spline over interval | `interpolation::spline` |
| `CubicSpline::derivative` | `fn derivative(&self, x: f64) → f64` | Evaluate spline derivative | `interpolation::spline` |
| `linear_interpolate` | `fn linear_interpolate(x0: f64, y0: f64, x1: f64, y1: f64, x: f64) → f64` | Linear interpolation | `interpolation::spline` |
| `bilinear_interpolate` | `fn bilinear_interpolate(x: f64, y: f64, x1: f64, x2: f64, y1: f64, y2: f64, q11: f64, q12: f64, q21: f64, q22: f64) → f64` | Bilinear interpolation | `interpolation::spline` |
| `hermite_interpolate` | `fn hermite_interpolate(x0: f64, f0: f64, d0: f64, x1: f64, f1: f64, d1: f64, x: f64) → f64` | Hermite interpolation with derivatives | `interpolation::spline` |
| `akima_interpolate` | `fn akima_interpolate(points: &[(f64, f64)], x: f64) → f64` | Akima sub-spline interpolation | `interpolation::spline` |
| `catmull_rom` | `fn catmull_rom(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) → f64` | Catmull-Rom spline segment | `interpolation::spline` |
| `monotone_cubic` | `fn monotone_cubic(points: &[(f64, f64)], x: f64) → f64` | Monotone cubic interpolation | `interpolation::spline` |
| `pchip_interpolate` | `fn pchip_interpolate(points: &[(f64, f64)], x: f64) → f64` | PCHIP interpolation | `interpolation::spline` |
| `nearest_neighbor` | `fn nearest_neighbor(points: &[(f64, f64)], x: f64) → f64` | Nearest-neighbor interpolation | `interpolation::spline` |
| `trilinear_interpolate` | `fn trilinear_interpolate(x: f64, y: f64, z: f64, corners: &[f64; 8], bounds: &[(f64, f64); 3]) → f64` | Trilinear interpolation in 3D | `interpolation::spline` |
| `bezier_cubic` | `fn bezier_cubic(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) → f64` | Cubic Bézier curve | `interpolation::spline` |
| `bspline_basis` | `fn bspline_basis(knots: &[f64], i: usize, p: usize, t: f64) → f64` | B-spline basis function | `interpolation::spline` |

### 9️⃣ ODE Solvers — maths::ode — 36 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `shooting_method` | `fn shooting_method(f: impl Fn(f64, &[f64]) → Vec<f64>, a: f64, b: f64, alpha: f64, beta: f64, guess: f64, tol: f64, max_iter: usize) → Vec<(f64, Vec<f64>)>` | Shooting method for BVP | `ode::bvp` |
| `finite_difference_bvp` | `fn finite_difference_bvp(p: impl Fn(f64) → f64, q: impl Fn(f64) → f64, r: impl Fn(f64) → f64, a: f64, b: f64, alpha: f64, beta: f64, n: usize) → Vec<(f64, f64)>` | Finite-difference BVP solver | `ode::bvp` |
| `collocation_bvp` | `fn collocation_bvp(f: impl Fn(f64, f64) → f64, a: f64, b: f64, alpha: f64, beta: f64, n: usize, tol: f64, max_iter: usize) → Vec<(f64, f64)>` | Collocation BVP solver | `ode::bvp` |
| `multiple_shooting` | `fn multiple_shooting(f: impl Fn(f64, &[f64]) → Vec<f64>, a: f64, b: f64, alpha: f64, beta: f64, n_segments: usize, tol: f64, max_iter: usize) → Vec<(f64, Vec<f64>)>` | Multiple shooting BVP | `ode::bvp` |
| `sturm_liouville_eigenvalues` | `fn sturm_liouville_eigenvalues(p: impl Fn(f64) → f64, q: impl Fn(f64) → f64, a: f64, b: f64, n: usize, n_eigenvalues: usize) → Vec<f64>` | Sturm-Liouville eigenvalues | `ode::bvp` |
| `relaxation_bvp` | `fn relaxation_bvp(f: impl Fn(f64, f64) → f64, a: f64, b: f64, alpha: f64, beta: f64, n: usize, tol: f64, max_iter: usize) → Vec<(f64, f64)>` | Relaxation BVP solver | `ode::bvp` |
| `OdeResult` | `struct { t: Vec<f64>, y: Vec<Vec<f64>> }` | ODE solution trajectory | `ode::solvers` |
| `euler` | `fn euler(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | Forward Euler method | `ode::solvers` |
| `rk4` | `fn rk4(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | Classical Runge-Kutta 4 | `ode::solvers` |
| `rk45_adaptive` | `fn rk45_adaptive(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], tol: f64) → OdeResult` | Dormand-Prince RK45 adaptive | `ode::solvers` |
| `implicit_euler` | `fn implicit_euler(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | Implicit Euler (backward) | `ode::solvers` |
| `velocity_verlet` | `fn velocity_verlet(accel: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], v0: &[f64], dt: f64) → OdeResult` | Velocity Verlet for Hamiltonian systems | `ode::solvers` |
| `midpoint_method` | `fn midpoint_method(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | Explicit midpoint method | `ode::solvers` |
| `heun` | `fn heun(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | Heun's method (improved Euler) | `ode::solvers` |
| `rk38` | `fn rk38(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | 3/8-rule Runge-Kutta | `ode::solvers` |
| `adams_bashforth_4` | `fn adams_bashforth_4(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | 4-step Adams-Bashforth | `ode::solvers` |
| `symplectic_euler` | `fn symplectic_euler(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | Symplectic Euler | `ode::solvers` |
| `stiff_bdf2` | `fn stiff_bdf2(f: impl Fn(f64, &[f64]) → Vec<f64>, t_span: (f64, f64), y0: &[f64], dt: f64) → OdeResult` | BDF-2 for stiff systems | `ode::solvers` |
| `lotka_volterra` | `fn lotka_volterra(alpha: f64, beta: f64, delta: f64, gamma: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Lotka-Volterra predator-prey | `ode::systems` |
| `lorenz` | `fn lorenz(sigma: f64, rho: f64, beta: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Lorenz attractor | `ode::systems` |
| `van_der_pol` | `fn van_der_pol(mu: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Van der Pol oscillator | `ode::systems` |
| `harmonic_oscillator` | `fn harmonic_oscillator(omega: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Simple harmonic oscillator | `ode::systems` |
| `double_pendulum` | `fn double_pendulum(l1: f64, l2: f64, m1: f64, m2: f64, g: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Double pendulum system | `ode::systems` |
| `sir_model` | `fn sir_model(beta: f64, gamma: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | SIR epidemic model | `ode::systems` |
| `rossler` | `fn rossler(a: f64, b: f64, c: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Rössler attractor | `ode::systems` |
| `three_body_planar` | `fn three_body_planar(m1: f64, m2: f64, m3: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Planar three-body problem | `ode::systems` |
| `brusselator` | `fn brusselator(a: f64, b: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Brusselator chemical oscillator | `ode::systems` |
| `oregonator` | `fn oregonator(eps: f64, delta: f64, q: f64, f_param: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Oregonator (Belousov-Zhabotinsky) | `ode::systems` |
| `fitzhugh_nagumo` | `fn fitzhugh_nagumo(a: f64, b: f64, i_ext: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | FitzHugh-Nagumo neuron model | `ode::systems` |
| `duffing` | `fn duffing(alpha: f64, beta: f64, delta: f64, gamma: f64, omega: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Duffing forced oscillator | `ode::systems` |
| `chen_system` | `fn chen_system(a: f64, b: f64, c: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Chen chaotic system | `ode::systems` |
| `chua_circuit` | `fn chua_circuit(alpha: f64, beta: f64, m0: f64, m1: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Chua's circuit | `ode::systems` |
| `predator_prey_holling` | `fn predator_prey_holling(r: f64, k: f64, a: f64, h: f64, e: f64, d: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Predator-prey with Holling type II | `ode::systems` |
| `stiff_robertson` | `fn stiff_robertson() → impl Fn(f64, &[f64]) → Vec<f64>` | Robertson stiff chemical kinetics | `ode::systems` |
| `rigid_body` | `fn rigid_body(i1: f64, i2: f64, i3: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | Euler rigid body rotation | `ode::systems` |
| `seir_model` | `fn seir_model(beta: f64, sigma: f64, gamma: f64) → impl Fn(f64, &[f64]) → Vec<f64>` | SEIR epidemic model | `ode::systems` |

### 🔟 PDE Solvers — maths::pde — 71 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `heat_equation_1d_explicit` | `fn heat_equation_1d_explicit(u: &mut [f64], alpha: f64, dx: f64, dt: f64, steps: usize)` | Explicit heat equation solver | `pde::diffusion` |
| `heat_equation_1d_implicit` | `fn heat_equation_1d_implicit(u: &mut [f64], alpha: f64, dx: f64, dt: f64, steps: usize)` | Implicit heat equation solver | `pde::diffusion` |
| `heat_equation_1d_crank_nicolson` | `fn heat_equation_1d_crank_nicolson(u: &mut [f64], alpha: f64, dx: f64, dt: f64, steps: usize)` | Crank-Nicolson heat equation | `pde::diffusion` |
| `diffusion_2d_explicit` | `fn diffusion_2d_explicit(u: &mut Vec<Vec<f64>>, alpha: f64, dx: f64, dy: f64, dt: f64, steps: usize)` | 2D explicit diffusion | `pde::diffusion` |
| `stability_criterion_explicit` | `fn stability_criterion_explicit(alpha: f64, dx: f64, dt: f64) → bool` | CFL stability check | `pde::diffusion` |
| `diffusion_green_function` | `fn diffusion_green_function(x: f64, t: f64, d: f64) → f64` | Gaussian Green's function | `pde::diffusion` |
| `advection_diffusion_1d` | `fn advection_diffusion_1d(u: &mut [f64], v: f64, d: f64, dx: f64, dt: f64, steps: usize)` | 1D advection-diffusion | `pde::diffusion` |
| `diffusion_reaction_1d` | `fn diffusion_reaction_1d(u: &mut [f64], d: f64, reaction: impl Fn(f64) → f64, dx: f64, dt: f64, steps: usize)` | 1D diffusion-reaction | `pde::diffusion` |
| `fisher_kpp_step` | `fn fisher_kpp_step(u: &mut [f64], d: f64, r: f64, dx: f64, dt: f64)` | Fisher-KPP equation step | `pde::diffusion` |
| `nonlinear_diffusion_1d` | `fn nonlinear_diffusion_1d(u: &mut [f64], d_fn: impl Fn(f64) → f64, dx: f64, dt: f64, steps: usize)` | Nonlinear diffusion | `pde::diffusion` |
| `diffusion_2d_adi` | `fn diffusion_2d_adi(u: &mut Vec<Vec<f64>>, alpha: f64, dx: f64, dy: f64, dt: f64, steps: usize)` | 2D ADI diffusion | `pde::diffusion` |
| `stability_criterion_2d` | `fn stability_criterion_2d(alpha: f64, dx: f64, dy: f64, dt: f64) → bool` | 2D stability criterion | `pde::diffusion` |
| `peclet_number` | `fn peclet_number(v: f64, l: f64, d: f64) → f64` | Péclet number | `pde::diffusion` |
| `diffusion_steady_state_1d` | `fn diffusion_steady_state_1d(n: usize, d: f64, source: impl Fn(f64) → f64, dx: f64) → Vec<f64>` | Steady-state diffusion | `pde::diffusion` |
| `diffusion_analytical_finite` | `fn diffusion_analytical_finite(x: f64, t: f64, d: f64, l: f64, n_terms: usize) → f64` | Analytical finite-domain solution | `pde::diffusion` |
| `mass_conservation_check` | `fn mass_conservation_check(u: &[f64], dx: f64) → f64` | Total mass integral | `pde::diffusion` |
| `diffusion_coefficient_from_msd` | `fn diffusion_coefficient_from_msd(msd: f64, t: f64, dim: usize) → f64` | Diffusion coefficient from MSD | `pde::diffusion` |
| `first_derivative` | `fn first_derivative(f: &[f64], dx: f64) → Vec<f64>` | Central difference first derivative | `pde::finite_diff` |
| `second_derivative` | `fn second_derivative(f: &[f64], dx: f64) → Vec<f64>` | Central difference second derivative | `pde::finite_diff` |
| `fourth_order_first_derivative` | `fn fourth_order_first_derivative(f: &[f64], dx: f64) → Vec<f64>` | 4th-order first derivative stencil | `pde::finite_diff` |
| `laplacian_2d` | `fn laplacian_2d(u: &[Vec<f64>], dx: f64, dy: f64) → Vec<Vec<f64>>` | 5-point 2D Laplacian | `pde::finite_diff` |
| `gradient_2d` | `fn gradient_2d(u: &[Vec<f64>], dx: f64, dy: f64) → (Vec<Vec<f64>>, Vec<Vec<f64>>)` | 2D gradient field | `pde::finite_diff` |
| `divergence_2d` | `fn divergence_2d(fx: &[Vec<f64>], fy: &[Vec<f64>], dx: f64, dy: f64) → Vec<Vec<f64>>` | 2D divergence of vector field | `pde::finite_diff` |
| `curl_2d` | `fn curl_2d(fx: &[Vec<f64>], fy: &[Vec<f64>], dx: f64, dy: f64) → Vec<Vec<f64>>` | 2D curl scalar | `pde::finite_diff` |
| `upwind_advection` | `fn upwind_advection(u: &mut [f64], v: f64, dx: f64, dt: f64, steps: usize)` | Upwind advection scheme | `pde::finite_diff` |
| `lax_wendroff` | `fn lax_wendroff(u: &mut [f64], v: f64, dx: f64, dt: f64, steps: usize)` | Lax-Wendroff scheme | `pde::finite_diff` |
| `third_derivative` | `fn third_derivative(f: &[f64], dx: f64) → Vec<f64>` | Third derivative stencil | `pde::finite_diff` |
| `mixed_partial_xy` | `fn mixed_partial_xy(u: &[Vec<f64>], dx: f64, dy: f64) → Vec<Vec<f64>>` | Mixed partial ∂²f/∂x∂y | `pde::finite_diff` |
| `compact_fourth_order` | `fn compact_fourth_order(f: &[f64], dx: f64) → Vec<f64>` | Compact 4th-order scheme | `pde::finite_diff` |
| `von_neumann_stability` | `fn von_neumann_stability(c: f64) → f64` | Von Neumann stability amplification factor | `pde::finite_diff` |
| `boundary_ghost_extrapolate` | `fn boundary_ghost_extrapolate(u: &[f64]) → (f64, f64)` | Ghost point extrapolation | `pde::finite_diff` |
| `hessian_2d` | `fn hessian_2d(u: &[Vec<f64>], dx: f64, dy: f64) → (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>)` | 2D Hessian matrix components | `pde::finite_diff` |
| `biharmonic_2d` | `fn biharmonic_2d(u: &[Vec<f64>], dx: f64, dy: f64) → Vec<Vec<f64>>` | Biharmonic operator ∇⁴ | `pde::finite_diff` |
| `forward_euler_pde` | `fn forward_euler_pde(u: &mut [f64], rhs: impl Fn(&[f64]) → Vec<f64>, dt: f64, steps: usize)` | Forward Euler for PDE semi-discretization | `pde::finite_diff` |
| `lax_friedrichs` | `fn lax_friedrichs(u: &mut [f64], v: f64, dx: f64, dt: f64, steps: usize)` | Lax-Friedrichs scheme | `pde::finite_diff` |
| `maccormack` | `fn maccormack(u: &mut [f64], v: f64, dx: f64, dt: f64, steps: usize)` | MacCormack predictor-corrector | `pde::finite_diff` |
| `laplace_jacobi` | `fn laplace_jacobi(u: &mut Vec<Vec<f64>>, max_iter: usize, tol: f64) → usize` | Laplace equation via Jacobi iteration | `pde::laplace` |
| `laplace_gauss_seidel` | `fn laplace_gauss_seidel(u: &mut Vec<Vec<f64>>, max_iter: usize, tol: f64) → usize` | Laplace via Gauss-Seidel | `pde::laplace` |
| `laplace_sor` | `fn laplace_sor(u: &mut Vec<Vec<f64>>, omega: f64, max_iter: usize, tol: f64) → usize` | Laplace via SOR | `pde::laplace` |
| `optimal_sor_omega` | `fn optimal_sor_omega(nx: usize, ny: usize) → f64` | Optimal SOR relaxation parameter | `pde::laplace` |
| `poisson_jacobi` | `fn poisson_jacobi(u: &mut Vec<Vec<f64>>, f: &[Vec<f64>], dx: f64, dy: f64, max_iter: usize, tol: f64) → usize` | Poisson equation via Jacobi | `pde::laplace` |
| `residual_norm` | `fn residual_norm(u: &[Vec<f64>], f: &[Vec<f64>], dx: f64, dy: f64) → f64` | Residual norm for iterative solvers | `pde::laplace` |
| `harmonic_function_disk` | `fn harmonic_function_disk(r: f64, theta: f64, coeffs: &[(f64, f64)]) → f64` | Harmonic function on disk via Fourier | `pde::laplace` |
| `poisson_gauss_seidel` | `fn poisson_gauss_seidel(u: &mut Vec<Vec<f64>>, f: &[Vec<f64>], dx: f64, dy: f64, max_iter: usize, tol: f64) → usize` | Poisson via Gauss-Seidel | `pde::laplace` |
| `poisson_sor` | `fn poisson_sor(u: &mut Vec<Vec<f64>>, f: &[Vec<f64>], dx: f64, dy: f64, omega: f64, max_iter: usize, tol: f64) → usize` | Poisson via SOR | `pde::laplace` |
| `max_principle_check` | `fn max_principle_check(u: &[Vec<f64>]) → bool` | Maximum principle verification | `pde::laplace` |
| `greens_function_2d_free` | `fn greens_function_2d_free(x: f64, y: f64, x0: f64, y0: f64) → f64` | 2D free-space Green's function | `pde::laplace` |
| `mean_value_property` | `fn mean_value_property(u: &[Vec<f64>], i: usize, j: usize) → f64` | Mean value property check | `pde::laplace` |
| `laplace_3d_jacobi` | `fn laplace_3d_jacobi(u: &mut Vec<Vec<Vec<f64>>>, max_iter: usize, tol: f64) → usize` | 3D Laplace via Jacobi | `pde::laplace` |
| `dirichlet_energy` | `fn dirichlet_energy(u: &[Vec<f64>], dx: f64, dy: f64) → f64` | Dirichlet energy functional | `pde::laplace` |
| `laplace_iteration_count` | `fn laplace_iteration_count(nx: usize, ny: usize, tol: f64) → usize` | Estimated iteration count | `pde::laplace` |
| `wave_equation_1d` | `fn wave_equation_1d(u: &mut [f64], u_prev: &[f64], c: f64, dx: f64, dt: f64)` | 1D wave equation explicit step | `pde::wave` |
| `wave_initial_step` | `fn wave_initial_step(u0: &[f64], v0: &[f64], c: f64, dx: f64, dt: f64) → Vec<f64>` | First time step from initial conditions | `pde::wave` |
| `wave_equation_2d` | `fn wave_equation_2d(u: &mut Vec<Vec<f64>>, u_prev: &[Vec<f64>], c: f64, dx: f64, dy: f64, dt: f64)` | 2D wave equation step | `pde::wave` |
| `courant_number` | `fn courant_number(c: f64, dt: f64, dx: f64) → f64` | Courant number | `pde::wave` |
| `dalembert_solution` | `fn dalembert_solution(x: f64, t: f64, c: f64, f_init: impl Fn(f64) → f64, g_init: impl Fn(f64) → f64) → f64` | d'Alembert analytical solution | `pde::wave` |
| `wave_energy_density` | `fn wave_energy_density(u: &[f64], u_prev: &[f64], c: f64, dx: f64, dt: f64) → Vec<f64>` | Local wave energy density | `pde::wave` |
| `absorbing_boundary` | `fn absorbing_boundary(u: &mut [f64], u_prev: &[f64], c: f64, dx: f64, dt: f64)` | Absorbing boundary conditions | `pde::wave` |
| `string_vibration_mode` | `fn string_vibration_mode(x: f64, t: f64, l: f64, c: f64, n: usize) → f64` | nth vibration mode of string | `pde::wave` |
| `wave_equation_1d_implicit` | `fn wave_equation_1d_implicit(u: &mut [f64], u_prev: &[f64], c: f64, dx: f64, dt: f64)` | Implicit 1D wave equation | `pde::wave` |
| `cfl_check` | `fn cfl_check(c: f64, dt: f64, dx: f64) → bool` | CFL condition check | `pde::wave` |
| `wave_reflection_coefficient` | `fn wave_reflection_coefficient(z1: f64, z2: f64) → f64` | Impedance reflection coefficient | `pde::wave` |
| `wave_transmission_coefficient` | `fn wave_transmission_coefficient(z1: f64, z2: f64) → f64` | Impedance transmission coefficient | `pde::wave` |
| `standing_wave` | `fn standing_wave(x: f64, t: f64, k: f64, omega: f64, a: f64) → f64` | Standing wave solution | `pde::wave` |
| `wave_phase_velocity` | `fn wave_phase_velocity(omega: f64, k: f64) → f64` | Phase velocity | `pde::wave` |
| `wave_group_velocity` | `fn wave_group_velocity(domega: f64, dk: f64) → f64` | Group velocity | `pde::wave` |
| `wave_total_energy` | `fn wave_total_energy(u: &[f64], u_prev: &[f64], c: f64, dx: f64, dt: f64) → f64` | Total wave energy | `pde::wave` |
| `spherical_wave_amplitude` | `fn spherical_wave_amplitude(r: f64, a0: f64) → f64` | Spherical wave amplitude decay | `pde::wave` |
| `wave_packet_gaussian` | `fn wave_packet_gaussian(x: f64, t: f64, k0: f64, sigma: f64, c: f64) → f64` | Gaussian wave packet | `pde::wave` |
| `wave_superposition` | `fn wave_superposition(x: f64, t: f64, components: &[(f64, f64, f64)]) → f64` | Superposition of wave components | `pde::wave` |
| `wave_impedance` | `fn wave_impedance(rho: f64, c: f64) → f64` | Acoustic impedance | `pde::wave` |

### 11. FFT — maths::fft — 51 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `bluestein_fft` | `fn bluestein_fft(x: &[(f64, f64)]) → Vec<(f64, f64)>` | Bluestein FFT (arbitrary length) | `fft::bluestein` |
| `bluestein_ifft` | `fn bluestein_ifft(x: &[(f64, f64)]) → Vec<(f64, f64)>` | Bluestein inverse FFT | `fft::bluestein` |
| `fft_arbitrary` | `fn fft_arbitrary(x: &[(f64, f64)]) → Vec<(f64, f64)>` | FFT for arbitrary-length input | `fft::bluestein` |
| `ifft_arbitrary` | `fn ifft_arbitrary(x: &[(f64, f64)]) → Vec<(f64, f64)>` | Inverse FFT for arbitrary length | `fft::bluestein` |
| `goertzel` | `fn goertzel(x: &[f64], freq_bin: usize) → (f64, f64)` | Goertzel single-frequency DFT | `fft::bluestein` |
| `goertzel_mag` | `fn goertzel_mag(x: &[f64], freq_bin: usize) → f64` | Goertzel magnitude | `fft::bluestein` |
| `chirp_z_transform` | `fn chirp_z_transform(x: &[(f64, f64)], m: usize, w: (f64, f64), a: (f64, f64)) → Vec<(f64, f64)>` | Chirp-Z transform | `fft::bluestein` |
| `fft_shift` | `fn fft_shift(x: &[(f64, f64)]) → Vec<(f64, f64)>` | Shift zero-frequency to center | `fft::bluestein` |
| `ifft_shift` | `fn ifft_shift(x: &[(f64, f64)]) → Vec<(f64, f64)>` | Inverse FFT shift | `fft::bluestein` |
| `hilbert_transform` | `fn hilbert_transform(x: &[f64]) → Vec<(f64, f64)>` | Hilbert transform via FFT | `fft::bluestein` |
| `analytic_signal` | `fn analytic_signal(x: &[f64]) → Vec<(f64, f64)>` | Analytic signal representation | `fft::bluestein` |
| `instantaneous_frequency` | `fn instantaneous_frequency(x: &[f64], sample_rate: f64) → Vec<f64>` | Instantaneous frequency | `fft::bluestein` |
| `sliding_dft` | `fn sliding_dft(x: &[f64], k: usize, window_size: usize) → Vec<(f64, f64)>` | Sliding DFT for streaming | `fft::bluestein` |
| `dct_ii` | `fn dct_ii(x: &[f64]) → Vec<f64>` | Type-II DCT (standard) | `fft::dct` |
| `idct_ii` | `fn idct_ii(x: &[f64]) → Vec<f64>` | Inverse type-II DCT | `fft::dct` |
| `dst_i` | `fn dst_i(x: &[f64]) → Vec<f64>` | Type-I DST | `fft::dct` |
| `dct_i` | `fn dct_i(x: &[f64]) → Vec<f64>` | Type-I DCT | `fft::dct` |
| `dct_iii` | `fn dct_iii(x: &[f64]) → Vec<f64>` | Type-III DCT | `fft::dct` |
| `dct_iv` | `fn dct_iv(x: &[f64]) → Vec<f64>` | Type-IV DCT | `fft::dct` |
| `dst_ii` | `fn dst_ii(x: &[f64]) → Vec<f64>` | Type-II DST | `fft::dct` |
| `dst_iii` | `fn dst_iii(x: &[f64]) → Vec<f64>` | Type-III DST | `fft::dct` |
| `dst_iv` | `fn dst_iv(x: &[f64]) → Vec<f64>` | Type-IV DST | `fft::dct` |
| `mdct` | `fn mdct(x: &[f64]) → Vec<f64>` | Modified DCT | `fft::dct` |
| `imdct` | `fn imdct(x: &[f64]) → Vec<f64>` | Inverse MDCT | `fft::dct` |
| `dct_2d` | `fn dct_2d(x: &[Vec<f64>]) → Vec<Vec<f64>>` | 2D DCT | `fft::dct` |
| `idct_2d` | `fn idct_2d(x: &[Vec<f64>]) → Vec<Vec<f64>>` | Inverse 2D DCT | `fft::dct` |
| `hartley_transform` | `fn hartley_transform(x: &[f64]) → Vec<f64>` | Discrete Hartley transform | `fft::dct` |
| `fft` | `fn fft(x: &[(f64, f64)]) → Vec<(f64, f64)>` | Radix-2 Cooley-Tukey FFT | `fft::radix2` |
| `ifft` | `fn ifft(x: &[(f64, f64)]) → Vec<(f64, f64)>` | Inverse FFT | `fft::radix2` |
| `fft_real` | `fn fft_real(x: &[f64]) → Vec<(f64, f64)>` | FFT of real-valued input | `fft::radix2` |
| `power_spectrum` | `fn power_spectrum(x: &[f64]) → Vec<f64>` | Power spectral density | `fft::radix2` |
| `frequency_bins` | `fn frequency_bins(n: usize, sample_rate: f64) → Vec<f64>` | FFT frequency axis | `fft::radix2` |
| `convolve` | `fn convolve(a: &[f64], b: &[f64]) → Vec<f64>` | FFT-based convolution | `fft::radix2` |
| `cross_correlate` | `fn cross_correlate(a: &[f64], b: &[f64]) → Vec<f64>` | FFT-based cross-correlation | `fft::radix2` |
| `autocorrelation` | `fn autocorrelation(x: &[f64]) → Vec<f64>` | Autocorrelation via FFT | `fft::radix2` |
| `magnitude_spectrum` | `fn magnitude_spectrum(x: &[(f64, f64)]) → Vec<f64>` | Magnitude spectrum | `fft::radix2` |
| `phase_spectrum` | `fn phase_spectrum(x: &[(f64, f64)]) → Vec<f64>` | Phase spectrum | `fft::radix2` |
| `cepstrum` | `fn cepstrum(x: &[f64]) → Vec<f64>` | Real cepstrum | `fft::radix2` |
| `spectral_centroid` | `fn spectral_centroid(x: &[f64], sample_rate: f64) → f64` | Spectral centroid | `fft::radix2` |
| `spectral_rolloff` | `fn spectral_rolloff(x: &[f64], sample_rate: f64, threshold: f64) → f64` | Spectral rolloff frequency | `fft::radix2` |
| `spectral_flatness` | `fn spectral_flatness(x: &[f64]) → f64` | Spectral flatness (Wiener entropy) | `fft::radix2` |
| `hann_window` | `fn hann_window(n: usize) → Vec<f64>` | Hann window function | `fft::radix2` |
| `hamming_window` | `fn hamming_window(n: usize) → Vec<f64>` | Hamming window function | `fft::radix2` |
| `blackman_window` | `fn blackman_window(n: usize) → Vec<f64>` | Blackman window function | `fft::radix2` |
| `kaiser_window` | `fn kaiser_window(n: usize, beta: f64) → Vec<f64>` | Kaiser window function | `fft::radix2` |
| `windowed_fft` | `fn windowed_fft(x: &[f64], window: &[f64]) → Vec<(f64, f64)>` | Windowed FFT | `fft::radix2` |
| `stft` | `fn stft(x: &[f64], window_size: usize, hop_size: usize) → Vec<Vec<(f64, f64)>>` | Short-time Fourier transform | `fft::radix2` |
| `zero_pad` | `fn zero_pad(x: &[(f64, f64)], new_len: usize) → Vec<(f64, f64)>` | Zero-pad to length | `fft::radix2` |
| `deconvolve` | `fn deconvolve(signal: &[f64], kernel: &[f64]) → Vec<f64>` | FFT-based deconvolution | `fft::radix2` |
| `spectral_bandwidth` | `fn spectral_bandwidth(x: &[f64], sample_rate: f64) → f64` | Spectral bandwidth | `fft::radix2` |
| `spectral_entropy` | `fn spectral_entropy(x: &[f64]) → f64` | Spectral entropy | `fft::radix2` |

### 12. Optimization — maths::optimization — 53 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `penalty_method` | `fn penalty_method(f: impl Fn(&[f64]) → f64, constraints: &[impl Fn(&[f64]) → f64], x0: &[f64], rho: f64, max_iter: usize) → Vec<f64>` | Penalty method for constrained optimization | `optimization::constrained` |
| `augmented_lagrangian` | `fn augmented_lagrangian(f: impl Fn(&[f64]) → f64, constraints: &[impl Fn(&[f64]) → f64], x0: &[f64], max_iter: usize) → Vec<f64>` | Augmented Lagrangian method | `optimization::constrained` |
| `barrier_method` | `fn barrier_method(f: impl Fn(&[f64]) → f64, constraints: &[impl Fn(&[f64]) → f64], x0: &[f64], mu: f64, max_iter: usize) → Vec<f64>` | Interior point barrier method | `optimization::constrained` |
| `project_box` | `fn project_box(x: &[f64], lower: &[f64], upper: &[f64]) → Vec<f64>` | Project onto box constraints | `optimization::constrained` |
| `project_simplex` | `fn project_simplex(x: &[f64]) → Vec<f64>` | Project onto probability simplex | `optimization::constrained` |
| `kkt_violation` | `fn kkt_violation(grad_f: &[f64], constraints: &[f64], lambdas: &[f64]) → f64` | KKT condition violation measure | `optimization::constrained` |
| `lagrangian` | `fn lagrangian(f: f64, constraints: &[f64], lambdas: &[f64]) → f64` | Lagrangian function value | `optimization::constrained` |
| `projected_gradient_step` | `fn projected_gradient_step(x: &[f64], grad: &[f64], lr: f64, lower: &[f64], upper: &[f64]) → Vec<f64>` | Projected gradient step | `optimization::constrained` |
| `frank_wolfe_step` | `fn frank_wolfe_step(grad: &[f64], lower: &[f64], upper: &[f64]) → Vec<f64>` | Frank-Wolfe linear minimization oracle | `optimization::constrained` |
| `admm_x_update` | `fn admm_x_update(a: &[Vec<f64>], b: &[f64], z: &[f64], u: &[f64], rho: f64) → Vec<f64>` | ADMM x-update step | `optimization::constrained` |
| `dual_ascent_step` | `fn dual_ascent_step(lambdas: &[f64], constraints: &[f64], step_size: f64) → Vec<f64>` | Dual ascent step | `optimization::constrained` |
| `feasibility_check` | `fn feasibility_check(constraints: &[f64], tol: f64) → bool` | Feasibility check for constraints | `optimization::constrained` |
| `quadratic_objective` | `fn quadratic_objective(q: &[Vec<f64>], c: &[f64], x: &[f64]) → f64` | Quadratic objective ½xᵀQx + cᵀx | `optimization::constrained` |
| `linear_constraint_violation` | `fn linear_constraint_violation(a: &[Vec<f64>], b: &[f64], x: &[f64]) → f64` | Linear constraint violation ‖Ax − b‖ | `optimization::constrained` |
| `l1_penalty` | `fn l1_penalty(constraints: &[f64]) → f64` | L1 penalty for constraints | `optimization::constrained` |
| `equality_penalty` | `fn equality_penalty(constraints: &[f64], rho: f64) → f64` | Quadratic equality penalty | `optimization::constrained` |
| `merit_function` | `fn merit_function(f: f64, constraints: &[f64], mu: f64) → f64` | Merit function for line search | `optimization::constrained` |
| `genetic_algorithm` | `fn genetic_algorithm(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], pop_size: usize, generations: usize, mutation_rate: f64) → Vec<f64>` | Genetic algorithm | `optimization::evolutionary` |
| `differential_evolution` | `fn differential_evolution(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], pop_size: usize, generations: usize, cr: f64, f_weight: f64) → Vec<f64>` | Differential evolution | `optimization::evolutionary` |
| `evolution_strategy` | `fn evolution_strategy(f: impl Fn(&[f64]) → f64, x0: &[f64], sigma: f64, pop_size: usize, generations: usize) → Vec<f64>` | (μ, λ) evolution strategy | `optimization::evolutionary` |
| `scatter_search` | `fn scatter_search(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], pop_size: usize, max_iter: usize) → Vec<f64>` | Scatter search metaheuristic | `optimization::evolutionary` |
| `multi_objective_dominates` | `fn multi_objective_dominates(a: &[f64], b: &[f64]) → bool` | Pareto dominance check | `optimization::evolutionary` |
| `crowding_distance` | `fn crowding_distance(objectives: &[Vec<f64>]) → Vec<f64>` | Crowding distance for NSGA-II | `optimization::evolutionary` |
| `nsga2_non_dominated_sort` | `fn nsga2_non_dominated_sort(objectives: &[Vec<f64>]) → Vec<Vec<usize>>` | NSGA-II non-dominated sorting | `optimization::evolutionary` |
| `hypervolume_2d` | `fn hypervolume_2d(points: &[(f64, f64)], reference: (f64, f64)) → f64` | 2D hypervolume indicator | `optimization::evolutionary` |
| `island_model` | `fn island_model(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n_islands: usize, pop_per_island: usize, generations: usize, migration_interval: usize) → Vec<f64>` | Island model parallel GA | `optimization::evolutionary` |
| `gradient_descent` | `fn gradient_descent(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], lr: f64, max_iter: usize, tol: f64) → Vec<f64>` | Gradient descent | `optimization::gradient` |
| `gradient_descent_momentum` | `fn gradient_descent_momentum(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], lr: f64, momentum: f64, max_iter: usize, tol: f64) → Vec<f64>` | Gradient descent with momentum | `optimization::gradient` |
| `adam` | `fn adam(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], lr: f64, max_iter: usize, tol: f64) → Vec<f64>` | Adam optimizer | `optimization::gradient` |
| `newton_method_1d` | `fn newton_method_1d(f: impl Fn(f64) → f64, df: impl Fn(f64) → f64, x0: f64, tol: f64, max_iter: usize) → f64` | Newton's method in 1D | `optimization::gradient` |
| `bisection` | `fn bisection(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Bisection line search | `optimization::gradient` |
| `secant_method` | `fn secant_method(f: impl Fn(f64) → f64, x0: f64, x1: f64, tol: f64, max_iter: usize) → f64` | Secant method | `optimization::gradient` |
| `golden_section_search` | `fn golden_section_search(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Golden section search | `optimization::gradient` |
| `numerical_gradient` | `fn numerical_gradient(f: impl Fn(&[f64]) → f64, x: &[f64], h: f64) → Vec<f64>` | Numerical gradient via central differences | `optimization::gradient` |
| `nesterov_momentum` | `fn nesterov_momentum(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], lr: f64, momentum: f64, max_iter: usize, tol: f64) → Vec<f64>` | Nesterov accelerated gradient | `optimization::gradient` |
| `rmsprop` | `fn rmsprop(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], lr: f64, decay: f64, max_iter: usize, tol: f64) → Vec<f64>` | RMSProp optimizer | `optimization::gradient` |
| `adagrad` | `fn adagrad(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], lr: f64, max_iter: usize, tol: f64) → Vec<f64>` | AdaGrad optimizer | `optimization::gradient` |
| `line_search_backtracking` | `fn line_search_backtracking(f: impl Fn(&[f64]) → f64, x: &[f64], direction: &[f64], alpha: f64, beta: f64, sigma: f64) → f64` | Armijo backtracking line search | `optimization::gradient` |
| `bfgs` | `fn bfgs(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], max_iter: usize, tol: f64) → Vec<f64>` | BFGS quasi-Newton method | `optimization::gradient` |
| `conjugate_gradient_min` | `fn conjugate_gradient_min(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, x0: &[f64], max_iter: usize, tol: f64) → Vec<f64>` | Nonlinear conjugate gradient | `optimization::gradient` |
| `hessian_numerical` | `fn hessian_numerical(f: impl Fn(&[f64]) → f64, x: &[f64], h: f64) → Vec<Vec<f64>>` | Numerical Hessian matrix | `optimization::gradient` |
| `ternary_search` | `fn ternary_search(f: impl Fn(f64) → f64, a: f64, b: f64, tol: f64) → f64` | Ternary search for unimodal functions | `optimization::gradient` |
| `newton_method_nd` | `fn newton_method_nd(f: impl Fn(&[f64]) → f64, grad: impl Fn(&[f64]) → Vec<f64>, hess: impl Fn(&[f64]) → Vec<Vec<f64>>, x0: &[f64], max_iter: usize, tol: f64) → Vec<f64>` | Newton's method in nD | `optimization::gradient` |
| `simulated_annealing` | `fn simulated_annealing(f: impl Fn(&[f64]) → f64, x0: &[f64], t0: f64, cooling: f64, max_iter: usize) → Vec<f64>` | Simulated annealing | `optimization::metaheuristic` |
| `particle_swarm` | `fn particle_swarm(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n_particles: usize, max_iter: usize) → Vec<f64>` | Particle swarm optimization | `optimization::metaheuristic` |
| `nelder_mead` | `fn nelder_mead(f: impl Fn(&[f64]) → f64, x0: &[f64], max_iter: usize, tol: f64) → Vec<f64>` | Nelder-Mead simplex method | `optimization::metaheuristic` |
| `tabu_search` | `fn tabu_search(f: impl Fn(&[f64]) → f64, x0: &[f64], max_iter: usize, tabu_size: usize) → Vec<f64>` | Tabu search | `optimization::metaheuristic` |
| `firefly_algorithm` | `fn firefly_algorithm(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n_fireflies: usize, max_iter: usize) → Vec<f64>` | Firefly algorithm | `optimization::metaheuristic` |
| `grey_wolf_optimizer` | `fn grey_wolf_optimizer(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n_wolves: usize, max_iter: usize) → Vec<f64>` | Grey wolf optimizer | `optimization::metaheuristic` |
| `hill_climbing` | `fn hill_climbing(f: impl Fn(&[f64]) → f64, x0: &[f64], step_size: f64, max_iter: usize) → Vec<f64>` | Stochastic hill climbing | `optimization::metaheuristic` |
| `random_search` | `fn random_search(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], max_iter: usize) → Vec<f64>` | Random search | `optimization::metaheuristic` |
| `multistart` | `fn multistart(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n_starts: usize, local_iter: usize) → Vec<f64>` | Multi-start local optimization | `optimization::metaheuristic` |
| `whale_optimization` | `fn whale_optimization(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n_whales: usize, max_iter: usize) → Vec<f64>` | Whale optimization algorithm | `optimization::metaheuristic` |

### 13. Statistics — maths::statistics — 71 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `mean` | `fn mean(data: &[f64]) → f64` | Arithmetic mean | `statistics::descriptive` |
| `variance` | `fn variance(data: &[f64]) → f64` | Population variance | `statistics::descriptive` |
| `std_dev` | `fn std_dev(data: &[f64]) → f64` | Population standard deviation | `statistics::descriptive` |
| `sample_variance` | `fn sample_variance(data: &[f64]) → f64` | Sample variance (Bessel correction) | `statistics::descriptive` |
| `sample_std_dev` | `fn sample_std_dev(data: &[f64]) → f64` | Sample standard deviation | `statistics::descriptive` |
| `median` | `fn median(data: &mut [f64]) → f64` | Median value | `statistics::descriptive` |
| `percentile` | `fn percentile(data: &mut [f64], p: f64) → f64` | Pth percentile | `statistics::descriptive` |
| `mode` | `fn mode(data: &[f64]) → f64` | Mode (most frequent value) | `statistics::descriptive` |
| `skewness` | `fn skewness(data: &[f64]) → f64` | Sample skewness | `statistics::descriptive` |
| `kurtosis` | `fn kurtosis(data: &[f64]) → f64` | Excess kurtosis | `statistics::descriptive` |
| `covariance` | `fn covariance(x: &[f64], y: &[f64]) → f64` | Sample covariance | `statistics::descriptive` |
| `correlation` | `fn correlation(x: &[f64], y: &[f64]) → f64` | Pearson correlation coefficient | `statistics::descriptive` |
| `weighted_mean` | `fn weighted_mean(data: &[f64], weights: &[f64]) → f64` | Weighted arithmetic mean | `statistics::descriptive` |
| `geometric_mean` | `fn geometric_mean(data: &[f64]) → f64` | Geometric mean | `statistics::descriptive` |
| `harmonic_mean` | `fn harmonic_mean(data: &[f64]) → f64` | Harmonic mean | `statistics::descriptive` |
| `entropy` | `fn entropy(probs: &[f64]) → f64` | Shannon entropy | `statistics::descriptive` |
| `kl_divergence` | `fn kl_divergence(p: &[f64], q: &[f64]) → f64` | Kullback-Leibler divergence | `statistics::descriptive` |
| `normal_pdf` | `fn normal_pdf(x: f64, mu: f64, sigma: f64) → f64` | Normal distribution PDF | `statistics::distributions` |
| `normal_cdf` | `fn normal_cdf(x: f64, mu: f64, sigma: f64) → f64` | Normal distribution CDF | `statistics::distributions` |
| `erf` | `fn erf(x: f64) → f64` | Error function | `statistics::distributions` |
| `erfc` | `fn erfc(x: f64) → f64` | Complementary error function | `statistics::distributions` |
| `poisson_pmf` | `fn poisson_pmf(k: u64, lambda: f64) → f64` | Poisson PMF | `statistics::distributions` |
| `binomial_pmf` | `fn binomial_pmf(k: u64, n: u64, p: f64) → f64` | Binomial PMF | `statistics::distributions` |
| `exponential_pdf` | `fn exponential_pdf(x: f64, lambda: f64) → f64` | Exponential distribution PDF | `statistics::distributions` |
| `exponential_cdf` | `fn exponential_cdf(x: f64, lambda: f64) → f64` | Exponential distribution CDF | `statistics::distributions` |
| `chi_squared_pdf` | `fn chi_squared_pdf(x: f64, k: u64) → f64` | Chi-squared PDF | `statistics::distributions` |
| `student_t_pdf` | `fn student_t_pdf(x: f64, nu: f64) → f64` | Student's t-distribution PDF | `statistics::distributions` |
| `cauchy_pdf` | `fn cauchy_pdf(x: f64, x0: f64, gamma: f64) → f64` | Cauchy distribution PDF | `statistics::distributions` |
| `uniform_pdf` | `fn uniform_pdf(x: f64, a: f64, b: f64) → f64` | Uniform distribution PDF | `statistics::distributions` |
| `beta_pdf` | `fn beta_pdf(x: f64, alpha: f64, beta: f64) → f64` | Beta distribution PDF | `statistics::distributions` |
| `gamma_pdf` | `fn gamma_pdf(x: f64, alpha: f64, beta: f64) → f64` | Gamma distribution PDF | `statistics::distributions` |
| `gamma` | `fn gamma(x: f64) → f64` | Gamma function Γ(x) | `statistics::distributions` |
| `ln_gamma` | `fn ln_gamma(x: f64) → f64` | Log-gamma function ln Γ(x) | `statistics::distributions` |
| `beta_function` | `fn beta_function(a: f64, b: f64) → f64` | Beta function B(a, b) | `statistics::distributions` |
| `incomplete_gamma_lower` | `fn incomplete_gamma_lower(a: f64, x: f64) → f64` | Lower incomplete gamma function | `statistics::distributions` |
| `z_test` | `fn z_test(sample_mean: f64, pop_mean: f64, pop_std: f64, n: usize) → f64` | Z-test statistic | `statistics::hypothesis` |
| `t_test_one_sample` | `fn t_test_one_sample(data: &[f64], mu0: f64) → f64` | One-sample t-test statistic | `statistics::hypothesis` |
| `t_test_two_sample` | `fn t_test_two_sample(data1: &[f64], data2: &[f64]) → f64` | Two-sample t-test statistic | `statistics::hypothesis` |
| `chi_squared_test` | `fn chi_squared_test(observed: &[f64], expected: &[f64]) → f64` | Chi-squared goodness of fit | `statistics::hypothesis` |
| `anova_one_way` | `fn anova_one_way(groups: &[&[f64]]) → f64` | One-way ANOVA F-statistic | `statistics::hypothesis` |
| `mann_whitney_u` | `fn mann_whitney_u(data1: &[f64], data2: &[f64]) → f64` | Mann-Whitney U statistic | `statistics::hypothesis` |
| `regularized_incomplete_beta` | `fn regularized_incomplete_beta(a: f64, b: f64, x: f64) → f64` | Regularized incomplete beta function | `statistics::hypothesis` |
| `cf_beta` | `fn cf_beta(a: f64, b: f64, x: f64, max_iter: usize) → f64` | Continued fraction for beta | `statistics::hypothesis` |
| `regularized_gamma_lower` | `fn regularized_gamma_lower(a: f64, x: f64) → f64` | Regularized lower gamma | `statistics::hypothesis` |
| `paired_t_test` | `fn paired_t_test(data1: &[f64], data2: &[f64]) → f64` | Paired t-test statistic | `statistics::hypothesis` |
| `welch_t_test` | `fn welch_t_test(data1: &[f64], data2: &[f64]) → f64` | Welch's t-test (unequal variances) | `statistics::hypothesis` |
| `kruskal_wallis` | `fn kruskal_wallis(groups: &[&[f64]]) → f64` | Kruskal-Wallis H-test | `statistics::hypothesis` |
| `kolmogorov_smirnov_statistic` | `fn kolmogorov_smirnov_statistic(data1: &[f64], data2: &[f64]) → f64` | Two-sample K-S statistic | `statistics::hypothesis` |
| `levene_test` | `fn levene_test(groups: &[&[f64]]) → f64` | Levene's test for equality of variances | `statistics::hypothesis` |
| `fisher_exact_2x2` | `fn fisher_exact_2x2(a: u64, b: u64, c: u64, d: u64) → f64` | Fisher exact test for 2×2 table | `statistics::hypothesis` |
| `spearman_rank_correlation` | `fn spearman_rank_correlation(x: &[f64], y: &[f64]) → f64` | Spearman rank correlation | `statistics::hypothesis` |
| `kendall_tau` | `fn kendall_tau(x: &[f64], y: &[f64]) → f64` | Kendall's tau rank correlation | `statistics::hypothesis` |
| `linear_regression` | `fn linear_regression(x: &[f64], y: &[f64]) → (f64, f64)` | Simple linear regression (slope, intercept) | `statistics::regression` |
| `standard_error_slope` | `fn standard_error_slope(x: &[f64], y: &[f64], slope: f64, intercept: f64) → f64` | Standard error of slope | `statistics::regression` |
| `r_squared` | `fn r_squared(x: &[f64], y: &[f64], slope: f64, intercept: f64) → f64` | Coefficient of determination | `statistics::regression` |
| `polynomial_regression` | `fn polynomial_regression(x: &[f64], y: &[f64], degree: usize) → Vec<f64>` | Polynomial regression coefficients | `statistics::regression` |
| `exponential_regression` | `fn exponential_regression(x: &[f64], y: &[f64]) → (f64, f64)` | Exponential regression y = ae^(bx) | `statistics::regression` |
| `multiple_linear_regression` | `fn multiple_linear_regression(x: &[Vec<f64>], y: &[f64]) → Vec<f64>` | Multiple linear regression | `statistics::regression` |
| `logarithmic_regression` | `fn logarithmic_regression(x: &[f64], y: &[f64]) → (f64, f64)` | Logarithmic regression y = a + b·ln(x) | `statistics::regression` |
| `power_regression` | `fn power_regression(x: &[f64], y: &[f64]) → (f64, f64)` | Power regression y = ax^b | `statistics::regression` |
| `adjusted_r_squared` | `fn adjusted_r_squared(r2: f64, n: usize, p: usize) → f64` | Adjusted R² | `statistics::regression` |
| `residuals` | `fn residuals(x: &[f64], y: &[f64], slope: f64, intercept: f64) → Vec<f64>` | Regression residuals | `statistics::regression` |
| `sum_of_squared_residuals` | `fn sum_of_squared_residuals(x: &[f64], y: &[f64], slope: f64, intercept: f64) → f64` | Sum of squared residuals (SSR) | `statistics::regression` |
| `mean_squared_error` | `fn mean_squared_error(x: &[f64], y: &[f64], slope: f64, intercept: f64) → f64` | Mean squared error | `statistics::regression` |
| `root_mean_squared_error` | `fn root_mean_squared_error(x: &[f64], y: &[f64], slope: f64, intercept: f64) → f64` | Root mean squared error | `statistics::regression` |
| `durbin_watson` | `fn durbin_watson(residuals: &[f64]) → f64` | Durbin-Watson autocorrelation statistic | `statistics::regression` |
| `leverage_hat` | `fn leverage_hat(x: &[f64]) → Vec<f64>` | Leverage (hat matrix diagonal) | `statistics::regression` |
| `cook_distance` | `fn cook_distance(x: &[f64], y: &[f64], slope: f64, intercept: f64) → Vec<f64>` | Cook's distance for influence | `statistics::regression` |
| `aic` | `fn aic(n: usize, k: usize, rss: f64) → f64` | Akaike information criterion | `statistics::regression` |
| `bic` | `fn bic(n: usize, k: usize, rss: f64) → f64` | Bayesian information criterion | `statistics::regression` |
| `ridge_regression` | `fn ridge_regression(x: &[Vec<f64>], y: &[f64], lambda: f64) → Vec<f64>` | Ridge regression (L2 regularization) | `statistics::regression` |

### 14. Probability — maths::probability — 74 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `uniform_pdf` | `fn uniform_pdf(x: f64, a: f64, b: f64) → f64` | Uniform PDF | `probability::distributions` |
| `uniform_cdf` | `fn uniform_cdf(x: f64, a: f64, b: f64) → f64` | Uniform CDF | `probability::distributions` |
| `normal_pdf` | `fn normal_pdf(x: f64, mu: f64, sigma: f64) → f64` | Normal distribution PDF | `probability::distributions` |
| `normal_cdf` | `fn normal_cdf(x: f64, mu: f64, sigma: f64) → f64` | Normal distribution CDF | `probability::distributions` |
| `exponential_pdf` | `fn exponential_pdf(x: f64, lambda: f64) → f64` | Exponential PDF | `probability::distributions` |
| `exponential_cdf` | `fn exponential_cdf(x: f64, lambda: f64) → f64` | Exponential CDF | `probability::distributions` |
| `poisson_pmf` | `fn poisson_pmf(k: u64, lambda: f64) → f64` | Poisson PMF | `probability::distributions` |
| `binomial_pmf` | `fn binomial_pmf(k: u64, n: u64, p: f64) → f64` | Binomial PMF | `probability::distributions` |
| `geometric_pmf` | `fn geometric_pmf(k: u64, p: f64) → f64` | Geometric PMF | `probability::distributions` |
| `gamma_pdf` | `fn gamma_pdf(x: f64, alpha: f64, beta: f64) → f64` | Gamma PDF | `probability::distributions` |
| `beta_pdf` | `fn beta_pdf(x: f64, alpha: f64, beta: f64) → f64` | Beta PDF | `probability::distributions` |
| `cauchy_pdf` | `fn cauchy_pdf(x: f64, x0: f64, gamma: f64) → f64` | Cauchy PDF | `probability::distributions` |
| `chi_squared_pdf` | `fn chi_squared_pdf(x: f64, k: u64) → f64` | Chi-squared PDF | `probability::distributions` |
| `weibull_pdf` | `fn weibull_pdf(x: f64, k: f64, lambda: f64) → f64` | Weibull PDF | `probability::distributions` |
| `weibull_cdf` | `fn weibull_cdf(x: f64, k: f64, lambda: f64) → f64` | Weibull CDF | `probability::distributions` |
| `log_normal_pdf` | `fn log_normal_pdf(x: f64, mu: f64, sigma: f64) → f64` | Log-normal PDF | `probability::distributions` |
| `student_t_pdf` | `fn student_t_pdf(x: f64, nu: f64) → f64` | Student's t PDF | `probability::distributions` |
| `pareto_pdf` | `fn pareto_pdf(x: f64, x_m: f64, alpha: f64) → f64` | Pareto PDF | `probability::distributions` |
| `pareto_cdf` | `fn pareto_cdf(x: f64, x_m: f64, alpha: f64) → f64` | Pareto CDF | `probability::distributions` |
| `laplace_pdf` | `fn laplace_pdf(x: f64, mu: f64, b: f64) → f64` | Laplace PDF | `probability::distributions` |
| `rayleigh_pdf` | `fn rayleigh_pdf(x: f64, sigma: f64) → f64` | Rayleigh PDF | `probability::distributions` |
| `rayleigh_cdf` | `fn rayleigh_cdf(x: f64, sigma: f64) → f64` | Rayleigh CDF | `probability::distributions` |
| `negative_binomial_pmf` | `fn negative_binomial_pmf(k: u64, r: u64, p: f64) → f64` | Negative binomial PMF | `probability::distributions` |
| `hypergeometric_pmf` | `fn hypergeometric_pmf(k: u64, n_pop: u64, k_pop: u64, n: u64) → f64` | Hypergeometric PMF | `probability::distributions` |
| `entropy_discrete` | `fn entropy_discrete(probs: &[f64]) → f64` | Discrete entropy | `probability::distributions` |
| `kl_divergence` | `fn kl_divergence(p: &[f64], q: &[f64]) → f64` | Kullback-Leibler divergence | `probability::distributions` |
| `transition_probability` | `fn transition_probability(matrix: &[Vec<f64>], from: usize, to: usize) → f64` | Single transition probability | `probability::markov` |
| `state_after_n_steps` | `fn state_after_n_steps(matrix: &[Vec<f64>], initial: &[f64], n: usize) → Vec<f64>` | State distribution after n steps | `probability::markov` |
| `steady_state` | `fn steady_state(matrix: &[Vec<f64>], max_iter: usize, tol: f64) → Vec<f64>` | Stationary distribution | `probability::markov` |
| `is_ergodic` | `fn is_ergodic(matrix: &[Vec<f64>]) → bool` | Ergodicity check | `probability::markov` |
| `absorbing_states` | `fn absorbing_states(matrix: &[Vec<f64>]) → Vec<usize>` | Absorbing state indices | `probability::markov` |
| `expected_visits` | `fn expected_visits(matrix: &[Vec<f64>], start: usize, steps: usize) → Vec<f64>` | Expected visits per state | `probability::markov` |
| `mean_first_passage_time` | `fn mean_first_passage_time(matrix: &[Vec<f64>], from: usize, to: usize) → f64` | Mean first passage time | `probability::markov` |
| `communicate_classes` | `fn communicate_classes(matrix: &[Vec<f64>]) → Vec<Vec<usize>>` | Communication classes | `probability::markov` |
| `transition_matrix_power` | `fn transition_matrix_power(matrix: &[Vec<f64>], n: usize) → Vec<Vec<f64>>` | n-step transition matrix | `probability::markov` |
| `hitting_probability` | `fn hitting_probability(matrix: &[Vec<f64>], from: usize, target: usize) → f64` | Hitting probability | `probability::markov` |
| `birth_death_steady_state` | `fn birth_death_steady_state(birth_rates: &[f64], death_rates: &[f64]) → Vec<f64>` | Birth-death steady state | `probability::markov` |
| `markov_chain_entropy_rate` | `fn markov_chain_entropy_rate(matrix: &[Vec<f64>], stationary: &[f64]) → f64` | Entropy rate of Markov chain | `probability::markov` |
| `is_doubly_stochastic` | `fn is_doubly_stochastic(matrix: &[Vec<f64>], tol: f64) → bool` | Doubly stochastic check | `probability::markov` |
| `mixing_time_estimate` | `fn mixing_time_estimate(matrix: &[Vec<f64>], epsilon: f64, max_iter: usize) → usize` | Estimated mixing time | `probability::markov` |
| `monte_carlo_integrate` | `fn monte_carlo_integrate(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Monte Carlo integration | `probability::monte_carlo` |
| `monte_carlo_pi` | `fn monte_carlo_pi(n: usize) → f64` | Estimate π via Monte Carlo | `probability::monte_carlo` |
| `importance_sampling` | `fn importance_sampling(f: impl Fn(f64) → f64, g: impl Fn(f64) → f64, sample_g: impl Fn() → f64, n: usize) → f64` | Importance sampling estimate | `probability::monte_carlo` |
| `metropolis_hastings` | `fn metropolis_hastings(target: impl Fn(f64) → f64, proposal_std: f64, x0: f64, n: usize) → Vec<f64>` | Metropolis-Hastings sampling | `probability::monte_carlo` |
| `rejection_sampling` | `fn rejection_sampling(target: impl Fn(f64) → f64, proposal: impl Fn() → f64, proposal_pdf: impl Fn(f64) → f64, m: f64, n: usize) → Vec<f64>` | Rejection sampling | `probability::monte_carlo` |
| `bootstrap_mean` | `fn bootstrap_mean(data: &[f64], n_resamples: usize) → (f64, f64)` | Bootstrap mean and standard error | `probability::monte_carlo` |
| `McRng` | `struct { state: u64 }` | Simple pseudo-random number generator | `probability::monte_carlo` |
| `McRng::new` | `fn new(seed: u64) → Self` | Construct with seed | `probability::monte_carlo` |
| `McRng::next_u64` | `fn next_u64(&mut self) → u64` | Next random u64 | `probability::monte_carlo` |
| `McRng::next_f64` | `fn next_f64(&mut self) → f64` | Next random f64 in [0, 1) | `probability::monte_carlo` |
| `McRng::next_usize` | `fn next_usize(&mut self, max: usize) → usize` | Next random usize in [0, max) | `probability::monte_carlo` |
| `monte_carlo_integrate_nd` | `fn monte_carlo_integrate_nd(f: impl Fn(&[f64]) → f64, bounds: &[(f64, f64)], n: usize) → f64` | N-dimensional Monte Carlo integration | `probability::monte_carlo` |
| `monte_carlo_variance` | `fn monte_carlo_variance(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → (f64, f64)` | Monte Carlo with variance estimate | `probability::monte_carlo` |
| `antithetic_variates` | `fn antithetic_variates(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Antithetic variates variance reduction | `probability::monte_carlo` |
| `control_variates` | `fn control_variates(f: impl Fn(f64) → f64, g: impl Fn(f64) → f64, eg: f64, a: f64, b: f64, n: usize) → f64` | Control variates variance reduction | `probability::monte_carlo` |
| `quasi_monte_carlo_integrate` | `fn quasi_monte_carlo_integrate(f: impl Fn(f64) → f64, a: f64, b: f64, n: usize) → f64` | Quasi-Monte Carlo via Halton | `probability::monte_carlo` |
| `gibbs_sampler_2d` | `fn gibbs_sampler_2d(cond_x: impl Fn(f64) → f64, cond_y: impl Fn(f64) → f64, x0: f64, y0: f64, n: usize) → Vec<(f64, f64)>` | 2D Gibbs sampler | `probability::monte_carlo` |
| `slice_sampling` | `fn slice_sampling(target: impl Fn(f64) → f64, x0: f64, w: f64, n: usize) → Vec<f64>` | Slice sampling | `probability::monte_carlo` |
| `permutation_test` | `fn permutation_test(data1: &[f64], data2: &[f64], n_permutations: usize) → f64` | Permutation test p-value | `probability::monte_carlo` |
| `latin_hypercube` | `fn latin_hypercube(n: usize, dim: usize) → Vec<Vec<f64>>` | Latin hypercube sampling | `probability::sampling` |
| `stratified_sampling` | `fn stratified_sampling(f: impl Fn(f64) → f64, a: f64, b: f64, n_strata: usize) → Vec<f64>` | Stratified sampling | `probability::sampling` |
| `sobol_sequence_1d` | `fn sobol_sequence_1d(n: usize) → Vec<f64>` | 1D Sobol quasi-random sequence | `probability::sampling` |
| `halton_sequence` | `fn halton_sequence(index: usize, base: usize) → f64` | Halton quasi-random sequence | `probability::sampling` |
| `inverse_transform_exponential` | `fn inverse_transform_exponential(u: f64, lambda: f64) → f64` | Inverse CDF for exponential | `probability::sampling` |
| `box_muller` | `fn box_muller(u1: f64, u2: f64) → (f64, f64)` | Box-Muller normal generation | `probability::sampling` |
| `reservoir_sampling` | `fn reservoir_sampling(data: &[f64], k: usize) → Vec<f64>` | Reservoir sampling | `probability::sampling` |
| `alias_table_build` | `fn alias_table_build(probs: &[f64]) → (Vec<f64>, Vec<usize>)` | Alias table construction | `probability::sampling` |
| `alias_sample` | `fn alias_sample(prob: &[f64], alias: &[usize], u: f64, v: f64) → usize` | Sample from alias table | `probability::sampling` |
| `systematic_sampling` | `fn systematic_sampling(weights: &[f64], n: usize) → Vec<usize>` | Systematic resampling | `probability::sampling` |
| `importance_resampling` | `fn importance_resampling(weights: &[f64], n: usize) → Vec<usize>` | Importance resampling indices | `probability::sampling` |
| `van_der_corput` | `fn van_der_corput(n: usize, base: usize) → f64` | Van der Corput sequence | `probability::sampling` |
| `hammersley_sequence` | `fn hammersley_sequence(i: usize, n: usize) → (f64, f64)` | 2D Hammersley point set | `probability::sampling` |
| `weighted_sampling` | `fn weighted_sampling(weights: &[f64], n: usize) → Vec<usize>` | Weighted sampling with replacement | `probability::sampling` |
| `poisson_disk_sampling_1d` | `fn poisson_disk_sampling_1d(min_dist: f64, domain: (f64, f64), max_attempts: usize) → Vec<f64>` | 1D Poisson disk sampling | `probability::sampling` |

### 15. Signal Processing — maths::signal — 70 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `convolve_direct` | `fn convolve_direct(signal: &[f64], kernel: &[f64]) → Vec<f64>` | Direct convolution | `signal::convolution` |
| `correlate` | `fn correlate(a: &[f64], b: &[f64]) → Vec<f64>` | Cross-correlation | `signal::convolution` |
| `autocorrelate` | `fn autocorrelate(signal: &[f64]) → Vec<f64>` | Auto-correlation | `signal::convolution` |
| `deconvolve` | `fn deconvolve(signal: &[f64], kernel: &[f64]) → Vec<f64>` | Deconvolution via FFT | `signal::convolution` |
| `convolve_2d` | `fn convolve_2d(image: &[Vec<f64>], kernel: &[Vec<f64>]) → Vec<Vec<f64>>` | 2D convolution | `signal::convolution` |
| `sobel_x` | `fn sobel_x(image: &[Vec<f64>]) → Vec<Vec<f64>>` | Sobel horizontal edge detector | `signal::convolution` |
| `sobel_y` | `fn sobel_y(image: &[Vec<f64>]) → Vec<Vec<f64>>` | Sobel vertical edge detector | `signal::convolution` |
| `gaussian_blur_2d` | `fn gaussian_blur_2d(image: &[Vec<f64>], sigma: f64, kernel_size: usize) → Vec<Vec<f64>>` | 2D Gaussian blur | `signal::convolution` |
| `laplacian_kernel` | `fn laplacian_kernel(image: &[Vec<f64>]) → Vec<Vec<f64>>` | Laplacian edge detection | `signal::convolution` |
| `box_blur` | `fn box_blur(image: &[Vec<f64>], size: usize) → Vec<Vec<f64>>` | Box blur filter | `signal::convolution` |
| `sharpen` | `fn sharpen(image: &[Vec<f64>]) → Vec<Vec<f64>>` | Unsharp mask sharpening | `signal::convolution` |
| `emboss` | `fn emboss(image: &[Vec<f64>]) → Vec<Vec<f64>>` | Emboss filter | `signal::convolution` |
| `prewitt_x` | `fn prewitt_x(image: &[Vec<f64>]) → Vec<Vec<f64>>` | Prewitt horizontal detector | `signal::convolution` |
| `prewitt_y` | `fn prewitt_y(image: &[Vec<f64>]) → Vec<Vec<f64>>` | Prewitt vertical detector | `signal::convolution` |
| `separable_convolve_2d` | `fn separable_convolve_2d(image: &[Vec<f64>], row_kernel: &[f64], col_kernel: &[f64]) → Vec<Vec<f64>>` | Separable 2D convolution | `signal::convolution` |
| `low_pass_rc` | `fn low_pass_rc(signal: &[f64], dt: f64, rc: f64) → Vec<f64>` | RC low-pass filter | `signal::filters` |
| `high_pass_rc` | `fn high_pass_rc(signal: &[f64], dt: f64, rc: f64) → Vec<f64>` | RC high-pass filter | `signal::filters` |
| `band_pass` | `fn band_pass(signal: &[f64], dt: f64, low_rc: f64, high_rc: f64) → Vec<f64>` | Band-pass filter | `signal::filters` |
| `notch_filter` | `fn notch_filter(signal: &[f64], dt: f64, freq: f64, q: f64) → Vec<f64>` | Notch (band-reject) filter | `signal::filters` |
| `butterworth_low_pass` | `fn butterworth_low_pass(signal: &[f64], dt: f64, cutoff: f64, order: usize) → Vec<f64>` | Butterworth low-pass filter | `signal::filters` |
| `chebyshev_low_pass` | `fn chebyshev_low_pass(signal: &[f64], dt: f64, cutoff: f64, ripple: f64) → Vec<f64>` | Chebyshev type-I low-pass | `signal::filters` |
| `median_filter` | `fn median_filter(signal: &[f64], window: usize) → Vec<f64>` | Median filter | `signal::filters` |
| `wiener_filter` | `fn wiener_filter(signal: &[f64], noise_power: f64) → Vec<f64>` | Wiener noise reduction | `signal::filters` |
| `savitzky_golay` | `fn savitzky_golay(signal: &[f64], window: usize, order: usize) → Vec<f64>` | Savitzky-Golay smoothing | `signal::filters` |
| `kalman_filter_1d` | `fn kalman_filter_1d(measurements: &[f64], q: f64, r: f64, x0: f64, p0: f64) → Vec<f64>` | 1D Kalman filter | `signal::filters` |
| `exponential_smoothing` | `fn exponential_smoothing(signal: &[f64], alpha: f64) → Vec<f64>` | Exponential smoothing | `signal::filters` |
| `moving_average_filter` | `fn moving_average_filter(signal: &[f64], window: usize) → Vec<f64>` | Moving average filter | `signal::filters` |
| `iir_filter` | `fn iir_filter(signal: &[f64], b: &[f64], a: &[f64]) → Vec<f64>` | Generic IIR filter | `signal::filters` |
| `fir_filter` | `fn fir_filter(signal: &[f64], coeffs: &[f64]) → Vec<f64>` | Generic FIR filter | `signal::filters` |
| `bessel_low_pass` | `fn bessel_low_pass(signal: &[f64], dt: f64, cutoff: f64) → Vec<f64>` | Bessel low-pass filter | `signal::filters` |
| `all_pass_filter` | `fn all_pass_filter(signal: &[f64], coeff: f64) → Vec<f64>` | All-pass filter | `signal::filters` |
| `comb_filter` | `fn comb_filter(signal: &[f64], delay: usize, gain: f64) → Vec<f64>` | Comb filter | `signal::filters` |
| `power_spectral_density` | `fn power_spectral_density(signal: &[f64], sample_rate: f64) → Vec<(f64, f64)>` | Power spectral density via Welch | `signal::spectral` |
| `dft` | `fn dft(signal: &[f64]) → Vec<(f64, f64)>` | Discrete Fourier transform | `signal::spectral` |
| `idft` | `fn idft(spectrum: &[(f64, f64)]) → Vec<f64>` | Inverse DFT | `signal::spectral` |
| `magnitude_spectrum` | `fn magnitude_spectrum(spectrum: &[(f64, f64)]) → Vec<f64>` | Magnitude spectrum | `signal::spectral` |
| `phase_spectrum` | `fn phase_spectrum(spectrum: &[(f64, f64)]) → Vec<f64>` | Phase spectrum | `signal::spectral` |
| `frequency_bins` | `fn frequency_bins(n: usize, sample_rate: f64) → Vec<f64>` | Frequency axis for DFT | `signal::spectral` |
| `spectral_centroid` | `fn spectral_centroid(spectrum: &[f64], sample_rate: f64) → f64` | Spectral centroid | `signal::spectral` |
| `spectral_bandwidth` | `fn spectral_bandwidth(spectrum: &[f64], sample_rate: f64) → f64` | Spectral bandwidth | `signal::spectral` |
| `spectral_rolloff` | `fn spectral_rolloff(spectrum: &[f64], sample_rate: f64, threshold: f64) → f64` | Spectral rolloff | `signal::spectral` |
| `windowed_signal` | `fn windowed_signal(signal: &[f64], window: &[f64]) → Vec<f64>` | Apply window function | `signal::spectral` |
| `hanning_window` | `fn hanning_window(n: usize) → Vec<f64>` | Hanning window | `signal::spectral` |
| `hamming_window` | `fn hamming_window(n: usize) → Vec<f64>` | Hamming window | `signal::spectral` |
| `blackman_window` | `fn blackman_window(n: usize) → Vec<f64>` | Blackman window | `signal::spectral` |
| `kaiser_window` | `fn kaiser_window(n: usize, beta: f64) → Vec<f64>` | Kaiser window | `signal::spectral` |
| `stft` | `fn stft(signal: &[f64], window_size: usize, hop_size: usize) → Vec<Vec<(f64, f64)>>` | Short-time Fourier transform | `signal::spectral` |
| `cross_spectral_density` | `fn cross_spectral_density(x: &[f64], y: &[f64], sample_rate: f64) → Vec<(f64, f64)>` | Cross-spectral density | `signal::spectral` |
| `spectral_flatness` | `fn spectral_flatness(spectrum: &[f64]) → f64` | Spectral flatness | `signal::spectral` |
| `spectral_flux` | `fn spectral_flux(prev: &[f64], curr: &[f64]) → f64` | Spectral flux | `signal::spectral` |
| `bark_scale` | `fn bark_scale(f: f64) → f64` | Frequency to Bark scale | `signal::spectral` |
| `mel_scale` | `fn mel_scale(f: f64) → f64` | Frequency to Mel scale | `signal::spectral` |
| `inverse_mel` | `fn inverse_mel(m: f64) → f64` | Mel to frequency | `signal::spectral` |
| `haar_transform` | `fn haar_transform(signal: &[f64]) → Vec<f64>` | Haar wavelet transform | `signal::wavelets` |
| `haar_inverse` | `fn haar_inverse(coeffs: &[f64]) → Vec<f64>` | Inverse Haar transform | `signal::wavelets` |
| `db4_scaling` | `fn db4_scaling() → [f64; 8]` | Daubechies-4 scaling coefficients | `signal::wavelets` |
| `wavelet_energy` | `fn wavelet_energy(coeffs: &[f64]) → f64` | Wavelet coefficient energy | `signal::wavelets` |
| `wavelet_entropy` | `fn wavelet_entropy(coeffs: &[f64]) → f64` | Wavelet entropy | `signal::wavelets` |
| `threshold_hard` | `fn threshold_hard(coeffs: &[f64], threshold: f64) → Vec<f64>` | Hard thresholding | `signal::wavelets` |
| `threshold_soft` | `fn threshold_soft(coeffs: &[f64], threshold: f64) → Vec<f64>` | Soft thresholding | `signal::wavelets` |
| `universal_threshold` | `fn universal_threshold(n: usize, sigma: f64) → f64` | Universal (VisuShrink) threshold | `signal::wavelets` |
| `noise_estimate_mad` | `fn noise_estimate_mad(coeffs: &[f64]) → f64` | Noise estimate via MAD | `signal::wavelets` |
| `db2_scaling` | `fn db2_scaling() → [f64; 4]` | Daubechies-2 scaling coefficients | `signal::wavelets` |
| `morlet_wavelet` | `fn morlet_wavelet(t: f64, omega0: f64) → f64` | Morlet wavelet function | `signal::wavelets` |
| `mexican_hat_wavelet` | `fn mexican_hat_wavelet(t: f64) → f64` | Mexican hat (Ricker) wavelet | `signal::wavelets` |
| `continuous_wavelet_transform_morlet` | `fn continuous_wavelet_transform_morlet(signal: &[f64], scales: &[f64], omega0: f64) → Vec<Vec<f64>>` | CWT with Morlet wavelet | `signal::wavelets` |
| `multiresolution_decomposition` | `fn multiresolution_decomposition(signal: &[f64], levels: usize) → Vec<Vec<f64>>` | Multi-resolution analysis | `signal::wavelets` |
| `wavelet_reconstruction_haar` | `fn wavelet_reconstruction_haar(levels: &[Vec<f64>]) → Vec<f64>` | Reconstruct from Haar decomposition | `signal::wavelets` |
| `wavelet_shrinkage_denoise` | `fn wavelet_shrinkage_denoise(signal: &[f64], levels: usize) → Vec<f64>` | Wavelet shrinkage denoising | `signal::wavelets` |
| `scalogram_energy` | `fn scalogram_energy(cwt: &[Vec<f64>]) → Vec<f64>` | Scalogram energy per scale | `signal::wavelets` |

### 16. Graph Theory — maths::graph — 58 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `ford_fulkerson` | `fn ford_fulkerson(capacity: &[Vec<f64>], source: usize, sink: usize) → f64` | Ford-Fulkerson max flow | `graph::flow` |
| `min_cut` | `fn min_cut(capacity: &[Vec<f64>], source: usize, sink: usize) → Vec<(usize, usize)>` | Minimum cut edges | `graph::flow` |
| `bipartite_matching` | `fn bipartite_matching(adj: &[Vec<usize>], n_left: usize, n_right: usize) → Vec<(usize, usize)>` | Maximum bipartite matching | `graph::flow` |
| `min_cost_flow` | `fn min_cost_flow(capacity: &[Vec<f64>], cost: &[Vec<f64>], source: usize, sink: usize, target_flow: f64) → (f64, f64)` | Minimum cost flow | `graph::flow` |
| `edmonds_karp` | `fn edmonds_karp(capacity: &[Vec<f64>], source: usize, sink: usize) → f64` | Edmonds-Karp max flow | `graph::flow` |
| `edge_connectivity` | `fn edge_connectivity(capacity: &[Vec<f64>]) → f64` | Edge connectivity | `graph::flow` |
| `vertex_connectivity` | `fn vertex_connectivity(adj: &[Vec<usize>], n: usize) → usize` | Vertex connectivity | `graph::flow` |
| `max_flow_scaling` | `fn max_flow_scaling(capacity: &[Vec<f64>], source: usize, sink: usize) → f64` | Capacity-scaling max flow | `graph::flow` |
| `multi_source_multi_sink_flow` | `fn multi_source_multi_sink_flow(capacity: &[Vec<f64>], sources: &[usize], sinks: &[usize]) → f64` | Multi-source multi-sink flow | `graph::flow` |
| `circulation_demand` | `fn circulation_demand(capacity: &[Vec<f64>], demand: &[f64]) → Option<Vec<Vec<f64>>>` | Circulation with demands | `graph::flow` |
| `residual_graph` | `fn residual_graph(capacity: &[Vec<f64>], flow: &[Vec<f64>]) → Vec<Vec<f64>>` | Residual graph from flow | `graph::flow` |
| `matching_size` | `fn matching_size(matching: &[(usize, usize)]) → usize` | Matching cardinality | `graph::flow` |
| `is_perfect_matching` | `fn is_perfect_matching(matching: &[(usize, usize)], n_left: usize, n_right: usize) → bool` | Check perfect matching | `graph::flow` |
| `dijkstra` | `fn dijkstra(adj: &[Vec<(usize, f64)>], source: usize) → Vec<f64>` | Dijkstra's shortest path | `graph::shortest_path` |
| `bellman_ford` | `fn bellman_ford(edges: &[(usize, usize, f64)], n: usize, source: usize) → Option<Vec<f64>>` | Bellman-Ford shortest path | `graph::shortest_path` |
| `floyd_warshall` | `fn floyd_warshall(adj: &[Vec<f64>]) → Vec<Vec<f64>>` | All-pairs shortest paths | `graph::shortest_path` |
| `dijkstra_path` | `fn dijkstra_path(adj: &[Vec<(usize, f64)>], source: usize, target: usize) → Option<Vec<usize>>` | Shortest path reconstruction | `graph::shortest_path` |
| `a_star` | `fn a_star(adj: &[Vec<(usize, f64)>], source: usize, target: usize, heuristic: impl Fn(usize) → f64) → Option<(Vec<usize>, f64)>` | A* search algorithm | `graph::shortest_path` |
| `johnson` | `fn johnson(adj: &[Vec<(usize, f64)>], n: usize) → Option<Vec<Vec<f64>>>` | Johnson's all-pairs shortest paths | `graph::shortest_path` |
| `spfa` | `fn spfa(adj: &[Vec<(usize, f64)>], source: usize) → Option<Vec<f64>>` | Shortest Path Faster Algorithm | `graph::shortest_path` |
| `dag_shortest_path` | `fn dag_shortest_path(adj: &[Vec<(usize, f64)>], source: usize) → Vec<f64>` | DAG shortest path via topological order | `graph::shortest_path` |
| `k_shortest_paths` | `fn k_shortest_paths(adj: &[Vec<(usize, f64)>], source: usize, target: usize, k: usize) → Vec<(Vec<usize>, f64)>` | k shortest paths (Yen's algorithm) | `graph::shortest_path` |
| `closeness_centrality` | `fn closeness_centrality(adj: &[Vec<(usize, f64)>], node: usize) → f64` | Closeness centrality | `graph::shortest_path` |
| `betweenness_centrality` | `fn betweenness_centrality(adj: &[Vec<(usize, f64)>], n: usize) → Vec<f64>` | Betweenness centrality | `graph::shortest_path` |
| `all_pairs_unweighted` | `fn all_pairs_unweighted(adj: &[Vec<usize>]) → Vec<Vec<usize>>` | All-pairs BFS distances | `graph::shortest_path` |
| `diameter_weighted` | `fn diameter_weighted(adj: &[Vec<(usize, f64)>]) → f64` | Weighted graph diameter | `graph::shortest_path` |
| `negative_cycle_exists` | `fn negative_cycle_exists(edges: &[(usize, usize, f64)], n: usize) → bool` | Negative cycle detection | `graph::shortest_path` |
| `kruskal` | `fn kruskal(edges: &[(usize, usize, f64)], n: usize) → Vec<(usize, usize, f64)>` | Kruskal's MST | `graph::spanning_tree` |
| `prim` | `fn prim(adj: &[Vec<(usize, f64)>]) → Vec<(usize, usize, f64)>` | Prim's MST | `graph::spanning_tree` |
| `boruvka` | `fn boruvka(edges: &[(usize, usize, f64)], n: usize) → Vec<(usize, usize, f64)>` | Borůvka's MST | `graph::spanning_tree` |
| `is_tree` | `fn is_tree(adj: &[Vec<usize>]) → bool` | Check if graph is a tree | `graph::spanning_tree` |
| `mst_weight` | `fn mst_weight(edges: &[(usize, usize, f64)]) → f64` | Total MST weight | `graph::spanning_tree` |
| `is_connected_weighted` | `fn is_connected_weighted(adj: &[Vec<(usize, f64)>]) → bool` | Connectivity check | `graph::spanning_tree` |
| `bottleneck_mst_edge` | `fn bottleneck_mst_edge(mst: &[(usize, usize, f64)]) → f64` | Bottleneck edge weight | `graph::spanning_tree` |
| `second_best_mst` | `fn second_best_mst(adj: &[Vec<(usize, f64)>], n: usize) → Vec<(usize, usize, f64)>` | Second-best MST | `graph::spanning_tree` |
| `mst_edges_count` | `fn mst_edges_count(n: usize) → usize` | Expected MST edge count = n − 1 | `graph::spanning_tree` |
| `steiner_tree_approx` | `fn steiner_tree_approx(adj: &[Vec<(usize, f64)>], terminals: &[usize]) → Vec<(usize, usize, f64)>` | Steiner tree approximation | `graph::spanning_tree` |
| `kirchhoff_spanning_tree_count` | `fn kirchhoff_spanning_tree_count(adj: &[Vec<usize>]) → f64` | Spanning tree count via Kirchhoff | `graph::spanning_tree` |
| `tree_diameter_weighted` | `fn tree_diameter_weighted(adj: &[Vec<(usize, f64)>]) → f64` | Tree diameter (weighted) | `graph::spanning_tree` |
| `bfs` | `fn bfs(adj: &[Vec<usize>], source: usize) → Vec<usize>` | Breadth-first search | `graph::traversal` |
| `dfs` | `fn dfs(adj: &[Vec<usize>], source: usize) → Vec<usize>` | Depth-first search | `graph::traversal` |
| `topological_sort` | `fn topological_sort(adj: &[Vec<usize>]) → Option<Vec<usize>>` | Topological sort (Kahn's) | `graph::traversal` |
| `connected_components` | `fn connected_components(adj: &[Vec<usize>]) → Vec<Vec<usize>>` | Connected components | `graph::traversal` |
| `is_bipartite` | `fn is_bipartite(adj: &[Vec<usize>]) → bool` | Bipartiteness check | `graph::traversal` |
| `has_cycle_directed` | `fn has_cycle_directed(adj: &[Vec<usize>]) → bool` | Cycle detection in directed graph | `graph::traversal` |
| `bfs_distance` | `fn bfs_distance(adj: &[Vec<usize>], source: usize) → Vec<usize>` | BFS distances from source | `graph::traversal` |
| `iterative_deepening_dfs` | `fn iterative_deepening_dfs(adj: &[Vec<usize>], source: usize, target: usize) → Option<usize>` | IDDFS search depth | `graph::traversal` |
| `strongly_connected_components` | `fn strongly_connected_components(adj: &[Vec<usize>]) → Vec<Vec<usize>>` | Tarjan's SCC | `graph::traversal` |
| `articulation_points` | `fn articulation_points(adj: &[Vec<usize>]) → Vec<usize>` | Articulation points (cut vertices) | `graph::traversal` |
| `bridges` | `fn bridges(adj: &[Vec<usize>]) → Vec<(usize, usize)>` | Bridge edges | `graph::traversal` |
| `graph_diameter` | `fn graph_diameter(adj: &[Vec<usize>]) → usize` | Unweighted diameter | `graph::traversal` |
| `eccentricity` | `fn eccentricity(adj: &[Vec<usize>], node: usize) → usize` | Node eccentricity | `graph::traversal` |
| `graph_center` | `fn graph_center(adj: &[Vec<usize>]) → Vec<usize>` | Graph center nodes | `graph::traversal` |
| `degree_sequence` | `fn degree_sequence(adj: &[Vec<usize>]) → Vec<usize>` | Sorted degree sequence | `graph::traversal` |
| `clustering_coefficient` | `fn clustering_coefficient(adj: &[Vec<usize>], node: usize) → f64` | Local clustering coefficient | `graph::traversal` |
| `pagerank` | `fn pagerank(adj: &[Vec<usize>], damping: f64, max_iter: usize, tol: f64) → Vec<f64>` | PageRank algorithm | `graph::traversal` |
| `bidirectional_bfs` | `fn bidirectional_bfs(adj: &[Vec<usize>], source: usize, target: usize) → Option<Vec<usize>>` | Bidirectional BFS | `graph::traversal` |
| `has_cycle_undirected` | `fn has_cycle_undirected(adj: &[Vec<usize>]) → bool` | Cycle detection in undirected graph | `graph::traversal` |

### 17. Non-Euclidean Geometry — maths::non_euclidean — 80 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| `schwarzschild_radius` | `fn schwarzschild_radius(mass: f64) → f64` | Schwarzschild radius $r_s = 2GM/c^2$ | `non_euclidean::black_hole` |
| `proper_time_schwarzschild` | `fn proper_time_schwarzschild(r: f64, rs: f64, dt: f64) → f64` | Proper time in Schwarzschild metric | `non_euclidean::black_hole` |
| `gravitational_redshift` | `fn gravitational_redshift(r_emit: f64, r_obs: f64, rs: f64) → f64` | Gravitational redshift factor | `non_euclidean::black_hole` |
| `kerr_ergosphere_radius` | `fn kerr_ergosphere_radius(m: f64, a: f64, theta: f64) → f64` | Kerr ergosphere radius | `non_euclidean::black_hole` |
| `isco_schwarzschild` | `fn isco_schwarzschild(mass: f64) → f64` | Innermost stable circular orbit | `non_euclidean::black_hole` |
| `photon_sphere_radius` | `fn photon_sphere_radius(mass: f64) → f64` | Photon sphere radius $r = 3GM/c^2$ | `non_euclidean::black_hole` |
| `hawking_temperature` | `fn hawking_temperature(mass: f64) → f64` | Hawking temperature | `non_euclidean::black_hole` |
| `black_hole_entropy` | `fn black_hole_entropy(mass: f64) → f64` | Bekenstein-Hawking entropy | `non_euclidean::black_hole` |
| `kerr_event_horizon` | `fn kerr_event_horizon(m: f64, a: f64) → f64` | Kerr outer event horizon | `non_euclidean::black_hole` |
| `kerr_cauchy_horizon` | `fn kerr_cauchy_horizon(m: f64, a: f64) → f64` | Kerr inner (Cauchy) horizon | `non_euclidean::black_hole` |
| `schwarzschild_time_dilation` | `fn schwarzschild_time_dilation(r: f64, rs: f64) → f64` | Time dilation factor | `non_euclidean::black_hole` |
| `gravitational_lensing_angle` | `fn gravitational_lensing_angle(mass: f64, b: f64) → f64` | Gravitational lensing deflection | `non_euclidean::black_hole` |
| `orbital_velocity_schwarzschild` | `fn orbital_velocity_schwarzschild(r: f64, rs: f64) → f64` | Circular orbital velocity | `non_euclidean::black_hole` |
| `tidal_force_schwarzschild` | `fn tidal_force_schwarzschild(mass_bh: f64, r: f64, delta_r: f64) → f64` | Tidal force gradient | `non_euclidean::black_hole` |
| `hawking_luminosity` | `fn hawking_luminosity(mass: f64) → f64` | Hawking radiation luminosity | `non_euclidean::black_hole` |
| `black_hole_evaporation_time` | `fn black_hole_evaporation_time(mass: f64) → f64` | Evaporation time | `non_euclidean::black_hole` |
| `bekenstein_bound` | `fn bekenstein_bound(energy: f64, radius: f64) → f64` | Bekenstein entropy bound | `non_euclidean::black_hole` |
| `penrose_energy_extraction` | `fn penrose_energy_extraction(m: f64, a: f64) → f64` | Penrose process max efficiency | `non_euclidean::black_hole` |
| `frame_dragging_rate` | `fn frame_dragging_rate(m: f64, a: f64, r: f64) → f64` | Frame-dragging angular velocity | `non_euclidean::black_hole` |
| `reissner_nordstrom_outer_horizon` | `fn reissner_nordstrom_outer_horizon(m: f64, q: f64) → f64` | Reissner-Nordström outer horizon | `non_euclidean::black_hole` |
| `flrw_scale_factor` | `fn flrw_scale_factor(t: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | FLRW scale factor a(t) | `non_euclidean::cosmology` |
| `hubble_parameter` | `fn hubble_parameter(z: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Hubble parameter H(z) | `non_euclidean::cosmology` |
| `comoving_distance` | `fn comoving_distance(z: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Comoving distance to redshift z | `non_euclidean::cosmology` |
| `luminosity_distance` | `fn luminosity_distance(z: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Luminosity distance | `non_euclidean::cosmology` |
| `angular_diameter_distance` | `fn angular_diameter_distance(z: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Angular diameter distance | `non_euclidean::cosmology` |
| `lookback_time` | `fn lookback_time(z: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Lookback time | `non_euclidean::cosmology` |
| `distance_modulus` | `fn distance_modulus(d_l: f64) → f64` | Distance modulus μ | `non_euclidean::cosmology` |
| `age_of_universe` | `fn age_of_universe(h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Age of universe | `non_euclidean::cosmology` |
| `critical_density` | `fn critical_density(h: f64) → f64` | Critical density ρ_c | `non_euclidean::cosmology` |
| `deceleration_parameter` | `fn deceleration_parameter(omega_m: f64, omega_lambda: f64) → f64` | Deceleration parameter q₀ | `non_euclidean::cosmology` |
| `cosmic_time_matter_dominated` | `fn cosmic_time_matter_dominated(a: f64, h0: f64) → f64` | Cosmic time (matter dominated) | `non_euclidean::cosmology` |
| `horizon_distance` | `fn horizon_distance(t: f64, h0: f64) → f64` | Particle horizon distance | `non_euclidean::cosmology` |
| `dark_energy_equation_of_state` | `fn dark_energy_equation_of_state(w0: f64, wa: f64, a: f64) → f64` | Dark energy equation of state w(a) | `non_euclidean::cosmology` |
| `proper_distance` | `fn proper_distance(z: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Proper distance | `non_euclidean::cosmology` |
| `volume_comoving` | `fn volume_comoving(z: f64, h0: f64, omega_m: f64, omega_lambda: f64) → f64` | Comoving volume element | `non_euclidean::cosmology` |
| `cmb_temperature_at_redshift` | `fn cmb_temperature_at_redshift(z: f64) → f64` | CMB temperature at redshift | `non_euclidean::cosmology` |
| `recombination_redshift` | `fn recombination_redshift() → f64` | Recombination redshift z ≈ 1100 | `non_euclidean::cosmology` |
| `matter_radiation_equality_redshift` | `fn matter_radiation_equality_redshift(omega_m: f64, omega_r: f64) → f64` | Matter-radiation equality | `non_euclidean::cosmology` |
| `riemann_component` | `fn riemann_component(metric: &[Vec<f64>], i: usize, j: usize, k: usize, l: usize, x: &[f64], h: f64) → f64` | Riemann curvature component | `non_euclidean::curvature` |
| `ricci_scalar` | `fn ricci_scalar(metric: &[Vec<f64>], x: &[f64], h: f64) → f64` | Ricci scalar curvature | `non_euclidean::curvature` |
| `gaussian_curvature_sphere` | `fn gaussian_curvature_sphere(radius: f64) → f64` | Gaussian curvature K = 1/R² | `non_euclidean::curvature` |
| `gaussian_curvature_hyperbolic` | `fn gaussian_curvature_hyperbolic(radius: f64) → f64` | Gaussian curvature K = −1/R² | `non_euclidean::curvature` |
| `poincare_disk_distance` | `fn poincare_disk_distance(x1: f64, y1: f64, x2: f64, y2: f64) → f64` | Poincaré disk hyperbolic distance | `non_euclidean::curvature` |
| `spherical_distance` | `fn spherical_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64, radius: f64) → f64` | Great-circle distance | `non_euclidean::curvature` |
| `parallel_transport_sphere` | `fn parallel_transport_sphere(v_theta: f64, v_phi: f64, theta: f64, delta_phi: f64) → (f64, f64)` | Parallel transport on sphere | `non_euclidean::curvature` |
| `sectional_curvature` | `fn sectional_curvature(metric: &[Vec<f64>], x: &[f64], u: &[f64], v: &[f64], h: f64) → f64` | Sectional curvature | `non_euclidean::curvature` |
| `ricci_tensor_component` | `fn ricci_tensor_component(metric: &[Vec<f64>], i: usize, j: usize, x: &[f64], h: f64) → f64` | Ricci tensor Rᵢⱼ | `non_euclidean::curvature` |
| `einstein_tensor_component` | `fn einstein_tensor_component(metric: &[Vec<f64>], i: usize, j: usize, x: &[f64], h: f64) → f64` | Einstein tensor Gᵢⱼ | `non_euclidean::curvature` |
| `geodesic_deviation_magnitude` | `fn geodesic_deviation_magnitude(metric: &[Vec<f64>], x: &[f64], u: &[f64], xi: &[f64], h: f64) → f64` | Geodesic deviation magnitude | `non_euclidean::curvature` |
| `upper_half_plane_distance` | `fn upper_half_plane_distance(x1: f64, y1: f64, x2: f64, y2: f64) → f64` | Upper half-plane hyperbolic distance | `non_euclidean::curvature` |
| `hyperboloid_distance` | `fn hyperboloid_distance(x: &[f64], y: &[f64]) → f64` | Hyperboloid model distance | `non_euclidean::curvature` |
| `spherical_area` | `fn spherical_area(radius: f64, theta1: f64, theta2: f64, phi1: f64, phi2: f64) → f64` | Spherical surface area element | `non_euclidean::curvature` |
| `spherical_excess` | `fn spherical_excess(a: f64, b: f64, c: f64) → f64` | Spherical excess of triangle | `non_euclidean::curvature` |
| `hyperbolic_area_triangle` | `fn hyperbolic_area_triangle(alpha: f64, beta: f64, gamma: f64) → f64` | Hyperbolic triangle area | `non_euclidean::curvature` |
| `weyl_tensor_vanishes_2d` | `fn weyl_tensor_vanishes_2d() → bool` | Weyl tensor vanishes in 2D | `non_euclidean::curvature` |
| `kretschmer_scalar_schwarzschild` | `fn kretschmer_scalar_schwarzschild(mass: f64, r: f64) → f64` | Kretschner scalar for Schwarzschild | `non_euclidean::curvature` |
| `GeodesicSolver` | `struct { dim: usize, metric_fn: Box<dyn Fn(&[f64]) → Vec<Vec<f64>>> }` | Geodesic equation integrator | `non_euclidean::geodesic` |
| `GeodesicSolver::new` | `fn new(dim: usize, metric_fn: impl Fn(&[f64]) → Vec<Vec<f64>> + 'static) → Self` | Construct solver with metric | `non_euclidean::geodesic` |
| `GeodesicSolver::step` | `fn step(&self, x: &[f64], v: &[f64], dt: f64) → (Vec<f64>, Vec<f64>)` | One geodesic integration step | `non_euclidean::geodesic` |
| `GeodesicSolver::integrate` | `fn integrate(&self, x0: &[f64], v0: &[f64], dt: f64, n_steps: usize) → Vec<Vec<f64>>` | Integrate geodesic trajectory | `non_euclidean::geodesic` |
| `MetricSpace` | `struct { dim: usize, metric_type: MetricType }` | Metric space representation | `non_euclidean::metric` |
| `MetricType` | `enum { Euclidean, Minkowski, Schwarzschild { rs: f64 }, Hyperbolic { k: f64 }, Spherical { r: f64 }, Custom(Arc<dyn Fn(&[f64]) → Vec<Vec<f64>>>) }` | Metric type variants | `non_euclidean::metric` |
| `MetricSpace::euclidean` | `fn euclidean(dim: usize) → Self` | Euclidean metric space | `non_euclidean::metric` |
| `MetricSpace::minkowski` | `fn minkowski() → Self` | Minkowski spacetime | `non_euclidean::metric` |
| `MetricSpace::schwarzschild` | `fn schwarzschild(rs: f64) → Self` | Schwarzschild metric | `non_euclidean::metric` |
| `MetricSpace::hyperbolic` | `fn hyperbolic(k: f64) → Self` | Hyperbolic metric | `non_euclidean::metric` |
| `MetricSpace::spherical` | `fn spherical(r: f64) → Self` | Spherical metric | `non_euclidean::metric` |
| `MetricSpace::custom` | `fn custom(dim: usize, f: impl Fn(&[f64]) → Vec<Vec<f64>> + 'static) → Self` | Custom metric | `non_euclidean::metric` |
| `MetricSpace::dimension` | `fn dimension(&self) → usize` | Space dimension | `non_euclidean::metric` |
| `MetricSpace::metric_tensor_at` | `fn metric_tensor_at(&self, x: &[f64]) → Vec<Vec<f64>>` | Metric tensor at point | `non_euclidean::metric` |
| `MetricSpace::line_element` | `fn line_element(&self, x: &[f64], dx: &[f64]) → f64` | Line element ds² | `non_euclidean::metric` |
| `MetricSpace::christoffel_at` | `fn christoffel_at(&self, x: &[f64], h: f64) → Vec<Vec<Vec<f64>>>` | Christoffel symbols Γⁱⱼₖ | `non_euclidean::metric` |
| `MetricSpace::inner_product` | `fn inner_product(&self, x: &[f64], u: &[f64], v: &[f64]) → f64` | Inner product with metric | `non_euclidean::metric` |
| `MetricSpace::vector_norm` | `fn vector_norm(&self, x: &[f64], v: &[f64]) → f64` | Vector norm with metric | `non_euclidean::metric` |
| `MetricSpace::angle_between` | `fn angle_between(&self, x: &[f64], u: &[f64], v: &[f64]) → f64` | Angle between vectors | `non_euclidean::metric` |
| `MetricSpace::curve_length` | `fn curve_length(&self, points: &[Vec<f64>]) → f64` | Curve length in metric | `non_euclidean::metric` |
| `MetricSpace::geodesic_distance` | `fn geodesic_distance(&self, x: &[f64], y: &[f64]) → f64` | Geodesic distance approximation | `non_euclidean::metric` |
| `MetricSpace::raise_index` | `fn raise_index(&self, x: &[f64], v: &[f64]) → Vec<f64>` | Raise index via inverse metric | `non_euclidean::metric` |
| `MetricSpace::lower_index` | `fn lower_index(&self, x: &[f64], v: &[f64]) → Vec<f64>` | Lower index via metric | `non_euclidean::metric` |
| `MetricSpace::volume_element` | `fn volume_element(&self, x: &[f64]) → f64` | Volume element √|det(g)| | `non_euclidean::metric` |
