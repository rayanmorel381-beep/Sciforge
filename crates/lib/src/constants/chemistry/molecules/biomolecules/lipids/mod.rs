use super::super::Molecule;

pub mod aldosterone;
pub mod beta_carotene;
pub mod cholesterol;
pub mod cholic_acid;
pub mod cortisol;
pub mod ergosterol;
pub mod estradiol;
pub mod lanosterol;
pub mod lecithin;
pub mod progesterone;
pub mod sphingomyelin;
pub mod squalene;
pub mod testosterone;
pub mod triolein;
pub mod tristearin;

pub fn all() -> Vec<Molecule> {
    vec![
        cholesterol::molecule(),
        ergosterol::molecule(),
        lanosterol::molecule(),
        testosterone::molecule(),
        estradiol::molecule(),
        progesterone::molecule(),
        cortisol::molecule(),
        aldosterone::molecule(),
        cholic_acid::molecule(),
        lecithin::molecule(),
        sphingomyelin::molecule(),
        triolein::molecule(),
        tristearin::molecule(),
        beta_carotene::molecule(),
        squalene::molecule(),
    ]
}
