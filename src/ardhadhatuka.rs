//! ardhadhatuka

use crate::constants::La;
use crate::constants::Tag as T;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::{s, SoundSet};
use crate::term::TermView;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref VAL: SoundSet = s("val");
}

/// Lookahead function for the following rules:
///
/// > 6.1.50 minātiminotidīṅāṃ lyapi ca
/// > 6.1.51 vibhāṣā līyateḥ
fn will_cause_guna(n: TermView) -> bool {
    let is_apit = !n.has_tag(T::pit);
    !(
        // Parasmaipada Ashir-liN will use yAsuT-Agama, which is kit.
        (n.has_lakshana("li~N") && n.all(&[T::Ardhadhatuka, T::Parasmaipada]))
        // sArvadhAtukam apit will be Nit.
        || (n.has_tag(T::Sarvadhatuka) && is_apit)
        // apit liT when not after samyoga will be kit.
        // TODO: check for samyoga? But it's not needed for current usage
        || (n.has_lakshana("li~w") && is_apit)
    )
    // ArdhadhAtuka and other sArvadhAtuka suffixes will cause guna.
}

/// Replaces the dhAtu based on the suffix that follows it.
///
/// These rules must run before we choose the verb pada because the results here affect which pada
/// we choose.
pub fn dhatu_adesha_before_pada(p: &mut Prakriya, la: La) {
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    if la.is_sarvadhatuka() {
        return;
    }

    // KyAY is Yit, which allow parasamipada.
    if p.has(i, |t| t.has_u("ca\\kzi~\\N")) {
        let mut use_khya = true;
        if la == La::Lit {
            if p.is_allowed("2.4.55") {
                use_khya = false
            } else {
                p.decline("2.4.55")
            }
        }
        if use_khya {
            p.op("2.4.54", |p| {
                op::upadesha(p, i, "KyAY");
                // Remove tags set by `ca\kzi~\N`
                p.set(i, |t| {
                    t.remove_tags(&[T::anudattet, T::Nit]);
                    // For anit on `KyAY`.
                    t.add_tag(T::Anudatta);
                });
            });
        }
    }
}

/// Replaces the dhAtu based on the suffix that follows it.
///
/// These rules must run before we choose the vikarana because the results here affect which
/// vikarana we add.
pub fn dhatu_adesha_before_vikarana(p: &mut Prakriya, la: La) {
    // Rules are under 2.4.35 "ArdhadhAtuke".
    if la.is_sarvadhatuka() {
        return;
    }

    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    let n = i + 1;

    let to_ghasl = |p: &mut Prakriya| op::upadesha(p, i, "Gasx~");
    if p.has(i, f::text("ad")) {
        if p.has(n, f::lakshana_in(&["lu~N", "san"])) {
            p.op("2.4.37", to_ghasl);
        } else if p.has(n, f::u_in(&["GaY", "ap"])) {
            p.op("2.4.38", to_ghasl);
        } else if p.has(n, f::lakshana("li~w")) {
            p.op_optional("2.4.40", to_ghasl);
        } else if p.has(n, |t| {
            t.has_u("lyap") || (t.has_adi('t') && t.has_tag(T::kit))
        }) {
            p.op("2.4.36", |p| op::upadesha(p, i, "jagDi~"));
        }
        // Skip 2.4.39 (bahulaM chandasi).
    } else if p.has(i, f::u("ve\\Y")) && p.has(n, f::lakshana("li~w")) {
        p.op_optional("2.4.41", |p| op::upadesha(p, i, "vayi~"));
    } else if p.has(i, f::text("han")) {
        let to_vadha = |p: &mut Prakriya| op::upadesha(p, i, "vaDa");
        if p.has(n, f::lakshana("li~N")) {
            p.op("2.4.42", to_vadha);
        } else if p.has(n, f::lakshana("lu~N")) {
            if p.has(n, f::tag(T::Atmanepada)) {
                p.op_optional("2.4.44", to_vadha);
            } else {
                p.op("2.4.43", to_vadha);
            }
        }
    } else if p.has(i, f::u_in(&["i\\R", "i\\k"])) {
        if p.has(i, f::u("i\\k")) {
            p.step("2.4.45.v1")
        }

        let to_gami = |p: &mut Prakriya| op::upadesha(p, i, "gami~");
        if p.has(i, f::lakshana("lu~N")) {
            p.op("2.4.45", |p| op::upadesha(p, i, "gA"));
        } else if p.has(n, f::u("Ric")) {
            p.op_optional("2.4.46", to_gami);
        } else if p.has(n, f::u("san")) {
            p.op_optional("2.4.47", to_gami);
        }
    } else if p.has(i, f::u("i\\N")) {
        let to_gaa = |p: &mut Prakriya| op::upadesha(p, i, "gAN");

        if p.has(n, f::u("san")) {
            p.op("2.4.48", |p| op::upadesha(p, i, "gami~"));
        } else if p.has(n, f::lakshana("li~w")) {
            p.op("2.4.49", to_gaa);
        } else if p.has(n, f::lakshana_in(&["lu~N", "lf~N"])) {
            p.op_optional("2.4.50", to_gaa);
        }
    } else if p.has(i, f::u("asa~")) {
        p.op("2.4.52", |p| op::upadesha(p, i, "BU"));
    } else if p.has(i, f::u("brUY")) {
        // anudAtta to prevent iT
        p.op("2.4.53", |p| op::upadesha(p, i, "va\\ci~"));
    } else if p.has(i, f::u("aja~")) && p.has(n, |t| !t.has_u_in(&["GaY", "ap"])) {
        let mut run = true;
        if p.has(n, f::u("lyuw")) {
            if p.is_allowed("2.4.57") {
                run = false;
            } else {
                p.decline("2.4.57")
            }
        }

        // vArttika: valAdAvArdhadhAtuke veSyate
        //
        // This vArttika is troublesome and highly constrained. To derive
        // vivAya, we must run in this order:
        //
        //   siddhi --> vArttika --> dvitva
        //
        // But tin-siddhi must follow dvitva for rule 3.4.109. I considered
        // breaking siddhi into two stages -- one for liT, and one for other
        // lakAras -- and that might be worth doing as the program matures.
        // But for now, I don't want to change the entire structure of the
        // program to handle a rare secondary rule like this.
        //
        // As a crude fix, just check for endings that we expect will start with
        // vowels.
        let will_yasut = la == La::AshirLin && p.has_tag(T::Parasmaipada);
        let is_lit_ajadi = la == La::Lit && p.terms().last().unwrap().has_adi(&*AC);
        let will_have_valadi = !(will_yasut || is_lit_ajadi);
        if p.has(n, |t| t.has_adi(&*VAL)) && will_have_valadi {
            if p.is_allowed("2.4.56.v2") {
                p.step("2.4.56.v2");
                run = false;
            } else {
                p.decline("2.4.56.v2");
            }
        }
        if run {
            // aniT-tva comes from anudAtta in upadesha.
            p.op("2.4.56", |p| op::upadesha(p, i, "vI\\"));
        }
    }
}

