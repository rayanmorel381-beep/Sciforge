# Physics Module

The **Physics** module of SciForge covers a broad spectrum of classical and modern physics, organized into **11 submodules** and **45 source files**. It provides computational tools for fundamental laws, field equations, wave phenomena, materials science, and quantum mechanics.

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

### 1. Acoustics

Acoustics studies the propagation of mechanical waves through material media (gases, liquids, solids). SciForge models this domain along four axes:

**Propagation** — Sound waves are pressure perturbations that travel at a speed dependent on the medium: $c = \sqrt{\gamma R T / M}$ in an ideal gas, $c = \sqrt{E/\rho}$ in a solid. The module computes acoustic impedance $Z = \rho c$, transmission and reflection coefficients at interfaces (analogous to Fresnel coefficients in optics), attenuation by geometric divergence (inverse-square law for spherical waves), and sound pressure level in dB SPL.

**Doppler Effect** — When a source and observer are in relative motion, the perceived frequency shifts according to $f' = f_0 \frac{c + v_\text{obs}}{c + v_\text{src}}$. The module implements the classical cases (approaching/receding source), the relativistic Doppler $f' = f_0 \sqrt{(1+\beta)/(1-\beta)}$, radar Doppler (round-trip: factor of 2), and the Mach cone angle $\theta = \arcsin(c/v)$ for supersonic objects.

**Absorption** — Sound energy is absorbed exponentially: $I(x) = I_0 e^{-\alpha x}$. The module computes atmospheric absorption (dependent on frequency, humidity, and temperature), A-weighting (standardized psychoacoustic filter), NRC (Noise Reduction Coefficient), transmission loss (mass law), dB level addition, and room constant.

**Resonance** — Acoustic systems resonate at discrete frequencies. The module calculates the fundamental frequency of a vibrating string $f_1 = \frac{1}{2L}\sqrt{T/\mu}$, harmonics of open and closed pipes, the Helmholtz resonator $f = \frac{c}{2\pi}\sqrt{A/(Vl)}$, beats, quality factor, Sabine reverberation time ($T_{60} = 0.161 V/A$), room eigenmodes for a rectangular enclosure, and Schroeder frequency.

---

### 2. Electrodynamics

Classical electrodynamics, grounded in Maxwell's equations, describes the interaction between electric and magnetic fields. SciForge structures this domain across three files:

**Fields** — The electric field of a point charge follows Coulomb's law: $\vec{E} = \frac{kq}{r^2}\hat{r}$, with $k = 1/(4\pi\epsilon_0)$. The module computes electric potential, the magnetic field of an infinite wire (Ampère's theorem: $B = \mu_0 I / 2\pi r$), of a solenoid ($B = \mu_0 n I$), of a current loop, and the Biot-Savart field for an arbitrary segment $d\vec{B} = \frac{\mu_0 I}{4\pi} \frac{d\vec{l} \times \hat{r}}{r^2}$. The Lorentz force $\vec{F} = q(\vec{E} + \vec{v}\times\vec{B})$ unifies electric and magnetic effects. The Poynting vector $\vec{S} = \frac{1}{\mu_0}\vec{E}\times\vec{B}$ gives the electromagnetic energy flux, and the energy density combines contributions: $u = \frac{1}{2}(\epsilon_0 E^2 + B^2/\mu_0)$.

**EM Waves** — Wave solutions of Maxwell's equations propagate at $c = 1/\sqrt{\mu_0\epsilon_0}$. The module computes the impedance of free space $Z_0 = \sqrt{\mu_0/\epsilon_0} \approx 377\,\Omega$, wave number, wavelength, phase velocity in a medium $v = c/\sqrt{\epsilon_r\mu_r}$, group velocity in a dispersive medium, skin depth (penetration into conductors), Fresnel coefficients $r_s$ and $r_p$, Brewster's angle, critical angle for total internal reflection, Snell's law, radiation pressure (absorbed $P=I/c$ and reflected $P=2I/c$), Larmor power (radiation from an accelerated charge), radiation resistance of a dipole, and a 1D FDTD (Finite-Difference Time-Domain) solver for direct simulation of wave propagation.

