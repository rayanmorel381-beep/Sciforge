# Mathematics Module

The **Mathematics** module provides **17 submodules** across **63 source files**, covering core computational mathematics from linear algebra and calculus through graph theory, signal processing, and non-Euclidean geometry.

---

## Chapter 1 - Module Scope

### Scientific Purpose
This section defines the scope: which problem this module solves, which abstraction level it targets, and which outputs are expected.

### Modeling Assumptions
- The equations are numerical models, not analytical proofs.
- Inputs must respect the documented units.
- Validity domains (linear regime, local approximation, and so on) should be verified by the reader.

### Reading Strategy
1. Read the module structure.
2. Identify equations and their conditions.
3. Map these equations to Rust functions in `docs/code/`.


## Reading Guide

### Goal
This page explains the module from a scientific perspective: assumptions, models, equations, and usage limits.

### How To Read This Document
- Start with the domain section relevant to your case.
- Identify key equations and their usage conditions.
- Then verify the corresponding Rust signatures in `docs/code/`.

### Conventions
- Units: SI unless explicitly stated.
- Variables: standard mathematical notation for the domain.
- Equations are presented as reference models, not formal proofs.


## Chapter 2 - Submodules and Models

### 1. Complex Numbers

Arithmetic and analysis on the complex plane, including quaternion extensions for 3D rotation.

**Arithmetic** — Full complex algebra: addition, subtraction, multiplication, division, conjugation, modulus $|z| = \sqrt{a^2+b^2}$, and argument $\arg(z) = \text{atan2}(b,a)$. Polar form interconversion: $z = r e^{i\theta}$.

**Complex Functions** — Complex exponential $e^z = e^a(\cos b + i\sin b)$, complex logarithm $\ln z = \ln|z| + i\arg(z)$, and complex polynomial evaluation via Horner's scheme in $\mathbb{C}$.

**Fractals** — Mandelbrot iteration $z_{n+1} = z_n^2 + c$ (escape-time algorithm with bailout $|z|^2 > 4$), Julia set iteration, and Newton fractal step $z \leftarrow z - f(z)/f'(z)$ for polynomial root basins.

**Roots of Unity** — The $n$ roots $e^{2\pi i k/n}$ for $k = 0, \ldots, n-1$, essential for FFT twiddle factors and cyclic group generators.

**Complex Linear Algebra** — Complex matrix multiplication and determinant (Laplace expansion), extending the linear algebra toolbox to $\mathbb{C}^{n \times n}$.

**Quaternions** — Hamilton's four-dimensional extension $q = w + xi + yj + zk$ with quaternion multiplication, conjugation, norm, and SLERP interpolation for smooth 3D rotations without gimbal lock.

---

### 2. FFT (Fast Fourier Transform)

Efficient frequency-domain transforms for spectral analysis, filtering, and convolution.

**Radix-2 FFT** — Cooley-Tukey recursive butterfly algorithm: the $N$-point DFT $X_k = \sum_{n=0}^{N-1} x_n e^{-2\pi i kn/N}$ is computed in $O(N \log N)$ by splitting into even/odd halves with twiddle factor $W_N^k = e^{-2\pi i k/N}$. Input length must be a power of two; the module pads automatically. Inverse FFT scales by $1/N$.

**Bluestein (Chirp-Z)** — Extends FFT to arbitrary-length sequences by rewriting the DFT as a convolution: $X_k = W_N^{k^2/2}\sum_n x_n W_N^{n^2/2}\cdot W_N^{-(k-n)^2/2}$, then applying radix-2 FFT on zero-padded sequences.

**DCT** — Discrete Cosine Transform (type-II): $X_k = \sum_{n=0}^{N-1} x_n \cos\!\left[\frac{\pi}{N}\left(n + \tfrac{1}{2}\right)k\right]$, used for signal compression (JPEG, MP3). The module also provides the inverse DCT.

**Power Spectrum** — $|X_k|^2$ from the FFT output gives the power spectral density. Frequency bin spacing: $\Delta f = f_s/N$ where $f_s$ is the sample rate.

