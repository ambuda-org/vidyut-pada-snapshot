/*!
it_agama
========
(7.2.8 - 7.2.78)

Various Sanskrit words have an "i" vowel inserted between the dhātu and the pratyaya. This "i" is
called *iṭ*. Roots use iṭ in one of three patterns:

- Roots that generally use iṭ are called *seṭ* (sa-iṭ).
- Roots that generally avoid iṭ are called *aniṭ* (an-iṭ).
- Roots that optionally use iṭ are called *veṭ* (vā-iṭ).

This prakaraṇa fully specifies the rules that add the iṭ-āgama to the prakriyā.

Order of operations:
- must run before `dvitva` for `undidizati`, etc.
- must run after `vikarana` since it checks for `sya`, `si~c`, etc.
*/

use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operations as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref VAL: SoundSet = s("val");
    static ref VASH: SoundSet = s("vaS");
    static ref UK: SoundSet = s("uk");
}

#[derive(Debug, Eq, PartialEq)]
enum It {
    Set(Rule),
    Anit(Rule),
    None,
}

fn is_hacky_eka_ac(t: &Term) -> bool {
    // HACK to have ekac apply for am-Agama
    f::is_eka_ac(t) || t.text.contains("fa")
}

fn add_it(rule: Rule, p: &mut Prakriya, i: usize) {
    let agama = Term::make_agama("iw");
    p.insert_after(i, agama);
    p.step(rule);
    it_samjna::run(p, i + 1).unwrap();
}

/// Runs general rules that prevent iT-Agama. Returns whether the iT-Agama procedure is complete.
///
/// (7.2.8 - 7.2.34)
fn try_general_anit(p: &mut Prakriya, i: usize) -> bool {
    let d = &p.terms()[i];
    let n = match p.view(i) {
        Some(x) => x,
        None => return false,
    };

    let mut it = It::None;
    let sri_uk = d.text == "Sri" || UK.contains_opt(d.antya());
    if n.has_tag(T::Krt) && !VASH.contains_opt(n.adi()) {
        it = It::Anit("7.2.8");
    } else if is_hacky_eka_ac(d) && sri_uk && n.has_tag(T::Kit) {
        it = It::Anit("7.2.11");
    } else if n.has_u("san") && d.has_text(&["Sri", "grah", "guh"]) {
        it = It::Anit("7.2.12");
    } else if n.has_tag(T::Nistha) {
        if d.text == "Svi" || d.has_tag(T::Idit) {
            it = It::Anit("7.2.14");
        } else if d.has_tag(T::Adit) {
            let code = "7.2.17";
            if p.any(&[T::Bhave, T::Karmani]) && p.is_allowed(code) {
            } else {
                p.decline(code);
                it = It::Anit("7.2.16");
            }
        }
        // TODO: 7.2.15
    }
    // TODO: 7.2.18 - 7.2.34

    match it {
        It::Anit(code) => {
            p.step(code);
            true
        }
        It::None | It::Set(_) => false,
    }
}

/// Runs iT rules specific to liT. Returns whether the iT-Agama procedure is complete.
fn try_lit_it(p: &mut Prakriya, i: usize) -> bool {
    let n = match p.view(i) {
        Some(x) => x,
        None => return false,
    };

    if !n.has_lakshana("li~w") {
        return false;
    }

    let mut it = It::None;

    let rule_7_2_10 = p.has(i, |t| t.has_tag(T::Anudatta) && is_hacky_eka_ac(t));

    let anga = &p.terms()[i];
    // These rules are always aniT.
    if anga.has_text(&["kf", "sf", "Bf", "vf", "stu", "dru", "sru", "Sru"]) {
        it = It::Anit("7.2.13");
    } else if anga.has_antya(&*AC) && n.has_u("Tal") && rule_7_2_10 {
        // Concise summary of rules:
        // - The roots in 7.2.13 are aniT. All others are seT by valAdi (7.2.35).
        // - However, there are the following exceptions for Tal:
        //   - roots ending in `f` (except `f`) are aniT.
        //   - roots ending in a vowel and roots with a middle 'a' are veT.

        // 7.2.63 Rto bhAradvAjasya
        // In Bharadvaja's opinion, rule 7.2.61 applies only for final R. So for all
        // other roots, this condition is optional:
        if !anga.has_antya('f') {
            let code = "7.2.63";
            if p.is_allowed(code) {
                it = It::Set(code);
            } else {
                p.decline(code);
                it = It::Anit("7.2.61");
            }
        // But for other anit roots, the condition is obligatory.
        } else if anga.has_u("f\\") {
            it = It::Set("7.2.66");
        } else {
            it = It::Anit("7.2.61");
        }
    } else if anga.has_text(&["sfj", "dfS"]) && n.has_u("Tal") {
        // By default, these will be seT. So the option allows aniT.
        let code = "7.2.65";
        if p.is_allowed(code) {
            it = It::Anit(code);
        } else {
            p.decline(code);
        }
    }

    match it {
        It::None => {
            // The effect of 7.2.13 is that all other roots are considerd `sew` by
            // default.
            p.step("7.2.13");
            let n = p.view(i).unwrap();
            if VAL.contains_opt(n.adi()) {
                add_it("7.2.35", p, i);
            }
        }
        It::Anit(code) => {
            p.step(code);
        }
        It::Set(code) => {
            add_it(code, p, i);
        }
    }

    true
}