/*
fn dhatu_adesha_after_vikarana(p: Prakriya):
    """
    This code depends on the Ric-vikaraNa being present.
    """
    index, c = p.find_first(T::DHATU)
    n = TermView.make(p, index)

    if not n.any(T::ARDHADHATUKA):
        return

    // HACK to make the below readable
    n.u = n.terms[0].u

    try:
        n2 = p.terms[index + 2]
    except IndexError:
        n2 = None
    if c.u == "i\\N" && n2:
        n2 = p.terms[index + 2]
        if n.u == "Ric" && n2 && n2.u in ("san", "caN"):
            op.optional(op.upadesha, "2.4.50", p, c, "gAN")


fn aa_adesha(p: Prakriya, index: int):
    c = p.terms[index]
    if not c.any(T::DHATU):
        return
    n = TermView.make(p, index)
    if not n:
        return

    // HACK
    n.u = n.terms[0].u

    // Substitution of A for root vowel

    if c.antya in s("ec") && not n.any("S"):
        if c.text == "vye" && n.any("li~w"):
            p.step("6.1.46")
        else:
            op.antya("6.1.45", p, c, "A")
    } else if  c.text in {"sPur", "sPul"} && n.u == "GaY":
        op.upadha("6.1.47", p, c, "A")
    } else if  c.u in {"qukrI\\Y", "i\\N", "ji\\"} && n.u == "Ric":
        op.antya("6.1.48", p, c, "A")
    // TODO: 6.1.49

    // 6.1.50 has a circular dependency:
    //
    // - 6.1.50 comes before dvitva
    // - dvitva comes before tin-siddhi
    // - tin-siddhi changes the application of guNa
    // - guNa affects the application of 6.1.50
    //
    // So, "look ahead" and use this rule only if the suffix will potentially
    // cause guNa. See `will_cause_guna` for details.
    ashiti_lyapi = not n.any("S") or n.u == "lyap"
    if c.u in {"mI\\Y", "qu\\mi\\Y", "dI\\N"} && ashiti_lyapi && will_cause_guna(n):
        op.antya("6.1.50", p, c, "A")
    } else if  c.text == "lI" && ashiti_lyapi && will_cause_guna(n) && c.gana != 10:
        // līyateriti yakā nirdeśo na tu śyanā. līlīṅorātvaṃ vā syādejviṣaye
        // lyapi ca. (SK)
        op.optional(op.antya, "6.1.51", p, c, "A")
    // TODO: 6.1.52 - 6.1.53
    } else if  c.u in {"ciY", "ci\\Y", "sPura~"} && n.u == "Ric":
        if c.text == "sPura~":
            op.optional(op.upadha, "6.1.54", p, c, "A")
        else:
            op.optional(op.antya, "6.1.54", p, c, "A")
    // TODO: 6.1.55 - 6.1.56
    } else if  c.text == "smi" && n.u == "Ric":
        op.optional(op.antya, "6.1.57", p, c, "A")


fn am_agama_for_term(p: Prakriya, index: int):
    c = p.terms[index]
    if not c.any(T::DHATU):
        return
    n = TermView.make(p, index)
    if not n:
        return

    if n.adi in s("Jal") && not n.any("k") {
        if c.text in {"sfj", "dfS"} {
            op.mit("6.1.58", p, c, "a")
        } else if  c.all(T::ANUDATTA) && c.upadha == "f":
            op.optional(op.mit, "6.1.59", p, c, "a")
        }
    }


fn run_before_vikarana(p: Prakriya):
    dhatu_adesha_before_vikarana(p)


fn run_before_dvitva(p: Prakriya):
    """Replace the dhAtu based on the following suffix.

    These rules must run after the vikarana is added and before dvitva.
    """
    dhatu_adesha_after_vikarana(p)

    for i, _ in enumerate(p.terms):
        aa_adesha(p, i)


fn am_agama(p: Prakriya):
    for i, _ in enumerate(p.terms):
        am_agama_for_term(p, i)
*/
