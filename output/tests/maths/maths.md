# SciForge Mathematics Test

70 benchmark entries across 5 formats.

---

## Grid
```
  calculus        :  FF  FDX   RF  SDX  SBP
  complex         :  CE   CS   MI
  dispatch        : DCS  DGK  DMP  DUF  DVM  DWP
  fft             :  FB   HW   PS
  graph           :  Bf   CC   Di
  integration     : HSI
  interpolation   :  HI   LI   LI
  linalg          : CEI  CEZ  LCA  MD2  TTI
  misc            :  PO   PZ   QP   TZ
  non_euclidean   :  HT  PDD   SR   SD
  ode             :  OE
  optimization    :  La   PB   QO
  pde             : HE1  WE1
  polynomial      :  CT   Le  PRR
  probability     : MCP  PEC  PNP
  signal          :  BG  KF1  LPR
  sparse          :  CG   JI   SA
  statistics      :  EZ   GM   HM  LRP   MB  MPI  SDB   WM  WMS
  tensor          :  TC   TD   TR
  vector          :  VD   VL   VP
```
---

## Directory Layout
```
csv/
  all.csv
json/
  {label}.json
  summary.json
yaml/
  {label}.yaml
  summary.yaml
toml/
  {label}.toml
  summary.toml
bmk/
  {label}.bmk
benchmark.html
benchmark.md
```