**FFT-Based Convolution** — Circular convolution via the convolution theorem: $h = \text{IFFT}(\text{FFT}(a) \cdot \text{FFT}(b))$, and cross-correlation by conjugation: $R_{ab} = \text{IFFT}(\overline{A} \cdot B)$.

---

### 3. Graph Theory

Classical algorithms on weighted and unweighted graph structures.

**Shortest Paths** — Dijkstra's algorithm (non-negative weights, $O(V^2)$ without priority queue), Bellman-Ford (handles negative weights, detects negative cycles), Floyd-Warshall (all-pairs, $O(V^3)$), A* (heuristic-guided search with admissible heuristic). Path reconstruction is available for all single-source algorithms.

**Network Flow** — Ford-Fulkerson maximum flow via augmenting paths, and min-cut extraction from the residual graph.

**Spanning Trees** — Kruskal's and Prim's algorithms for minimum spanning tree (MST) on weighted graphs.

**Traversal** — BFS (breadth-first) and DFS (depth-first) traversal with connected component identification, topological sort, and cycle detection.

---

### 4. Integration

Numerical computation of definite integrals $\int_a^b f(x)\,dx$.

**Newton-Cotes Rules** — Trapezoidal rule $\int \approx \frac{h}{2}[f(a)+f(b)+2\sum f(x_i)]$ with error $O(h^2)$. Simpson's rule $\int \approx \frac{h}{3}[f(a)+4\sum_\text{odd}+2\sum_\text{even}+f(b)]$ with error $O(h^4)$. Simpson's 3/8 rule and Boole's rule for higher-order accuracy. Midpoint rule.

**Romberg Integration** — Richardson extrapolation on the trapezoidal rule to build a triangular table of superior approximations, achieving high accuracy without changing quadrature type.

**Gaussian Quadrature** — Exact for polynomials of degree $\le 2n-1$ using optimal node placement and weight coefficients, via Gauss-Legendre rules.

**Adaptive Quadrature** — Recursive interval bisection with local error estimation (adaptive Simpson), automatically concentrating effort where the integrand varies rapidly.

**Multidimensional** — Monte Carlo integration $\int_\Omega f \approx V \cdot \frac{1}{N}\sum f(x_i)$ and cubature methods for integrals over arbitrary domains in $\mathbb{R}^d$.

---

### 5. Interpolation

Constructing smooth approximations through known data points.

**Lagrange Interpolation** — Polynomial through $n$ points: $P(x) = \sum_{i=0}^{n-1} y_i \prod_{j\neq i}\frac{x-x_j}{x_i-x_j}$. Barycentric form for efficient repeated evaluation without recomputing products.

**Cubic Splines** — Natural cubic spline: piecewise $S_i(x) = y_i + b_i(x-x_i) + c_i(x-x_i)^2 + d_i(x-x_i)^3$ with $C^2$ continuity throughout. Coefficients determined by a tridiagonal system solved via Thomas algorithm. Integration and differentiation on the spline are also supported.

**Other Methods** — Linear interpolation, nearest-neighbor, and Newton's divided difference polynomial.

---

### 6. Linear Algebra

Matrix and vector computations forming the backbone of scientific computing.

**Decompositions** — LU decomposition (with partial pivoting for numerical stability), QR factorization (Gram-Schmidt), and Cholesky decomposition $A = LL^T$ for symmetric positive-definite matrices. Each decomposition is used to solve $Ax = b$ by back-substitution.

**Eigenvalues** — Power iteration for the dominant eigenvalue and eigenvector: $v^{(k+1)} = Av^{(k)}/\|Av^{(k)}\|$ with convergence rate proportional to $|\lambda_2/\lambda_1|$. Inverse iteration for eigenvalues near a shift $\sigma$. Rayleigh quotient $\rho = v^TAv/(v^Tv)$ for refined eigenvalue estimates. Gershgorin disks for bounding eigenvalue locations. Spectral radius, trace, and condition number $\kappa = |\lambda_{\max}|/|\lambda_{\min}|$.

