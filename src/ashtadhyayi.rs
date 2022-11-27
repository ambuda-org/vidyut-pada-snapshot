use crate::ac_sandhi;
use crate::angasya;
use crate::ardhadhatuka;
use crate::atidesha;
use crate::atmanepada;
use crate::constants::{La, Prayoga, Purusha, Vacana};
use crate::dhatu_karya;
use crate::dvitva;
use crate::it_agama;
use crate::la_karya;
use crate::prakriya::Prakriya;
use crate::samjna;
use crate::sanadi;
use crate::tin_pratyaya;
use crate::tripadi;
use crate::vikarana;
use std::error::Error;

///  Samprasarana of the dhatu is conditioned on several other operations, which we must execute
///  first:
///
/// jha_adesha --> it_agama --> atidesha --> samprasarana
fn dhatu_samprasarana_tasks(p: &mut Prakriya) {
    // Needed transitively for dhatu-samprasarana.
    angasya::try_pratyaya_adesha(p);
    // Depends on jha_adesha since it conditions on the first sound.
    it_agama::run_before_attva(p);
    // Depends on it_agama for certain rules.
    atidesha::run_before_attva(p);

    // Depends on atidesha (for kit-Nit).
    // samprasarana::run_for_dhatu(p)
    // Ad-Adeza and other special tasks for Ardhadhatuka
    // ardhadhatuka::run_before_dvitva(p)

    // Now finish it_agama and atidesha
    // angasya::it_agama::run_after_attva(p)
    atidesha::run_after_attva(p);
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

    // Create the dhatu.
    dhatu_karya::run(&mut p, dhatu, code)?;
    sanadi::run(&mut p, la.is_sarvadhatuka())?;

    // Add the lakAra and convert it to a basic tin ending.
    la_karya::run(&mut p, la)?;
    ardhadhatuka::dhatu_adesha_before_pada(&mut p, la);
    atmanepada::run(&mut p);
    tin_pratyaya::adesha(&mut p, purusha, vacana);
    samjna::run(&mut p);

    // Do lit-siddhi and AzIrlin-siddhi first to support the valAdi vArttika for aj>vi.
    let is_lit_or_ashirlin = matches!(la, La::Lit | La::AshirLin);
    if is_lit_or_ashirlin {
        tin_pratyaya::siddhi(&mut p, la)?;
    }

    // Add necessary vikaranas.
    ardhadhatuka::dhatu_adesha_before_vikarana(&mut p, la);
    vikarana::run(&mut p)?;
    samjna::run(&mut p);

    // --- Code below this line needs to be cleaned up. ---
    //
    if !la.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(&mut p)
    }

    dvitva::run(&mut p);

    if !is_lit_or_ashirlin {
        tin_pratyaya::siddhi(&mut p, la)?;
    }

    if la.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(&mut p)
    }

    angasya::run_remainder(&mut p);
    ac_sandhi::run(&mut p);

    tripadi::run(&mut p);

    Ok(p)
}
