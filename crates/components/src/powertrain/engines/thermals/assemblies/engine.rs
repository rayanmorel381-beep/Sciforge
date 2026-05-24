use super::aspiration::Aspiration;
use super::bloc::Bloc;
use super::fuel::Fuel;
use super::super::parts;
use sciforge_core::materials::alus::cast::AC4B;
use sciforge_core::materials::alus::forged::AL_2618;
use sciforge_core::materials::irons::steels::STEEL_4340;
use sciforge_core::materials::nickels::inconel::INCONEL_718;
use sciforge_core::materials::titaniums::alloys::TI6AL4V_GR5;

#[derive(Debug, Clone)]
pub struct Engine {
    pub bloc: Bloc,
    pub fuel_kind: Fuel,
    pub cylinders: parts::cylinders::CylinderBank,
    pub headgasket: parts::headgaskets::HeadGasket,
    pub crankshaft: parts::crankshafts::Crankshaft,
    pub pistons: parts::pistons::PistonSet,
    pub camshaft: parts::camshafts::Camshaft,
    pub valves: parts::valves::ValveTrain,
    pub intake: parts::intake::IntakeSystem,
    pub turbo: Option<parts::turbos::Turbocharger>,
    pub supercharger: Option<parts::compressors::Supercharger>,
    pub fuel: parts::fuel_systems::FuelSystem,
    pub culasse: parts::culasses::Culasse,
    pub ignition: parts::ignition::Ignition,
    pub exhaust: parts::exhausts::ExhaustSystem,
    pub flywheel: parts::flywheels::Flywheel,
    pub estimated_power_kw: f64,
    pub bank_angle_deg: f64,
    pub peak_pressure_bar: f64,
    pub piston_force_n: f64,
    pub conrod_stress_pa: f64,
    pub conrod_strain: f64,
    pub crank_pin_stress_pa: f64,
    pub von_mises_stress_pa: f64,
    pub crankshaft_mass_kg: f64,
    pub eta_thermal: f64,
    pub bsfc_g_per_kwh: f64,
    pub cycles_to_failure: f64,
    pub fatigue_life_hours: f64,
    pub piston_crown_temp_k: f64,
    pub heat_flux_w_m2: f64,
    pub conrod_length_m: f64,
    pub crank_pin_d_m: f64,
    pub mean_piston_speed_m_s: f64,
    pub rpm_max: f64,
    pub knock_factor: f64,
    pub charge_temp_k: f64,
    pub charge_density_kg_m3: f64,
    pub egt_k: f64,
    pub cam_contact_pa: f64,
    pub valve_cycles_to_failure: f64,
    pub wiebe_completeness: f64,
    pub runner_tuning_factor: f64,
    pub runner_target_length_m: f64,
    pub runner_actual_length_m: f64,
    pub eta_compressor: f64,
    pub turbo_inertia_kg_m2: f64,
    pub turbo_target_rpm: f64,
    pub turbo_lag_s: f64,

    pub hours_run: f64,
    pub crank_cycles_consumed: f64,
    pub valve_cycles_consumed: f64,
    pub wear_factor: f64,
    pub oil_pressure_bar: f64,
    pub coolant_temp_k_runtime: f64,
    pub failures: Vec<FailureMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureMode {
    ThrownConrod,
    BlownHeadGasket,
    PistonHoled,
    ValveBurnt,
    CamLobeWorn,
    CrankshaftFatigueFracture,
    TurboSeized,
    OverheatSeizure,
    OilStarvation,
    DetonationDamage,
    CoolantLeak,
    OilLeak,
}

#[derive(Debug, Clone, Copy)]
pub struct OperatingPoint {
    pub rpm: f64,
    pub power_kw: f64,
    pub torque_nm: f64,
    pub bmep_bar: f64,
    pub fmep_bar: f64,
    pub eta_thermal: f64,
    pub bsfc_g_per_kwh: f64,
    pub t_peak_k: f64,
    pub egt_k: f64,
    pub knock_factor: f64,
    pub nox_ppm: f64,
    pub co_ppm: f64,
    pub hc_ppm: f64,
    pub soot_g_per_kwh: f64,
    pub fuel_flow_g_s: f64,
    pub air_flow_g_s: f64,
}

impl FailureMode {
    pub fn power_loss_fraction(self) -> f64 {
        match self {
            FailureMode::ThrownConrod
            | FailureMode::CrankshaftFatigueFracture
            | FailureMode::OverheatSeizure
            | FailureMode::OilStarvation
            | FailureMode::PistonHoled => 1.0,
            FailureMode::BlownHeadGasket => 0.50,
            FailureMode::ValveBurnt => 0.20,
            FailureMode::CamLobeWorn => 0.10,
            FailureMode::TurboSeized => 0.40,
            FailureMode::DetonationDamage => 0.15,
            FailureMode::CoolantLeak | FailureMode::OilLeak => 0.0,
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            FailureMode::ThrownConrod
                | FailureMode::CrankshaftFatigueFracture
                | FailureMode::OverheatSeizure
                | FailureMode::PistonHoled
                | FailureMode::OilStarvation
        )
    }
}

impl Engine {
    pub fn code(&self) -> String {
        format!("{}-{}", self.bloc.code(), self.culasse.code())
    }