**Least Squares** — Ordinary least squares $\hat{x} = (A^TA)^{-1}A^Tb$ via the normal equations or QR factorization. Weighted least squares, pseudoinverse, and SVD-based rank-deficient solutions.

---

### 7. Non-Euclidean Geometry

Curved-space geometry for general relativity, cosmology, and differential geometry.

**Metric Tensors** — Definition of the metric $g_{\mu\nu}$ in arbitrary coordinates (Schwarzschild, FLRW, Kerr). Line element $ds^2 = g_{\mu\nu}dx^\mu dx^\nu$, metric inverse $g^{\mu\nu}$, metric determinant, and coordinate transformations.

**Christoffel Symbols** — Numerical computation $\Gamma^\sigma_{\mu\nu} = \frac{1}{2}g^{\sigma\rho}(\partial_\mu g_{\nu\rho} + \partial_\nu g_{\rho\mu} - \partial_\rho g_{\mu\nu})$ via finite differences at configurable step size.

**Geodesics** — Integration of the geodesic equation $\frac{d^2x^\sigma}{d\tau^2} + \Gamma^\sigma_{\mu\nu}\frac{dx^\mu}{d\tau}\frac{dx^\nu}{d\tau} = 0$ using a symplectic (leapfrog) integrator. Full trajectory output for visualization of light bending, orbital precession, etc.

**Curvature** — Ricci scalar, Gaussian curvature of 2-surfaces, and Riemann tensor components for measuring manifold curvature.

**Black Holes** — Schwarzschild metric solutions: photon sphere at $r = 3M$, innermost stable circular orbit (ISCO) at $r = 6M$, and gravitational redshift $z = (1-r_s/r)^{-1/2} - 1$.

---

### 8. ODE (Ordinary Differential Equations)

Numerical solvers for initial and boundary value problems $y' = f(t, y)$.

**Euler Method** — First-order explicit: $y_{n+1} = y_n + h\,f(t_n, y_n)$. Simple but with $O(h)$ global error; serves as a baseline.

**Runge-Kutta 4** — The classical fourth-order method: $y_{n+1} = y_n + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4)$ with four function evaluations per step and $O(h^4)$ global error.

**Adaptive Methods** — Runge-Kutta-Fehlberg (RK45) with embedded error estimation for automatic step-size control, maintaining a target local error tolerance.

**Systems** — All solvers accept vector-valued ODEs $\vec{y}' = \vec{f}(t,\vec{y})$ for coupled systems (e.g., Lotka-Volterra, orbital mechanics, coupled oscillators).

**Boundary Value Problems** — Shooting method: convert a BVP into an IVP by guessing initial conditions and iterating (bisection or Newton) until boundary conditions are satisfied.

---

### 9. Optimization

Algorithms for minimizing (or maximizing) objective functions.

**Gradient Descent** — $x_{k+1} = x_k - \alpha\nabla f(x_k)$ with fixed or line-search learning rate. Momentum variant $v_{k+1} = \beta v_k - \alpha\nabla f$, $x_{k+1} = x_k + v_{k+1}$ for faster convergence through ravines.

**Adam Optimizer** — Adaptive moment estimation: bias-corrected first ($\hat{m}$) and second ($\hat{v}$) moment estimates with update $x \leftarrow x - \alpha\hat{m}/(\sqrt{\hat{v}}+\varepsilon)$. Default $\beta_1 = 0.9$, $\beta_2 = 0.999$.

**Newton's Method** — Second-order: $x_{k+1} = x_k - f(x_k)/f'(x_k)$ for root-finding (1D); quadratic convergence near the root.

**Constrained** — Penalty methods and Lagrange multiplier approach for constrained optimization.

**Evolutionary** — Genetic algorithms (selection, crossover, mutation) and differential evolution for global optimization over non-smooth landscapes.

