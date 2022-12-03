/*!
abhyasasya
==========
(7.4.58 - end of 7.4)

Rules that modify the abhyāsa.
*/

use crate::constants::Tag as T;
use crate::filters as f;
use crate::it_samjna;
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

/// Runs abhyasa rules conditioned on either `san` or `caN`.
///
/// Constraints:
/// - must follow 7.4.1 etc. which change the dhatu vowel before `caN`.
/// - must follow guna of the dhatu vowel, which affects 7.4.1 etc. above.
pub fn run_for_sani_cani(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Abhyasa)?;
    let i_abhyasta = p.find_last(T::Abhyasta)?;
    let anga = p.get(i_abhyasta)?;

    let is_ni = anga.has_u_in(&["Ric", "RiN"]);
    let is_cani = p.has(i_abhyasta + 1, f::u("caN"));
    let is_laghu = f::is_laghu(anga);
    let has_at_lopa = anga.has_tag(T::FlagAtLopa);
    let is_laghu_cani = is_ni && is_laghu && is_cani && !has_at_lopa;

    let is_sanvat = is_laghu_cani || p.find_next_where(i, f::u("san")).is_some();
    if is_sanvat {
        if is_laghu_cani && anga.has_text_in(&["smf", "dF", "tvar", "praT", "mrad", "stF", "spaS"])
        {
            p.op_term("7.4.95", i, op::antya("a"));
        } else if anga.has_antya('a') {
            p.op_term("7.4.79", i, op::antya("i"));
        }
    }

    Some(())
}

/*
    if sanvat:
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
*/

/// Runs abhyasa rules that apply generally.
fn try_general_rules(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_dhatu = i + 1;
    let dhatu = p.get(i_dhatu)?;
    let abhyasa = p.get(i)?;
    let last = p.terms().last()?;

    if dhatu.text == "dyut" {
        p.op_term("7.4.67", i_dhatu, op::text("dit"));
    } else if dhatu.text == "vyaT" && last.has_lakshana("li~w") {
        // TODO: move this to `try_rules_for_lit`?
        p.op_term("7.4.68", i_dhatu, op::text("viT"));
    } else if SHAR.contains_opt(abhyasa.adi()) && KHAY.contains_opt(abhyasa.get(1)) {
        let mut abhyasa = &mut p.get_mut(i)?;
        let res = try_shar_purva(&abhyasa.text);
        if res != abhyasa.text {
            abhyasa.text = res;
            p.step("7.4.61");
        }
    } else {
        let mut abhyasa = &mut p.get_mut(i)?;
        let res = try_haladi(&abhyasa.text);
        if res != abhyasa.text {
            abhyasa.text = res;
            p.step("7.4.60");
        }
    }

    let abhyasa = p.get(i)?;
    if let Some(val) = KUH_CU.get(&abhyasa.adi()?) {
        p.op_term("7.4.62", i, op::adi(&val.to_string()));
    }

    let abhyasa = p.get(i)?;
    if al::is_dirgha(abhyasa.antya()?) {
        let val = al::to_hrasva(abhyasa.antya()?)?;
        p.op_term("7.4.62", i, op::antya(&val.to_string()));
    }

    if p.has(i, |t| t.has_antya('f')) {
        p.op_term("7.4.66", i, op::antya("a"));
    }

    let dhatu = p.get(i_dhatu)?;
    let last = p.terms().last()?;
    if dhatu.has_u("i\\R") && last.has_tag(T::kit) {
        p.op_term("7.4.69", i, op::adi("I"));
    }

    Some(())
}

/// Runs abhyasa rules specific to liT.
///
/// Args:
/// - `i`: the index of the abhyasa.
///
/// Example: bu + BU + va -> baBUva.
///
/// (7.4.70 - 7.4.74)
fn try_rules_for_lit(p: &mut Prakriya, i: usize) {
    let i_dhatu = i + 1;
    let abhyasa = &p.terms()[i];
    let last = p.terms().last().unwrap();

    let add_nut_agama = |rule, p: &mut Prakriya, i: usize| {
        op::insert_agama_before(p, i, "nu~w");
        p.step(rule);
        it_samjna::run(p, i).unwrap();
    };

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
                    let code = "7.4.71.k";
                    if p.is_allowed(code) {
                        add_nut_agama(code, p, i + 1);
                    } else {
                        p.decline(code);
                    }
                } else {
                    add_nut_agama("7.4.71", p, i + 1);
                }
            // For aSnoti only, not aSnAti
            } else if dhatu.text == "aS" && dhatu.gana == Some(5) {
                add_nut_agama("7.4.72", p, i + 1);
            }
        } else if p.has(i_dhatu, |t| {
            t.text == "BU" && (t.gana == Some(1) || t.gana == Some(2))
        }) {
            // gana 1 for `BU`, gana 2 for `as` replaced by `BU`.
            op::text2("7.4.73", p, i, "ba");
            // TODO: 7.4.74
        }
    }
}

/// Runs abhyasa rules specific to Slu-pratyaya.
/// Example: `ni + nij + anti` -> `nenijanti
///
/// Args:
/// - `i_abhyasa`: the index of the abhyasa.
///
/// Example: bu + BU + va -> baBUva.
///
/// (7.4.75 - 7.4.77)
fn try_rules_for_slu(p: &mut Prakriya, i: usize) -> Option<()> {
    p.find_last(T::Slu)?;

    let i_dhatu = i + 1;
    let abhyasa = p.get(i)?;
    let dhatu = p.get(i_dhatu)?;

    if dhatu.has_text_in(&["nij", "vij", "viz"]) {
        let sub = al::to_guna(abhyasa.antya()?)?;
        p.op_term("7.4.75", i, op::antya(sub));
    } else if dhatu.has_u_in(&["quBf\\Y", "mA\\N", "o~hA\\N"]) {
        p.op_term("7.4.76", i, op::antya("i"));
    } else if dhatu.has_text_in(&["f", "pf", "pF"]) {
        p.op_term("7.4.77", i, op::antya("i"));
    }

    Some(())
    // TODO: 7.4.78 bahulaM chandasi
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Abhyasa)?;

    // TODO: expand for abhyasa after dhatu.
    let i_dhatu = i + 1;
    if !p.has(i_dhatu, f::dhatu) {
        return None;
    }

    try_general_rules(p, i);
    try_rules_for_lit(p, i);
    try_rules_for_slu(p, i);

    Some(())
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