    pub fn assemble(bloc: &Bloc) -> Engine {
        Self::assemble_with(bloc, Fuel::Gasoline, 0)
    }

    pub fn assemble_all_variants(bloc: &Bloc, fuel: Fuel) -> Vec<Engine> {
        (0..bloc.head_variants.len())
            .map(|i| Self::assemble_with(bloc, fuel, i))
            .collect()
    }

    pub fn assemble_with_fuel(bloc: &Bloc, fuel: Fuel) -> Engine {
        Self::assemble_with(bloc, fuel, 0)
    }

    pub fn assemble_with(bloc: &Bloc, fuel: Fuel, head_index: usize) -> Engine {
        let cyl = bloc.cylinders;
        let disp_l = bloc.displacement_l();
        let bore_m = bloc.bore_mm / 1000.0;
        let stroke_m = bloc.stroke_mm / 1000.0;
        let piston_area_m2 = std::f64::consts::PI * bore_m.powi(2) / 4.0;

        let topology = bloc.topology();
        let culasse = bloc
            .head_variants
            .get(head_index)
            .expect("head_index out of range")
            .clone();
        let headgasket = if bloc.aspiration.turbo_count() >= 2 || bloc.max_boost_bar >= 1.0 {
            parts::headgaskets::HeadGasket::copper(cyl, bloc.bore_mm)
        } else if matches!(bloc.aspiration, Aspiration::Turbo(_) | Aspiration::Supercharged | Aspiration::TwinCharged(_)) {
            parts::headgaskets::HeadGasket::mls(cyl, bloc.bore_mm)
        } else {
            parts::headgaskets::HeadGasket::composite(cyl, bloc.bore_mm)
        };
        let rpm_max = bloc.rpm_max();
        let mean_piston_speed_limit = bloc.mean_piston_speed_limit_m_s();
        let boost = bloc.boost(rpm_max);

        let gamma_air_fuel = fuel.gamma_air_fuel();
        let gamma_air = 1.40;
        let lhv_fuel_j_per_kg = fuel.lhv_j_per_kg();
        let afr_stoich = fuel.afr_stoich();
        let r_specific_air = 287.05;
        let cv_air = r_specific_air / (gamma_air - 1.0);
        let cr = bloc.compression_ratio;

        let otto_eff = 1.0 - cr.powf(1.0 - gamma_air_fuel);

        let charge_cooling_dt_k = match bloc.aspiration {
            Aspiration::Turbo(_) | Aspiration::TwinCharged(_) => fuel.charge_cooling_dt_k(),
            _ => 0.0,
        };
        let t_ivc_k = boost.t_charge_k - charge_cooling_dt_k;
        let p_ivc_pa = boost.charge_density_kg_m3 * r_specific_air * t_ivc_k;
        let lambda_rod_kin = bloc.conrod_ratio_lambda;

        let knock_factor = if fuel.knock_limited() {
            let octane_ron = fuel.cetane_or_octane();
            let lw_a_s = 17.68e-3;
            let lw_n = 1.7;
            let lw_ea_over_r_k = 3800.0 - (octane_ron - 91.0) * 60.0;
            let combustion_duration_lw = fuel.combustion_duration_deg();
            let theta_total_deg = 180.0 + combustion_duration_lw;
            let mut theta_deg = theta_total_deg;
            let mut lw_integral = 0.0_f64;
            let dt_per_deg = 1.0 / (6.0 * rpm_max);
            while theta_deg > 0.0 {
                let theta_rad = (theta_deg - combustion_duration_lw).max(0.0).to_radians();
                let v_ratio = 1.0
                    + 0.5 * (cr - 1.0)
                        * (1.0 - theta_rad.cos()
                            + (1.0 - (2.0 * theta_rad).cos()) / (4.0 / lambda_rod_kin));
                let mfb = if theta_deg > 180.0 {
                    0.0
                } else {
                    let frac = 1.0 - theta_deg / 180.0;
                    1.0 - (-5.0 * frac.powf(3.0)).exp()
                };
                let v_combustion_ratio = v_ratio / (1.0 + mfb * (cr - 1.0) * 0.1).max(1.0);
                let v_over_v_ivc = v_combustion_ratio / cr;
                let p_pa = p_ivc_pa * v_over_v_ivc.powf(-gamma_air);
                let t_k = t_ivc_k * v_over_v_ivc.powf(-(gamma_air - 1.0));
                let tau_s = lw_a_s
                    * (p_pa / 101325.0).powf(-lw_n)
                    * (lw_ea_over_r_k / t_k).exp();
                lw_integral += dt_per_deg / tau_s.max(1.0e-9);
                theta_deg -= 1.0;
            }
            if lw_integral >= 1.0 {
                (1.0 / lw_integral).max(0.80)
            } else {
                1.0
            }
        } else {
            1.0
        };

        let eta_combustion_chem = 0.97 - 0.005 * (cr - 9.0).max(0.0) / 4.0;
        let eta_heat_loss = 1.0 - (0.27 - 0.03 * (cr - 9.0) / 4.0).clamp(0.18, 0.32);

        let wiebe_a = 5.0;
        let wiebe_m = 2.0;
        let combustion_duration_deg = fuel.combustion_duration_deg();
        let mfb_at = |theta_local: f64| -> f64 {
            let frac = (theta_local / combustion_duration_deg).clamp(0.0, 1.0);
            1.0 - (-wiebe_a * frac.powf(wiebe_m + 1.0)).exp()
        };
        let wiebe_completeness = mfb_at(combustion_duration_deg);
        let wiebe_finite_combustion_loss = (1.0 - wiebe_completeness).powi(2);
        let eta_combustion = eta_combustion_chem * eta_heat_loss * (1.0 - 0.5 * wiebe_finite_combustion_loss);

        let p_intake_pa = boost.charge_density_kg_m3 * r_specific_air * boost.t_charge_k;
        let t_after_compression_k = boost.t_charge_k * cr.powf(gamma_air - 1.0);
        let p_after_compression_pa = p_intake_pa * cr.powf(gamma_air);

        let q_specific_added_j_per_kg_air =
            lhv_fuel_j_per_kg * eta_combustion * knock_factor * wiebe_completeness / afr_stoich;
        let delta_t_combustion_k = q_specific_added_j_per_kg_air / cv_air;
        let t_combustion_peak_k = t_after_compression_k + delta_t_combustion_k;
        let peak_pressure_pa =
            p_after_compression_pa * t_combustion_peak_k / t_after_compression_k.max(1.0);
        let peak_pressure_bar = peak_pressure_pa / 1.0e5;
        let piston_force_n = peak_pressure_pa * piston_area_m2;

        let t_exhaust_k = t_combustion_peak_k * cr.powf(1.0 - gamma_air);
        let egt_k = t_exhaust_k;
        let coolant_temp_k = 368.0;

        let bmep_ideal_pa = otto_eff
            * knock_factor
            * eta_combustion
            * boost.eta_volumetric
            * boost.charge_density_kg_m3
            * lhv_fuel_j_per_kg
            / afr_stoich;
        let imep_pa = bmep_ideal_pa;

        let n_krpm = rpm_max / 1000.0;
        let fmep_bar = 0.97 + 0.15 * n_krpm + 0.05 * n_krpm.powi(2);
        let fmep_pa = fmep_bar * 1.0e5;
        let bmep_pa = (imep_pa - fmep_pa).max(0.0);
        let eta_mech = if imep_pa > 1.0 { bmep_pa / imep_pa } else { 0.0 };
        let bmep_bar = bmep_pa / 1.0e5;
        let power_kw = bmep_bar * disp_l * rpm_max / 1200.0;

        let safety = bloc.design_safety_factor;
        let steel_mat = &STEEL_4340;
        let alu_forged_mat = &AL_2618;
        let alu_cast_mat = &AC4B;
        let titanium_mat = &TI6AL4V_GR5;
        let inconel_mat = &INCONEL_718;

        let crank_radius_m = stroke_m / 2.0;
        let conrod_length_m = crank_radius_m / bloc.conrod_ratio_lambda;
        let allowable_steel_pa = steel_mat.yield_strength_pa / safety;

        let conrod_area_required_m2 = piston_force_n / allowable_steel_pa;
        let conrod_aspect = 1.78;
        let conrod_shank_h_m = (conrod_area_required_m2 * conrod_aspect).sqrt();
        let conrod_shank_b_m = conrod_shank_h_m / conrod_aspect;
        let conrod_area_m2 = conrod_shank_b_m * conrod_shank_h_m;
        let conrod_strain = piston_force_n / (conrod_area_m2 * steel_mat.young_modulus_pa);
        let conrod_stress_pa = piston_force_n / conrod_area_m2;
        let conrod_i_m4 = conrod_shank_b_m * conrod_shank_h_m.powi(3) / 12.0;
        let conrod_buckling_load_n = std::f64::consts::PI.powi(2)
            * steel_mat.young_modulus_pa
            * conrod_i_m4
            / conrod_length_m.powi(2);
        let buckling_safety = conrod_buckling_load_n / piston_force_n.max(1.0);

        let crank_moment_nm = piston_force_n * crank_radius_m;
        let crank_pin_d_m = (32.0 * crank_moment_nm * safety
            / (std::f64::consts::PI * steel_mat.yield_strength_pa))
            .powf(1.0 / 3.0);
        let crank_pin_i_m4 = std::f64::consts::PI * crank_pin_d_m.powi(4) / 64.0;
        let crank_pin_y_m = crank_pin_d_m / 2.0;
        let crank_stress_pa = crank_moment_nm * crank_pin_y_m / crank_pin_i_m4;

        let tau_crank_pa = crank_stress_pa / 2.0;
        let von_mises_pa = (conrod_stress_pa.powi(2) + 3.0 * tau_crank_pa.powi(2)).sqrt();

        let valve_spring_force_n = 200.0 + (rpm_max / 1000.0).powi(2) * 30.0;
        let cam_lobe_radius_m = bore_m * 0.04;
        let follower_radius_m = bore_m * 0.08;
        let e_star_steel = steel_mat.young_modulus_pa / (2.0 * (1.0 - 0.3_f64.powi(2)));
        let cam_r_eff_m = (cam_lobe_radius_m * follower_radius_m)
            / (cam_lobe_radius_m + follower_radius_m);
        let cam_face_width_m = bore_m * 0.25;
        let cam_contact_pa = (valve_spring_force_n * e_star_steel
            / (std::f64::consts::PI * cam_r_eff_m.max(1.0e-6) * cam_face_width_m))
            .sqrt();

        let valve_temp_swing_k = (egt_k - coolant_temp_k).max(0.0);
        let alpha_inconel = inconel_mat.thermal_expansion_per_k;
        let valve_thermal_strain = alpha_inconel * valve_temp_swing_k;
        let valve_strain_amplitude: f64 = valve_thermal_strain / 2.0;
        let valve_stress_amplitude_pa = valve_strain_amplitude * inconel_mat.young_modulus_pa;
        let valve_cycles_to_failure = if valve_stress_amplitude_pa > 0.0
            && inconel_mat.fatigue_strength_exponent != 0.0
        {
            0.5 * (valve_stress_amplitude_pa / inconel_mat.fatigue_strength_coeff_pa)
                .powf(1.0 / inconel_mat.fatigue_strength_exponent)
        } else {
            f64::INFINITY
        };

        let eta_thermal = otto_eff * eta_combustion * eta_mech;
        let woschni_p_kpa = peak_pressure_pa / 1000.0;
        let woschni_t_k = t_combustion_peak_k;
        let woschni_w_m_s = 2.28 * mean_piston_speed_limit;
        let woschni_h_w_m2_k = 3.26
            * bore_m.powf(-0.2)
            * woschni_p_kpa.powf(0.8)
            * woschni_t_k.powf(-0.55)
            * woschni_w_m_s.powf(0.8);
        let h_coolant_w_m2_k = 4000.0;
        let piston_thickness_m = bore_m * 0.06;
        let alu_k = alu_forged_mat.thermal_conductivity_w_mk;
        let r_gas_m2k_w = 1.0 / woschni_h_w_m2_k.max(1.0);
        let r_wall_m2k_w = piston_thickness_m / alu_k;
        let r_coolant_m2k_w = 1.0 / h_coolant_w_m2_k;
        let r_total_m2k_w = r_gas_m2k_w + r_wall_m2k_w + r_coolant_m2k_w;
        let heat_flux_w_m2 = ((woschni_t_k - coolant_temp_k) / r_total_m2k_w).max(0.0);
        let t_wall_gas_side_k = woschni_t_k - heat_flux_w_m2 * r_gas_m2k_w;
        let piston_crown_temp_k = t_wall_gas_side_k - heat_flux_w_m2 * (r_wall_m2k_w * 0.5);

        let crankshaft_mass_kg = (std::f64::consts::PI * crank_pin_d_m.powi(2) / 4.0)
            * (stroke_m * 4.0 + crank_radius_m * cyl as f64 * 2.0)
            * steel_mat.density();

        let sigma_a_pa = von_mises_pa * 0.5;
        let cycles_to_failure = if sigma_a_pa > 0.0
            && steel_mat.fatigue_strength_exponent != 0.0
        {
            0.5 * (sigma_a_pa / steel_mat.fatigue_strength_coeff_pa)
                .powf(1.0 / steel_mat.fatigue_strength_exponent)
        } else {
            f64::INFINITY
        };
        let cycles_per_minute = rpm_max / 2.0;
        let fatigue_life_hours = cycles_to_failure / (cycles_per_minute * 60.0);

        let lhv_mj_per_kg = lhv_fuel_j_per_kg / 1.0e6;
        let bsfc_g_per_kwh = 3600.0 / (eta_thermal * lhv_mj_per_kg);
        let fuel_g_min = power_kw * bsfc_g_per_kwh / 60.0;
        let fuel_density_g_cc: f64 = fuel.density_g_per_cc();
        let injector_count = if bloc.aspiration.turbo_count() >= 2 { cyl * 2 } else { cyl };
        let duty: f64 = 0.85;
        let injector_flow_cc_min = (fuel_g_min / fuel_density_g_cc) / (injector_count as f64 * duty);

        let allowable_cast_alu = alu_cast_mat.yield_strength_pa / safety;
        let allowable_forged_alu = alu_forged_mat.yield_strength_pa / safety;
        let allowable_titanium = titanium_mat.yield_strength_pa / safety;

        let selected = bloc.select_parts(
            power_kw,
            von_mises_pa,
            piston_crown_temp_k,
            buckling_safety,
            injector_flow_cc_min,
            allowable_cast_alu,
            allowable_forged_alu,
            allowable_titanium,
            alu_cast_mat.max_service_temp_k,
            alu_forged_mat.max_service_temp_k,
        );

        let (turbo_inertia_kg_m2, turbo_target_rpm, turbo_lag_s) = if let Some(turbo) = &selected.turbo {
            let cp_air_local = gamma_air * r_specific_air / (gamma_air - 1.0);

            let mw_fuel_g = fuel.molecular_weight_g_per_mol();
            let n_c = fuel.carbon_atoms();
            let n_h = fuel.hydrogen_atoms();
            let m_co2_per_kg_fuel = n_c * 44.01 / mw_fuel_g;
            let m_h2o_per_kg_fuel = (n_h / 2.0) * 18.02 / mw_fuel_g;
            let m_n2_per_kg_fuel = afr_stoich * 0.767;
            let m_total_per_kg_fuel = 1.0 + afr_stoich;
            let y_co2 = m_co2_per_kg_fuel / m_total_per_kg_fuel;
            let y_h2o = m_h2o_per_kg_fuel / m_total_per_kg_fuel;
            let y_n2 = m_n2_per_kg_fuel / m_total_per_kg_fuel;
            let mw_co2 = 44.01e-3;
            let mw_h2o = 18.02e-3;
            let mw_n2 = 28.01e-3;
            let inv_mw_mix = y_co2 / mw_co2 + y_h2o / mw_h2o + y_n2 / mw_n2;
            let mw_mix = 1.0 / inv_mw_mix;
            let r_universal = 8.314_462_6_f64;
            let r_specific_exh = r_universal / mw_mix;
            let cp_co2_at_egt = 0.85e3 + 0.30e-3 * egt_k;
            let cp_h2o_at_egt = 1.85e3 + 0.50e-3 * egt_k;
            let cp_n2_at_egt = 1.04e3 + 0.10e-3 * egt_k;
            let cp_exh = y_co2 * cp_co2_at_egt + y_h2o * cp_h2o_at_egt + y_n2 * cp_n2_at_egt;
            let gamma_exh = cp_exh / (cp_exh - r_specific_exh);

            let d_comp_m = turbo.compressor_inducer_mm / 1000.0;
            let d_turb_m = turbo.turbine_exducer_mm / 1000.0;
            let r_comp_m = d_comp_m / 2.0;
            let r_turb_m = d_turb_m / 2.0;

            let comp_thickness_m = r_comp_m * 0.30;
            let turb_thickness_m = r_turb_m * 0.30;
            let comp_mass_kg = std::f64::consts::PI * r_comp_m.powi(2) * comp_thickness_m
                * turbo.compressor_wheel_material().density();
            let turb_mass_kg = std::f64::consts::PI * r_turb_m.powi(2) * turb_thickness_m
                * turbo.turbine_wheel_material().density();
            let j_comp = 0.5 * comp_mass_kg * r_comp_m.powi(2);
            let j_turb = 0.5 * turb_mass_kg * r_turb_m.powi(2);
            let j_total = j_comp + j_turb;

            let pr_comp = boost.pressure_ratio.max(1.0);
            let slip_factor = turbo.slip_factor();
            let euler_work_j_per_kg =
                cp_air_local * boost.t_charge_k * (pr_comp.powf((gamma_air - 1.0) / gamma_air) - 1.0)
                    / boost.eta_compressor.max(0.5);
            let u_tip_comp_m_s = (euler_work_j_per_kg / slip_factor).sqrt();
            let omega_target_rad_s = u_tip_comp_m_s / r_comp_m.max(1.0e-3);
            let target_rpm = omega_target_rad_s * 60.0 / (2.0 * std::f64::consts::PI);

            let v_disp_m3 = disp_l / 1000.0;
            let m_dot_air_kg_s = boost.charge_density_kg_m3 * v_disp_m3 * (rpm_max / 120.0)
                * boost.eta_volumetric;
            let m_dot_exh_kg_s = m_dot_air_kg_s * (1.0 + 1.0 / afr_stoich);
            let pr_turb = turbo.design_expansion_ratio();
            let eta_turb = turbo.isentropic_turbine_efficiency();
            let p_turbine_w = m_dot_exh_kg_s
                * cp_exh
                * egt_k
                * (1.0 - pr_turb.powf(-(gamma_exh - 1.0) / gamma_exh))
                * eta_turb;

            let ke_target_j = 0.5 * j_total * omega_target_rad_s.powi(2);
            let lag_s = 2.3 * ke_target_j / p_turbine_w.max(1.0);
            (j_total, target_rpm, lag_s)
        } else {
            (0.0, 0.0, 0.0)
        };

        Engine {
            bloc: bloc.clone(),
            fuel_kind: fuel,
            cylinders: topology.cylinders,
            headgasket,
            crankshaft: topology.crankshaft,
            pistons: selected.pistons,
            camshaft: topology.camshaft,
            valves: topology.valves,
            intake: selected.intake,
            turbo: selected.turbo,
            supercharger: selected.supercharger,
            fuel: selected.fuel,
            culasse: culasse.clone(),
            ignition: match fuel {
                Fuel::Gasoline | Fuel::GasolineE85 | Fuel::Ethanol | Fuel::Methanol => {
                    parts::ignition::Ignition::for_gasoline(bloc.cylinders, culasse.plugs_per_cylinder)
                }
                Fuel::Diesel
                | Fuel::BiodieselB20
                | Fuel::BiodieselB100
                | Fuel::Kerosene
                | Fuel::JetB => {
                    parts::ignition::Ignition::for_diesel(bloc.cylinders, culasse.plugs_per_cylinder)
                }
            },
            exhaust: selected.exhaust,
            flywheel: selected.flywheel,
            estimated_power_kw: power_kw,
            bank_angle_deg: topology.bank_angle_deg,
            peak_pressure_bar,
            piston_force_n,
            conrod_stress_pa,
            conrod_strain,
            crank_pin_stress_pa: crank_stress_pa,
            von_mises_stress_pa: von_mises_pa,
            crankshaft_mass_kg,
            eta_thermal,
            bsfc_g_per_kwh,
            cycles_to_failure,
            fatigue_life_hours,
            piston_crown_temp_k,
            heat_flux_w_m2,
            conrod_length_m,
            crank_pin_d_m,
            mean_piston_speed_m_s: mean_piston_speed_limit,
            rpm_max,
            knock_factor,
            charge_temp_k: boost.t_charge_k,
            charge_density_kg_m3: boost.charge_density_kg_m3,
            egt_k,
            cam_contact_pa,
            valve_cycles_to_failure,
            wiebe_completeness,
            runner_tuning_factor: boost.runner_tuning_factor,
            runner_target_length_m: boost.runner_target_length_m,
            runner_actual_length_m: boost.runner_actual_length_m,
            eta_compressor: boost.eta_compressor,
            turbo_inertia_kg_m2,
            turbo_target_rpm,
            turbo_lag_s,

            hours_run: 0.0,
            crank_cycles_consumed: 0.0,
            valve_cycles_consumed: 0.0,
            wear_factor: 0.0,
            oil_pressure_bar: 4.0,
            coolant_temp_k_runtime: 368.0,
            failures: Vec::new(),
        }
    }