**Circuits** — The series RLC circuit is modeled as a complete structure: resonant frequency $f_0 = 1/(2\pi\sqrt{LC})$, quality factor $Q = (1/R)\sqrt{L/C}$, complex impedance $Z(\omega) = R + j(\omega L - 1/\omega C)$, damping coefficient, bandwidth, and transient response (underdamped, overdamped, critically damped). The module also provides RC and RL time constants and AC regime analysis.

---

### 3. Electronics

Applied electronics uses the laws of physics to design practical circuits. SciForge covers four sub-areas:

**Circuits** — Fundamental laws: Ohm's law ($V = IR$), resistors in series/parallel, voltage and current dividers, DC power, RC and RL charge/discharge (exponentials), RLC circuit (resonance, Q factor, bandwidth), capacitive and inductive impedance, stored energy $E = \frac{1}{2}CV^2$ and $\frac{1}{2}LI^2$, Wheatstone bridge, Thévenin/Norton theorems, and maximum power transfer.

**Amplifiers** — Inverting amplifier gain ($-R_f/R_{in}$), non-inverting ($1 + R_f/R_{in}$), differential, summing. Op-amp integrator and differentiator. Gain-bandwidth product. Common-emitter gain, transconductance $g_m = I_C/V_T$, thermal voltage $V_T = k_B T/q \approx 26\,\text{mV}$ at 300 K. Conversion to dB (voltage and power), cascade gain, noise figure, and Friis formula for cascaded stages.

**Semiconductor Devices** — Shockley diode equation $I = I_s(e^{V/nV_T} - 1)$, Zener regulator, bipolar transistor ($I_C = \beta I_B$, $\alpha$ factor), MOSFET in saturation and linear regimes, body effect, solar cell (I-V model), LED (current-limiting resistor calculation), photodiode, tunnel diode, PN junction capacitance, Early effect, and DIBL (Drain-Induced Barrier Lowering).

**Digital Logic** — Fundamental gates (AND, OR, NOT, NAND, NOR, XOR, XNOR), half/full adder, ripple-carry adder, multiplexer, demultiplexer, decoder/encoder, SR/D/JK flip-flops, and binary↔Gray code conversion.

---

### 4. Fluid Mechanics

Fluid mechanics describes the behavior of liquids and gases in motion. SciForge organizes this domain across four files:

**Flow** — Reynolds number $Re = \rho v D / \mu$ (laminar/turbulent transition), Bernoulli equation $P + \frac{1}{2}\rho v^2 + \rho g h = \text{const}$, Poiseuille flow in a pipe (parabolic profile, flow rate $Q = \pi \Delta P r^4 / 8\mu L$), Mach number, drag coefficient, drag force $F_D = \frac{1}{2}\rho v^2 C_D A$, lift (coefficient $C_L$), Froude number, Weber number, mass flow rate, and terminal velocity.

**Boundary Layer** — Laminar boundary layer thickness on a flat plate (Blasius solution $\delta \propto \sqrt{x/Re_x}$), local and global friction coefficients, laminar-to-turbulent transition, turbulent boundary layer thickness ($\delta \propto x \cdot Re_x^{-1/5}$), displacement thickness, momentum thickness, Nusselt number ($Nu = 0.332\sqrt{Re}\cdot Pr^{1/3}$ for laminar plate), and Stanton number.

