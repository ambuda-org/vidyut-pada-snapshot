use crate::ac_sandhi;
use crate::angasya;
use crate::ardhadhatuka;
use crate::args::{Dhatu, Lakara, SubantaArgs, TinantaArgs};
use crate::atidesha;
use crate::atmanepada;
use crate::dhatu_karya;
use crate::dvitva;
use crate::it_agama;
use crate::la_karya;
use crate::prakriya::{Prakriya, PrakriyaStack};
use crate::pratipadika_karya;
use crate::samjna;
use crate::samprasarana;
use crate::sanadi;
use crate::sup_karya;
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

fn derive_tinanta(
    p: &mut Prakriya,
    dhatu: &Dhatu,
    args: &TinantaArgs,
) -> Result<(), Box<dyn Error>> {
    let prayoga = args.prayoga();
    let lakara = args.lakara();
    let purusha = args.purusha();
    let vacana = args.vacana();

    p.add_tags(&[prayoga.as_tag(), purusha.as_tag(), vacana.as_tag()]);

    // Create the dhatu.
    dhatu_karya::run(p, dhatu)?;
    sanadi::run(p, lakara.is_ardhadhatuka());

    // Add the lakAra and convert it to a basic tin ending.
    la_karya::run(p, lakara)?;
    ardhadhatuka::dhatu_adesha_before_pada(p, lakara);
    atmanepada::run(p);
    tin_pratyaya::adesha(p, purusha, vacana);
    samjna::run(p);

    // Do lit-siddhi and AzIrlin-siddhi first to support the valAdi vArttika for aj -> vi.
    let is_lit_or_ashirlin = matches!(lakara, Lakara::Lit | Lakara::AshirLin);
    if is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, lakara, vacana);
    }

    // Add necessary vikaranas.
    ardhadhatuka::run_before_vikarana(p, lakara);
    vikarana::run(p)?;
    samjna::run(p);

    // --- Code below this line needs to be cleaned up. ---

    if !lakara.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(p)
    }

    angasya::hacky_before_dvitva(p);
    dvitva::run(p);
    samprasarana::run_for_abhyasa(p);

    if !is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, lakara, vacana);
    }

    if lakara.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(p)
    }

    // Must follow tin-siddhi (for valAdi)
    ardhadhatuka::run_am_agama(p);

    // --- Code above this line needs to be cleaned up. ---

    angasya::iit_agama(p);
    angasya::run_remainder(p);

    // Apply sandhi rules and return.
    ac_sandhi::run(p);
    // Finally, run the tripAdi.
    tripadi::run(p);

    Ok(())
}

fn derive_subanta(p: &mut Prakriya, pratipadika: &str, args: &SubantaArgs) {
    pratipadika_karya::run(p, pratipadika, args);

    sup_karya::run(p, args);
    samjna::run(p);

    ac_sandhi::try_sup_sandhi_before_angasya(p);

    angasya::run_remainder(p);

    ac_sandhi::try_sup_sandhi_after_angasya(p);
    ac_sandhi::run_common(p);

    tripadi::run(p);
}

/// An interface to the rules of the Ashtadhyayi.
///
/// This lightweight struct contains configuration options that might affect how a word is derived,
/// such as:
///
/// - whether to store the full derivation history or to disable it for performance reasons.
/// - whether to disable certain optional rules
///
/// To run with our suggested defaults, use:
///
/// ```
/// use vidyut_prakriya::Ashtadhyayi;
///
/// let a = Ashtadhyayi::new();
/// ```
///
/// For tighter control over options, use `Ashtadhyayi::builder`:
///
/// ```no_run
/// use vidyut_prakriya::Ashtadhyayi;
///
/// let a = Ashtadhyayi::builder().log_steps(false).build();
/// ```
#[derive(Debug)]
pub struct Ashtadhyayi {
    log_steps: bool,
}

impl Default for Ashtadhyayi {
    fn default() -> Self {
        Self::new()
    }
}

impl Ashtadhyayi {
    /// Creates an interface with sane defaults.
    pub fn new() -> Self {
        Ashtadhyayi { log_steps: true }
    }

    /// Returns a builder that exposes configuration options for how the engine runs rules and
    /// saves prakriya data.
    pub fn builder() -> AshtadhyayiBuilder {
        AshtadhyayiBuilder::new()
    }

    /// Returns all possible tinanta prakriyas that can be derived with the given initial
    /// conditions.
    ///
    /// TODO: add support for upasargas and sanAdi-pratyayas.
    pub fn derive_tinantas(&self, dhatu: &Dhatu, args: &TinantaArgs) -> Vec<Prakriya> {
        let mut stack = PrakriyaStack::new();
        stack.find_all(|p| derive_tinanta(p, dhatu, args).unwrap(), self.log_steps);
        stack.prakriyas()
    }

    /// Returns all possible tinanta prakriyas that can be derived with the given initial
    /// conditions.
    pub fn derive_subantas(&self, pratipadika: &str, args: &SubantaArgs) -> Vec<Prakriya> {
        let mut stack = PrakriyaStack::new();
        stack.find_all(|p| derive_subanta(p, pratipadika, args), self.log_steps);
        stack.prakriyas()
    }
}

/// A builder for creating an `Ashtadhyayi` struct.
pub struct AshtadhyayiBuilder {
    a: Ashtadhyayi,
}

impl AshtadhyayiBuilder {
    /// Creates a new builder.
    fn new() -> Self {
        Self {
            a: Ashtadhyayi::new(),
        }
    }

    /// Controls whether or not to log individual steps of the prakriya.
    ///
    /// - If `true`, each `Prakriya` will contain a full history, but the program will run more
    ///   slowly. This is the default value and is best for most use cases.
    ///
    /// - If `false`, the program will run faster, but only the final output of the `Prakriya` will
    ///   be available. This is best used when you just need to generate a word list.
    pub fn log_steps(mut self, value: bool) -> Self {
        self.a.log_steps = value;
        self
    }

    /// Creates an `Ashtadhyayi` object.
    pub fn build(self) -> Ashtadhyayi {
        self.a
    }
}