    pub fn tick(
        &mut self,
        rpm: f64,
        load_fraction: f64,
        ambient_k: f64,
        oil_pressure_bar: f64,
        coolant_temp_k: f64,
        dt_s: f64,
    ) {
        if self.failures.iter().any(|f| f.is_terminal()) {
            return;
        }
        let dt_h = dt_s / 3600.0;
        self.hours_run += dt_h;
        self.oil_pressure_bar = oil_pressure_bar;
        self.coolant_temp_k_runtime = coolant_temp_k;

        let cycles_this_tick = (rpm / 2.0) * (dt_s / 60.0);
        let load_stress_factor = load_fraction.clamp(0.0, 1.5).powi(3);
        self.crank_cycles_consumed += cycles_this_tick * (1.0 + load_stress_factor);
        self.valve_cycles_consumed += cycles_this_tick * 2.0;

        let crank_usage = self.crank_cycles_consumed / self.cycles_to_failure.max(1.0);
        let valve_usage = self.valve_cycles_consumed / self.valve_cycles_to_failure.max(1.0);
        let overrev = (rpm / self.rpm_max - 1.0).max(0.0);
        let overheat_coolant = (coolant_temp_k - 378.0).max(0.0) / 30.0;
        let oil_starve = (3.0 - oil_pressure_bar).max(0.0) / 3.0;
        let detonation = (1.0 - self.knock_factor).max(0.0);
        let turbo_thermal = if self.turbo.is_some() {
            (self.egt_k - 1255.0).max(0.0) / 200.0
        } else {
            0.0
        };

        let ambient_stress = ((ambient_k - 293.0) / 30.0).max(0.0);
        self.wear_factor = (crank_usage + valve_usage
            + overrev * 5.0 * dt_h
            + overheat_coolant * 2.0 * dt_h
            + oil_starve * 10.0 * dt_h
            + detonation * 3.0 * dt_h
            + turbo_thermal * 4.0 * dt_h
            + ambient_stress * 0.5 * dt_h)
            .min(2.0);

        if oil_starve > 0.5 && oil_pressure_bar < 1.0 {
            self.failures.push(FailureMode::OilStarvation);
            return;
        }
        if coolant_temp_k > 393.0 + 20.0 * load_fraction {
            self.failures.push(FailureMode::OverheatSeizure);
            return;
        }
        if overrev > 0.20 && load_fraction > 0.5 {
            self.failures.push(FailureMode::ThrownConrod);
            return;
        }
        if self.crank_cycles_consumed > self.cycles_to_failure {
            self.failures.push(FailureMode::CrankshaftFatigueFracture);
            return;
        }
        if self.valve_cycles_consumed > self.valve_cycles_to_failure
            && !self.failures.contains(&FailureMode::ValveBurnt)
        {
            self.failures.push(FailureMode::ValveBurnt);
        }
        if turbo_thermal > 0.6 && self.turbo.is_some()
            && !self.failures.contains(&FailureMode::TurboSeized)
        {
            self.failures.push(FailureMode::TurboSeized);
        }
        if detonation > 0.15 && self.peak_pressure_bar > 120.0
            && !self.failures.contains(&FailureMode::DetonationDamage)
        {
            self.failures.push(FailureMode::DetonationDamage);
        }
        if self.peak_pressure_bar > 150.0
            && !self.failures.contains(&FailureMode::BlownHeadGasket)
            && self.wear_factor > 0.7
        {
            self.failures.push(FailureMode::BlownHeadGasket);
        }
        if self.piston_crown_temp_k > 600.0 && load_fraction > 0.9
            && !self.failures.contains(&FailureMode::PistonHoled)
        {
            self.failures.push(FailureMode::PistonHoled);
            return;
        }
        if self.cam_contact_pa > 2.5e9 && self.wear_factor > 0.5
            && !self.failures.contains(&FailureMode::CamLobeWorn)
        {
            self.failures.push(FailureMode::CamLobeWorn);
        }
        if self.wear_factor > 0.8 && !self.failures.contains(&FailureMode::CoolantLeak)
            && coolant_temp_k > 380.0
        {
            self.failures.push(FailureMode::CoolantLeak);
        }
        if self.wear_factor > 0.9 && !self.failures.contains(&FailureMode::OilLeak) {
            self.failures.push(FailureMode::OilLeak);
        }
    }

