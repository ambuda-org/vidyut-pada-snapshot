use crate::atmanepada;
use crate::constants::Tag as T;
use crate::constants::{Prayoga, Purusha, Vacana};
use crate::dhatu_karya;
use crate::la_karya;
use crate::prakriya::Prakriya;
use crate::sanadi;
use crate::tin_pratyaya;
use std::error::Error;

pub fn tinanta(
    dhatu: &str,
    code: &str,
    la: &str,
    prayoga: Prayoga,
    purusha: Purusha,
    vacana: Vacana,
) -> Result<Prakriya, Box<dyn Error>> {
    let mut p = Prakriya::new();
    p.add_tags(&[prayoga.as_tag(), purusha.as_tag(), vacana.as_tag()]);

    dhatu_karya::run(&mut p, dhatu, code)?;

    let vidhi_lin = la == "li~N" && !p.has_tag(T::Ashih);
    let is_sarvadhatuka = vidhi_lin || ["la~w", "lo~w", "la~N"].contains(&la);
    sanadi::run(&mut p, is_sarvadhatuka)?;

    la_karya::run(&mut p, la)?;
    atmanepada::run(&mut p);
    tin_pratyaya::adesha(&mut p, purusha, vacana);

    Ok(p)
}
