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
| calculus_falling_factorial | calculus | FF | Falling factorial | maths | calculus::falling_factorial | f64 | 1 | 1990031.00 | 1000.00 |
| calculus_first_derivative_x_squared | calculus | FDX | First derivative x squared | maths | calculus::first_derivative_x_squared | f64 | 1 | 1957200.00 | 1000.00 |
| calculus_rising_factorial | calculus | RF | Rising factorial | maths | calculus::rising_factorial | f64 | 1 | 1983070.00 | 1000.00 |
| calculus_second_derivative_x_squared | calculus | SDX | Second derivative x squared | maths | calculus::second_derivative_x_squared | f64 | 1 | 1908211.00 | 1000.00 |
| calculus_stat_binomial_pmf | calculus | SBP | Stat binomial pmf | maths | calculus::stat_binomial_pmf | f64 | 1 | 1882920.00 | 1000.00 |
| complex_complex_exp | complex | CE | Complex exp | maths | complex::complex_exp | f64 | 1 | 2920860.00 | 500.00 |
| complex_complex_sqrt | complex | CS | Complex sqrt | maths | complex::complex_sqrt | f64 | 1 | 2484111.00 | 500.00 |
| complex_mandelbrot_iterate | complex | MI | Mandelbrot iterate | maths | complex::mandelbrot_iterate | f64 | 1 | 2574690.00 | 500.00 |
| dispatch_dispatch_complex_sqrt | dispatch | DCS | Dispatch complex sqrt | maths | dispatch::dispatch_complex_sqrt | f64 | 1 | 2284841.00 | 500.00 |
| dispatch_dispatch_gaussian_kernel_smooth | dispatch | DGK | Dispatch gaussian kernel smooth | maths | dispatch::dispatch_gaussian_kernel_smooth | f64 | 1 | 2393200.00 | 500.00 |
| dispatch_dispatch_missing_parameter_returns_error | dispatch | DMP | Dispatch missing parameter returns error | maths | dispatch::dispatch_missing_parameter_returns_error | f64 | 1 | 2029660.00 | 500.00 |
| dispatch_dispatch_unknown_function_returns_error | dispatch | DUF | Dispatch unknown function returns error | maths | dispatch::dispatch_unknown_function_returns_error | f64 | 1 | 1959641.00 | 1000.00 |
| dispatch_dispatch_vec_midpoint | dispatch | DVM | Dispatch vec midpoint | maths | dispatch::dispatch_vec_midpoint | f64 | 1 | 1864300.00 | 1000.00 |
| dispatch_dispatch_wave_packet_gaussian | dispatch | DWP | Dispatch wave packet gaussian | maths | dispatch::dispatch_wave_packet_gaussian | f64 | 1 | 1816500.00 | 1000.00 |
| fft_fft_basic | fft | FB | Fft basic | maths | fft::fft_basic | f64 | 1 | 1828561.00 | 1000.00 |
| fft_hann_window | fft | HW | Hann window | maths | fft::hann_window | f64 | 1 | 1853800.00 | 1000.00 |
| fft_power_spectrum | fft | PS | Power spectrum | maths | fft::power_spectrum | f64 | 1 | 1813960.00 | 1000.00 |
| graph_bfs | graph | Bf | Bfs | maths | graph::bfs | f64 | 1 | 1982491.00 | 1000.00 |
| graph_connected_components | graph | CC | Connected components | maths | graph::connected_components | f64 | 1 | 2281410.00 | 500.00 |
| graph_dijkstra | graph | Di | Dijkstra | maths | graph::dijkstra | f64 | 1 | 2460870.00 | 500.00 |
| integration_halton_sequence_integration | integration | HSI | Halton sequence integration | maths | integration::halton_sequence_integration | f64 | 1 | 2444481.00 | 500.00 |
| interpolation_hermite_interpolate | interpolation | HI | Hermite interpolate | maths | interpolation::hermite_interpolate | f64 | 1 | 2562340.00 | 500.00 |
| interpolation_lagrange_interpolate | interpolation | LI | Lagrange interpolate | maths | interpolation::lagrange_interpolate | f64 | 1 | 2872781.00 | 500.00 |
| interpolation_linear_interpolate | interpolation | LI | Linear interpolate | maths | interpolation::linear_interpolate | f64 | 1 | 2073270.00 | 500.00 |
| linalg_complex_exp_i_pi | linalg | CEI | Complex exp i pi | maths | linalg::complex_exp_i_pi | f64 | 1 | 2250840.00 | 500.00 |
| linalg_complex_exp_zero | linalg | CEZ | Complex exp zero | maths | linalg::complex_exp_zero | f64 | 1 | 1949281.00 | 1000.00 |
| linalg_levi_civita_antisymmetry | linalg | LCA | Levi civita antisymmetry | maths | linalg::levi_civita_antisymmetry | f64 | 1 | 1968600.00 | 1000.00 |
| linalg_matrix_det_2x2 | linalg | MD2 | Matrix det 2x2 | maths | linalg::matrix_det_2x2 | f64 | 1 | 2276590.00 | 500.00 |
| linalg_tensor_trace_identity | linalg | TTI | Tensor trace identity | maths | linalg::tensor_trace_identity | f64 | 1 | 2053151.00 | 500.00 |
| misc_poly_one | misc | PO | Poly one | maths | misc::poly_one | f64 | 1 | 2082020.00 | 500.00 |
| misc_poly_zero | misc | PZ | Poly zero | maths | misc::poly_zero | f64 | 1 | 1918300.00 | 1000.00 |
| misc_quaternion_pure | misc | QP | Quaternion pure | maths | misc::quaternion_pure | f64 | 1 | 1839071.00 | 1000.00 |
| misc_tensor_zeros | misc | TZ | Tensor zeros | maths | misc::tensor_zeros | f64 | 1 | 1972660.00 | 1000.00 |
| non_euclidean_hawking_temperature | non_euclidean | HT | Hawking temperature | maths | non_euclidean::hawking_temperature | f64 | 1 | 1924360.00 | 1000.00 |
| non_euclidean_poincare_disk_distance | non_euclidean | PDD | Poincare disk distance | maths | non_euclidean::poincare_disk_distance | f64 | 1 | 1804441.00 | 1000.00 |
| non_euclidean_schwarzschild_radius | non_euclidean | SR | Schwarzschild radius | maths | non_euclidean::schwarzschild_radius | f64 | 1 | 1946810.00 | 1000.00 |
| non_euclidean_spherical_distance | non_euclidean | SD | Spherical distance | maths | non_euclidean::spherical_distance | f64 | 1 | 2413530.00 | 500.00 |
| ode_ode_euler | ode | OE | Ode euler | maths | ode::ode_euler | f64 | 1 | 2024311.00 | 500.00 |
| optimization_lagrangian | optimization | La | Lagrangian | maths | optimization::lagrangian | f64 | 1 | 1847660.00 | 1000.00 |
| optimization_project_box | optimization | PB | Project box | maths | optimization::project_box | f64 | 1 | 1844210.00 | 1000.00 |
| optimization_quadratic_objective | optimization | QO | Quadratic objective | maths | optimization::quadratic_objective | f64 | 1 | 1778321.00 | 1000.00 |
| pde_heat_equation_1d_explicit | pde | HE1 | Heat equation 1d explicit | maths | pde::heat_equation_1d_explicit | f64 | 1 | 1848590.00 | 1000.00 |
| pde_wave_equation_1d | pde | WE1 | Wave equation 1d | maths | pde::wave_equation_1d | f64 | 1 | 1797220.00 | 1000.00 |
| polynomial_chebyshev_t | polynomial | CT | Chebyshev t | maths | polynomial::chebyshev_t | f64 | 1 | 1804861.00 | 1000.00 |
| polynomial_legendre | polynomial | Le | Legendre | maths | polynomial::legendre | f64 | 1 | 2455470.00 | 500.00 |
| polynomial_polynomial_roots_real | polynomial | PRR | Polynomial roots real | maths | polynomial::polynomial_roots_real | f64 | 1 | 2537660.00 | 500.00 |
| probability_monte_carlo_pi | probability | MCP | Monte carlo pi | maths | probability::monte_carlo_pi | f64 | 1 | 3738621.00 | 333.33 |
| probability_prob_exponential_cdf | probability | PEC | Prob exponential cdf | maths | probability::prob_exponential_cdf | f64 | 1 | 2172550.00 | 500.00 |
| probability_prob_normal_pdf | probability | PNP | Prob normal pdf | maths | probability::prob_normal_pdf | f64 | 1 | 2133941.00 | 500.00 |
| signal_butterworth_gain | signal | BG | Butterworth gain | maths | signal::butterworth_gain | f64 | 1 | 2258720.00 | 500.00 |
| signal_kalman_filter_1d | signal | KF1 | Kalman filter 1d | maths | signal::kalman_filter_1d | f64 | 1 | 2035640.00 | 500.00 |
| signal_low_pass_rc | signal | LPR | Low pass rc | maths | signal::low_pass_rc | f64 | 1 | 2351241.00 | 500.00 |
| sparse_conjugate_gradient | sparse | CG | Conjugate gradient | maths | sparse::conjugate_gradient | f64 | 1 | 2125160.00 | 500.00 |
| sparse_jacobi_iterate | sparse | JI | Jacobi iterate | maths | sparse::jacobi_iterate | f64 | 1 | 2233721.00 | 500.00 |
| sparse_sparse_add | sparse | SA | Sparse add | maths | sparse::sparse_add | f64 | 1 | 2027670.00 | 500.00 |
| statistics_erf_zero | statistics | EZ | Erf zero | maths | statistics::erf_zero | f64 | 1 | 2030930.00 | 500.00 |
| statistics_geometric_mean | statistics | GM | Geometric mean | maths | statistics::geometric_mean | f64 | 1 | 2027001.00 | 500.00 |
| statistics_harmonic_mean | statistics | HM | Harmonic mean | maths | statistics::harmonic_mean | f64 | 1 | 2340340.00 | 500.00 |
| statistics_linear_regression_perfect | statistics | LRP | Linear regression perfect | maths | statistics::linear_regression_perfect | f64 | 1 | 2280320.00 | 500.00 |
| statistics_mean_basic | statistics | MB | Mean basic | maths | statistics::mean_basic | f64 | 1 | 1934541.00 | 1000.00 |
| statistics_mean_permutation_invariant | statistics | MPI | Mean permutation invariant | maths | statistics::mean_permutation_invariant | f64 | 1 | 1906830.00 | 1000.00 |
| statistics_std_dev_basic | statistics | SDB | Std dev basic | maths | statistics::std_dev_basic | f64 | 1 | 2184880.00 | 500.00 |
| statistics_weighted_mean | statistics | WM | Weighted mean | maths | statistics::weighted_mean | f64 | 1 | 2107701.00 | 500.00 |
| statistics_weighted_mean_scale_invariant_on_weights | statistics | WMS | Weighted mean scale invariant on weights | maths | statistics::weighted_mean_scale_invariant_on_weights | f64 | 1 | 2120880.00 | 500.00 |
| tensor_tensor_contract | tensor | TC | Tensor contract | maths | tensor::tensor_contract | f64 | 1 | 2281260.00 | 500.00 |
| tensor_tensor_determinant | tensor | TD | Tensor determinant | maths | tensor::tensor_determinant | f64 | 1 | 2509311.00 | 500.00 |
| tensor_tensor_reshape | tensor | TR | Tensor reshape | maths | tensor::tensor_reshape | f64 | 1 | 2353490.00 | 500.00 |
| vector_vec_distance | vector | VD | Vec distance | maths | vector::vec_distance | f64 | 1 | 2458681.00 | 500.00 |
| vector_vec_lerp | vector | VL | Vec lerp | maths | vector::vec_lerp | f64 | 1 | 2324770.00 | 500.00 |
| vector_vec_project | vector | VP | Vec project | maths | vector::vec_project | f64 | 1 | 2059010.00 | 500.00 |
## Analytics
### Category Distribution
| Category | Count | Avg (ns) | Min (ns) | Max (ns) |
|---|---|---|---|---|
| calculus | 5 | 1944286.40 | 1882920.00 | 1990031.00 |
| complex | 3 | 2659887.00 | 2484111.00 | 2920860.00 |
| dispatch | 6 | 2058023.67 | 1816500.00 | 2393200.00 |
| fft | 3 | 1832107.00 | 1813960.00 | 1853800.00 |
| graph | 3 | 2241590.33 | 1982491.00 | 2460870.00 |
| integration | 1 | 2444481.00 | 2444481.00 | 2444481.00 |
| interpolation | 3 | 2502797.00 | 2073270.00 | 2872781.00 |
| linalg | 5 | 2099692.40 | 1949281.00 | 2276590.00 |
| misc | 4 | 1953012.75 | 1839071.00 | 2082020.00 |
| non_euclidean | 4 | 2022285.25 | 1804441.00 | 2413530.00 |
| ode | 1 | 2024311.00 | 2024311.00 | 2024311.00 |
| optimization | 3 | 1823397.00 | 1778321.00 | 1847660.00 |
| pde | 2 | 1822905.00 | 1797220.00 | 1848590.00 |
| polynomial | 3 | 2265997.00 | 1804861.00 | 2537660.00 |
| probability | 3 | 2681704.00 | 2133941.00 | 3738621.00 |
| signal | 3 | 2215200.33 | 2035640.00 | 2351241.00 |
| sparse | 3 | 2128850.33 | 2027670.00 | 2233721.00 |
| statistics | 9 | 2103713.67 | 1906830.00 | 2340340.00 |
| tensor | 3 | 2381353.67 | 2281260.00 | 2509311.00 |
| vector | 3 | 2280820.33 | 2059010.00 | 2458681.00 |