    pub fn current_power_kw(&self) -> f64 {
        if self.failures.iter().any(|f| f.is_terminal()) {
            return 0.0;
        }
        let total_loss: f64 = self
            .failures
            .iter()
            .map(|f| f.power_loss_fraction())
            .sum::<f64>()
            .min(1.0);
        self.estimated_power_kw * (1.0 - total_loss)
    }

    pub fn is_running(&self) -> bool {
        !self.failures.iter().any(|f| f.is_terminal())
    }

    pub fn operating_point(&self, rpm: f64) -> OperatingPoint {
        let bloc = &self.bloc;
        let fuel = self.fuel_kind;
        let disp_l = bloc.displacement_l();
        let boost = bloc.boost(rpm);

        let gamma_air = 1.40;
        let gamma_air_fuel = fuel.gamma_air_fuel();
        let r_specific_air = 287.05;
        let cv_air = r_specific_air / (gamma_air - 1.0);
        let cr = bloc.compression_ratio;
        let lhv = fuel.lhv_j_per_kg();
        let afr = fuel.afr_stoich();

        let charge_cooling_dt_k = match bloc.aspiration {
            Aspiration::Turbo(_) | Aspiration::TwinCharged(_) => fuel.charge_cooling_dt_k(),
            _ => 0.0,
        };
        let t_ivc_k = boost.t_charge_k - charge_cooling_dt_k;
        let p_ivc_pa = boost.charge_density_kg_m3 * r_specific_air * t_ivc_k;
        let lambda_rod_kin = bloc.conrod_ratio_lambda;

        let knock_factor = if fuel.knock_limited() {
            let octane_ron = fuel.cetane_or_octane();
            let lw_a_s = 17.68e-3;
            let lw_n = 1.7;
            let lw_ea_over_r_k = 3800.0 - (octane_ron - 91.0) * 60.0;
            let combustion_dur = fuel.combustion_duration_deg();
            let theta_total = 180.0 + combustion_dur;
            let dt_per_deg = 1.0 / (6.0 * rpm.max(100.0));
            let mut theta_deg = theta_total;
            let mut lw_integral = 0.0_f64;
            while theta_deg > 0.0 {
                let theta_rad = (theta_deg - combustion_dur).max(0.0).to_radians();
                let v_ratio = 1.0
                    + 0.5 * (cr - 1.0)
                        * (1.0 - theta_rad.cos()
                            + (1.0 - (2.0 * theta_rad).cos()) / (4.0 / lambda_rod_kin));
                let mfb = if theta_deg > 180.0 {
                    0.0
                } else {
                    let frac = 1.0 - theta_deg / 180.0;
                    1.0 - (-5.0 * frac.powf(3.0)).exp()
                };
                let v_combustion_ratio = v_ratio / (1.0 + mfb * (cr - 1.0) * 0.1).max(1.0);
                let v_over_v_ivc = v_combustion_ratio / cr;
                let p_pa = p_ivc_pa * v_over_v_ivc.powf(-gamma_air);
                let t_k = t_ivc_k * v_over_v_ivc.powf(-(gamma_air - 1.0));
                let tau_s = lw_a_s
                    * (p_pa / 101325.0).powf(-lw_n)
                    * (lw_ea_over_r_k / t_k).exp();
                lw_integral += dt_per_deg / tau_s.max(1.0e-9);
                theta_deg -= 1.0;
            }
            if lw_integral >= 1.0 {
                (1.0 / lw_integral).max(0.80)
            } else {
                1.0
            }
        } else {
            1.0
        };

        let wiebe = self.wiebe_completeness;
        let eta_combustion_chem = 0.97 - 0.005 * (cr - 9.0).max(0.0) / 4.0;
        let eta_heat_loss = 1.0 - (0.27 - 0.03 * (cr - 9.0) / 4.0).clamp(0.18, 0.32);
        let eta_combustion = eta_combustion_chem * eta_heat_loss * (1.0 - 0.5 * (1.0 - wiebe).powi(2));
        let otto_eff = 1.0 - cr.powf(1.0 - gamma_air_fuel);

        let t_after_compression_k = boost.t_charge_k * cr.powf(gamma_air - 1.0);
        let q = lhv * eta_combustion * knock_factor * wiebe / afr;
        let t_peak_k = t_after_compression_k + q / cv_air;
        let egt_k = t_peak_k * cr.powf(1.0 - gamma_air);

        let bmep_ideal_pa = otto_eff
            * knock_factor
            * eta_combustion
            * boost.eta_volumetric
            * boost.charge_density_kg_m3
            * lhv
            / afr;
        let n_krpm = rpm / 1000.0;
        let fmep_pa = (0.97 + 0.15 * n_krpm + 0.05 * n_krpm.powi(2)) * 1.0e5;
        let bmep_pa = (bmep_ideal_pa - fmep_pa).max(0.0);
        let bmep_bar = bmep_pa / 1.0e5;
        let fmep_bar = fmep_pa / 1.0e5;
        let power_kw = bmep_bar * disp_l * rpm / 1200.0;
        let omega = rpm * 2.0 * std::f64::consts::PI / 60.0;
        let torque_nm = if omega > 1.0 { power_kw * 1000.0 / omega } else { 0.0 };
        let eta_mech = if bmep_ideal_pa > 1.0 { bmep_pa / bmep_ideal_pa } else { 0.0 };
        let eta_thermal = otto_eff * eta_combustion * eta_mech;
        let bsfc_g_per_kwh = 3600.0 / (eta_thermal.max(1.0e-6) * (lhv / 1.0e6));

        let v_disp_m3 = disp_l / 1000.0;
        let air_flow_kg_s = boost.charge_density_kg_m3 * v_disp_m3 * (rpm / 120.0)
            * boost.eta_volumetric;
        let fuel_flow_kg_s = air_flow_kg_s / afr;

        let nox_ppm = (((t_peak_k - 1800.0) / 200.0).exp() * 50.0).min(5000.0);
        let co_ppm = 8000.0 * (1.0 - eta_combustion * wiebe).max(0.05);
        let surface_to_volume = 6.0 / (bloc.bore_mm / 1000.0).max(0.05);
        let hc_ppm = 500.0
            + 3000.0 * (1.0 - wiebe)
            + 200.0 * surface_to_volume / 100.0
            + 400.0 * (1.0 - knock_factor);
        let soot_g_per_kwh = match bloc.aspiration {
            Aspiration::Na | Aspiration::Supercharged => fuel.soot_baseline_g_per_kwh_na(),
            Aspiration::Turbo(_) | Aspiration::TwinCharged(_) => fuel.soot_baseline_g_per_kwh_boosted(),
        } * (1.0 + (1.0 - wiebe) * 4.0);

        OperatingPoint {
            rpm,
            power_kw,
            torque_nm,
            bmep_bar,
            fmep_bar,
            eta_thermal,
            bsfc_g_per_kwh,
            t_peak_k,
            egt_k,
            knock_factor,
            nox_ppm,
            co_ppm,
            hc_ppm,
            soot_g_per_kwh,
            fuel_flow_g_s: fuel_flow_kg_s * 1000.0,
            air_flow_g_s: air_flow_kg_s * 1000.0,
        }
    }

    pub fn power_curve(&self, rpm_min: f64, rpm_max: f64, samples: usize) -> Vec<OperatingPoint> {
        if samples < 2 {
            return vec![self.operating_point(rpm_min.max(rpm_max))];
        }
        let step = (rpm_max - rpm_min) / (samples - 1) as f64;
        (0..samples)
            .map(|i| self.operating_point(rpm_min + step * i as f64))
            .collect()
    }

    pub fn peak_power_point(&self, rpm_min: f64, rpm_max: f64, samples: usize) -> OperatingPoint {
        let curve = self.power_curve(rpm_min, rpm_max, samples);
        curve
            .into_iter()
            .fold(self.operating_point(rpm_max), |acc, op| {
                if op.power_kw > acc.power_kw { op } else { acc }
            })
    }

    pub fn peak_torque_point(&self, rpm_min: f64, rpm_max: f64, samples: usize) -> OperatingPoint {
        let curve = self.power_curve(rpm_min, rpm_max, samples);
        curve
            .into_iter()
            .fold(self.operating_point(rpm_min), |acc, op| {
                if op.torque_nm > acc.torque_nm { op } else { acc }
            })
    }
}
