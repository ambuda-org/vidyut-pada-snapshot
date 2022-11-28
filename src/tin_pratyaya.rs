//! tin_pratyaya ============
//! (3.4.77 - end of 3.4)
//!
//! The rules in this section have two main functions:
//!
//! 1. Replace a giving lakāra with the appropriate tiṅ-pratyaya. This is called tiṅ-ādeśa ("verb
//!    ending substitution"). To perform tiṅ-ādeśa, we must know the puruṣa, vacana, and pada
//!    associated with this prakriyā.
//!
//! 2. Modify the basic tiṅ-pratyaya according to the lakāra and any other conditions relevant to
//!    the prakriyā (for example, vidhi-liṅ vs. āśīr-liṅ). This is called tiṅ-siddhi ("verb ending
//!    completion").
//!
//! All of these rules are found at the end of section 3.4 of the Ashtadhyayi.

use crate::constants::Tag as T;
use crate::constants::{La, Purusha, Vacana};
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::s;
use crate::term::Term;
use std::error::Error;

const TIN_PARA: &[&str] = &["tip", "tas", "Ji", "sip", "Tas", "Ta", "mip", "vas", "mas"];
const NAL_PARA: &[&str] = &["Ral", "atus", "us", "Tal", "aTus", "a", "Ral", "va", "ma"];

fn find_tin_parasmai(purusha: Purusha, vacana: Vacana) -> &'static str {
    match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => "tip",
        (Purusha::Prathama, Vacana::Dvi) => "tas",
        (Purusha::Prathama, Vacana::Bahu) => "Ji",
        (Purusha::Madhyama, Vacana::Eka) => "sip",
        (Purusha::Madhyama, Vacana::Dvi) => "Tas",
        (Purusha::Madhyama, Vacana::Bahu) => "Ta",
        (Purusha::Uttama, Vacana::Eka) => "mip",
        (Purusha::Uttama, Vacana::Dvi) => "vas",
        (Purusha::Uttama, Vacana::Bahu) => "mas",
    }
}

fn find_tin_atmane(purusha: Purusha, vacana: Vacana) -> &'static str {
    match (purusha, vacana) {
        (Purusha::Prathama, Vacana::Eka) => "ta",
        (Purusha::Prathama, Vacana::Dvi) => "AtAm",
        (Purusha::Prathama, Vacana::Bahu) => "Ja",
        (Purusha::Madhyama, Vacana::Eka) => "TAs",
        (Purusha::Madhyama, Vacana::Dvi) => "ATAm",
        (Purusha::Madhyama, Vacana::Bahu) => "Dvam",
        (Purusha::Uttama, Vacana::Eka) => "iw",
        (Purusha::Uttama, Vacana::Dvi) => "vahi",
        (Purusha::Uttama, Vacana::Bahu) => "mahiN",
    }
}

/// Replaces the lakAra with a tiN-pratyaya.
pub fn adesha(p: &mut Prakriya, purusha: Purusha, vacana: Vacana) {
    let (tin, pada) = if p.has_tag(T::Parasmaipada) {
        let e = find_tin_parasmai(purusha, vacana);
        (e, T::Parasmaipada)
    } else {
        assert!(p.has_tag(T::Atmanepada));
        let e = find_tin_atmane(purusha, vacana);
        (e, T::Atmanepada)
    };

    if let Some(i) = p.find_last(T::Pratyaya) {
        p.set(i, |t| {
            t.add_tags(&[
                // 1.4.104
                T::Vibhakti,
                T::Tin,
                purusha.as_tag(),
                vacana.as_tag(),
                pada,
            ]);
        });
        op::adesha("3.4.78", p, i, tin);

        // Ignore Nit-tva that we get from the lakAra. Kashika on 3.4.103:
        //
        //   lakArAzrayaGitvam AdezAnAM na bhavati.
        //
        // Likewise, this rule ignores the N of mahiN, which is just for the sake
        // of making a pratyAhAra.
        if p.has(i, |t| t.has_tag(T::Nit)) {
            p.set(i, |t| t.remove_tag(T::Nit));
        }
    }
}

