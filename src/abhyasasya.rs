/*!
abhyasasya
==========
(7.4.58 - end of 7.4)

Rules that modify the abhyāsa.
*/

use crate::constants::Tag as T;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{map_sounds, s, SoundMap, SoundSet};
use lazy_static::lazy_static;

lazy_static! {
    static ref SHAR: SoundSet = s("Sar");
    static ref KHAY: SoundSet = s("Kay");
    static ref HAL: SoundSet = s("hal");
    static ref F_HAL: SoundSet = s("f hal");
    static ref KUH_CU: SoundMap = map_sounds("ku~ h", "cu~");
}

/*
fn _has_following_san(p: Prakriya, needle: Term) -> bool:
    seen_needle = False
    for t in p.terms:
        if needle is t:
            seen_needle = True
            continue
        if seen_needle:
            if t.u == "san":
                return True
            } else if  not t.any(T.DHATU, T.AGAMA):
                return False
            else:
                continue
    return False
*/

/// Simplifies the abhyasa per 7.4.60.
fn try_haladi(text: &str) -> String {
    let mut ret = String::new();
    for (i, c) in text.chars().enumerate() {
        if al::is_hal(c) {
            if i == 0 {
                ret.push(c);
            }
        } else {
            ret.push(c);
            break;
        }
    }
    ret
}

/// Simplifies the absyasa per 7.4.61.
fn try_shar_purva(text: &str) -> String {
    let mut ret = String::new();
    for (i, c) in text.chars().enumerate() {
        if i == 0 {
            assert!(SHAR.contains_char(c));
        } else if KHAY.contains_char(c) {
            ret.push(c)
        } else if al::is_ac(c) {
            ret.push(c);
            break;
        }
    }
    ret
}

/*
fn run_sani_cani_for_each(p: Prakriya, c: Term, dhatu: Term):
    # san and sanvat changes
    abhyasta_index, _ = p.find_last(T.ABHYASTA)
    laghu_cani = (
        # caN-pare
        p.terms[abhyasta_index].u in ("Ric", "RiN")
        and p.find(lambda x: x.u == "caN")
        # laghuni
        and f.is_laghu(dhatu)
        # an-ak-lope
        and not dhatu.any(T.F_AT_LOPA)
    )

    sanvat = laghu_cani or _has_following_san(p, c)
    if sanvat:
        if laghu_cani and dhatu.text in {
            "smf",
            "dF",
            "tvar",
            "praT",
            "mrad",
            "stF",
            "spaS",
        }:
            op.antya("7.4.95", p, c, "a")
            return

        } else if  c.antya == "a":
            op.antya("7.4.79", p, c, "i")
        } else if  (
            dhatu.adi in s("pu~ yaR j")
            and len(dhatu.text) >= 2
            and dhatu.text[1] == "a"
        ):
            op.antya("7.4.80", p, c, "i")
        # TODO: 7.4.81

    # TODO: 7.4.95
    if laghu_cani:
        if not f.samyogadi(dhatu):
            op.antya("7.4.94", p, c, sounds.dirgha(c.antya))

    # TODO: scope of this? Sarvadhatuka only?
    if dhatu.u in MAN_BADHA:
        op.antya("3.1.6", p, c, sounds.dirgha(c.antya))


fn run_sani_cani(p: Prakriya):
    for i, t in enumerate(p.terms):
        if not t.any(T.ABHYASA):
            continue

        dhatu = p.terms[i + 1]
        run_sani_cani_for_each(p, t, dhatu)
*/

/*

operation
rule
optional

op::do("1.2.3", op::args))
op::optional(|| op::do("1.2.3", args))
op::optional("1.2.3", || op::do("1.2.3", args))
op::optional("1.2.3", op::do, args))

optionality suggests:
- one reference to the rule
- "wrapping" of the rule (`if rule.is_allowed ...`)
--> implies a separation of meta (p.op vs p.op_optional) and operation

p.op
p.op_optional
p.op_then
p.op_optional_then

*/

