use super::super::Molecule;

pub mod acenaphthylene;
pub mod anthracene;
pub mod benzene;
pub mod biphenyl;
pub mod chrysene;
pub mod coronene;
pub mod cumene;
pub mod ethylbenzene;
pub mod fluorene;
pub mod mesitylene;
pub mod naphthalene;
pub mod phenanthrene;
pub mod pyrene;
pub mod styrene;
pub mod toluene;
pub mod xylene_m;
pub mod xylene_o;
pub mod xylene_p;

pub fn all() -> Vec<Molecule> {
    vec![
        benzene::molecule(),
        toluene::molecule(),
        ethylbenzene::molecule(),
        xylene_o::molecule(),
        xylene_m::molecule(),
        xylene_p::molecule(),
        styrene::molecule(),
        cumene::molecule(),
        mesitylene::molecule(),
        naphthalene::molecule(),
        anthracene::molecule(),
        phenanthrene::molecule(),
        pyrene::molecule(),
        biphenyl::molecule(),
        fluorene::molecule(),
        acenaphthylene::molecule(),
        chrysene::molecule(),
        coronene::molecule(),
    ]
}
