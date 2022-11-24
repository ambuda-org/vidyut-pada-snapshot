use crate::atmanepada;
use crate::constants::{La, Prayoga, Purusha, Vacana};
use crate::dhatu_karya;
use crate::la_karya;
use crate::prakriya::Prakriya;
use crate::sanadi;
use crate::tin_pratyaya;
use std::error::Error;

fn is_sarvadhatuka(la: La) -> bool {
    matches!(la, La::Lat | La::Lot | La::Lan | La::VidhiLin)
}

pub fn tinanta(
    dhatu: &str,
    code: &str,
    la: La,
    prayoga: Prayoga,
    purusha: Purusha,
    vacana: Vacana,
) -> Result<Prakriya, Box<dyn Error>> {
    let mut p = Prakriya::new();
    p.add_tags(&[prayoga.as_tag(), purusha.as_tag(), vacana.as_tag()]);

    dhatu_karya::run(&mut p, dhatu, code)?;
    sanadi::run(&mut p, is_sarvadhatuka(la))?;
    la_karya::run(&mut p, la)?;
    atmanepada::run(&mut p);
    tin_pratyaya::adesha(&mut p, purusha, vacana);

    Ok(p)
}
