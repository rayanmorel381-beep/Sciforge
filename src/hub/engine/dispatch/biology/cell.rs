//! Dispatch handler for cell biology functions.

use super::super::params::*;
use crate::hub::domain::biology as bio;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "cell_adhesion_energy" => Ok(RunOutput::Scalar(
            bio::cell::adhesion::cell_adhesion_energy(
                get_f(p, "contact_area")?,
                get_f(p, "cadherin_density")?,
                get_f(p, "bond_energy")?,
            ),
        )),
        "integrin_focal_adhesion_force" => Ok(RunOutput::Scalar(
            bio::cell::adhesion::integrin_focal_adhesion_force(
                get_f(p, "integrin_count")?,
                get_f(p, "force_per_integrin")?,
            ),
        )),
        "adhesion_receptor_binding" => Ok(RunOutput::Scalar(
            bio::cell::adhesion::adhesion_receptor_binding(
                get_f(p, "ligand")?,
                get_f(p, "receptor")?,
                get_f(p, "kd")?,
            ),
        )),
        "cell_cell_junction_strength" => Ok(RunOutput::Scalar(
            bio::cell::adhesion::cell_cell_junction_strength(
                get_f(p, "tight_junction")?,
                get_f(p, "adherens_junction")?,
                get_f(p, "desmosome")?,
            ),
        )),
        "chemotaxis_velocity" => Ok(RunOutput::Scalar(bio::cell::adhesion::chemotaxis_velocity(
            get_f(p, "gradient")?,
            get_f(p, "sensitivity")?,
            get_f(p, "max_speed")?,
        ))),
        "haptotaxis_velocity" => Ok(RunOutput::Scalar(bio::cell::adhesion::haptotaxis_velocity(
            get_f(p, "ecm_gradient")?,
            get_f(p, "adhesion_strength")?,
            get_f(p, "drag")?,
        ))),
        "durotaxis_force" => Ok(RunOutput::Scalar(bio::cell::adhesion::durotaxis_force(
            get_f(p, "stiffness_gradient")?,
            get_f(p, "cell_contractility")?,
        ))),
        "collective_migration_speed" => Ok(RunOutput::Scalar(
            bio::cell::adhesion::collective_migration_speed(
                get_f(p, "leader_speed")?,
                get_u(p, "follower_count")?,
                get_f(p, "coupling")?,
            ),
        )),
        "wound_healing_rate" => Ok(RunOutput::Scalar(bio::cell::adhesion::wound_healing_rate(
            get_f(p, "gap_width")?,
            get_f(p, "migration_speed")?,
            get_f(p, "proliferation_rate")?,
        ))),
        "ecm_remodeling_rate" => Ok(RunOutput::Scalar(bio::cell::adhesion::ecm_remodeling_rate(
            get_f(p, "mmp_activity")?,
            get_f(p, "timp_activity")?,
            get_f(p, "substrate")?,
        ))),
        "cell_spreading_area" => Ok(RunOutput::Scalar(bio::cell::adhesion::cell_spreading_area(
            get_f(p, "adhesion_strength")?,
            get_f(p, "cortical_tension")?,
        ))),
        "catch_bond_lifetime" => Ok(RunOutput::Scalar(bio::cell::adhesion::catch_bond_lifetime(
            get_f(p, "force")?,
            get_f(p, "k0")?,
            get_f(p, "x1")?,
            get_f(p, "x2")?,
        ))),
        "cell_cycle_ode" => {
            let (a, b, c) = bio::cell::cycle::cell_cycle_ode(
                get_f(p, "cyclin")?,
                get_f(p, "cdk")?,
                get_f(p, "apc")?,
                get_f(p, "k_syn")?,
                get_f(p, "k_deg")?,
                get_f(p, "k_act")?,
                get_f(p, "k_inact")?,
            );
            Ok(RunOutput::Triple(a, b, c))
        }
        "cell_cycle_simulate" => Ok(RunOutput::Matrix(
            bio::cell::cycle::cell_cycle_simulate(
                get_f(p, "cyclin0")?,
                get_f(p, "cdk0")?,
                get_f(p, "apc0")?,
                get_f(p, "k_syn")?,
                get_f(p, "k_deg")?,
                get_f(p, "k_act")?,
                get_f(p, "k_inact")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            )
            .into_iter()
            .map(|(a, b, c)| vec![a, b, c])
            .collect(),
        )),
        "mitotic_index" => Ok(RunOutput::Scalar(bio::cell::cycle::mitotic_index(
            get_u(p, "dividing_cells")?,
            get_u(p, "total_cells")?,
        ))),
        "cell_growth_logistic" => Ok(RunOutput::Vector(bio::cell::cycle::cell_growth_logistic(
            get_f(p, "n")?,
            get_f(p, "r")?,
            get_f(p, "k")?,
            get_f(p, "dt")?,
            get_u(p, "steps")?,
        ))),
        "g1_checkpoint" => Ok(RunOutput::Boolean(bio::cell::cycle::g1_checkpoint(
            get_f(p, "dna_damage")?,
            get_f(p, "p53_threshold")?,
            get_f(p, "rb_active")?,
        ))),
        "g2_checkpoint" => Ok(RunOutput::Boolean(bio::cell::cycle::g2_checkpoint(
            get_f(p, "dna_damage")?,
            get_f(p, "repair_capacity")?,
            get_f(p, "cdk1_activity")?,
        ))),
        "spindle_assembly_checkpoint" => Ok(RunOutput::Boolean(
            bio::cell::cycle::spindle_assembly_checkpoint(get_u(p, "unattached_kinetochores")?),
        )),
        "apoptosis_probability" => Ok(RunOutput::Scalar(bio::cell::cycle::apoptosis_probability(
            get_f(p, "dna_damage")?,
            get_f(p, "p53")?,
            get_f(p, "bcl2")?,
            get_f(p, "bax")?,
        ))),
        "cell_doubling_time" => Ok(RunOutput::Scalar(bio::cell::cycle::cell_doubling_time(
            get_f(p, "growth_rate")?,
        ))),
        "contact_inhibition" => Ok(RunOutput::Scalar(bio::cell::cycle::contact_inhibition(
            get_f(p, "density")?,
            get_f(p, "max_density")?,
            get_f(p, "steepness")?,
        ))),
        "phase_duration" => {
            let r = bio::cell::cycle::phase_duration(
                get_f(p, "total_cycle_time")?,
                get_f(p, "g1_fraction")?,
                get_f(p, "s_fraction")?,
                get_f(p, "g2_fraction")?,
            );
            Ok(RunOutput::Vector(vec![r.0, r.1, r.2, r.3]))
        }
        "dna_damage_accumulation" => Ok(RunOutput::Scalar(
            bio::cell::cycle::dna_damage_accumulation(
                get_f(p, "damage")?,
                get_f(p, "production_rate")?,
                get_f(p, "repair_rate")?,
                get_f(p, "dt")?,
            ),
        )),
        "restriction_point" => Ok(RunOutput::Boolean(bio::cell::cycle::restriction_point(
            get_f(p, "growth_factor")?,
            get_f(p, "threshold")?,
            get_f(p, "rb_phosphorylation")?,
        ))),
        "cyclin_oscillator" => Ok(RunOutput::Scalar(bio::cell::cycle::cyclin_oscillator(
            get_f(p, "cyclin")?,
            get_f(p, "cdk")?,
            get_f(p, "k_syn")?,
            get_f(p, "k_deg")?,
        ))),
        "cell_senescence_probability" => Ok(RunOutput::Scalar(
            bio::cell::cycle::cell_senescence_probability(
                get_f(p, "telomere_length")?,
                get_f(p, "critical_length")?,
                get_f(p, "dna_damage")?,
            ),
        )),
        "proliferation_index" => Ok(RunOutput::Scalar(bio::cell::cycle::proliferation_index(
            get_u(p, "s_phase")?,
            get_u(p, "g2m_phase")?,
            get_u(p, "total")?,
        ))),
        "growth_fraction" => Ok(RunOutput::Scalar(bio::cell::cycle::growth_fraction(
            get_u(p, "proliferating")?,
            get_u(p, "total")?,
        ))),
        "cell_loss_factor" => Ok(RunOutput::Scalar(bio::cell::cycle::cell_loss_factor(
            get_f(p, "growth_rate")?,
            get_f(p, "doubling_time")?,
        ))),
        "cell_hayflick_limit" => Ok(RunOutput::Scalar(bio::cell::cycle::hayflick_limit(
            get_f(p, "initial_telomere")?,
            get_f(p, "loss_per_division")?,
            get_f(p, "critical")?,
        ))),
        "quiescence_entry" => Ok(RunOutput::Boolean(bio::cell::cycle::quiescence_entry(
            get_f(p, "growth_factor")?,
            get_f(p, "nutrient")?,
            get_f(p, "gf_threshold")?,
            get_f(p, "nutrient_threshold")?,
        ))),
        "cell_autophagy_flux" => Ok(RunOutput::Scalar(bio::cell::organelles::autophagy_flux(
            get_f(p, "lc3_ii")?,
            get_f(p, "p62")?,
            get_f(p, "bafilomycin_effect")?,
        ))),
        "proteasome_degradation_rate" => Ok(RunOutput::Scalar(
            bio::cell::organelles::proteasome_degradation_rate(
                get_f(p, "ubiquitin_tags")?,
                get_f(p, "proteasome_activity")?,
                get_f(p, "km")?,
            ),
        )),
        "lysosome_ph" => Ok(RunOutput::Scalar(bio::cell::organelles::lysosome_ph(
            get_f(p, "v_atpase_rate")?,
            get_f(p, "proton_leak")?,
            get_f(p, "buffer_capacity")?,
            get_f(p, "volume")?,
        ))),
        "endosome_maturation" => {
            let (a, b) = bio::cell::organelles::endosome_maturation(
                get_f(p, "rab5")?,
                get_f(p, "rab7")?,
                get_f(p, "conversion_rate")?,
                get_f(p, "dt")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "receptor_recycling" => Ok(RunOutput::Scalar(
            bio::cell::organelles::receptor_recycling(
                get_f(p, "internalized")?,
                get_f(p, "recycling_rate")?,
                get_f(p, "degradation_rate")?,
            ),
        )),
        "mitochondrial_fission_rate" => Ok(RunOutput::Scalar(
            bio::cell::organelles::mitochondrial_fission_rate(
                get_f(p, "drp1")?,
                get_f(p, "fis1")?,
                get_f(p, "threshold")?,
            ),
        )),
        "mitochondrial_fusion_rate" => Ok(RunOutput::Scalar(
            bio::cell::organelles::mitochondrial_fusion_rate(
                get_f(p, "mfn1")?,
                get_f(p, "mfn2")?,
                get_f(p, "opa1")?,
            ),
        )),
        "er_stress_upr" => Ok(RunOutput::Scalar(bio::cell::organelles::er_stress_upr(
            get_f(p, "misfolded")?,
            get_f(p, "bip")?,
            get_f(p, "ire1_threshold")?,
        ))),
        "golgi_transport_rate" => Ok(RunOutput::Scalar(
            bio::cell::organelles::golgi_transport_rate(
                get_f(p, "cargo")?,
                get_f(p, "coat_protein")?,
                get_f(p, "gtp")?,
                get_f(p, "km_coat")?,
            ),
        )),
        "peroxisome_beta_oxidation" => Ok(RunOutput::Scalar(
            bio::cell::organelles::peroxisome_beta_oxidation(
                get_f(p, "vlcfa")?,
                get_f(p, "enzyme_activity")?,
                get_f(p, "km")?,
            ),
        )),
        "cytoskeleton_treadmilling" => Ok(RunOutput::Scalar(
            bio::cell::organelles::cytoskeleton_treadmilling(
                get_f(p, "polymerization_rate")?,
                get_f(p, "depolymerization_rate")?,
            ),
        )),
        "nuclear_import_rate" => Ok(RunOutput::Scalar(
            bio::cell::organelles::nuclear_import_rate(
                get_f(p, "cargo")?,
                get_f(p, "importin")?,
                get_f(p, "ran_gtp")?,
                get_f(p, "kd")?,
            ),
        )),
        "cell_volume_regulation" => Ok(RunOutput::Scalar(
            bio::cell::organelles::cell_volume_regulation(
                get_f(p, "volume")?,
                get_f(p, "target_volume")?,
                get_f(p, "permeability")?,
                get_f(p, "osmotic_difference")?,
            ),
        )),
        "ligand_receptor_binding" => Ok(RunOutput::Scalar(
            bio::cell::signaling::ligand_receptor_binding(
                get_f(p, "l")?,
                get_f(p, "r_total")?,
                get_f(p, "kd")?,
            ),
        )),
        "hill_response" => Ok(RunOutput::Scalar(bio::cell::signaling::hill_response(
            get_f(p, "signal")?,
            get_f(p, "k")?,
            get_f(p, "n")?,
        ))),
        "bistable_switch" => Ok(RunOutput::Scalar(bio::cell::signaling::bistable_switch(
            get_f(p, "x")?,
            get_f(p, "k1")?,
            get_f(p, "k2")?,
            get_f(p, "n")?,
            get_f(p, "alpha")?,
            get_f(p, "beta")?,
        ))),
        "bistable_simulate" => Ok(RunOutput::Vector(bio::cell::signaling::bistable_simulate(
            get_f(p, "x0")?,
            get_f(p, "k1")?,
            get_f(p, "k2")?,
            get_f(p, "n")?,
            get_f(p, "alpha")?,
            get_f(p, "beta")?,
            get_f(p, "dt")?,
            get_u(p, "steps")?,
        ))),
        "mapk_cascade" => {
            let k_activate_v = get_v(p, "k_activate")?;
            let k_activate = [k_activate_v[0], k_activate_v[1], k_activate_v[2]];
            let k_deactivate_v = get_v(p, "k_deactivate")?;
            let k_deactivate = [k_deactivate_v[0], k_deactivate_v[1], k_deactivate_v[2]];
            {
                let (a, b, c) = bio::cell::signaling::mapk_cascade(
                    get_f(p, "raf")?,
                    get_f(p, "mek")?,
                    get_f(p, "erk")?,
                    get_f(p, "signal")?,
                    &k_activate,
                    &k_deactivate,
                );
                Ok(RunOutput::Triple(a, b, c))
            }
        }
        "mapk_simulate" => {
            let k_activate_v = get_v(p, "k_activate")?;
            let k_activate = [k_activate_v[0], k_activate_v[1], k_activate_v[2]];
            let k_deactivate_v = get_v(p, "k_deactivate")?;
            let k_deactivate = [k_deactivate_v[0], k_deactivate_v[1], k_deactivate_v[2]];
            Ok(RunOutput::Matrix(
                bio::cell::signaling::mapk_simulate(
                    get_f(p, "raf0")?,
                    get_f(p, "mek0")?,
                    get_f(p, "erk0")?,
                    get_f(p, "signal")?,
                    &k_activate,
                    &k_deactivate,
                    get_f(p, "dt")?,
                    get_u(p, "steps")?,
                )
                .into_iter()
                .map(|(a, b, c)| vec![a, b, c])
                .collect(),
            ))
        }
        "goldbeter_koshland" => Ok(RunOutput::Scalar(bio::cell::signaling::goldbeter_koshland(
            get_f(p, "v1")?,
            get_f(p, "v2")?,
            get_f(p, "j1")?,
            get_f(p, "j2")?,
        ))),
        "cell_negative_feedback" => Ok(RunOutput::Scalar(bio::cell::signaling::negative_feedback(
            get_f(p, "output")?,
            get_f(p, "k_prod")?,
            get_f(p, "k_deg")?,
            get_f(p, "k_inh")?,
            get_f(p, "n")?,
        ))),
        "cell_positive_feedback" => Ok(RunOutput::Scalar(bio::cell::signaling::positive_feedback(
            get_f(p, "x")?,
            get_f(p, "basal")?,
            get_f(p, "vmax")?,
            get_f(p, "k")?,
            get_f(p, "n")?,
            get_f(p, "deg")?,
        ))),
        "receptor_desensitization" => Ok(RunOutput::Scalar(
            bio::cell::signaling::receptor_desensitization(
                get_f(p, "active")?,
                get_f(p, "ligand")?,
                get_f(p, "kd")?,
                get_f(p, "k_intern")?,
                get_f(p, "k_recycle")?,
                get_f(p, "total")?,
            ),
        )),
        "dual_phosphorylation" => Ok(RunOutput::Scalar(
            bio::cell::signaling::dual_phosphorylation(
                get_f(p, "x")?,
                get_f(p, "kinase")?,
                get_f(p, "phosphatase")?,
                get_f(p, "k1")?,
                get_f(p, "k2")?,
            ),
        )),
        "coherent_feedforward" => Ok(RunOutput::Scalar(
            bio::cell::signaling::coherent_feedforward(
                get_f(p, "signal")?,
                get_f(p, "x")?,
                get_f(p, "k_sx")?,
                get_f(p, "k_xy")?,
                get_f(p, "k_sy")?,
                get_f(p, "threshold")?,
            ),
        )),
        "incoherent_feedforward" => Ok(RunOutput::Scalar(
            bio::cell::signaling::incoherent_feedforward(
                get_f(p, "signal")?,
                get_f(p, "x")?,
                get_f(p, "k_activation")?,
                get_f(p, "k_repression")?,
            ),
        )),
        "michaelis_menten_cascade" => Ok(RunOutput::Scalar(
            bio::cell::signaling::michaelis_menten_cascade(
                get_f(p, "substrate")?,
                get_f(p, "enzyme")?,
                get_f(p, "km")?,
                get_f(p, "vmax")?,
            ),
        )),
        "scaffold_complex_formation" => Ok(RunOutput::Scalar(
            bio::cell::signaling::scaffold_complex_formation(
                get_f(p, "a")?,
                get_f(p, "b")?,
                get_f(p, "scaffold")?,
                get_f(p, "ka")?,
                get_f(p, "kb")?,
            ),
        )),
        "crosstalk_inhibition" => {
            let (a, b) = bio::cell::signaling::crosstalk_inhibition(
                get_f(p, "pathway_a")?,
                get_f(p, "pathway_b")?,
                get_f(p, "k_inh_ab")?,
                get_f(p, "k_inh_ba")?,
            );
            Ok(RunOutput::Pair(a, b))
        }
        "fick_first_law" => Ok(RunOutput::Scalar(bio::cell::transport::fick_first_law(
            get_f(p, "d")?,
            get_f(p, "dc_dx")?,
        ))),
        "cell_nernst_potential" => Ok(RunOutput::Scalar(bio::cell::transport::nernst_potential(
            get_f(p, "z")?,
            get_f(p, "t")?,
            get_f(p, "c_out")?,
            get_f(p, "c_in")?,
        ))),
        "goldman_equation" => Ok(RunOutput::Scalar(bio::cell::transport::goldman_equation(
            get_f(p, "p_na")?,
            get_f(p, "p_k")?,
            get_f(p, "p_cl")?,
            get_f(p, "na_out")?,
            get_f(p, "na_in")?,
            get_f(p, "k_out")?,
            get_f(p, "k_in")?,
            get_f(p, "cl_out")?,
            get_f(p, "cl_in")?,
            get_f(p, "t")?,
        ))),
        "osmotic_pressure" => Ok(RunOutput::Scalar(bio::cell::transport::osmotic_pressure(
            get_f(p, "c")?,
            get_f(p, "t")?,
        ))),
        "donnan_ratio" => Ok(RunOutput::Scalar(bio::cell::transport::donnan_ratio(
            get_f(p, "z_ion")?,
            get_f(p, "z_macro")?,
            get_f(p, "c_macro")?,
            get_f(p, "c_salt")?,
        ))),
        "active_transport_rate" => Ok(RunOutput::Scalar(
            bio::cell::transport::active_transport_rate(
                get_f(p, "vmax")?,
                get_f(p, "substrate")?,
                get_f(p, "km")?,
                get_f(p, "atp")?,
                get_f(p, "km_atp")?,
            ),
        )),
        "cell_membrane_capacitance_current" => Ok(RunOutput::Scalar(
            bio::cell::transport::membrane_capacitance_current(get_f(p, "cm")?, get_f(p, "dv_dt")?),
        )),
        "electrochemical_gradient" => Ok(RunOutput::Scalar(
            bio::cell::transport::electrochemical_gradient(
                get_f(p, "z")?,
                get_f(p, "vm")?,
                get_f(p, "equilibrium_potential")?,
            ),
        )),
        "vesicle_fusion_rate" => Ok(RunOutput::Scalar(
            bio::cell::transport::vesicle_fusion_rate(
                get_f(p, "calcium")?,
                get_f(p, "kd")?,
                get_f(p, "n")?,
                get_f(p, "k_max")?,
            ),
        )),
        "endocytosis_rate" => Ok(RunOutput::Scalar(bio::cell::transport::endocytosis_rate(
            get_f(p, "receptor_bound")?,
            get_f(p, "k_intern")?,
            get_f(p, "coat_protein")?,
            get_f(p, "kd_coat")?,
        ))),
        "exocytosis_rate" => Ok(RunOutput::Scalar(bio::cell::transport::exocytosis_rate(
            get_f(p, "vesicles")?,
            get_f(p, "calcium")?,
            get_f(p, "kd")?,
        ))),
        "gap_junction_flux" => Ok(RunOutput::Scalar(bio::cell::transport::gap_junction_flux(
            get_f(p, "c1")?,
            get_f(p, "c2")?,
            get_f(p, "permeability")?,
        ))),
        "facilitated_diffusion" => Ok(RunOutput::Scalar(
            bio::cell::transport::facilitated_diffusion(
                get_f(p, "c_out")?,
                get_f(p, "c_in")?,
                get_f(p, "vmax")?,
                get_f(p, "km")?,
            ),
        )),
        "cotransport_rate" => Ok(RunOutput::Scalar(bio::cell::transport::cotransport_rate(
            get_f(p, "substrate")?,
            get_f(p, "ion")?,
            get_f(p, "vmax")?,
            get_f(p, "km_s")?,
            get_f(p, "km_i")?,
        ))),
        "pinocytosis_uptake" => Ok(RunOutput::Scalar(bio::cell::transport::pinocytosis_uptake(
            get_f(p, "volume_rate")?,
            get_f(p, "concentration")?,
        ))),
        "ion_channel_conductance" => Ok(RunOutput::Scalar(
            bio::cell::transport::ion_channel_conductance(
                get_f(p, "g_max")?,
                get_f(p, "open_probability")?,
                get_f(p, "driving_force")?,
            ),
        )),
        "autophagy_flux" => Ok(RunOutput::Scalar(bio::cell::organelles::autophagy_flux(
            get_f(p, "lc3_ii")?,
            get_f(p, "p62")?,
            get_f(p, "bafilomycin_effect")?,
        ))),
        "hayflick_limit" => Ok(RunOutput::Scalar(bio::cell::cycle::hayflick_limit(
            get_f(p, "initial_telomere")?,
            get_f(p, "loss_per_division")?,
            get_f(p, "critical")?,
        ))),
        "negative_feedback" => Ok(RunOutput::Scalar(bio::cell::signaling::negative_feedback(
            get_f(p, "output")?,
            get_f(p, "k_prod")?,
            get_f(p, "k_deg")?,
            get_f(p, "k_inh")?,
            get_f(p, "n")?,
        ))),
        "positive_feedback" => Ok(RunOutput::Scalar(bio::cell::signaling::positive_feedback(
            get_f(p, "x")?,
            get_f(p, "basal")?,
            get_f(p, "vmax")?,
            get_f(p, "k")?,
            get_f(p, "n")?,
            get_f(p, "deg")?,
        ))),
        "fick_second_law_1d" => {
            let mut conc = get_v(p, "conc")?.to_vec();
            bio::cell::transport::fick_second_law_1d(
                &mut conc,
                get_f(p, "d")?,
                get_f(p, "dx")?,
                get_f(p, "dt")?,
                get_u(p, "steps")?,
            );
            Ok(RunOutput::Vector(conc))
        }
        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
