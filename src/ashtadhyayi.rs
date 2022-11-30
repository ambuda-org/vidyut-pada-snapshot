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
use crate::prakriya::{Prakriya, RuleChoice};
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
    sanadi::run(p, la)?;

    // Add the lakAra and convert it to a basic tin ending.
    la_karya::run(p, la)?;
    ardhadhatuka::dhatu_adesha_before_pada(p, la);
    atmanepada::run(p);
    tin_pratyaya::adesha(p, purusha, vacana);
    samjna::run(p);

    // Do lit-siddhi and AzIrlin-siddhi first to support the valAdi vArttika for aj -> vi.
    let is_lit_or_ashirlin = matches!(la, La::Lit | La::AshirLin);
    if is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, la)?;
    }

    // Add necessary vikaranas.
    ardhadhatuka::dhatu_adesha_before_vikarana(p, la);
    vikarana::run(p)?;
    samjna::run(p);

    // --- Code below this line needs to be cleaned up. ---

    if !la.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(p)
    }

    dvitva::run(p);

    if !is_lit_or_ashirlin {
        tin_pratyaya::siddhi(p, la)?;
    }

    if la.is_sarvadhatuka() {
        dhatu_samprasarana_tasks(p)
    }

    angasya::run_remainder(p);

    // Apply sandhi rules and return.
    ac_sandhi::run(p);
    tripadi::run(p);

    Ok(())
}

/// Explores all optional derivations for some input.
///
/// Many of the rules in the Ashtadhyayi are optional, and by accepting or declining these optional
/// rules, we create different final results. `PrakriyaStack` manages the work required in finding
/// and exploring the various combinations of optional rules.
#[derive(Default)]
struct PrakriyaStack {
    /// Completed prakriyas.
    prakriyas: Vec<Prakriya>,
    /// Combinations of optional rules that we have yet to try.
    paths: Vec<Vec<RuleChoice>>,
}

impl PrakriyaStack {
    /// Creates an empty `PrakriyaStack`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Finds all variants of the given derivation function.
    ///
    /// `derive` should accept an empty `Prakriya` and mutate it in-place.
    pub fn find_all(&mut self, derive: impl Fn(&mut Prakriya)) {
        let mut p_init = Prakriya::new();
        derive(&mut p_init);
        self.add_prakriya(p_init, &[]);

        while let Some(path) = self.pop_path() {
            let mut p = Prakriya::new();
            p.set_options(&path);
            derive(&mut p);
            self.add_prakriya(p, &path);
        }
    }

    /// Adds a prakriya to the result set and adds new paths to the stack.
    ///
    /// We find new paths as follows. Suppose our initial prakriya followed the following path:
    ///
    ///     Accept(A), Accept(B), Accept(C)
    ///
    /// We then add one candidate path for each alternate choice we could have made:
    ///
    ///     Decline(A)
    ///     Accept(A), Decline(B)
    ///     Accept(A), Accept(B), Decline(C)
    ///
    /// Suppose we then try `Decline(A)` and make the following choices:
    ///
    ///     Decline(A), Accept(B), Accept(D)
    ///
    /// After this, adding an `Accept(A) path to the stack would be a mistake, as it would cause an
    /// infinite loop. Instead, we freeze our initial decision to use `Decline(A)` and add only the
    /// following paths:
    ///
    ///     Decline(A), Decline(B)
    ///     Decline(A), Accept(B), Decline(D)
    fn add_prakriya(&mut self, p: Prakriya, initial_choices: &[RuleChoice]) {
        let choices = p.rule_choices();
        let offset = initial_choices.len();
        for i in offset..choices.len() {
            let mut path = choices[..=i].to_vec();

            // Swap the last choice.
            let i = path.len() - 1;
            path[i] = match path[i] {
                RuleChoice::Accept(code) => RuleChoice::Decline(code),
                RuleChoice::Decline(code) => RuleChoice::Accept(code),
            };

            self.paths.push(path);
        }
        self.prakriyas.push(p);
    }

    fn pop_path(&mut self) -> Option<Vec<RuleChoice>> {
        self.paths.pop()
    }

    /// Retuns all of the prakriyas this stack has found. This consumes the stack.
    fn prakriyas(self) -> Vec<Prakriya> {
        self.prakriyas
    }
}

pub fn derive_tinantas(
    dhatu: &str,
    code: &str,
    la: La,
    prayoga: Prayoga,
    purusha: Purusha,
    vacana: Vacana,
) -> Vec<Prakriya> {
    let mut stack = PrakriyaStack::new();
    stack.find_all(|p| derive_tinanta(p, dhatu, code, la, prayoga, purusha, vacana).unwrap());
    stack.prakriyas()
}
