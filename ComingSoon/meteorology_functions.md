## 9️⃣ Atmospheric Physics

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Barometric pressure | `barometric_pressure(h, T0, P0, M, g) → f64` | ISA barometric formula (troposphere + stratosphere) | meteorology::atmosphere |
| Scale height | `scale_height(T, M, g) → f64` | $H = \frac{RT}{Mg}$ | meteorology::atmosphere |
| Rayleigh phase function | `rayleigh_phase(cos_θ) → f64` | $\frac{3}{16\pi}(1+\cos^2\theta)$ | meteorology::atmosphere |
| Mie phase function | `mie_phase(cos_θ, g) → f64` | Henyey-Greenstein: $\frac{3(1-g^2)(1+\cos^2\theta)}{8\pi(2+g^2)(1+g^2-2g\cos\theta)^{3/2}}$ | meteorology::atmosphere |

## 🔟 Winds & Storms

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Geostrophic wind speed | `geostrophic_wind(dp_dn, ρ, f) → f64` | $v_g = \frac{1}{\rho f}\frac{\partial p}{\partial n}$ | meteorology::winds |
| Gradient wind speed | `gradient_wind(dp_dn, ρ, f, R_curv) → f64` | Gradient wind equation with centripetal term | meteorology::winds |
| Thermal wind shear | `thermal_wind_shear(dT_dn, g, f, T, dp, p) → f64` | $\Delta v = -\frac{g}{fT}\frac{\partial T}{\partial n}\frac{\Delta p}{p}$ | meteorology::winds |
| Ekman spiral depth | `ekman_depth(K, f) → f64` | $D_E = \pi\sqrt{\frac{2K}{f}}$ | meteorology::winds |
| Beaufort to m/s | `beaufort_to_m_s(B) → f64` | $v = 0.836 \cdot B^{1.5}$ | meteorology::winds |
| Wind chill temperature | `wind_chill(T, v) → f64` | Environment Canada formula | meteorology::winds |
| Potential intensity | `potential_intensity(Ck, Cd, η, Δk) → f64` | Emanuel MPI: $v_{pot} = \sqrt{\frac{C_k}{C_d}\cdot\eta\cdot\Delta k}$ | meteorology::storms |
| ACE index | `accumulated_cyclone_energy(v_kt_series) → f64` | $\text{ACE} = \sum v_{kt}^2 / 10^4$ | meteorology::storms |
| CAPE | `cape(T_parcel, T_env, g, Δz) → f64` | $\text{CAPE} = g\frac{T_p - T_e}{T_e}\Delta z$ | meteorology::storms |
| Rossby deformation radius | `rossby_deformation_radius(N, H, f) → f64` | $L_R = NH/f$ | meteorology::storms |
| Fujita scale | `fujita_scale(v) → u8` | Enhanced Fujita classification | meteorology::storms |

## 1️⃣1️⃣ Oceanography

| Function | Signature | Formula / Description | Module |
|---|---|---|---|
| Seawater density | `seawater_density(T, S) → f64` | UNESCO polynomial equation of state | meteorology::oceanography |
| Sound speed in seawater | `sound_speed(T, S, d) → f64` | Mackenzie/Chen-Millero equation | meteorology::oceanography |
| Ocean heat content | `ocean_heat_content(ρ, cp, A, d, ΔT) → f64` | $Q = \rho c_p A d \Delta T$ | meteorology::oceanography |
| Thermosteric sea level rise | `sea_level_rise_thermal(α, ΔT, d) → f64` | $\Delta h = \alpha \Delta T \cdot d$ | meteorology::oceanography |
| Henry solubility | `henry_solubility(kH, T, ΔH) → f64` | Henry's law + van 't Hoff temperature correction | meteorology::oceanography |
| Phillips wave spectrum | `phillips_spectrum(kx, ky, v_wind) → f64` | Directional wave energy spectrum | meteorology::oceanography |
| Wave dispersion | `wave_dispersion(k, d) → f64` | $\omega = \sqrt{gk\tanh(kd)}$ — deep/shallow water | meteorology::oceanography |
