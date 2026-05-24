# Mathematics Source Code Guide

This page documents the source implementation behind `sciforge::maths`, including module layout and Hub dispatch wiring.

## Source Coverage

### What is explained
- File-level implementation layout in `src/maths/`.
- Main algorithm families and where they are implemented.
- Runtime call path when maths functions are executed through Hub dispatch.

### Visibility and external access
- This domain module is `pub(crate)` in `src/lib.rs` and is not part of the external crate API.
- External consumers should use the public `sciforge::hub` API for these computations.
- Direct module paths shown here are for internal development and source-level understanding.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/maths/`
- Module entry point: `src/maths/mod.rs`
- Hub routing: `src/hub/engine/dispatch/maths.rs`

### Internal logic
1. The module is split into algorithmic families (`linalg`, `ode`, `pde`, `statistics`, etc.).
2. Each submodule groups related numerical methods behind public functions.
3. Hub dispatch maps experiment function names to direct Rust calls with typed parameters.

### What to verify in source code
- Numeric preconditions (matrix shape compatibility, positive step sizes, tolerances).
- Stability conditions (CFL-like constraints in PDE solvers, convergence criteria).
- Complexity and allocation behavior for large inputs.

## Modules

- `complex`
- `fft`
- `graph`
- `integration`
- `interpolation`
- `linalg`
- `non_euclidean`
- `ode`
- `optimization`
- `pde`
- `polynomial`
- `probability` — submodules: `bayesian`, `distributions`, `markov`, `monte_carlo`, `sampling`
- `signal`
- `sparse`
- `statistics` — submodules: `clustering`, `descriptive`, `distributions`, `hypothesis`, `regression`
- `tensor`
- `vector`

---

## Complex API (26 functions)

### Submodules
- `types` — `Complex<f64>` type, constructors, polar/rectangular forms
- `ops` — arithmetic, exp, log, trig, hyperbolic functions on complex numbers
- `quaternion` — `Quaternion` type, Hamilton product, rotation, normalization

### Selected functions

- `Complex::new(re, im)`, `Complex::from_polar(r, theta)`
- `complex_exp(z)`, `complex_ln(z)`, `complex_sqrt(z)`
- `complex_sin(z)`, `complex_cos(z)`, `complex_tan(z)`
- `Quaternion::new(w, x, y, z)`, `quaternion_rotate(q, v)`

---

## FFT API (51 functions)

### Submodules
- `radix2` — Cooley-Tukey FFT/IFFT for power-of-two sizes
- `dct` — Discrete Cosine Transform (types I–IV)
- `bluestein` — Chirp-Z / Bluestein FFT for arbitrary sizes

### Selected functions

- `fft(signal)` → complex spectrum
- `ifft(spectrum)` → time-domain signal
- `dct_ii(signal)`, `dct_iii(signal)`, `dct_iv(signal)`
- `bluestein_fft(signal, n_out)` — arbitrary-length DFT
- `power_spectrum(signal)`

---

## Graph API (58 functions)

### Submodules
- `traversal` — BFS, DFS, topological sort, connected components
- `shortest_path` — Dijkstra, Bellman-Ford, Floyd-Warshall, A*
- `spanning_tree` — Kruskal, Prim, minimum/maximum spanning tree
- `flow` — Ford-Fulkerson, Edmonds-Karp, max-flow/min-cut

### Selected functions

- `dijkstra(graph, source)` → distances
- `bellman_ford(graph, source)` → distances (handles negative weights)
- `floyd_warshall(graph)` → all-pairs shortest paths
- `kruskal_mst(graph)`, `prim_mst(graph, start)`
- `bfs(graph, start)`, `dfs(graph, start)`
- `ford_fulkerson(graph, source, sink)`

---

## Integration API (47 functions)

### Submodules
- `quadrature` — trapezoidal, Simpson, Gauss-Legendre, Romberg
- `adaptive` — adaptive Simpson, adaptive Gauss-Kronrod (GK15, GK21)
- `multidim` — double/triple integrals, Monte Carlo integration

### Selected functions

- `trapezoidal(f, a, b, n)`
- `simpsons(f, a, b, n)`
- `gauss_legendre(f, a, b, n_points)`
- `romberg(f, a, b, max_iter, tol)`
- `adaptive_simpson(f, a, b, tol)`
- `monte_carlo_integral(f, domain, n_samples)`
- `double_integral(f, ax, bx, ay, by, n)`

---

## Interpolation API (34 functions)

### Submodules
- `lagrange` — Lagrange polynomial interpolation, Neville's algorithm
- `methods` — linear, bilinear, cubic, nearest-neighbor
- `spline` — natural cubic spline, clamped spline, B-spline

### Selected functions

- `lagrange_interpolate(xs, ys, x)`
- `neville(xs, ys, x)`
- `linear_interpolate(x0, y0, x1, y1, x)`
- `cubic_spline(xs, ys)` → spline object
- `spline_eval(spline, x)`
- `bspline_basis(i, k, t, knots)`

---

## Linear Algebra API (57 functions)

### Submodules
- `decomposition` — LU, QR, Cholesky, SVD decompositions
- `eigenvalue` — power iteration, QR algorithm, Jacobi method
- `factorization` — LDL^T, Schur, polar decomposition
- `least_squares` — normal equations, QR least squares, pseudoinverse

### Selected functions

- `lu_decompose(matrix)` → (L, U, P)
- `qr_decompose(matrix)` → (Q, R)
- `cholesky(matrix)` → L (lower triangular)
- `svd(matrix)` → (U, Σ, V^T)
- `eigenvalues_qr(matrix, tol, max_iter)`
- `solve_linear(a, b)` — Ax = b via LU
- `pseudoinverse(matrix)`
- `least_squares_qr(a, b)`

---

## Non-Euclidean Geometry API (56 functions)

### Submodules
- `metric` — metric tensors (Minkowski, Schwarzschild, Kerr, FRW)
- `geodesic` — geodesic equations, numerical integration
- `curvature` — Riemann tensor, Ricci scalar, Christoffel symbols
- `cosmology` — scale factor, Friedmann equations
- `black_hole` — Schwarzschild radius, Hawking temperature, ergosphere

### Selected functions

- `schwarzschild_radius(mass)`
- `hawking_temperature(mass)`
- `christoffel_symbols(metric, coord)`
- `riemann_tensor(metric, coord)`
- `ricci_scalar(metric, coord)`
- `geodesic_equation(metric, initial_pos, initial_vel, tau_span)`

---

## ODE API (35 functions)

### Submodules
- `solvers` — Euler, RK4, RK45, Adams-Bashforth, implicit methods
- `systems` — system of ODEs interface
- `bvp` — boundary value problems (shooting method, collocation)

### Selected functions

- `euler(f, y0, t0, tf, dt)`
- `rk4(f, y0, t0, tf, dt)`
- `rk45_adaptive(f, y0, t0, tf, tol, dt_init)`
- `adams_bashforth4(f, y0, t_span, dt)`
- `shooting_method(f, ya, yb, t_span)`
- `solve_bvp(f, bc, t_span, n_points)`

---

## Optimization API (53 functions)

### Submodules
- `gradient` — gradient descent, Adam, L-BFGS, conjugate gradient
- `constrained` — Lagrangian, interior-point, sequential quadratic programming
- `evolutionary` — genetic algorithm, differential evolution, CMA-ES
- `metaheuristic` — simulated annealing, particle swarm, tabu search

### Selected functions

- `gradient_descent(f, grad_f, x0, lr, max_iter)`
- `adam_optimizer(f, grad_f, x0, params)`
- `lbfgs(f, grad_f, x0, max_iter)`
- `conjugate_gradient(a, b)` — linear system solver
- `genetic_algorithm(fitness, bounds, pop_size, generations)`
- `simulated_annealing(f, x0, t_init, cooling)`
- `particle_swarm(f, bounds, n_particles, max_iter)`

---

## PDE API (71 functions)

### Submodules
- `finite_diff` — finite difference schemes (explicit, implicit, Crank-Nicolson)
- `diffusion` — heat equation (1D/2D/3D), reaction-diffusion
- `wave` — wave equation (1D/2D), SH waves
- `laplace` — Laplace/Poisson equation, SOR, multigrid

### Selected functions

- `heat_equation_1d_explicit(u0, alpha, dx, dt, steps)`
- `heat_equation_2d_adi(u0, alpha, dx, dy, dt, steps)`
- `crank_nicolson_diffusion(u0, alpha, dx, dt, steps)`
- `wave_equation_1d(u0, v0, c, dx, dt, steps)`
- `laplace_2d_sor(boundary, omega, tol)`
- `poisson_2d(rhs, dx, dy, boundary)`

---

## Polynomial API (29 functions)

### Submodules
- `poly` — `Polynomial` type, evaluation, arithmetic, derivative, integral
- `roots` — root finding (Durand-Kerner, Jenkins-Traub, Bairstow)
- `special` — Legendre, Hermite, Laguerre, Chebyshev polynomials

### Selected functions

- `Polynomial::new(coeffs)`, `poly_eval(poly, x)`
- `poly_add(p, q)`, `poly_mul(p, q)`, `poly_div(p, q)`
- `poly_derivative(poly)`, `poly_integral(poly)`
- `poly_roots(poly)` → complex roots
- `legendre(n, x)`, `hermite(n, x)`, `chebyshev_t(n, x)`, `laguerre(n, x)`

---

## Probability API (69 functions)

### Submodules
- `distributions` — PDF, CDF, quantile for Normal, Poisson, Binomial, etc.
- `sampling` — Box-Muller, rejection sampling, inverse CDF
- `markov` — transition matrices, stationary distribution, MCMC
- `monte_carlo` — MC estimation, variance reduction, quasi-random

### Selected functions

- `normal_pdf(x, mu, sigma)`, `normal_cdf(x, mu, sigma)`
- `poisson_pmf(k, lambda)`, `binomial_pmf(k, n, p)`
- `sample_normal(mu, sigma)`, `sample_poisson(lambda)`
- `metropolis_hastings(target, proposal, x0, n_samples)`
- `markov_stationary(transition_matrix)`

---

## Signal Processing API (70 functions)

### Submodules
- `filters` — FIR/IIR filters, Butterworth, Chebyshev, window functions
- `spectral` — power spectral density, spectrogram, periodogram
- `convolution` — 1D/2D linear/circular convolution, cross-correlation
- `wavelets` — DWT, IDWT, Daubechies, Haar wavelets

### Selected functions

- `fir_filter(signal, coeffs)`, `iir_filter(signal, b, a)`
- `butterworth_lowpass(order, cutoff_freq)`
- `hamming_window(n)`, `hann_window(n)`, `blackman_window(n)`
- `psd_welch(signal, fs, nperseg)`
- `convolve(signal, kernel)`, `correlate(x, y)`
- `dwt(signal, wavelet)`, `idwt(coeffs, wavelet)`

---

## Sparse Matrix API (24 functions)

### Submodules
- `csr` — Compressed Sparse Row matrix storage
- `ops` — sparse matrix arithmetic, transpose, norms
- `solvers` — CG, GMRES, sparse LU, iterative solvers

### Selected functions

- `CsrMatrix::new(rows, cols, data, indices, indptr)`
- `sparse_add(a, b)`, `sparse_mul(a, b)`
- `sparse_mv(matrix, vec)` — matrix-vector product
- `sparse_transpose(matrix)`
- `conjugate_gradient_sparse(a, b, x0, tol)`
- `gmres(a, b, x0, restart, tol)`

---

## Statistics API (71 functions)

### Submodules
- `descriptive` — mean, variance, skewness, kurtosis, quantiles
- `distributions` — t, chi², F, KS distributions and tests
- `hypothesis` — t-test, ANOVA, chi-square test, Wilcoxon
- `regression` — linear, polynomial, logistic regression, R²

### Selected functions

- `mean(data)`, `variance(data)`, `std_dev(data)`
- `median(data)`, `percentile(data, p)`, `iqr(data)`
- `skewness(data)`, `kurtosis(data)`
- `t_test_one_sample(data, mu0)`, `t_test_two_sample(a, b)`
- `chi_square_test(observed, expected)`
- `linear_regression(xs, ys)` → (slope, intercept, r_squared)
- `pearson_correlation(x, y)`, `spearman_correlation(x, y)`

---

## Tensor API (27 functions)

### Submodules
- `storage` — N-dimensional dense tensor storage
- `ops` — element-wise arithmetic, contraction, outer product
- `linalg` — tensor norms, trace, symmetric/antisymmetric parts
- `decompose` — Tucker decomposition, CP decomposition
- `display` — pretty-print tensors

### Selected functions

- `Tensor::new(shape, data)`, `Tensor::zeros(shape)`, `Tensor::ones(shape)`
- `tensor_add(a, b)`, `tensor_mul_scalar(t, s)`
- `tensor_contract(a, b, axes)` — Einstein summation
- `tensor_outer(a, b)`, `tensor_transpose(t, perm)`
- `tucker_decompose(tensor, ranks)`, `cp_decompose(tensor, rank)`

---

## Vector Fields API (41 functions)

### Submodules
- `types` — `Vec2`, `Vec3`, `VecN` with arithmetic operators
- `ops` — dot product, cross product, norm, normalization
- `fields` — gradient, divergence, curl, Laplacian
- `integrators` — Euler, RK4 integration of vector field ODEs
- `sim` — particle simulation utilities

### Selected functions

- `Vec3::new(x, y, z)`, `dot(a, b)`, `cross(a, b)`, `norm(v)`
- `gradient(f, point, h)`, `divergence(f, point, h)`
- `curl(f, point, h)`, `laplacian(f, point, h)`
- `euler_step(field, pos, dt)`, `rk4_step(field, pos, dt)`

---

## Hub dispatch mapping

All mathematics functions are wired through:

- `src/hub/engine/dispatch/maths.rs`

```rust
use sciforge::hub::prelude::*;

let exp = Experiment::new(DomainType::Maths, "linear_regression")
        .param("xs", ParameterValue::Vector(vec![1.0, 2.0, 3.0]))
        .param("ys", ParameterValue::Vector(vec![2.1, 4.0, 5.9]));

let out = ExperimentRunner::new().run(&exp)?;
```