**Turbulence** — Turbulence modeling via Kolmogorov scales: length scale $\eta = (\nu^3/\epsilon)^{1/4}$, time scale $\tau = (\nu/\epsilon)^{1/2}$, velocity scale $v = (\nu\epsilon)^{1/4}$. Turbulent kinetic energy $k = \frac{1}{2}(u'^2 + v'^2 + w'^2)$. Taylor microscale, integral scale, friction velocity $u_\tau = \sqrt{\tau_w/\rho}$, law of the wall (viscous sublayer, buffer, logarithmic), mixing length, turbulent viscosity, turbulence intensity, and Kolmogorov energy spectrum $E(k) \propto \epsilon^{2/3} k^{-5/3}$.

**Fluid Waves** — Surface waves: dispersion relation $\omega^2 = gk\tanh(kh)$, phase and group velocity, significant wave height, period, power per unit wave front. Deep-water approximation ($c = \sqrt{g/k}$) and shallow-water approximation ($c = \sqrt{gh}$). Ursell number (nonlinearity), Stokes wave, and solitary wave.

---

### 5. Materials Science

This submodule deals with physical properties of condensed matter, at the intersection of physics and engineering.

**Crystallography** — Bragg's law $n\lambda = 2d\sin\theta$ for X-ray diffraction, interplanar spacings in cubic systems $d = a/\sqrt{h^2+k^2+l^2}$, planar density, packing fractions (BCC: $\pi\sqrt{3}/8 \approx 0.68$, FCC: $\pi\sqrt{2}/6 \approx 0.74$), structure factor (BCC/FCC extinction rules), lattice parameter from density, Debye temperature, Debye specific heat, and vacancy concentration $c_v = e^{-E_v/k_BT}$.

**Diffusion** — Fick's first law $J = -D \nabla c$ (steady state) and second law $\partial c/\partial t = D \nabla^2 c$ (transient, error-function solution). Arrhenius diffusion coefficient $D = D_0 e^{-Q/RT}$, diffusion length $\sqrt{2Dt}$, interdiffusion coefficient, Kirkendall effect (vacancy flux), grain-boundary diffusion, permeability, carburization depth, and mean free path.

**Phases** — Gibbs phase rule $F = C - P + 2$ (degrees of freedom), lever rule for phase diagrams, Clausius-Clapeyron slope $dP/dT = \Delta H / (T \Delta V)$, Gibbs free energy of mixing (regular solution), spinodal temperature, nucleation barrier $\Delta G^* = 16\pi\gamma^3/(3\Delta G_v^2)$, critical nucleus radius, nucleation rate, coarsening kinetics (LSW theory), JMAK transformation (Johnson-Mehl-Avrami-Kolmogorov $f = 1 - e^{-kt^n}$), partition coefficient, and Scheil equation for non-equilibrium solidification.

**Semiconductors** — Fermi energy, Fermi-Dirac distribution, intrinsic carrier concentration $n_i = \sqrt{N_c N_v} e^{-E_g/2k_BT}$, conductivity $\sigma = q(n\mu_e + p\mu_h)$, Hall coefficient, drift velocity, depletion width, built-in potential, diode current, bandgap variation with temperature (Varshni model), and dopant ionization.

---

### 6. Nucleosynthesis

Nucleosynthesis deals with the formation of atomic nuclei, primarily inside stars. This submodule is one of the richest in the Physics module.

**Nuclides** — Liquid-drop model (Bethe-Weizsäcker semi-empirical mass formula): $B = a_V A - a_S A^{2/3} - a_C Z(Z-1)/A^{1/3} - a_A(N-Z)^2/A \pm \delta$, with volume, surface, Coulomb, asymmetry, and pairing terms. Nuclear radius $R = r_0 A^{1/3}$, binding energy per nucleon, atomic mass, and nuclear stability.

**Decay** — Exponential decay law $N(t) = N_0 \cdot 2^{-t/t_{1/2}}$, decay constant $\lambda = \ln 2 / t_{1/2}$, activity in Bq and Ci, mean lifetime, decay chains (Bateman equations for radioactive series), secular and transient equilibrium, branching ratio, Geiger-Nuttall law for alpha decay, and Q-value calculation.

**Nuclear Reactions** — Reaction Q-value (mass balance), Coulomb barrier $E_C = 1.44 Z_1 Z_2 / (r_0(A_1^{1/3}+A_2^{1/3}))$ MeV, Gamow peak $E_0 = (E_G k_B^2 T^2/4)^{1/3}$ (most probable energy for thermonuclear reactions), Gamow window, reduced mass, astrophysical S-factor, Sommerfeld parameter, barrier penetration factor, thermonuclear rate (integrating over Maxwell-Boltzmann distribution × cross-section), estimates for pp, triple-alpha, and $^{12}$C($\alpha$,$\gamma$) rates, and reaction mean free path.

**Stellar Processes** — Complete classification of processes: pp chain (4 H → He, 26.7 MeV, $T > 4 \times 10^6$ K), CNO cycle (same product, catalyzed by C/N/O, $T > 15 \times 10^6$ K), triple-alpha (3 He → C, $T > 10^8$ K), carbon burning, neon burning, oxygen burning, silicon burning, and slow (s) and rapid (r) neutron-capture processes for synthesis of heavy elements beyond iron. Each process includes its threshold temperature and energy released.

**Stellar Physics** — `StellarCore` structure: luminosity ($L \propto M^{3.5}$), main-sequence lifetime ($\tau \propto M^{-2.5} \times 10^{10}$ years), dominant processes as a function of core temperature, temporal evolution (hydrogen consumption → helium → metals). Chandrasekhar limit (1.4 $M_\odot$), Tolman-Oppenheimer-Volkoff limit (~2.17 $M_\odot$), Eddington luminosity, maximum binding energy at the iron peak.

---

### 7. Optics

Optics describes the behavior of light, viewed as an electromagnetic wave in the visible range and beyond.

**Refraction** — Snell's law $n_1 \sin\theta_1 = n_2 \sin\theta_2$, critical angle $\theta_c = \arcsin(n_2/n_1)$ for total internal reflection, Brewster's angle $\theta_B = \arctan(n_2/n_1)$, Fresnel reflectance coefficients (s and p polarizations), thin-lens equation $1/f = 1/d_o + 1/d_i$, magnification, lensmaker's equation, numerical aperture, optical path length, Cauchy dispersion $n(\lambda) = A + B/\lambda^2$, and Abbe number.

**Diffraction** — Single-slit intensity (sinc²: $I = (\sin\beta/\beta)^2$), double-slit (interference term $\cos^2$), grating maxima ($d\sin\theta = m\lambda$), resolving power ($R = mN$), Rayleigh criterion $\theta = 1.22\lambda/D$, Airy disk radius, Fraunhofer distance, grating dispersion, Bragg condition, and first zero of a circular aperture.

**Interference** — Resultant intensity of two beams $I = I_1 + I_2 + 2\sqrt{I_1 I_2}\cos\delta$, thin-film phase shift, constructive condition, fringe spacing $\Delta y = \lambda L / d$, fringe visibility, coherence length and time, Fabry-Pérot transmittance (Airy function), finesse $\mathcal{F} = \pi\sqrt{R}/(1-R)$, free spectral range, Michelson path difference, and Newton's rings.

**Polarization** — Malus's law $I = I_0\cos^2\theta$, Stokes parameters $(S_0, S_1, S_2, S_3)$, degree of polarization, Jones matrix (rotation), quarter-wave plate, specific rotatory power, ellipticity, circular dichroism, birefringence $\Delta n = n_e - n_o$, and phase retardation $\Gamma = 2\pi\Delta n \cdot d / \lambda$.

---

### 8. Quantum Mechanics

The heart of modern physics. SciForge implements the mathematical formalisms using its own `Complex` type for complex arithmetic.

**Wave Functions** — Plane wave $\psi = e^{i(kx - \omega t)}$, Gaussian wave packet (localized superposition), normalization $\int|\psi|^2 dx = 1$, probability density, expectation values of position $\langle x \rangle$ and momentum $\langle p \rangle = -i\hbar \int \psi^* \frac{d\psi}{dx} dx$. Time evolution via the split-step (Fourier) method: alternation between real space (potential) and momentum space (kinetic energy).

**Operators** — Commutator $[A,B] = AB - BA$ and anticommutator $\{A,B\}$, expectation value $\langle A \rangle = \langle\psi|A|\psi\rangle$, variance $\sigma_A^2 = \langle A^2\rangle - \langle A\rangle^2$, uncertainty product $\sigma_A \sigma_B \geq \frac{1}{2}|\langle[A,B]\rangle|$ (Heisenberg's uncertainty principle), Hermiticity test, trace, and determinant.

**Angular Momentum** — Associated Legendre polynomials $P_l^m(\cos\theta)$, spherical harmonics $Y_l^m(\theta,\phi) = N_{lm} P_l^m(\cos\theta) e^{im\phi}$ (eigenfunctions of $L^2$ and $L_z$), Clebsch-Gordan coefficients (coupling of two angular momenta), and the Wigner formalism.

**Spin** — Pauli matrices $\sigma_x, \sigma_y, \sigma_z$ (2×2), up/down and $\pm x$, $\pm y$ spinors, Dirac gamma matrices (4×4) for relativistic theory, and the 2×2 identity.

**Perturbation Theory** — Stationary perturbation theory: first-order energy correction $E_n^{(1)} = \langle n|V|n\rangle$, second-order correction $E_n^{(2)} = \sum_{k\neq n} |V_{kn}|^2/(E_n-E_k)$, and first-order state correction. These corrections enable treatment of the Stark effect (electric field) and the Zeeman effect (magnetic field).

**Quantum Systems** — Hydrogen atom: energy levels $E_n = -13.6/n^2$ eV, radial functions $R_{10}$, $R_{20}$, $R_{21}$. Harmonic oscillator: $E_n = \hbar\omega(n+1/2)$, wave functions via Hermite polynomials. Infinite potential well: $E_n = n^2\pi^2\hbar^2/(2mL^2)$. Quantum tunneling $T = [1 + V_0^2\sinh^2(\kappa a)/(4E(V_0-E))]^{-1}$, Bohr radius $a_0 = \hbar^2/(me^2)$, Landau levels (magnetic field), and Zeeman effect.

**Quantum Information** — Von Neumann entropy $S = -\text{Tr}(\rho\ln\rho)$, purity $\text{Tr}(\rho^2)$, fidelity of pure and mixed states, concurrence of 2 qubits (entanglement measure), Bell states ($|\Phi^\pm\rangle$, $|\Psi^\pm\rangle$), and tensor product for composite systems.

---

### 9. Relativity

Einstein's special relativity transforms our understanding of space and time at velocities approaching $c$.

**Lorentz Transformations** — Lorentz factor $\gamma = 1/\sqrt{1-v^2/c^2}$, 4×4 boost matrix (in all three spatial directions), rapidity parameter $\phi = \text{arctanh}(v/c)$, and space-time interval invariant $s^2 = c^2\Delta t^2 - \Delta x^2 - \Delta y^2 - \Delta z^2$.

**Kinematics** — Time dilation $\Delta t' = \gamma \Delta t$, length contraction $L' = L/\gamma$, relativistic velocity addition $v = (v_1+v_2)/(1+v_1 v_2/c^2)$, proper time along a trajectory, and relativistic aberration $\cos\theta' = (\cos\theta - \beta)/(1-\beta\cos\theta)$.

**Dynamics** — Relativistic momentum $p = \gamma m v$, total energy $E = \gamma mc^2$, kinetic energy $K = (\gamma-1)mc^2$, energy-momentum relation $E^2 = (pc)^2 + (mc^2)^2$, four-momentum, invariant mass of two bodies, Mandelstam variables, Compton shift $\Delta\lambda = (h/m_e c)(1-\cos\theta)$, particle-production threshold energy, synchrotron radiation, and classical Bremsstrahlung.

---

### 10. Solid Mechanics

Solid mechanics studies the deformation and failure of materials under stress.

**Elasticity** — Hooke's law $\sigma = E\varepsilon$, Poisson's ratio $\nu$, shear modulus $G = E/(2(1+\nu))$, bulk modulus $K = E/(3(1-2\nu))$, first Lamé parameter, plane stress/strain, strain energy density $u = \frac{1}{2}\sigma\varepsilon$, thermal strain $\varepsilon = \alpha\Delta T$, volumetric strain, and hydrostatic stress.

**Stress Analysis** — 2D principal stresses (eigenvalues of the stress tensor), maximum shear stress, Mohr's circle (radius and center), stress tensor rotation, deviatoric tensor, second invariant $J_2$, beam bending $\sigma = My/I$, cantilever deflection $\delta = PL^3/3EI$, torsional shear $\tau = Tr/J$, Euler buckling $P_{cr} = \pi^2 EI/L^2$, and Hertzian contact pressure.

**Plasticity** — Offset yield strength, Ramberg-Osgood model (nonlinear stress-strain curve), true vs. engineering stress/strain, power-law hardening $\sigma = K\varepsilon_p^n$, Von Mises and Tresca yield criteria, isotropic hardening, Bauschinger effect, plastic work, necking criterion ($n$), and strain-rate sensitivity.

**Fracture** — Stress intensity factor $K_I = Y\sigma\sqrt{\pi a}$, Griffith critical stress $\sigma_c = \sqrt{2E\gamma/\pi a}$, energy release rate $G = K^2/E$, J-integral, crack-tip plastic zone (Irwin estimate), Paris law for fatigue crack growth $da/dN = C(\Delta K)^m$, fatigue life (Basquin for stress, Coffin-Manson for strain), Miner's cumulative damage rule, plane-strain fracture toughness, stress-corrosion cracking, and CTOD (Crack Tip Opening Displacement).

---

### 11. Thermodynamics

Thermodynamics studies energy transformations and the macroscopic properties of matter, complemented by statistical mechanics at the microscopic level.

**Equations of State** — Ideal gas law $PV = nRT$, Van der Waals $(P + a/V_m^2)(V_m - b) = RT$ (corrections for molecular interactions and volume), Redlich-Kwong, virial equation (density series), compressibility factor $Z = PV/(nRT)$. Internal energy $U = nC_vT$, enthalpy $H = nC_pT$, entropy $\Delta S = nC_v\ln(T_2/T_1) + nR\ln(V_2/V_1)$, Gibbs free energy $G = H - TS$ and Helmholtz free energy $F = U - TS$, chemical potential, Clausius-Clapeyron equation (vapor pressure), heat capacity ratio $\gamma = C_p/C_v$, speed of sound $c = \sqrt{\gamma RT/M}$, Maxwell speed distribution (three characteristic speeds: most probable, mean, and root-mean-square).

**Processes** — Carnot efficiency $\eta = 1 - T_c/T_h$, heating and cooling COP, isothermal/adiabatic/isobaric work, adiabatic relations ($TV^{\gamma-1} = \text{const}$, $PV^\gamma = \text{const}$), Otto efficiency, Diesel efficiency, Joule-Thomson coefficient, thermal conduction (Fourier's law $q = -kA\,dT/dx$), 1D thermal diffusion (explicit FTCS scheme), entropy of mixing, reaction free energy $\Delta G = \Delta G^0 + RT\ln Q$, equilibrium constant, and Van't Hoff equation.

**Statistical Mechanics** — Boltzmann distribution $P \propto e^{-E/k_BT}$, partition function $Z = \sum_i e^{-E_i/k_BT}$, harmonic oscillator partition function, canonical average energy and entropy. Quantum distributions: Fermi-Dirac $f = 1/(e^{(E-\mu)/k_BT}+1)$ (fermions) and Bose-Einstein $f = 1/(e^{(E-\mu)/k_BT}-1)$ (bosons). Blackbody radiation: Planck's law (in frequency and wavelength), Wien's displacement law $\lambda_{\max} T = 2898\,\mu\text{m·K}$, Stefan-Boltzmann law $P = \sigma A T^4$. Specific heat of solids: Debye model (numerical integration) and Einstein model. Sackur-Tetrode entropy for the ideal monoatomic gas.

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
