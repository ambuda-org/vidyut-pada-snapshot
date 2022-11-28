//! ac_sandhi
//! =========
//! (6.1.66 - 6.1.101)

use crate::char_view::{char_rule_legacy, get_at, set_at, xy2};
use crate::constants::Tag as T;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use lazy_static::lazy_static;

/// Runs various general rules of vowel sandhi.
fn apply_general_ac_sandhi(p: &mut Prakriya) {
    lazy_static! {
        static ref A: SoundSet = s("a");
        static ref AK: SoundSet = s("ak");
        static ref IK: SoundSet = s("ik");
        static ref AC: SoundSet = s("ac");
        static ref EC: SoundSet = s("ec");
        static ref VAL: SoundSet = s("val");
    }

    char_rule_legacy(
        p,
        |p, x, y, i, _| {
            let vyor_vali = (x == 'v' || x == 'y') && VAL.contains_char(y);
            let t = get_at(p, i).expect("should be present");
            // Ignore if it starts an upadesha, otherwise roots like "vraj" would by vyartha.
            // Likewise for roots ending with 'v'.
            // For now, just check if the term is a dhatu.
            let is_upadesha = t.has_tag(T::Dhatu);
            vyor_vali && !is_upadesha
        },
        |p, _, _, i, _| {
            set_at(p, i, "");
            p.step("6.1.66");
        },
    );

    char_rule_legacy(
        p,
        xy2(|x, y| x == 'a' && al::is_guna(y)),
        |p, _, _, i, _| {
            set_at(p, i, "");
            p.step("6.1.97");
        },
    );

    char_rule_legacy(
        p,
        xy2(|x, y| EC.contains_char(x) && AC.contains_char(y)),
        |p, x, _, i, _| {
            let sub = match x {
                'e' => "ay",
                'E' => "Ay",
                'o' => "av",
                'O' => "Av",
                _ => panic!("Unexpected sub"),
            };
            set_at(p, i, sub);
            p.step("6.1.78");
        },
    );

    char_rule_legacy(
        p,
        xy2(|x, y| AK.contains_char(x) && AK.contains_char(y) && al::savarna(x).contains_char(y)),
        |p, x, _, i, j| {
            set_at(p, j, "");
            set_at(p, i, &al::to_dirgha(x).expect("should be ac").to_string());
            p.step("6.1.101");
        },
    );

    char_rule_legacy(
        p,
        xy2(|x, y| IK.contains_char(x) && AC.contains_char(y)),
        |p, x, _, i, _| {
            let res = match x {
                'i' | 'I' => "y",
                'u' | 'U' => "v",
                'f' | 'F' => "r",
                'x' | 'X' => "l",
                _ => panic!("Unexpected res"),
            };
            set_at(p, i, res);
            p.step("6.1.77");
        },
    );

    char_rule_legacy(
        p,
        xy2(|x, y| A.contains_char(x) && AC.contains_char(y)),
        |p, _, y, i, j| {
            if EC.contains_char(y) {
                set_at(p, j, al::to_vrddhi(y).expect("should be set"));
                set_at(p, i, "");
                p.step("6.1.88");
            } else {
                set_at(p, j, al::to_guna(y).expect("should be set"));
                set_at(p, i, "");
                p.step("6.1.87");
            }
        },
    );
}

fn sup_sandhi_before_angasya(p: &mut Prakriya) {
    let y = p.terms().len() - 1;
    if p.has(y, |t| !t.has_tag(T::Sup)) {
        return;
    }
    let x = y - 1;

    if p.has(x, |t| t.has_antya('o')) || p.has(y, f::u_in(&["am", "Sas"])) {
        p.set(x, op::antya("A"));
        p.set(y, op::adi(""));
        p.step("6.1.93");
    }
}

fn sup_sandhi_after_angasya(p: &mut Prakriya) {
    /*
    // Program cannot model "antAdivacca" so split the rule.
    let y = p.terms().len() - 1;
    if !p.has(y, |t| t.has_tag(T::Sup)) {
        return;
    }
    let x = y - 1;

    let mut base = p.terms()[x];
    let mut sup = p.terms()[y];

    if base.has_antya(&s("ak")) && sup.any(&[T::V1, T::V2]) {
        if sup.text == "am" {
            p.op("6.1.107", op::t(y, op::adi("")));
        } else if base.has_antya(&s("a")) && sup.has_adi(&s("ic")) {
            p.step("6.1.104");
        } else if base.antya() and (sup.has_adi(&s("ic")) || sup.has_u("jas")) {
            p.step("6.1.105");
        } else if sup.has_adi(&s("ac")) {
            antya = c.antya
            c.text = c.text[:-1]
            op.adi("6.1.102", p, n, sounds.dirgha(antya))

            if n.u == "Sas" and c.all(T.PUM) {
                p.op("6.1.103", op::t(y, op::antya("n")));
            }
        }
    } else if p.has(y, f::u_in(&["Nasi~", "Nas"])) {
        if p.has(x, |t| t.has_antya(&s("eN"))) {
            p.op("6.1.110", op::t(s, op::adi("")));
        } else if p.has(x, |t| t.has_antya('f')) {
            c.text = c.text[:-1] + "ur";
            p.op("6.1.110", op::t(y, op::adi("")));
        }
    }
    */
}

/// Runs vowel sandhi rules that apply between terms (as opposed to between sounds).
fn apply_ac_sandhi_at_term_boundary(p: &mut Prakriya, i: usize) {
    /*
    // TODO: NI, Ap
    // Check for Agama to avoid lopa on yAs + t.
    if (
        c.antya in s("hal")
        && n
        && len(n.text) == 1
        && n.u in ("su~", "tip", "sip")
        && not c.all(T.AGAMA)
    ) {
        op.antya("6.1.68", p, n, "")
    }

    if (c.antya in sounds.HRASVA or c.antya in s("eN")) && p.terms[-1].any(
        T.SAMBUDDHI
    ) {
        op.lopa("6.1.69", p, p.terms[-1])
    }

    if c.antya in s("a") && n.text == "us" {
        op.antya("6.1.96", p, c, "")

    // ekaH pUrvapara (6.1.84)

    } else if c.u == "Aw" && n.adi in s("ik") {
        c.text = ""
        op.adi("6.1.90", p, n, sounds.vrddhi(n.adi))
    }
    */
}

fn run_common(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        apply_ac_sandhi_at_term_boundary(p, i);
    }

    apply_general_ac_sandhi(p);

    /*
    for i, c in enumerate(p.terms) {
        try {
            n = p.terms[i + 1]
        except IndexError {
            continue
        // HACK: duplicate 6.4.92 from the asiddhavat section for ci -> cAy, cap
        if c.all("m") and n.u in {"Ric", "pu~k"} and c.text in {"cAy", "cA"} {
            if c.text == "cA" {
                p.op("6.4.92", op::t(i, op::antya("a")));
            } else {
                p.op("6.4.92", op::t(i, op::upadha("a")));
            }
        }
    }
    */
}

pub fn run(p: &mut Prakriya) {
    run_common(p);
    sup_sandhi_before_angasya(p);
    sup_sandhi_after_angasya(p);
}
