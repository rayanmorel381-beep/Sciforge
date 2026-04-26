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
| calculus_falling_factorial | calculus | FF | Falling factorial | maths | calculus::falling_factorial | f64 | 1 | 2097549.00 | 500.00 |
| calculus_first_derivative_x_squared | calculus | FDX | First derivative x squared | maths | calculus::first_derivative_x_squared | f64 | 1 | 1928369.00 | 1000.00 |
| calculus_rising_factorial | calculus | RF | Rising factorial | maths | calculus::rising_factorial | f64 | 1 | 1884860.00 | 1000.00 |
| calculus_second_derivative_x_squared | calculus | SDX | Second derivative x squared | maths | calculus::second_derivative_x_squared | f64 | 1 | 1798479.00 | 1000.00 |
| calculus_stat_binomial_pmf | calculus | SBP | Stat binomial pmf | maths | calculus::stat_binomial_pmf | f64 | 1 | 1840889.00 | 1000.00 |
| complex_complex_exp | complex | CE | Complex exp | maths | complex::complex_exp | f64 | 1 | 1705199.00 | 1000.00 |
| complex_complex_sqrt | complex | CS | Complex sqrt | maths | complex::complex_sqrt | f64 | 1 | 1787240.00 | 1000.00 |
| complex_mandelbrot_iterate | complex | MI | Mandelbrot iterate | maths | complex::mandelbrot_iterate | f64 | 1 | 2351108.00 | 500.00 |
| dispatch_dispatch_complex_sqrt | dispatch | DCS | Dispatch complex sqrt | maths | dispatch::dispatch_complex_sqrt | f64 | 1 | 2057909.00 | 500.00 |
| dispatch_dispatch_gaussian_kernel_smooth | dispatch | DGK | Dispatch gaussian kernel smooth | maths | dispatch::dispatch_gaussian_kernel_smooth | f64 | 1 | 2152220.00 | 500.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | maths | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 1859399.00 | 1000.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | maths | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 1785359.00 | 1000.00 |
| dispatch_dispatch_vec_midpoint | dispatch | DVM | Dispatch vec midpoint | maths | dispatch::dispatch_vec_midpoint | f64 | 1 | 1789629.00 | 1000.00 |
| dispatch_dispatch_wave_packet_gaussian | dispatch | DWP | Dispatch wave packet gaussian | maths | dispatch::dispatch_wave_packet_gaussian | f64 | 1 | 1752669.00 | 1000.00 |
| fft_fft_basic | fft | FB | Fft basic | maths | fft::fft_basic | f64 | 1 | 1799410.00 | 1000.00 |
| fft_hann_window | fft | HW | Hann window | maths | fft::hann_window | f64 | 1 | 1721869.00 | 1000.00 |
| fft_power_spectrum | fft | PS | Power spectrum | maths | fft::power_spectrum | f64 | 1 | 1760199.00 | 1000.00 |
| graph_bfs | graph | Bf | Bfs | maths | graph::bfs | f64 | 1 | 1824929.00 | 1000.00 |
| graph_connected_components | graph | CC | Connected components | maths | graph::connected_components | f64 | 1 | 1751820.00 | 1000.00 |
| graph_dijkstra | graph | Di | Dijkstra | maths | graph::dijkstra | f64 | 1 | 1701189.00 | 1000.00 |
| integration_halton_sequence_integration | integration | HSI | Halton sequence integration | maths | integration::halton_sequence_integration | f64 | 1 | 1689589.00 | 1000.00 |
| interpolation_hermite_interpolate | interpolation | HI | Hermite interpolate | maths | interpolation::hermite_interpolate | f64 | 1 | 1660459.00 | 1000.00 |
| interpolation_lagrange_interpolate | interpolation | LI | Lagrange interpolate | maths | interpolation::lagrange_interpolate | f64 | 1 | 1660540.00 | 1000.00 |
| interpolation_linear_interpolate | interpolation | LI | Linear interpolate | maths | interpolation::linear_interpolate | f64 | 1 | 1987209.00 | 1000.00 |
| linalg_complex_exp_i_pi | linalg | CEI | Complex exp i pi | maths | linalg::complex_exp_i_pi | f64 | 1 | 2444709.00 | 500.00 |
| linalg_complex_exp_zero | linalg | CEZ | Complex exp zero | maths | linalg::complex_exp_zero | f64 | 1 | 2179469.00 | 500.00 |
| linalg_levi_civita_antisymmetry | linalg | LCA | Levi civita antisymmetry | maths | linalg::levi_civita_antisymmetry | f64 | 1 | 2012899.00 | 500.00 |
| linalg_matrix_det_2x2 | linalg | MD2 | Matrix det 2x2 | maths | linalg::matrix_det_2x2 | f64 | 1 | 1764019.00 | 1000.00 |
| linalg_tensor_trace_identity | linalg | TTI | Tensor trace identity | maths | linalg::tensor_trace_identity | f64 | 1 | 1772459.00 | 1000.00 |
| misc_poly_one | misc | PO | Poly one | maths | misc::poly_one | f64 | 1 | 1701799.00 | 1000.00 |
| misc_poly_zero | misc | PZ | Poly zero | maths | misc::poly_zero | f64 | 1 | 2202689.00 | 500.00 |
| misc_quaternion_pure | misc | QP | Quaternion pure | maths | misc::quaternion_pure | f64 | 1 | 1961720.00 | 1000.00 |
| misc_tensor_zeros | misc | TZ | Tensor zeros | maths | misc::tensor_zeros | f64 | 1 | 1889299.00 | 1000.00 |
| non_euclidean_hawking_temperature | non_euclidean | HT | Hawking temperature | maths | non_euclidean::hawking_temperature | f64 | 1 | 1857649.00 | 1000.00 |
| non_euclidean_poincare_disk_distance | non_euclidean | PDD | Poincare disk distance | maths | non_euclidean::poincare_disk_distance | f64 | 1 | 1762299.00 | 1000.00 |
| non_euclidean_schwarzschild_radius | non_euclidean | SR | Schwarzschild radius | maths | non_euclidean::schwarzschild_radius | f64 | 1 | 1723750.00 | 1000.00 |
| non_euclidean_spherical_distance | non_euclidean | SD | Spherical distance | maths | non_euclidean::spherical_distance | f64 | 1 | 1794409.00 | 1000.00 |
| ode_ode_euler | ode | OE | Ode euler | maths | ode::ode_euler | f64 | 1 | 1969669.00 | 1000.00 |
| optimization_lagrangian | optimization | La | Lagrangian | maths | optimization::lagrangian | f64 | 1 | 1821789.00 | 1000.00 |
| optimization_project_box | optimization | PB | Project box | maths | optimization::project_box | f64 | 1 | 1922459.00 | 1000.00 |
| optimization_quadratic_objective | optimization | QO | Quadratic objective | maths | optimization::quadratic_objective | f64 | 1 | 1774419.00 | 1000.00 |
| pde_heat_equation_1d_explicit | pde | HE1 | Heat equation 1d explicit | maths | pde::heat_equation_1d_explicit | f64 | 1 | 1753250.00 | 1000.00 |
| pde_wave_equation_1d | pde | WE1 | Wave equation 1d | maths | pde::wave_equation_1d | f64 | 1 | 1734079.00 | 1000.00 |
| polynomial_chebyshev_t | polynomial | CT | Chebyshev t | maths | polynomial::chebyshev_t | f64 | 1 | 1755359.00 | 1000.00 |
| polynomial_legendre | polynomial | Le | Legendre | maths | polynomial::legendre | f64 | 1 | 1748909.00 | 1000.00 |
| polynomial_polynomial_roots_real | polynomial | PRR | Polynomial roots real | maths | polynomial::polynomial_roots_real | f64 | 1 | 1926249.00 | 1000.00 |
| probability_monte_carlo_pi | probability | MCP | Monte carlo pi | maths | probability::monte_carlo_pi | f64 | 1 | 3363089.00 | 333.33 |
| probability_prob_exponential_cdf | probability | PEC | Prob exponential cdf | maths | probability::prob_exponential_cdf | f64 | 1 | 1985959.00 | 1000.00 |
| probability_prob_normal_pdf | probability | PNP | Prob normal pdf | maths | probability::prob_normal_pdf | f64 | 1 | 1760199.00 | 1000.00 |
| signal_butterworth_gain | signal | BG | Butterworth gain | maths | signal::butterworth_gain | f64 | 1 | 1839189.00 | 1000.00 |
| signal_kalman_filter_1d | signal | KF1 | Kalman filter 1d | maths | signal::kalman_filter_1d | f64 | 1 | 2155400.00 | 500.00 |
| signal_low_pass_rc | signal | LPR | Low pass rc | maths | signal::low_pass_rc | f64 | 1 | 2035739.00 | 500.00 |
| sparse_conjugate_gradient | sparse | CG | Conjugate gradient | maths | sparse::conjugate_gradient | f64 | 1 | 1964579.00 | 1000.00 |
| sparse_jacobi_iterate | sparse | JI | Jacobi iterate | maths | sparse::jacobi_iterate | f64 | 1 | 1769559.00 | 1000.00 |
| sparse_sparse_add | sparse | SA | Sparse add | maths | sparse::sparse_add | f64 | 1 | 1799959.00 | 1000.00 |
| statistics_erf_zero | statistics | EZ | Erf zero | maths | statistics::erf_zero | f64 | 1 | 1779029.00 | 1000.00 |
| statistics_geometric_mean | statistics | GM | Geometric mean | maths | statistics::geometric_mean | f64 | 1 | 1788200.00 | 1000.00 |
| statistics_harmonic_mean | statistics | HM | Harmonic mean | maths | statistics::harmonic_mean | f64 | 1 | 1748879.00 | 1000.00 |
| statistics_linear_regression_perfect | statistics | LRP | Linear regression perfect | maths | statistics::linear_regression_perfect | f64 | 1 | 1733909.00 | 1000.00 |
| statistics_mean_basic | statistics | MB | Mean basic | maths | statistics::mean_basic | f64 | 1 | 1722679.00 | 1000.00 |
| statistics_mean_permutation_invariant | statistics | MPI | Mean permutation invariant | maths | statistics::mean_permutation_invariant | f64 | 1 | 1768160.00 | 1000.00 |
| statistics_std_dev_basic | statistics | SDB | Std dev basic | maths | statistics::std_dev_basic | f64 | 1 | 1723279.00 | 1000.00 |
| statistics_weighted_mean | statistics | WM | Weighted mean | maths | statistics::weighted_mean | f64 | 1 | 1765579.00 | 1000.00 |
| statistics_weighted_mean_scale_invariant_on_weights | statistics | WMS | Weighted mean scale invariant on weights | maths | statistics::weighted_mean_scale_invariant_on_weights | f64 | 1 | 1824729.00 | 1000.00 |
| tensor_tensor_contract | tensor | TC | Tensor contract | maths | tensor::tensor_contract | f64 | 1 | 1872079.00 | 1000.00 |
| tensor_tensor_determinant | tensor | TD | Tensor determinant | maths | tensor::tensor_determinant | f64 | 1 | 1737300.00 | 1000.00 |
| tensor_tensor_reshape | tensor | TR | Tensor reshape | maths | tensor::tensor_reshape | f64 | 1 | 1689109.00 | 1000.00 |
| vector_vec_distance | vector | VD | Vec distance | maths | vector::vec_distance | f64 | 1 | 1730279.00 | 1000.00 |
| vector_vec_lerp | vector | VL | Vec lerp | maths | vector::vec_lerp | f64 | 1 | 1788729.00 | 1000.00 |
| vector_vec_project | vector | VP | Vec project | maths | vector::vec_project | f64 | 1 | 1988819.00 | 1000.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| calculus | 5 | 1910029.25 | 1798479.00 | 2097549.00 |
| complex | 3 | 1947849.00 | 1705199.00 | 2351108.00 |
| dispatch | 6 | 1899530.88 | 1752669.00 | 2152220.00 |
| fft | 3 | 1760492.62 | 1721869.00 | 1799410.00 |
| graph | 3 | 1759312.62 | 1701189.00 | 1824929.00 |
| integration | 1 | 1689589.00 | 1689589.00 | 1689589.00 |
| interpolation | 3 | 1769402.62 | 1660459.00 | 1987209.00 |
| linalg | 5 | 2034711.00 | 1764019.00 | 2444709.00 |
| misc | 4 | 1938876.75 | 1701799.00 | 2202689.00 |
| non_euclidean | 4 | 1784526.75 | 1723750.00 | 1857649.00 |
| ode | 1 | 1969669.00 | 1969669.00 | 1969669.00 |
| optimization | 3 | 1839555.62 | 1774419.00 | 1922459.00 |
| pde | 2 | 1743664.50 | 1734079.00 | 1753250.00 |
| polynomial | 3 | 1810172.38 | 1748909.00 | 1926249.00 |
| probability | 3 | 2369749.00 | 1760199.00 | 3363089.00 |
| signal | 3 | 2010109.38 | 1839189.00 | 2155400.00 |
| sparse | 3 | 1844699.00 | 1769559.00 | 1964579.00 |
| statistics | 9 | 1761604.75 | 1722679.00 | 1824729.00 |
| tensor | 3 | 1766162.62 | 1689109.00 | 1872079.00 |
| vector | 3 | 1835942.38 | 1730279.00 | 1988819.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | interpolation_hermite_interpolate | 1660459.00 |
| 2 | interpolation_lagrange_interpolate | 1660540.00 |
| 3 | tensor_tensor_reshape | 1689109.00 |
| 4 | integration_halton_sequence_integration | 1689589.00 |
| 5 | graph_dijkstra | 1701189.00 |
| 6 | misc_poly_one | 1701799.00 |
| 7 | complex_complex_exp | 1705199.00 |
| 8 | fft_hann_window | 1721869.00 |
| 9 | statistics_mean_basic | 1722679.00 |
| 10 | statistics_std_dev_basic | 1723279.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | probability_monte_carlo_pi | 3363089.00 |
| 2 | linalg_complex_exp_i_pi | 2444709.00 |
| 3 | complex_mandelbrot_iterate | 2351108.00 |
| 4 | misc_poly_zero | 2202689.00 |
| 5 | linalg_complex_exp_zero | 2179469.00 |
| 6 | signal_kalman_filter_1d | 2155400.00 |
| 7 | dispatch_dispatch_gaussian_kernel_smooth | 2152220.00 |
| 8 | calculus_falling_factorial | 2097549.00 |
| 9 | dispatch_dispatch_complex_sqrt | 2057909.00 |
| 10 | signal_low_pass_rc | 2035739.00 |

