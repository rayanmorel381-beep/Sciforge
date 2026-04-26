## 1️⃣2️⃣ Hydrology

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Manning velocity | `manning_velocity(n, Rh, S) → f64` | $v = \frac{1}{n}R_h^{2/3}S^{1/2}$ | geology::hydrology |
| Chézy velocity | `chezy_velocity(C, Rh, S) → f64` | $v = C\sqrt{R_h S}$ | geology::hydrology |
| Froude number | `froude_number(v, g, d) → f64` | $Fr = \frac{v}{\sqrt{gd}}$ | geology::hydrology |
| Reynolds number | `reynolds_number(v, d, ν) → f64` | $Re = \frac{vd}{\nu}$ | geology::hydrology |
| Stream power | `stream_power(ρ, g, Q, S) → f64` | $\Omega = \rho g Q S$ W/m | geology::hydrology |
| Hjulström threshold | `hjulstrom_erosion_threshold(d_grain) → f64` | Hjulström empirical erosion curve | geology::hydrology |

## 1️⃣3️⃣ Glaciology

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Glen strain rate | `glen_strain_rate(A, τ, n) → f64` | $\dot\varepsilon = A(T)\tau^n$ — Arrhenius + Glen flow law | geology::glaciology |
| Shallow ice velocity | `shallow_ice_velocity(A, n, ρ, g, α, H) → f64` | $u = \frac{2A}{n+1}(\rho g\sin\alpha)^n H^{n+1}$ — SIA | geology::glaciology |
| Ice rheology viscosity | `ice_viscosity(A, τ, n) → f64` | $\eta = \frac{1}{2A\tau^{n-1}}$ | geology::glaciology |
| Glacial bed erosion rate | `glacial_bed_erosion(Kg, vb, l) → f64` | $\dot e = K_g v_b^l$ — Hallet 1979 abrasion law | geology::glaciology |

## 1️⃣4️⃣ Erosion & Weathering

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Fluvial erosion rate | `fluvial_erosion_rate(k, P, α, Vc) → f64` | $E = k \cdot P^{1.5}\tan\alpha(1 - V_c)$ — USLE-style | geology::erosion |
| Chemical weathering rate | `chemical_weathering_rate(A, Ea, T, P) → f64` | Arrhenius: $A\exp(-E_a/RT) \cdot P^{0.65}$ | geology::erosion |
| Frost weathering rate | `frost_weathering_rate(n_FT, φ) → f64` | $0.001 \cdot n_{FT} \cdot \phi$ — freeze-thaw cycles | geology::erosion |
| Wind erosion threshold | `wind_erosion_threshold(ρ_p, ρ_a, g, d) → f64` | Bagnold: $u_t = 0.1\sqrt{\frac{(\rho_p - \rho_a)gd}{\rho_a}}$ | geology::erosion |
| VEI classification | `volcanic_explosivity_index(volume) → u8` | VEI from ejecta volume | geology::petrology |
