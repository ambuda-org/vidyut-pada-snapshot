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
use crate::prakriya::{Prakriya, PrakriyaStack};
use crate::samjna;
use crate::samprasarana;
use crate::sanadi;
use crate::tin_pratyaya;
use crate::tripadi;
use crate::vikarana;
use std::error::Error;

/// Samprasarana of the dhatu is conditioned on several other operations, which we must execute
/// first:
///
/// 1. jha_adesha
/// 2. it_agama
/// 3. atidesha
fn dhatu_samprasarana_tasks(p: &mut Prakriya) {
    // Needed transitively for dhatu-samprasarana.
    angasya::try_pratyaya_adesha(p);
    // Depends on jha_adesha since it conditions on the first sound.
    it_agama::run_before_attva(p);
    // Depends on it_agama for certain rules.
    atidesha::run_before_attva(p);

    // Depends on atidesha (for kit-Nit).
    samprasarana::run_for_dhatu(p);
    // Ad-Adeza and other special tasks for Ardhadhatuka
    ardhadhatuka::run_before_dvitva(p);

    // Now finish it_agama and atidesha
    it_agama::run_after_attva(p);
    atidesha::run_after_attva(p);
}

pub fn derive_tinanta(
    p: &mut Prakriya,
    dhatu: &str,
    code: &str,
    la: La,
    prayoga: Prayoga,
    purusha: Purusha,
    vacana: Vacana,
) -> Result<(), Box<dyn Error>> {
    p.add_tags(&[prayoga.as_tag(), purusha.as_tag(), vacana.as_tag()]);

    // Create the dhatu.
    dhatu_karya::run(p, dhatu, code)?;
    sanadi::run(p, la);

    // Add the lakAra and convert it to a basic tin ending.
    la_karya::run(p, la)?;
    ardhadhatuka::dhatu_adesha_before_pada(p, la); // [TODO implement all below this line.]
    atmanepada::run(p);
    tin_pratyaya::adesha(p, purusha, vacana);
    samjna::run(p);

    // Do lit-siddhi and AzIrlin-siddhi first to support the valAdi vArttika for aj -> vi.
    let is_lit_or_ashirlin = matches!(la, La::Lit | La::AshirLin);
    if is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, la);
    }

    // Add necessary vikaranas.
    ardhadhatuka::run_before_vikarana(p, la);
    vikarana::run(p)?;
    samjna::run(p);

    // --- Code below this line needs to be cleaned up. ---

    if !la.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(p)
    }

    angasya::hacky_before_dvitva(p);
    dvitva::run(p);
    samprasarana::run_for_abhyasa(p);

    if !is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, la);
    }

    if la.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(p)
    }

    angasya::iit_agama(p);

    // Must follow tin-siddhi (for valAdi)
    ardhadhatuka::run_am_agama(p);

    angasya::run_remainder(p);

    // Apply sandhi rules and return.
    ac_sandhi::run(p);
    // Finally, run the tripAdi.
    tripadi::run(p);

    Ok(())
}

pub fn derive_tinantas(
    dhatu: &str,
    code: &str,
    la: La,
    prayoga: Prayoga,
    purusha: Purusha,
    vacana: Vacana,
    log_steps: bool,
) -> Vec<Prakriya> {
    let mut stack = PrakriyaStack::new();
    stack.find_all(
        |p| derive_tinanta(p, dhatu, code, la, prayoga, purusha, vacana).unwrap(),
        log_steps,
    );
    stack.prakriyas()
}