fn maybe_replace_jhi_with_jus(p: &mut Prakriya, i: usize, la: La) {
    if !p.has(i, |t| t.has_u("Ji")) {
        return;
    }

    if matches!(la, La::AshirLin | La::VidhiLin) {
        op::adesha("3.4.108", p, i, "jus");
    } else if la.is_nit() {
        let i_dhatu = match p.find_last(T::Dhatu) {
            Some(i) => i,
            None => return,
        };
        let i_prev = p
            .terms()
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, t)| t.text.is_empty())
            .map(|(i, _)| i)
            .nth(1);
        let i_prev = match i_prev {
            Some(i) => i,
            None => return,
        };

        let is_vid = p.has(i_dhatu, |t: &Term| t.text == "vid" && t.gana == Some(2));
        if p.has(i_prev, |t| {
            t.has_u("si~c") || t.has_tag(T::Abhyasta) || is_vid
        }) {
            op::adesha("3.4.109", p, i, "jus");
        } else if p.has(i_dhatu, |t| t.has_antya(&s("A"))) && p.has(i_prev, |t| t.has_u("si~c")) {
            op::adesha("3.4.110", p, i, "jus");
        } else if la == La::Lan {
            if p.has(i_prev, |t| t.has_antya(&s("A")) && t.has_tag(T::Dhatu)) {
                p.op_optional("3.4.111", |p| op::upadesha(p, i, "jus"));
            } else if p.has(i_dhatu, |t| t.text == "dviz") {
                p.op_optional("3.4.112", |p| op::upadesha(p, i, "jus"));
            }
        }
    }
}

fn maybe_do_lut_siddhi(p: &mut Prakriya, i_la: usize, la: La) -> bool {
    if p.has(i_la, |t| t.has_tag(T::Prathama) && la == La::Lut) {
        if let Some(tin) = p.get_mut(i_la) {
            let ending = if tin.has_tag(T::Ekavacana) {
                "qA"
            } else if tin.has_tag(T::Dvivacana) {
                "rO"
            } else if tin.has_tag(T::Bahuvacana) {
                "ras"
            } else {
                panic!("Unknown state");
            };
            op::adesha("2.4.85", p, i_la, ending);
        }
        true
    } else {
        false
    }
}

/// Applies tin-siddhi rules that apply to just loT.
fn maybe_do_lot_only_siddhi(p: &mut Prakriya, i: usize) -> Result<(), Box<dyn Error>> {
    if p.has(i, |t| t.has_lakshana("lo~w")) {
        // let mut t = p.get_mut(i).unwrap();
        if p.has(i, |t| t.text == "si") {
            p.op(
                "3.4.87",
                op::t(i, |t| {
                    t.u = Some("hi".to_string());
                    t.text = "hi".to_string();
                    t.remove_tag(T::pit);
                }),
            );

            if p.has_tag(T::Chandasi) {
                p.op_optional("3.4.88", op::t(i, op::add_tag(T::Pit)));
            }
        } else if p.has(i, f::ends_with("mi")) {
            p.op_term("3.4.89", i, op::text("ni"));
        } else if p.has(i, f::ends_with("i")) {
            p.op_term("3.4.86", i, op::antya("u"));
        } else if p.has(i, f::ends_with("e")) {
            if p.has(i, |t| t.has_tag(T::Uttama) && t.text.ends_with('e')) {
                p.op_term("3.4.93", i, op::antya("E"));
            } else if p.has(i, |t| t.text.ends_with("se") || t.text.ends_with("ve")) {
                p.set(i, |t| {
                    let n = t.text.len();
                    if t.text.ends_with("se") {
                        t.text = String::from(&t.text[..n - 2]) + "sva";
                    } else {
                        t.text = String::from(&t.text[..n - 2]) + "vam";
                    }
                });
                p.step("3.4.91")
            } else {
                p.op_term("3.4.90", i, op::antya("Am"));
            }
        }

        if p.has(i, |t| t.has_tag(T::Uttama)) {
            p.op("3.4.92", |p| {
                let agama = Term::make_agama("Aw");
                // Add pit to the pratyaya, not the Agama.
                p.set(i, |t| t.add_tag(T::Pit));
                p.insert_before(i, agama);
            });
            it_samjna::run(p, i)?;
        }
    }

    Ok(())
}

fn maybe_do_lin_siddhi(p: &mut Prakriya, i_tin: usize, la: La) -> Result<(), Box<dyn Error>> {
    let mut i = i_tin;

    if !p.has(i, |t| t.has_lakshana("li~N")) {
        return Ok(());
    }
    if p.has(i, |t| t.has_tag(T::Parasmaipada)) {
        p.insert_before(i, Term::make_agama("yAsu~w"));
        i += 1;

        if la == La::AshirLin {
            // Add kit to the pratyaya, not the Agama.
            p.op_term("3.4.104", i, op::add_tag(T::kit));
        } else {
            // Add Nit to the pratyaya, not the Agama.
            p.op_term("3.4.103", i, op::add_tag(T::Nit));
        }
        it_samjna::run(p, i - 1)?;
    } else {
        p.insert_before(i, Term::make_agama("sIyu~w"));
        i += 1;

        p.step("3.4.102");
        it_samjna::run(p, i - 1)?;

        if p.has(i, |t| t.has_u("Ja")) {
            op::adesha("3.4.105", p, i, "ran");
        } else if p.has(i, |t| t.has_u("iw")) {
            op::adesha("3.4.106", p, i, "a");
        }
    }

    if p.has(i, |t| t.text.contains('t') || t.text.contains('T')) {
        p.set(i, |t| t.text = t.text.replace('t', "st").replace('T', "sT"));
        p.step("3.4.107");
    }

    Ok(())
}