**Metaheuristic** — Simulated annealing (acceptance probability $e^{-\Delta E/T}$) and particle swarm optimization.

---

### 10. PDE (Partial Differential Equations)

Numerical solvers for diffusion, wave propagation, and potential problems on grids.

**Heat/Diffusion Equation** — $\partial T/\partial t = \alpha\nabla^2 T$ solved by three finite-difference schemes: explicit (forward-time central-space, CFL $r = \alpha\Delta t/\Delta x^2 \le 1/2$), implicit (backward Euler, unconditionally stable via Thomas tridiagonal solve), and Crank-Nicolson (second-order in time, average of explicit and implicit operators). 2D extension available.

**Wave Equation** — $\partial^2 u/\partial t^2 = c^2\nabla^2 u$ solved by central-difference leapfrog in 1D and 2D. D'Alembert analytical solution $u = \frac{1}{2}[f(x-ct)+f(x+ct)]$ for validation. Courant number $C = c\Delta t/\Delta x \le 1$ for stability. Energy density and absorbing boundary conditions.

**Laplace/Poisson** — $\nabla^2\phi = \rho$ solved by Jacobi and Gauss-Seidel iterative relaxation on a grid, with configurable tolerance and boundary conditions.

---

### 11. Polynomial

Polynomial algebra and numerical root-finding.