fn optional_set(rule: Rule, p: &mut Prakriya) -> It {
    if p.is_allowed(rule) {
        It::Set(rule)
    } else {
        p.decline(rule);
        It::None
    }
}

fn optional_anit(rule: Rule, p: &mut Prakriya) -> It {
    if p.is_allowed(rule) {
        It::Anit(rule)
    } else {
        p.decline(rule);
        It::None
    }
}

/// Runs iT rules that condition on a following ArdhadhAtuka suffix.
///
/// (7.2.35 - 7.2.36 and 7.2.41 - 7.2.75)
fn try_ardhadhatuke(p: &mut Prakriya, i: usize) -> bool {
    let n = match p.view(i + 1) {
        Some(x) => x,
        None => {
            return false;
        }
    };
    if !n.has_tag(T::Ardhadhatuka) {
        return false;
    }

    let mut it = It::None;
    let anga = &p.terms()[i];

    // Special cases
    let mut add_sak = false;
    if (anga.has_antya('f') || anga.text == "han") && n.has_u("sya") {
        it = It::Set("7.2.70");
    } else if n.has_u("si~c") {
        if anga.text == "aYj" {
            it = It::Set("7.2.71");
        } else if n.has_tag(T::Parasmaipada) {
            if anga.has_u_in(&["zwu\\Y", "zu\\Y", "DUY"]) {
                it = It::Set("7.2.72");
            } else if anga.has_text(&["yam", "ram", "nam"]) {
                add_sak = true;
                it = It::Set("7.2.73");
            } else if anga.has_antya('A') {
                // Handle this after running Attva. See the run_after_attva function for details.
                return false;
            }
        }
    } else if anga.text == "IS" && n.adi() == Some('s') {
        add_it("7.2.77", p, i);
        return false;
    } else if anga.has_text(&["Is", "Iq", "jan"])
        && (n.adi() == Some('s') || n.last().unwrap().has_u("Dvam"))
    {
        // See kAshika on 7.2.78 for inclusion of IS here.
        add_it("7.2.78", p, i);
        return false;
    }

    let antya_para = n.has_tag(T::Parasmaipada);
    let se = n.adi() == Some('s');
    let krta_crta = &["kft", "cft", "Cfd", "tfd", "nft"];
    let ishu_saha = &["izu~", "zaha~\\", "luBa~", "ruza~", "riza~"];

    if matches!(it, It::Set(_) | It::Anit(_)) {
        // Do nothing
    } else if anga.has_u_in(gana::RADH_ADI) && VAL.contains_opt(n.adi()) {
        // All of these roots are in scope for 7.2.10 (aniT).
        // So, this option allows seT.
        it = optional_set("7.2.45", p);
    } else if anga.has_u_in(ishu_saha) && n.adi() == Some('t') {
        it = optional_anit("7.2.48", p);
    } else if anga.has_u_in(krta_crta) && se && !n.has_u("si~c") {
    } else if anga.text == "gam" && antya_para && se {
        it = It::Set("7.2.58");
    } else if anga.has_u_in(gana::VRDBHYAH) && anga.gana == Some(1) && antya_para && se {
        it = It::Anit("7.2.59");
    } else if anga.has_u("kfpU~\\") && antya_para && (se || n.has_u("tAsi~")) {
        it = It::Anit("7.2.60");
    }

    // TODO: not sure ...
    //
    // General cases

    let anga = &p.terms()[i];
    let n = p.view(i + 1).expect("");
    if matches!(it, It::Set(_) | It::Anit(_)) {
        // Do nothing
    } else if anga.has_tag(T::Anudatta) && is_hacky_eka_ac(anga) && !n.has_lakshana("li~w") {
        // 7.2.10 is a niyama to the general rule, which applies only to
        // ArdhadhAtuka suffixes. So we add a check for ArdhadhAtukatva here.
        //
        // Any li~w root not explictly included in 7.2.13 is also iT.
        it = It::Anit("7.2.10");
    } else if VAL.contains_opt(n.adi()) && n.has_tag(T::Ardhadhatuka) {
        it = It::Set("7.2.35");
    }

    match it {
        It::Anit(code) => {
            p.step(code);
            true
        }
        It::Set(code) => {
            add_it(code, p, i);
            false
        }
        It::None => false,
    }
}
/*
    // TODO: not sure I undesrtand the scope of this rule.
    } else if c.text in {"snu", "kram"} and n.adi in s("val"):
        if p.terms[-1].all(T.ATMANEPADA) and n.terms[0].u == "sIyu~w":
            anit_rule = "7.2.36"

    // Optional rules (Udit and others)

    if anit_rule or set_rule:
        pass
    } else if n.adi in s("val"):
        if c.u in ("svf", "zUN", "DUY") or c.any("U"):
            // Synchronize choice of "it" with the choice of lun-vikarana.
            if p.all(T.F_ANIT_KSA):
                anit_rule = "7.2.44"
            } else if p.all(T.F_SET_SIC):
                pass
            else:
                anit_rule = optional_rule("7.2.44", p)
        } else if (n.any("li~N") or n.u == "si~c") and p.terms[-1].any(T.ATMANEPADA):
            vrt = c.text == "vf" or c.antya == "F"
            if vrt and n.any(T.ARDHADHATUKA):
                // By default, all of these roots are seT.
                // So, the option allows anit.
                anit_rule = optional_rule("7.2.42", p)
            } else if c.antya == "f" and f.samyogadi(c):
                if c.all(T.ANUDATTA):
                    // For anit roots, optional seT.
                    set_rule = optional_rule("7.2.43", p)
                else:
                    // For seT roots, optional aniT.
                    anit_rule = optional_rule("7.2.43", p)

    // General cases

*/

