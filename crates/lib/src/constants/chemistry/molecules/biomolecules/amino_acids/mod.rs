use super::super::Molecule;

pub mod alanine;
pub mod arginine;
pub mod asparagine;
pub mod aspartic_acid;
pub mod citrulline;
pub mod cysteine;
pub mod glutamic_acid;
pub mod glutamine;
pub mod glycine;
pub mod histidine;
pub mod hydroxylysine;
pub mod hydroxyproline;
pub mod isoleucine;
pub mod leucine;
pub mod lysine;
pub mod methionine;
pub mod ornithine;
pub mod phenylalanine;
pub mod proline;
pub mod pyrrolysine;
pub mod sarcosine;
pub mod selenocysteine;
pub mod serine;
pub mod taurine;
pub mod threonine;
pub mod tryptophan;
pub mod tyrosine;
pub mod valine;

pub fn all() -> Vec<Molecule> {
    vec![
        glycine::molecule(),
        alanine::molecule(),
        valine::molecule(),
        leucine::molecule(),
        isoleucine::molecule(),
        proline::molecule(),
        phenylalanine::molecule(),
        tryptophan::molecule(),
        methionine::molecule(),
        serine::molecule(),
        threonine::molecule(),
        cysteine::molecule(),
        tyrosine::molecule(),
        asparagine::molecule(),
        glutamine::molecule(),
        aspartic_acid::molecule(),
        glutamic_acid::molecule(),
        lysine::molecule(),
        arginine::molecule(),
        histidine::molecule(),
        selenocysteine::molecule(),
        pyrrolysine::molecule(),
        ornithine::molecule(),
        citrulline::molecule(),
        hydroxyproline::molecule(),
        hydroxylysine::molecule(),
        sarcosine::molecule(),
        taurine::molecule(),
    ]
}
