use sciforge_hub::prelude::*;

#[derive(Clone, Copy)]
struct StarScenario {
    name: &'static str,
    initial_mass_solar: f64,
    metallicity_z: f64,
    lithium_mass_fraction: f64,
    rotation_fraction_crit: f64,
    mass_loss_fraction: f64,
    fallback_fraction: f64,
}

#[derive(Clone, Copy)]
struct StarOutcome {
    final_mass_solar: f64,
    helium_core_solar: f64,
    collapse_core_solar: f64,
    bh_forms: bool,
    dimensionless_spin: f64,
    schwarzschild_radius_m: f64,
    lithium_swallowed_solar: f64,
    lithium_escape_solar: f64,
    lithium_destruction_factor: f64,
}

fn salpeter_weight(mass_solar: f64) -> f64 {
    mass_solar.powf(-2.35)
}

fn clamp01(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

fn gev_cm3_to_kg_m3(gev_cm3: f64) -> f64 {
    gev_cm3 * 1.782_661_92e-27 * 1.0e6
}

fn bondi_hoyle_rate(
    mass_kg: f64,
    rho_kg_m3: f64,
    rel_velocity_m_s: f64,
    sound_speed_m_s: f64,
) -> f64 {
    let v2 = rel_velocity_m_s * rel_velocity_m_s + sound_speed_m_s * sound_speed_m_s;
    4.0 * std::f64::consts::PI * constants::G * constants::G * mass_kg * mass_kg * rho_kg_m3
        / v2.powf(1.5)
}

fn bondi_radius(mass_kg: f64, rel_velocity_m_s: f64, sound_speed_m_s: f64) -> f64 {
    2.0 * constants::G * mass_kg
        / (rel_velocity_m_s * rel_velocity_m_s + sound_speed_m_s * sound_speed_m_s)
}

fn simulate_star(s: StarScenario) -> StarOutcome {
    let final_mass_solar = s.initial_mass_solar * (1.0 - s.mass_loss_fraction);

    let mut helium_core_fraction = 0.18 + 0.0035 * s.initial_mass_solar;
    helium_core_fraction += 0.10 * s.rotation_fraction_crit;
    helium_core_fraction += 0.06 * (s.metallicity_z / 0.02);
    helium_core_fraction = helium_core_fraction.clamp(0.15, 0.80);

    let helium_core_solar = final_mass_solar * helium_core_fraction;
    let collapse_core_solar = helium_core_solar * (0.50 + 0.50 * s.fallback_fraction);

    let dimensionless_spin =
        clamp01(0.10 + 0.90 * s.rotation_fraction_crit * (1.0 - 0.25 * s.mass_loss_fraction));
    let bh_forms = collapse_core_solar >= 2.5;

    let bh_mass_kg = if bh_forms {
        collapse_core_solar * constants::SOLAR_MASS
    } else {
        0.0
    };
    let schwarzschild_radius_m = if bh_forms {
        astronomy::stellar::schwarzschild_radius(bh_mass_kg)
    } else {
        0.0
    };

    let lithium_initial_solar = s.initial_mass_solar * s.lithium_mass_fraction;
    let lithium_destroyed_interior =
        lithium_initial_solar * (0.92 + 0.06 * s.rotation_fraction_crit).clamp(0.0, 0.995);
    let lithium_surviving_pre_sn = (lithium_initial_solar - lithium_destroyed_interior).max(0.0);

    let lithium_ejected_solar = lithium_surviving_pre_sn * (1.0 - s.fallback_fraction);
    let lithium_swallowed_solar = lithium_surviving_pre_sn * s.fallback_fraction;
    let lithium_escape_solar = lithium_ejected_solar;

    let lithium_destruction_factor = if lithium_initial_solar > 0.0 {
        1.0 - lithium_escape_solar / lithium_initial_solar
    } else {
        1.0
    };

    StarOutcome {
        final_mass_solar,
        helium_core_solar,
        collapse_core_solar,
        bh_forms,
        dimensionless_spin,
        schwarzschild_radius_m,
        lithium_swallowed_solar,
        lithium_escape_solar,
        lithium_destruction_factor,
    }
}

fn main() {
    println!("====================================================================");
    println!(" CHRONOS - Etoiles supermassives riches en lithium -> trou noir ?");
    println!("====================================================================\n");

    let scenarios = [
        StarScenario {
            name: "A1 - 35 Msun, Z bas, rotation moderee",
            initial_mass_solar: 35.0,
            metallicity_z: 0.004,
            lithium_mass_fraction: 1.5e-8,
            rotation_fraction_crit: 0.35,
            mass_loss_fraction: 0.28,
            fallback_fraction: 0.45,
        },
        StarScenario {
            name: "A2 - 60 Msun, Z moyen, rotation rapide",
            initial_mass_solar: 60.0,
            metallicity_z: 0.010,
            lithium_mass_fraction: 3.0e-8,
            rotation_fraction_crit: 0.70,
            mass_loss_fraction: 0.40,
            fallback_fraction: 0.62,
        },
        StarScenario {
            name: "A3 - 85 Msun, Z solaire, vents forts",
            initial_mass_solar: 85.0,
            metallicity_z: 0.020,
            lithium_mass_fraction: 5.0e-8,
            rotation_fraction_crit: 0.60,
            mass_loss_fraction: 0.58,
            fallback_fraction: 0.55,
        },
        StarScenario {
            name: "A4 - 120 Msun, Z bas, rotation extreme",
            initial_mass_solar: 120.0,
            metallicity_z: 0.003,
            lithium_mass_fraction: 8.0e-8,
            rotation_fraction_crit: 0.92,
            mass_loss_fraction: 0.33,
            fallback_fraction: 0.78,
        },
        StarScenario {
            name: "A5 - 150 Msun, Z tres bas, hyper-fallback",
            initial_mass_solar: 150.0,
            metallicity_z: 0.001,
            lithium_mass_fraction: 1.0e-7,
            rotation_fraction_crit: 0.80,
            mass_loss_fraction: 0.30,
            fallback_fraction: 0.88,
        },
        StarScenario {
            name: "A6 - 220 Msun, Z ultra bas, quasi collapsar",
            initial_mass_solar: 220.0,
            metallicity_z: 0.0005,
            lithium_mass_fraction: 2.0e-7,
            rotation_fraction_crit: 0.95,
            mass_loss_fraction: 0.22,
            fallback_fraction: 0.93,
        },
    ];

    println!("Parametres suivis pour creation de trou noir :");
    println!("  - masse finale et masse coeur en effondrement");
    println!("  - spin adimensionne a*");
    println!("  - rayon de Schwarzschild final");
    println!("  - retention/echappement du lithium\n");

    println!(
        "{:38} | {:>6} | {:>6} | {:>6} | {:>4} | {:>8} | {:>10} | {:>8}",
        "Scenario", "Mfin", "Mcoeur", "Mcol", "BH?", "a*", "Rs(km)", "Li det%"
    );
    println!(
        "{:-<38}-+-{:-<6}-+-{:-<6}-+-{:-<6}-+-{:-<4}-+-{:-<8}-+-{:-<10}-+-{:-<8}",
        "", "", "", "", "", "", "", ""
    );

    let mut weighted_total = 0.0;
    let mut weighted_bh = 0.0;
    let mut weighted_li_depleted = 0.0;
    let mut weighted_li_escape = 0.0;
    let mut weighted_li_swallowed = 0.0;
    let mut weighted_match_h4 = 0.0;
    let mut weighted_bh_mass_solar = 0.0;

    let required_lithium_depletion_h4 = 0.7189;

    for s in scenarios {
        let out = simulate_star(s);
        let weight = salpeter_weight(s.initial_mass_solar);
        weighted_total += weight;

        if out.bh_forms {
            weighted_bh += weight;
            weighted_bh_mass_solar += weight * out.collapse_core_solar;
        }

        let match_h4 =
            (out.lithium_destruction_factor - required_lithium_depletion_h4).abs() <= 0.08;
        if match_h4 {
            weighted_match_h4 += weight;
        }

        weighted_li_depleted += weight * out.lithium_destruction_factor;
        weighted_li_escape += weight * out.lithium_escape_solar;
        weighted_li_swallowed += weight * out.lithium_swallowed_solar;

        let rs_km = out.schwarzschild_radius_m / 1000.0;
        println!(
            "{:38} | {:6.1} | {:6.1} | {:6.1} | {:>4} | {:8.3} | {:10.1} | {:7.1}%",
            s.name,
            out.final_mass_solar,
            out.helium_core_solar,
            out.collapse_core_solar,
            if out.bh_forms { "oui" } else { "non" },
            out.dimensionless_spin,
            rs_km,
            out.lithium_destruction_factor * 100.0
        );
    }

    let p_bh = if weighted_total > 0.0 {
        weighted_bh / weighted_total
    } else {
        0.0
    };
    let p_match_h4 = if weighted_total > 0.0 {
        weighted_match_h4 / weighted_total
    } else {
        0.0
    };
    let mean_li_depletion = if weighted_total > 0.0 {
        weighted_li_depleted / weighted_total
    } else {
        0.0
    };
    let mean_li_escape = if weighted_total > 0.0 {
        weighted_li_escape / weighted_total
    } else {
        0.0
    };
    let mean_li_swallowed = if weighted_total > 0.0 {
        weighted_li_swallowed / weighted_total
    } else {
        0.0
    };
    let mean_bh_mass_solar = if weighted_bh > 0.0 {
        weighted_bh_mass_solar / weighted_bh
    } else {
        0.0
    };

    println!("\n-- Probabilites generales (dans cet ensemble + pond. IMF Salpeter) --\n");
    println!(
        "Probabilite de former un trou noir              : {:6.2}%",
        p_bh * 100.0
    );
    println!(
        "Probabilite d'etre proche du facteur H4 requis  : {:6.2}%",
        p_match_h4 * 100.0
    );
    println!(
        "Depletion lithium moyenne                       : {:6.2}%",
        mean_li_depletion * 100.0
    );
    println!(
        "Lithium moyen ejecte (Msun eq)                  : {:.3e}",
        mean_li_escape
    );
    println!(
        "Lithium moyen englouti par BH (Msun eq)         : {:.3e}",
        mean_li_swallowed
    );
    println!(
        "Masse BH typique (ponderee IMF)                 : {:.2} Msun",
        mean_bh_mass_solar
    );

    let bh_mass_kg = mean_bh_mass_solar * constants::SOLAR_MASS;
    let rs = astronomy::stellar::schwarzschild_radius(bh_mass_kg);

    let envs = [
        ("Halo diffus", 1.0e-21, 2.0e5, 1.0e4, 0.3),
        ("Nuage dense circumstellaire", 1.0e-16, 5.0e4, 1.0e4, 300.0),
    ];

    println!("\n-- Absorption de matiere hors rayon de Schwarzschild --\n");
    println!("Masse BH de reference : {:.2} Msun", mean_bh_mass_solar);
    println!("Rayon de Schwarzschild Rs = {:.3e} m", rs);
    println!(
        "L'absorption gravitationnelle pertinente commence a l'echelle du rayon de Bondi rB, avec rB >> Rs.\n"
    );

    println!(
        "{:28} | {:>10} | {:>10} | {:>10} | {:>12} | {:>12}",
        "Environnement", "rB(m)", "rB/Rs", "rho_DM", "Mdot_baryon", "Mdot_DM"
    );
    println!(
        "{:-<28}-+-{:-<10}-+-{:-<10}-+-{:-<10}-+-{:-<12}-+-{:-<12}",
        "", "", "", "", "", ""
    );

    let year_s = 365.25 * 24.0 * 3600.0;
    let solar_mass_year = constants::SOLAR_MASS / year_s;

    for &(name, rho_baryon, v_rel, c_s, rho_dm_gev_cm3) in &envs {
        let rho_dm = gev_cm3_to_kg_m3(rho_dm_gev_cm3);
        let r_b = bondi_radius(bh_mass_kg, v_rel, c_s);
        let r_b_over_rs = r_b / rs;

        let mdot_b = bondi_hoyle_rate(bh_mass_kg, rho_baryon, v_rel, c_s);
        let mdot_dm = bondi_hoyle_rate(bh_mass_kg, rho_dm, v_rel, c_s);

        println!(
            "{:28} | {:10.3e} | {:10.3e} | {:10.3e} | {:12.3e} | {:12.3e}",
            name,
            r_b,
            r_b_over_rs,
            rho_dm,
            mdot_b / solar_mass_year,
            mdot_dm / solar_mass_year
        );
    }

    println!("\nLecture directe :");
    println!("1) Oui, l'absorption se fait tres majoritairement en dehors de Rs (rB/Rs enorme).");
    println!("2) La DM locale standard (~0.3 GeV/cm^3) donne un Mdot_DM tres faible.");
    println!(
        "3) Meme en environnement DM dense, Mdot_DM reste en general inferieur au flux baryonique local dense."
    );

    println!("\n-- Interpretation physique --\n");
    println!("1) Oui, une etoile supermassive peut finir en trou noir si Mcol > ~2.5 Msun.");
    println!("2) Mais le lithium est majoritairement detruit bien avant l'effondrement final.");
    println!("3) Le BH avale surtout le coeur profond, pas tout le lithium galactique.");
    println!("4) Ce canal est local et tardif (evolution stellaire), pas cosmologique primordial.");

    let li7_bbn = 5.62e-10;
    let li7_obs = 1.58e-10;
    let required_global = 1.0 - li7_obs / li7_bbn;

    println!("\n-- Peut-il expliquer le manque de lithium cosmologique ? --\n");
    println!(
        "Reduction globale requise (BBN -> plateau) : {:.2}%",
        required_global * 100.0
    );
    println!(
        "Depletion moyenne de ce mecanisme stellaire: {:.2}%",
        mean_li_depletion * 100.0
    );

    if mean_li_depletion >= required_global && p_bh > 0.2 {
        println!(
            "Verdict modelise: potentiellement contributif, mais peu plausible comme solution unique."
        );
    } else {
        println!(
            "Verdict modelise: peut contribuer localement, mais n'explique pas a lui seul le manque de lithium primordial."
        );
    }

    println!("\nConclusion: le scenario 'etoiles tres massives riches en Li -> BH' est possible");
    println!(
        "pour certains objets, mais insuffisant comme explication generale du paradoxe du lithium."
    );
}