fn try_sarvadhatuke(p: &mut Prakriya, i: usize) -> bool {
    let n = match p.view(i) {
        Some(x) => x,
        None => return false,
    };

    let anga = &p.terms()[i];
    let rudh_adi = &["rudi~r", "Yizva\\pa~", "Svasa~", "ana~", "jakza~"];
    if VAL.contains_opt(n.adi()) && n.has_tag(T::Sarvadhatuka) && anga.has_u_in(rudh_adi) {
        add_it("7.2.76", p, i);
        true
    } else {
        false
    }
}

/*
fn it_dirgha(p: &mut Prakriya, c: Term, n: TermView):
    """Rules that lengthen the iṭ.

    (7.2.37 - 7.2.40)
    """

    it = n.terms[0]
    la = p.terms[-1]

    if not la.any("li~w"):
        if c.text == "grah":
            it.text = "I"
            p.step("7.2.37")
        } else if c.antya == "F" or c.text == "vf":
            if la.any("li~N"):
                p.step("7.2.39")
            } else if any(x.u == "si~c" for x in n.terms) and la.any(T.PARASMAIPADA):
                p.step("7.2.40")
            else:
                op.optional(op.text, "7.2.38", p, it, "I")
*/

pub fn run_before_attva(p: &mut Prakriya) {
    // The abhyasa might come second, so match on it specifically.
    let i = match p.find_last_where(f::tag_in(&[T::Dhatu, T::Abhyasa])) {
        Some(i) => i,
        None => return,
    };

    if try_lit_it(p, i) {
        return;
    }
    if try_general_anit(p, i) {
        return;
    }
    if try_ardhadhatuke(p, i) {
        return;
    }
    if try_sarvadhatuke(p, i) {
        return;
    }

    /*
    n = TermView.make_pratyaya(p, index)
    if not n:
        return
    it = n.terms[0]
    if f.is_it_agama(it):
        it_dirgha(p, c, n)
    */
}

/*
fn run_after_attva_for_index(p: &mut Prakriya, i: usize):
    c = p.terms[index]
    n = TermView.make_pratyaya(p, index)
    if not n or not n.all(T.ARDHADHATUKA):
        return

    if n.terms[0].u == "si~c":
        para = p.terms[-1].all(T.PARASMAIPADA)
        if para:
            if c.antya == "A" and n.adi in s("val"):
                c.text += "s"
                op.insert_agama_after("7.2.73", p, index, "iw")
*/

/*
fn run_after_attva(p: &mut Prakriya):
    for index, _ in enumerate(p.terms):
        run_after_attva_for_index(p, index)
*/