/// Runs abhyasa rules specific to liT.
///
/// Example: bu + BU + va -> baBUva.
///
/// (7.4.70 - 7.4.74)
fn try_rules_for_lit(p: &mut Prakriya, i: usize, i_dhatu: usize) {
    let abhyasa = &p.terms()[i];
    let last = p.terms().last().unwrap();

    if last.has_lakshana("li~w") {
        if abhyasa.text == "a" {
            op::text2("7.4.70", p, i, "A");
            // From the Kashika-vrtti:
            //
            //     ṛkāraikadeśo repho halgrahaṇena gṛhyate, tena iha api dvihalo
            //     'ṅgasya nuḍāgamo bhavati. ānṛdhatuḥ, ānṛdhuḥ.
            //
            // if HAL.contains(dhatu.antya()) && (h
            let dhatu = &p.terms()[i_dhatu];
            if dhatu.has_antya(&*HAL) && dhatu.has_upadha(&*F_HAL) {
                // 'A' acepted only by some grammarians
                if dhatu.has_adi('A') {
                    p.op_optional("7.4.71.k", |p| op::insert_agama(p, i + 1, "nu~w"));
                } else {
                    p.op("7.4.71", |p| op::insert_agama(p, i + 1, "nu~w"));
                }
            // For aSnoti only, not aSnAti
            } else if dhatu.text == "aS" && dhatu.gana == Some(5) {
                p.op("7.4.72", |p| op::insert_agama(p, i + 1, "nu~w"));
            }
        } else if p.has(i_dhatu, |t| {
            t.text == "BU" && (t.gana == Some(1) || t.gana == Some(2))
        }) {
            // gana 1 for `BU`, gana 2 for `as` replaced by `BU`.
            op::text2("7.4.73", p, i, "ba");
        }
    }
}

/*
if la.all("li~w"):
    if c.text == "a":
    # 2 is for as -> bhU
    } else if  dhatu.text == "BU" and dhatu.gana in (1, 2):
        op.text("7.4.73", p, c, "ba")
    # TODO: 7.4.74
    */

pub fn run(p: &mut Prakriya) {
    let i = match p.find_first(T::Abhyasa) {
        Some(i) => i,
        None => return,
    };

    // TODO: expand for abhyasa after dhatu.
    let i_dhatu = i + 1;
    if !p.has(i_dhatu, f::dhatu) {
        return;
    }

    let dhatu = &p.terms()[i_dhatu];
    let abhyasa = &p.terms()[i];
    let last = p.terms().last().unwrap();

    if dhatu.text == "dyut" {
        p.op_term("7.4.67", i_dhatu, op::text("dit"));
    } else if dhatu.text == "vyaT" && last.has_lakshana("li~w") {
        p.op_term("7.4.68", i_dhatu, op::text("viT"));
    } else if SHAR.contains_opt(abhyasa.adi()) && KHAY.contains_opt(abhyasa.get(1)) {
        let mut abhyasa = &mut p.terms_mut()[i];
        let res = try_shar_purva(&abhyasa.text);
        if res != abhyasa.text {
            abhyasa.text = res;
            p.step("7.4.61");
        }
    } else {
        let mut abhyasa = &mut p.terms_mut()[i];
        let res = try_haladi(&abhyasa.text);
        if res != abhyasa.text {
            abhyasa.text = res;
            p.step("7.4.60");
        }
    }

    let abhyasa = &p.terms()[i];
    if KUH_CU.contains_key(&abhyasa.adi().unwrap()) {
        if let Some(val) = KUH_CU.get(&abhyasa.adi().unwrap()) {
            p.op_term("7.4.62", i, op::adi(&val.to_string()));
        }
    }

    let abhyasa = &p.terms()[i];
    if al::is_dirgha(abhyasa.antya().unwrap()) {
        let val = al::to_hrasva(abhyasa.antya().unwrap()).unwrap();
        p.op_term("7.4.62", i, op::antya(&val.to_string()));
    }

    if p.has(i, |t| t.has_antya('f')) {
        p.op_term("7.4.66", i, op::antya("a"));
    }

    let dhatu = &p.terms()[i_dhatu];
    let last = p.terms().last().unwrap();
    if dhatu.has_u("i\\R") && last.has_tag(T::kit) {
        p.op_term("7.4.69", i, op::adi("I"));
    }

    try_rules_for_lit(p, i, i_dhatu);

    /*
    # Slu changes
    if p.find(lambda x: x.all(T.SLU)):
        if dhatu.text in ("nij", "vij", "viz"):
            op.antya("7.4.75", p, c, sounds.guna(c.antya))
        } else if  dhatu.u in ("quBf\\Y", "mA\\N", "o~hA\\N"):
            op.antya("7.4.76", p, c, "i")
        } else if  dhatu.text in ("f", "pf", "pF"):
            op.antya("7.4.77", p, c, "i")
        # TODO: 7.4.78


     */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_haladi() {
        assert_eq!(try_haladi("BU"), "BU");
        assert_eq!(try_haladi("i"), "i");
        assert_eq!(try_haladi("kram"), "ka");
    }

    #[test]
    fn test_try_shar_purva() {
        assert_eq!(try_shar_purva("sTA"), "TA");
        assert_eq!(try_shar_purva("Scyut"), "cu");
        assert_eq!(try_shar_purva("sparD"), "pa");
    }
}
