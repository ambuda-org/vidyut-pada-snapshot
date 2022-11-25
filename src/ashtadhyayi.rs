use crate::ardhadhatuka;
use crate::atmanepada;
use crate::constants::{La, Prayoga, Purusha, Vacana};
use crate::dhatu_karya;
use crate::la_karya;
use crate::prakriya::Prakriya;
use crate::ac_sandhi;
use crate::samjna;
use crate::sanadi;
use crate::tin_pratyaya;
use crate::vikarana;
use std::error::Error;

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
    sanadi::run(&mut p, la.is_sarvadhatuka())?;
    la_karya::run(&mut p, la)?;

    ardhadhatuka::dhatu_adesha_before_pada(&mut p, la);

    atmanepada::run(&mut p);
    tin_pratyaya::adesha(&mut p, purusha, vacana);

    samjna::run(&mut p);
    vikarana::run(&mut p)?;

    if la == La::AshirLin {
        tin_pratyaya::siddhi(&mut p, la)?;
    }

    if la != La::AshirLin {
        tin_pratyaya::siddhi(&mut p, la)?;
    }

    ac_sandhi::run(&mut p);

    Ok(p)
}
