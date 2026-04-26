//! Catalog entries for biochemistry functions.

use super::FunctionInfo;
use super::reg;
use crate::hub::engine::experience::experiment::DomainType::Biochemistry;

pub(super) fn register(e: &mut Vec<FunctionInfo>) {
    reg(
        e,
        Biochemistry,
        "michaelis_menten_rate",
        &["vmax", "substrate", "km"],
        "Michaelis-Menten enzyme kinetics",
    );
    reg(
        e,
        Biochemistry,
        "henderson_hasselbalch",
        &["pka", "base_conc", "acid_conc"],
        "Henderson-Hasselbalch pH equation",
    );
    reg(
        e,
        Biochemistry,
        "gibbs_free_energy",
        &["delta_h", "delta_s", "temperature"],
        "Gibbs free energy of reaction",
    );
    reg(
        e,
        Biochemistry,
        "nernst_potential",
        &["z", "c_out", "c_in", "temperature"],
        "Nernst electrochemical potential",
    );
    reg(
        e,
        Biochemistry,
        "enzyme_turnover_number",
        &["vmax", "enzyme_conc"],
        "Enzyme catalytic turnover number",
    );
    reg(
        e,
        Biochemistry,
        "competitive_inhibition_rate",
        &["vmax", "substrate", "km", "inhibitor", "ki"],
        "Competitive inhibition rate",
    );
    reg(
        e,
        Biochemistry,
        "osmotic_pressure",
        &["molarity", "temperature", "i_factor"],
        "Van't Hoff osmotic pressure",
    );
    reg(
        e,
        Biochemistry,
        "arrhenius_rate",
        &["prefactor", "activation_energy", "temperature"],
        "Arrhenius reaction rate",
    );
    reg(
        e,
        Biochemistry,
        "binding_free_energy",
        &["kd", "temperature"],
        "Binding free energy from Kd",
    );
    reg(
        e,
        Biochemistry,
        "cooperativity_hill",
        &["substrate", "k_half", "n_hill"],
        "Hill cooperativity equation",
    );
    reg(
        e,
        Biochemistry,
        "ph_from_concentration",
        &["h_concentration"],
        "pH from H+ concentration",
    );
}