### Top 10 Fastest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | optimization_quadratic_objective | 1778321.00 |
| 2 | pde_wave_equation_1d | 1797220.00 |
| 3 | non_euclidean_poincare_disk_distance | 1804441.00 |
| 4 | polynomial_chebyshev_t | 1804861.00 |
| 5 | fft_power_spectrum | 1813960.00 |
| 6 | dispatch_dispatch_wave_packet_gaussian | 1816500.00 |
| 7 | fft_fft_basic | 1828561.00 |
| 8 | misc_quaternion_pure | 1839071.00 |
| 9 | optimization_project_box | 1844210.00 |
| 10 | optimization_lagrangian | 1847660.00 |

### Top 10 Slowest Entries
| # | Entry | Avg (ns) |
|---|---|---|
| 1 | probability_monte_carlo_pi | 3738621.00 |
| 2 | complex_complex_exp | 2920860.00 |
| 3 | interpolation_lagrange_interpolate | 2872781.00 |
| 4 | complex_mandelbrot_iterate | 2574690.00 |
| 5 | interpolation_hermite_interpolate | 2562340.00 |
| 6 | polynomial_polynomial_roots_real | 2537660.00 |
| 7 | tensor_tensor_determinant | 2509311.00 |
| 8 | complex_complex_sqrt | 2484111.00 |
| 9 | graph_dijkstra | 2460870.00 |
| 10 | vector_vec_distance | 2458681.00 |