## Results
| Label | category | symbol | name | domain | test | Precision | Iterations | Avg (ns) | Iters/s |
|---|---|---|---|---|---|---|---|---|---|
| calculus_falling_factorial | calculus | FF | Falling factorial | maths | calculus::falling_factorial | f64 | 1 | 2356905.00 | 500.00 |
| calculus_first_derivative_x_squared | calculus | FDX | First derivative x squared | maths | calculus::first_derivative_x_squared | f64 | 1 | 2204724.00 | 500.00 |
| calculus_rising_factorial | calculus | RF | Rising factorial | maths | calculus::rising_factorial | f64 | 1 | 2119373.00 | 500.00 |
| calculus_second_derivative_x_squared | calculus | SDX | Second derivative x squared | maths | calculus::second_derivative_x_squared | f64 | 1 | 2098254.00 | 500.00 |
| calculus_stat_binomial_pmf | calculus | SBP | Stat binomial pmf | maths | calculus::stat_binomial_pmf | f64 | 1 | 1851564.00 | 1000.00 |
| complex_complex_exp | complex | CE | Complex exp | maths | complex::complex_exp | f64 | 1 | 1748573.00 | 1000.00 |
| complex_complex_sqrt | complex | CS | Complex sqrt | maths | complex::complex_sqrt | f64 | 1 | 1848794.00 | 1000.00 |
| complex_mandelbrot_iterate | complex | MI | Mandelbrot iterate | maths | complex::mandelbrot_iterate | f64 | 1 | 2418154.00 | 500.00 |
| dispatch_dispatch_complex_sqrt | dispatch | DCS | Dispatch complex sqrt | maths | dispatch::dispatch_complex_sqrt | f64 | 1 | 1966204.00 | 1000.00 |
| dispatch_dispatch_gaussian_kernel_smooth | dispatch | DGK | Dispatch gaussian kernel smooth | maths | dispatch::dispatch_gaussian_kernel_smooth | f64 | 1 | 2425794.00 | 500.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | maths | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 2037054.00 | 500.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | maths | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 1882223.00 | 1000.00 |
| dispatch_dispatch_vec_midpoint | dispatch | DVM | Dispatch vec midpoint | maths | dispatch::dispatch_vec_midpoint | f64 | 1 | 2012304.00 | 500.00 |
| dispatch_dispatch_wave_packet_gaussian | dispatch | DWP | Dispatch wave packet gaussian | maths | dispatch::dispatch_wave_packet_gaussian | f64 | 1 | 2438955.00 | 500.00 |
| fft_fft_basic | fft | FB | Fft basic | maths | fft::fft_basic | f64 | 1 | 2333844.00 | 500.00 |
| fft_hann_window | fft | HW | Hann window | maths | fft::hann_window | f64 | 1 | 2655205.00 | 500.00 |
| fft_power_spectrum | fft | PS | Power spectrum | maths | fft::power_spectrum | f64 | 1 | 2190354.00 | 500.00 |
| graph_bfs | graph | Bf | Bfs | maths | graph::bfs | f64 | 1 | 1910143.00 | 1000.00 |
| graph_connected_components | graph | CC | Connected components | maths | graph::connected_components | f64 | 1 | 1857234.00 | 1000.00 |
| graph_dijkstra | graph | Di | Dijkstra | maths | graph::dijkstra | f64 | 1 | 1868643.00 | 1000.00 |
| integration_halton_sequence_integration | integration | HSI | Halton sequence integration | maths | integration::halton_sequence_integration | f64 | 1 | 1943144.00 | 1000.00 |
| interpolation_hermite_interpolate | interpolation | HI | Hermite interpolate | maths | interpolation::hermite_interpolate | f64 | 1 | 2566635.00 | 500.00 |
| interpolation_lagrange_interpolate | interpolation | LI | Lagrange interpolate | maths | interpolation::lagrange_interpolate | f64 | 1 | 2301494.00 | 500.00 |
| interpolation_linear_interpolate | interpolation | LI | Linear interpolate | maths | interpolation::linear_interpolate | f64 | 1 | 2484164.00 | 500.00 |
| linalg_complex_exp_i_pi | linalg | CEI | Complex exp i pi | maths | linalg::complex_exp_i_pi | f64 | 1 | 2180605.00 | 500.00 |
| linalg_complex_exp_zero | linalg | CEZ | Complex exp zero | maths | linalg::complex_exp_zero | f64 | 1 | 1952873.00 | 1000.00 |
| linalg_levi_civita_antisymmetry | linalg | LCA | Levi civita antisymmetry | maths | linalg::levi_civita_antisymmetry | f64 | 1 | 2120644.00 | 500.00 |
| linalg_matrix_det_2x2 | linalg | MD2 | Matrix det 2x2 | maths | linalg::matrix_det_2x2 | f64 | 1 | 2110474.00 | 500.00 |
| linalg_tensor_trace_identity | linalg | TTI | Tensor trace identity | maths | linalg::tensor_trace_identity | f64 | 1 | 2182964.00 | 500.00 |
| misc_poly_one | misc | PO | Poly one | maths | misc::poly_one | f64 | 1 | 2522265.00 | 500.00 |
| misc_poly_zero | misc | PZ | Poly zero | maths | misc::poly_zero | f64 | 1 | 2435884.00 | 500.00 |
| misc_quaternion_pure | misc | QP | Quaternion pure | maths | misc::quaternion_pure | f64 | 1 | 2355745.00 | 500.00 |
| misc_tensor_zeros | misc | TZ | Tensor zeros | maths | misc::tensor_zeros | f64 | 1 | 2104403.00 | 500.00 |
| non_euclidean_hawking_temperature | non_euclidean | HT | Hawking temperature | maths | non_euclidean::hawking_temperature | f64 | 1 | 1974734.00 | 1000.00 |
| non_euclidean_poincare_disk_distance | non_euclidean | PDD | Poincare disk distance | maths | non_euclidean::poincare_disk_distance | f64 | 1 | 1948684.00 | 1000.00 |
| non_euclidean_schwarzschild_radius | non_euclidean | SR | Schwarzschild radius | maths | non_euclidean::schwarzschild_radius | f64 | 1 | 1864973.00 | 1000.00 |
| non_euclidean_spherical_distance | non_euclidean | SD | Spherical distance | maths | non_euclidean::spherical_distance | f64 | 1 | 2202784.00 | 500.00 |
| ode_ode_euler | ode | OE | Ode euler | maths | ode::ode_euler | f64 | 1 | 2443275.00 | 500.00 |
| optimization_lagrangian | optimization | La | Lagrangian | maths | optimization::lagrangian | f64 | 1 | 2479774.00 | 500.00 |
| optimization_project_box | optimization | PB | Project box | maths | optimization::project_box | f64 | 1 | 2335505.00 | 500.00 |
| optimization_quadratic_objective | optimization | QO | Quadratic objective | maths | optimization::quadratic_objective | f64 | 1 | 2256014.00 | 500.00 |
| pde_heat_equation_1d_explicit | pde | HE1 | Heat equation 1d explicit | maths | pde::heat_equation_1d_explicit | f64 | 1 | 2139254.00 | 500.00 |
| pde_wave_equation_1d | pde | WE1 | Wave equation 1d | maths | pde::wave_equation_1d | f64 | 1 | 2193404.00 | 500.00 |
| polynomial_chebyshev_t | polynomial | CT | Chebyshev t | maths | polynomial::chebyshev_t | f64 | 1 | 1985664.00 | 1000.00 |
| polynomial_legendre | polynomial | Le | Legendre | maths | polynomial::legendre | f64 | 1 | 2530374.00 | 500.00 |
| polynomial_polynomial_roots_real | polynomial | PRR | Polynomial roots real | maths | polynomial::polynomial_roots_real | f64 | 1 | 2606765.00 | 500.00 |
| probability_monte_carlo_pi | probability | MCP | Monte carlo pi | maths | probability::monte_carlo_pi | f64 | 1 | 3564017.00 | 333.33 |
| probability_prob_exponential_cdf | probability | PEC | Prob exponential cdf | maths | probability::prob_exponential_cdf | f64 | 1 | 1865943.00 | 1000.00 |
| probability_prob_normal_pdf | probability | PNP | Prob normal pdf | maths | probability::prob_normal_pdf | f64 | 1 | 2083054.00 | 500.00 |
| signal_butterworth_gain | signal | BG | Butterworth gain | maths | signal::butterworth_gain | f64 | 1 | 2236694.00 | 500.00 |
| signal_kalman_filter_1d | signal | KF1 | Kalman filter 1d | maths | signal::kalman_filter_1d | f64 | 1 | 2367275.00 | 500.00 |
| signal_low_pass_rc | signal | LPR | Low pass rc | maths | signal::low_pass_rc | f64 | 1 | 2764315.00 | 500.00 |
| sparse_conjugate_gradient | sparse | CG | Conjugate gradient | maths | sparse::conjugate_gradient | f64 | 1 | 2655215.00 | 500.00 |
| sparse_jacobi_iterate | sparse | JI | Jacobi iterate | maths | sparse::jacobi_iterate | f64 | 1 | 2362844.00 | 500.00 |
| sparse_sparse_add | sparse | SA | Sparse add | maths | sparse::sparse_add | f64 | 1 | 2037924.00 | 500.00 |
| statistics_erf_zero | statistics | EZ | Erf zero | maths | statistics::erf_zero | f64 | 1 | 2283064.00 | 500.00 |
| statistics_geometric_mean | statistics | GM | Geometric mean | maths | statistics::geometric_mean | f64 | 1 | 2055144.00 | 500.00 |
| statistics_harmonic_mean | statistics | HM | Harmonic mean | maths | statistics::harmonic_mean | f64 | 1 | 2030283.00 | 500.00 |
| statistics_linear_regression_perfect | statistics | LRP | Linear regression perfect | maths | statistics::linear_regression_perfect | f64 | 1 | 2280235.00 | 500.00 |
| statistics_mean_basic | statistics | MB | Mean basic | maths | statistics::mean_basic | f64 | 1 | 2119654.00 | 500.00 |
| statistics_mean_permutation_invariant | statistics | MPI | Mean permutation invariant | maths | statistics::mean_permutation_invariant | f64 | 1 | 2027263.00 | 500.00 |
| statistics_std_dev_basic | statistics | SDB | Std dev basic | maths | statistics::std_dev_basic | f64 | 1 | 2006704.00 | 500.00 |
| statistics_weighted_mean | statistics | WM | Weighted mean | maths | statistics::weighted_mean | f64 | 1 | 2369994.00 | 500.00 |
| statistics_weighted_mean_scale_invariant_on_weights | statistics | WMS | Weighted mean scale invariant on weights | maths | statistics::weighted_mean_scale_invariant_on_weights | f64 | 1 | 2109924.00 | 500.00 |
| tensor_tensor_contract | tensor | TC | Tensor contract | maths | tensor::tensor_contract | f64 | 1 | 2111934.00 | 500.00 |
| tensor_tensor_determinant | tensor | TD | Tensor determinant | maths | tensor::tensor_determinant | f64 | 1 | 2018274.00 | 500.00 |
| tensor_tensor_reshape | tensor | TR | Tensor reshape | maths | tensor::tensor_reshape | f64 | 1 | 2056984.00 | 500.00 |
| vector_vec_distance | vector | VD | Vec distance | maths | vector::vec_distance | f64 | 1 | 2087094.00 | 500.00 |
| vector_vec_lerp | vector | VL | Vec lerp | maths | vector::vec_lerp | f64 | 1 | 2642455.00 | 500.00 |
| vector_vec_project | vector | VP | Vec project | maths | vector::vec_project | f64 | 1 | 2081863.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| calculus | 5 | 2126164.00 | 1851564.00 | 2356905.00 |
| complex | 3 | 2005173.67 | 1748573.00 | 2418154.00 |
| dispatch | 6 | 2127089.00 | 1882223.00 | 2438955.00 |
| fft | 3 | 2393134.33 | 2190354.00 | 2655205.00 |
| graph | 3 | 1878673.33 | 1857234.00 | 1910143.00 |
| integration | 1 | 1943144.00 | 1943144.00 | 1943144.00 |
| interpolation | 3 | 2450764.33 | 2301494.00 | 2566635.00 |
| linalg | 5 | 2109512.00 | 1952873.00 | 2182964.00 |
| misc | 4 | 2354574.25 | 2104403.00 | 2522265.00 |
| non_euclidean | 4 | 1997793.75 | 1864973.00 | 2202784.00 |
| ode | 1 | 2443275.00 | 2443275.00 | 2443275.00 |
| optimization | 3 | 2357097.67 | 2256014.00 | 2479774.00 |
| pde | 2 | 2166329.00 | 2139254.00 | 2193404.00 |
| polynomial | 3 | 2374267.67 | 1985664.00 | 2606765.00 |
| probability | 3 | 2504338.00 | 1865943.00 | 3564017.00 |
| signal | 3 | 2456094.67 | 2236694.00 | 2764315.00 |
| sparse | 3 | 2351994.33 | 2037924.00 | 2655215.00 |
| statistics | 9 | 2142473.89 | 2006704.00 | 2369994.00 |
| tensor | 3 | 2062397.33 | 2018274.00 | 2111934.00 |
| vector | 3 | 2270470.67 | 2081863.00 | 2642455.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | complex_complex_exp | 1748573.00 |
| 2 | complex_complex_sqrt | 1848794.00 |
| 3 | calculus_stat_binomial_pmf | 1851564.00 |
| 4 | graph_connected_components | 1857234.00 |
| 5 | non_euclidean_schwarzschild_radius | 1864973.00 |
| 6 | probability_prob_exponential_cdf | 1865943.00 |
| 7 | graph_dijkstra | 1868643.00 |
| 8 | dispatch_dispatch_unknown_function_returns_error | 1882223.00 |
| 9 | graph_bfs | 1910143.00 |
| 10 | integration_halton_sequence_integration | 1943144.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | probability_monte_carlo_pi | 3564017.00 |
| 2 | signal_low_pass_rc | 2764315.00 |
| 3 | sparse_conjugate_gradient | 2655215.00 |
| 4 | fft_hann_window | 2655205.00 |
| 5 | vector_vec_lerp | 2642455.00 |
| 6 | polynomial_polynomial_roots_real | 2606765.00 |
| 7 | interpolation_hermite_interpolate | 2566635.00 |
| 8 | polynomial_legendre | 2530374.00 |
| 9 | misc_poly_one | 2522265.00 |
| 10 | interpolation_linear_interpolate | 2484164.00 |