**Polynomial Struct** — Evaluation (Horner's method), addition, multiplication, differentiation, and integration of polynomials with real coefficients.

**Root-Finding** — Newton-Raphson $x_{n+1} = x_n - f(x_n)/f'(x_n)$ (quadratic convergence), bisection (robust, linear convergence), secant method (superlinear, no derivative needed), and Brent's method (combining bisection, secant, and inverse quadratic interpolation for guaranteed convergence).

**Special Polynomials** — Legendre $P_n(x)$, Chebyshev $T_n(x) = \cos(n\arccos x)$, Hermite $H_n(x)$, and Laguerre $L_n(x)$ — orthogonal polynomial families used in quadrature, approximation theory, and quantum mechanics.

---

### 12. Probability

Distributions, stochastic processes, and simulation.

**Distributions** — PDF, CDF, and quantiles for: Uniform, Normal $f(x) = \frac{1}{\sigma\sqrt{2\pi}}e^{-z^2/2}$, Exponential $f(x) = \lambda e^{-\lambda x}$, Poisson $P(k) = e^{-\lambda}\lambda^k/k!$, Binomial $\binom{n}{k}p^k(1-p)^{n-k}$, Geometric, Gamma, Beta, Cauchy, Chi-squared, Student's $t$, and $F$-distribution.

**Markov Chains** — Transition matrix representation, $n$-step transition $P^n$, stationary distribution $\pi P = \pi$ (eigenvector method), and absorbing chain analysis.

**Monte Carlo** — Random sampling, Monte Carlo integration, and variance reduction techniques (importance sampling, stratification).

**Sampling** — Inverse transform sampling, rejection sampling, and Box-Muller transform for normal variates.

---

### 13. Signal Processing

Time-series analysis, filtering, and spectral methods.

**Filters** — RC low-pass $y_n = \alpha x_n + (1-\alpha)y_{n-1}$ with $\alpha = \Delta t/(RC+\Delta t)$ and RC high-pass $y_n = \alpha(y_{n-1}+x_n-x_{n-1})$. Moving average (simple and exponential EMA). Median filter for impulse noise removal.

**Convolution** — Discrete linear convolution $(f*g)[n] = \sum_k f[k]\,g[n-k]$ both directly and via FFT for $O(N\log N)$ performance. Cross-correlation for lag detection.

**Spectral Analysis** — Power spectral density estimation, periodogram, Welch's method, and spectral peak detection.

**Wavelets** — Discrete wavelet transform (Haar, Daubechies), multiresolution analysis, and wavelet decomposition/reconstruction for time-frequency localization.

---

### 14. Sparse Matrices

Memory-efficient representation and operations for large matrices with mostly zero entries.

**CSR Format** — Compressed Sparse Row: storage via three arrays (`row_ptr`, `col_idx`, `values`). Construction from triplet lists $(i,j,v)$, identity matrix, random sparse generation. Element access $O(\text{nnz per row})$.

**Operations** — Sparse matrix-vector product $y = Ax$ in $O(\text{nnz})$, sparse-sparse addition, transposition, and Frobenius norm.

**Iterative Solvers** — Conjugate gradient method for symmetric positive-definite sparse systems: convergence in at most $n$ iterations for exact arithmetic, with practical convergence rate determined by the condition number. GMRES and Jacobi iteration for non-symmetric systems.

---

### 15. Statistics

Descriptive analysis, hypothesis testing, and regression.

**Descriptive** — Mean, median, mode, variance (sample and population), standard deviation, skewness, kurtosis, percentiles, and interquartile range.

**Hypothesis Testing** — One-sample and two-sample $t$-tests, paired $t$-test, chi-squared goodness-of-fit and independence tests, $p$-value computation, and confidence interval construction.

**Regression** — Simple linear regression $y = \beta_0 + \beta_1 x$ via $\beta_1 = \frac{n\sum xy - \sum x\sum y}{n\sum x^2 - (\sum x)^2}$. Multiple linear regression, polynomial regression, coefficient of determination $R^2 = 1 - \text{SS}_\text{res}/\text{SS}_\text{tot}$, standard error of estimate, and residual analysis.

---

### 16. Tensor

Multi-dimensional array algebra for physics and machine learning.

**Storage** — Row-major contiguous storage with shape and stride arrays. Construction from nested data, functional initialization (`from_fn`), zeros, ones, and random tensors. Arbitrary rank support.

**Operations** — Element-wise arithmetic, scalar operations, broadcasting, reshaping, slicing, and transposition via axis permutation.

**Contraction** — Generalized tensor contraction $C_{i\ldots k\ldots} = \sum_m A_{i\ldots m\ldots}\,B_{k\ldots m\ldots}$ along specified axes — the foundation for matrix multiplication, inner products, and Einstein summation.

**Decomposition** — CP (CANDECOMP/PARAFAC) and Tucker decomposition for low-rank tensor approximation.

---

### 17. Vector Calculus

Differential operators and integral calculus on vector fields in $\mathbb{R}^3$.

**Vec3 Type** — Three-dimensional vectors with dot product $\vec{a}\cdot\vec{b} = \sum a_ib_i$, cross product $\vec{a}\times\vec{b}$, magnitude, normalization, and scalar multiplication.

**Operations** — Linear interpolation (lerp), spherical linear interpolation (slerp) for smooth direction transitions, projection $\text{proj}_{\vec{b}}\vec{a} = \frac{\vec{a}\cdot\vec{b}}{\vec{b}\cdot\vec{b}}\vec{b}$, rejection, reflection $\vec{v} - 2(\vec{v}\cdot\hat{n})\hat{n}$, angle between vectors, scalar triple product $\vec{a}\cdot(\vec{b}\times\vec{c})$ (parallelepiped volume), and vector triple product.

**Differential Operators** — Numerical gradient $\nabla f$, divergence $\nabla\cdot\vec{F}$, curl $\nabla\times\vec{F}$, and Laplacian $\nabla^2 f$ on 3D vector fields via central differences.

**Integration** — Line integrals along parameterized curves, surface integrals, and flux computation $\iint \vec{F}\cdot d\vec{A}$.

**Simulation** — Vector field flow visualization, streamline tracing, and field evolution.

---

## Chapter 3 - Limits, Precision, and Validation

### Numerical Limits
- `f64` rounding errors can accumulate in long simulations.
- Extreme regimes (very large or very small scales) require explicit numerical stability checks.

### Recommended Verification
- Compare against a simple analytical case when available.
- Check the order of magnitude of results.
- Run sensitivity analysis on dominant parameters.

### Transition to Implementation
For concrete function calls, Rust signatures and module paths are documented in `docs/code/`.