// Includes lo~w by 3.4.85
fn maybe_do_lot_and_nit_siddhi(p: &mut Prakriya, la: La) {
    let i = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return,
    };

    if la == La::Lot || la.is_nit() {
        let tas_thas = &["tas", "Tas", "Ta", "mi"];
        let taam_tam = &["tAm", "tam", "ta", "am"];
        if p.has(i, |t| t.has_text(tas_thas)) {
            p.op("3.4.101", |p| op::text_yatha(p, i, tas_thas, taam_tam));
        }

        if p.has(i, |t| t.has_tag(T::Parasmaipada)) {
            if p.has(i, |t| t.has_tag(T::Uttama) && t.text.ends_with('s')) {
                p.op_term("3.4.99", i, op::antya(""));
            }

            // lo~w excluded by existence of 3.4.86
            if p.has(i, |t| t.text.ends_with('i')) && la != La::Lot {
                p.op_term("3.4.100", i, op::antya(""));
            }
        }
    }
}

/// Applies substitutions to the given tin suffix.
///
/// Due to rule 3.4.109 ("sic-abhyasta-vidibhyaH ca"), this should run after dvitva and the
/// insertion of vikaraNas.
pub fn siddhi(p: &mut Prakriya, la: La) -> Result<(), Box<dyn Error>> {
    let i_dhatu = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return Ok(()),
    };
    let i = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return Ok(()),
    };

    // Special case: handle lut_siddhi first.
    if maybe_do_lut_siddhi(p, i, la) {
        return Ok(());
    }

    let tin = p.get(i).unwrap();
    // Matching for "w" will cause errors because the ending 'iw' has 'w' as an
    // anubandha. So, match the wit-lakAras by name so we can exclude 'iw':
    let wits = &["la~w", "li~w", "lu~w", "lf~w", "le~w", "lo~w"];
    if tin.has_tag(T::Atmanepada) && tin.has_any_lakshana(wits) {
        let ta_jha = &["ta", "Ja"];
        let es_irec = &["eS", "irec"];
        if p.has(i, |t| t.has_lakshana("li~w") && t.has_text(ta_jha)) {
            p.op("3.4.81", |p| op::upadesha_yatha(p, i, ta_jha, es_irec));
        } else if p.has(i, |t| t.text == "TAs") {
            op::adesha("3.4.80", p, i, "se");
        } else {
            p.op_term("3.4.79", i, op::ti("e"));
        }
    } else if tin.has_lakshana("li~w") && tin.has_tag(T::Parasmaipada) {
        p.op("3.4.82", |p| op::upadesha_yatha(p, i, TIN_PARA, NAL_PARA));
    } else if tin.has_lakshana("la~w") && tin.has_tag(T::Parasmaipada) {
        if p.has(i_dhatu, f::u("vida~")) && p.has(i, f::text_in(TIN_PARA)) {
            p.op_optional("3.4.83", |p| op::upadesha_yatha(p, i, TIN_PARA, NAL_PARA));
        } else if p.has(i_dhatu, |t| t.text == "brU")
            && p.has(i, |t| TIN_PARA[..5].contains(&t.text.as_str()))
        {
            p.op_optional("3.4.84", |p| {
                p.set(i_dhatu, |t| t.text = "Ah".to_string());
                op::upadesha_yatha(p, i, TIN_PARA, NAL_PARA);
            });
        }
    }

    // TODO: 3.4.94 - 3.4.98

    maybe_do_lot_only_siddhi(p, i)?;
    // Must occur before 3.4.100 in loT/nit siddhi.
    maybe_replace_jhi_with_jus(p, i, la);
    maybe_do_lot_and_nit_siddhi(p, la);
    maybe_do_lin_siddhi(p, i, la)?;

    // The 'S' of 'eS' is just for sarva-Adeza (1.1.55). If it is kept, it will
    // cause many problems when deriving li~T. So, remove it here.
    if p.has(i, |t| t.has_u("eS")) {
        p.set(i, |t| t.remove_tag(T::Sit));
    }

    Ok(())
}
