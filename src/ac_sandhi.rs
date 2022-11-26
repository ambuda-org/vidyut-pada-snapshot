//! ac_sandhi
//! =========
//! (6.1.66 - 6.1.101)

use crate::constants::Tag as T;
use crate::dhatu_gana::{DYUT_ADI, PUSH_ADI, TAN_ADI};
use crate::filters as f;
use crate::it_samjna;
use crate::operations as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use lazy_static::lazy_static;
use std::error::Error;

/*
fn sup_sandhi_before_angasya(p: Prakriya) {
    n = p.terms[-1]
    if not n.all(T.SUP) {
        return
    }
    c = p.terms[-2]

    if c.antya == "o" and n.u in {"am", "Sas"} {
        n.text = n.text[1:]
        op.antya("6.1.93", p, c, "A")
    }
}
*/

/// Replaces character `i` of the current prakriya with the given substitute.
fn set_at(p: &mut Prakriya, index: usize, substitute: &str) {
    let mut cur = 0;
    for t in p.terms() {
        let delta = t.text.len();
        if (cur..cur + delta).contains(&index) {
            let t_offset = index - cur;
            t.text = String::from(&t.text[..t_offset]) + substitute + &t.text[t_offset + 1..];
            return;
        } else {
            cur += delta;
        }
    }
}

fn char_rule(
    p: &mut Prakriya,
    f: impl Fn(char, char) -> bool,
    op: impl Fn(&mut Prakriya, char, char, usize, usize),
) {
    loop {
        let text = p.text();
        let mut applied_rule = false;

        for i in 0..text.len() {
            let j = i + 1;
            // Set up windowed iteration of characters.
            let x = text.as_bytes().get(i);
            let y = text.as_bytes().get(j);
            let (x, y) = match (x, y) {
                (Some(a), Some(b)) => (*a as char, *b as char),
                _ => continue,
            };

            if f(x, y) {
                op(p, x, y, i, j);
                applied_rule = true;
                break;
            }
        }

        if !applied_rule {
            break;
        }
    }
}

/// Runs various general rules of vowel sandhi.
fn general_vowel_sandhi(p: &mut Prakriya) {
    lazy_static! {
        static ref A: SoundSet = s("a");
        static ref AK: SoundSet = s("ak");
        static ref IK: SoundSet = s("ik");
        static ref AC: SoundSet = s("ac");
        static ref EC: SoundSet = s("ec");
        static ref VAL: SoundSet = s("val");
    }

    /*
    char_rule(
        p,
        |x, y| (x == 'v' || x == 'y') && VAL.contains_char(y),
        |p, _, _, i, _| {
            set_at(p, i, "");
            p.step("6.1.97");
        },
    );
    */

    char_rule(
        p,
        |x, y| x == 'a' && al::is_guna(y),
        |p, _, _, i, _| {
            set_at(p, i, "");
            p.step("6.1.97");
        },
    );

    char_rule(
        p,
        |x, y| EC.contains_char(x) && AC.contains_char(y),
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

    char_rule(
        p,
        |x, y| AK.contains_char(x) && AK.contains_char(y) && al::savarna(x).contains_char(y),
        |p, x, _, i, j| {
            set_at(p, j, "");
            set_at(p, i, &al::to_dirgha(x).to_string());
            p.step("6.1.101");
        },
    );

    char_rule(
        p,
        |x, y| EC.contains_char(x) && AC.contains_char(y),
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

    char_rule(
        p,
        |x, y| A.contains_char(x) && AC.contains_char(y),
        |p, _, y, i, j| {
            if EC.contains_char(y) {
                set_at(p, j, al::to_vrddhi(y));
                set_at(p, i, "");
                p.step("6.1.88");
            } else {
                set_at(p, j, al::to_guna(y));
                set_at(p, i, "");
                p.step("6.1.87");
            }
        },
    );
}

/*
fn sup_sandhi_after_angasya(p: &mut Prakriya) {
    // Program cannot model "antAdivacca" so split the rule.
    n = p.terms[-1]
    if not n.all(T.SUP) {
        return
    }

    c = p.terms[-2]

    if c.antya in s("ak") and n.any(T.V1, T.V2) {
        if n.text == "am" {
            op.adi("6.1.107", p, n, "")
        } else if  c.antya in s("a") and n.adi in s("ic") {
            p.step("6.1.104")
        } else if  c.antya in sounds.DIRGHA and (n.adi in s("ic") or n.u == "jas") {
            p.step("6.1.105")
        } else if  n.adi in s("ac") {
            antya = c.antya
            c.text = c.text[:-1]
            op.adi("6.1.102", p, n, sounds.dirgha(antya))

            if n.u == "Sas" and c.all(T.PUM) {
                op.antya("6.1.103", p, n, "n")
            }
        }

    } else if  n.u in {"Nasi~", "Nas"} {
        if c.antya in s("eN") {
            op.adi("6.1.110", p, n, "")
        } else if  c.antya == "f" {
            c.text = c.text[:-1] + "ur"
            op.adi("6.1.110", p, n, "")
        }
    }
}
*/

/// Runs vowel sandhi rules that apply between terms (as opposed to between sounds).
fn run_for_term(p: &mut Prakriya, i: usize) {
    /*
        terms = p.terms
        c = terms[index]

        // Ignore this case if it starts an upadesha, otherwise roots like "vraj"
        // would by vyartha. Likewise for roots ending with 'v'
        // TODO: handle term boundaries more elegantly
        val = s("val")
        s_val = "".join(val.items)
        re_vyor_vali = f"[vy]([{s_val}])"
        if re.search(re_vyor_vali, c.text) and not c.all(T.DHATU) {
            c.text = re.sub(re_vyor_vali, r"\1", c.text)
            p.step("6.1.66")

        try {
            n = [u for u in terms[index + 1 :] if u.text][0]
        except IndexError {
            return

        if c.antya in s("v y") and n.adi in val and not c.all(T.DHATU) {
            op.antya("6.1.66", p, c, "")
        }

        // TODO: NI, Ap
        // Check for Agama to avoid lopa on yAs + t.
        if (
            c.antya in s("hal")
            and n
            and len(n.text) == 1
            and n.u in ("su~", "tip", "sip")
            and not c.all(T.AGAMA)
        ) {
            op.antya("6.1.68", p, n, "")
        }

        if (c.antya in sounds.HRASVA or c.antya in s("eN")) and p.terms[-1].any(
            T.SAMBUDDHI
        ) {
            op.lopa("6.1.69", p, p.terms[-1])
        }

        if c.antya in s("a") and n.text == "us" {
            op.antya("6.1.96", p, c, "")

        // ekaH pUrvapara (6.1.84)

        } else if c.u == "Aw" and n.adi in s("ik") {
            c.text = ""
            op.adi("6.1.90", p, n, sounds.vrddhi(n.adi))
        }
    */
}

fn run_common(p: &mut Prakriya) {
    /*
    for i in 0..p.terms().len() {
        run_for_term(p, i)
    }
    */

    general_vowel_sandhi(p)

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
    run_common(p)
    /*
    sup_sandhi_before_angasya(p)
    sup_sandhi_after_angasya(p)
    */
}
