//! vikarana
//! ========
//! (3.1.33 - 3.1.90)
//!
//! Rules that add various intermediate suffixes, called **vikaraṇas**, between the
//! dhātu && the tiṅ ending. Roughly, we can split these rules into four groups:
//!
//! - rules for lr̥t, lr̥ṅ, luṭ, && leṭ (3.1.33 - 3.1.34)
//! - rules for ām-pratyaya (3.1.35 - 3.1.42)
//! - rules for luṅ (3.1.43 - 3.1.67)
//! - rules for sārvadhātuka pratyayas (3.1.68 - 3.1.90), which includes laṭ, loṭ,
//!   laṅ, && vidhi-liṅ.
//!
//! (āśīr-liṅ && liṭ do not use any vikaraṇas.)

// The it-prakarana is applied at the very end, since there might be various
// substitutions by lopa that block the prakarana.

use crate::constants::Tag as T;
use crate::dhatu_gana::{DYUT_ADI, PUSH_ADI, TAN_ADI};
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use compact_str::CompactString;
use lazy_static::lazy_static;
use std::error::Error;

lazy_static! {
    static ref SHAL: SoundSet = s("Sal");
    static ref IK: SoundSet = s("ik");
    static ref IC: SoundSet = s("ic");
}

/// Returns a function that inserts the vikarana `v` after the dhatu.
fn add_vikarana(v: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        let mut vikarana = Term::make_upadesha(v);
        vikarana.add_tags(&[T::Pratyaya, T::Vikarana]);
        if let Some(i) = p.find_last(T::Dhatu) {
            p.insert_after(i, vikarana);
        }
    }
}

// Returns a function that inserts the `Am` pratyaya after the dhatu.
fn add_aam(p: &mut Prakriya) {
    let mut aam = Term::make_upadesha("Am");
    aam.add_tags(&[T::Pratyaya]);
    if let Some(i) = p.find_last(T::Dhatu) {
        p.insert_after(i, aam);
    }
}

fn replace_with(i: usize, sub: &'static str) -> impl Fn(&mut Prakriya) {
    move |p| {
        op::upadesha_no_it(p, i, sub);
    }
}

fn xyz(p: &mut Prakriya, i: usize, f: impl Fn(&Term, &Term, &Term) -> bool) -> bool {
    match (p.get(i), p.get(i + 1), p.get(i + 2)) {
        (Some(x), Some(y), Some(z)) => f(x, y, z),
        _ => false,
    }
}

/// Returns whether the dhatu at index `i` is followed by the `cli~` vikarana as opposed to some
/// substitution.
fn has_cli(p: &Prakriya, i: usize) -> bool {
    p.has(i + 1, |t| t.has_u("cli~"))
}

/// Applies rules that might replace `cli~` with `ksa`.
fn maybe_replace_cli_with_ksa(p: &mut Prakriya, i: usize) {
    if !has_cli(p, i) {
        return;
    }

    // The vArttika doesn't say this specifically, but the commentator examples
    // imply that this holds only for parasmaipada.
    let sprs = &["spfS", "mfS", "kfz", "tfp", "dfp"];
    if xyz(p, i, |x, _, z| {
        x.has_text_in(sprs) && z.has_tag(T::Parasmaipada)
    }) {
        //
        if p.op_optional("3.1.44.v1", |p| op::upadesha_no_it(p, i + 1, "si~c")) {
            return;
        }
    }

    let shal_igupadha_anit = |t: &Term| {
        t.has_antya(&*SHAL)
        && t.has_upadha(&*IK)
        // iT hasn't been added yet, so check for "U" (veT) && anudAtta (aniT).
        && t.any(&[T::Anudatta, T::Udit])
    };

    let pushadi_dyutadi_ldit = |t: &Term| {
        (t.has_u_in(PUSH_ADI) && t.gana == Some(4))
            || (t.has_u_in(DYUT_ADI) && t.gana == Some(1))
            || t.has_tag(T::xdit)
    };

    let to_ksa = replace_with(i + 1, "ksa");

    // Takes priority over shala igupadha
    if xyz(p, i, |x, _, z| {
        pushadi_dyutadi_ldit(x) && z.has_tag(T::Parasmaipada)
    }) {
        p.op("3.1.55", |p| op::upadesha_no_it(p, i + 1, "aN"));
    } else if p.has(i, shal_igupadha_anit) {
        if p.has(i, |t| t.text == "dfS") {
            p.step("3.1.47")
        } else if p.has(i, |t| t.text == "Sliz" && t.gana == Some(4)) {
            p.op_optional("3.1.46", to_ksa);
        } else if p.has(i, |t| t.has_tag(T::Udit)) {
            p.op_optional("3.1.45", |p| {
                to_ksa(p);
                // Needed if we use "ksa" with a veT root.
                p.add_tag(T::FlagAnitKsa);
            });
        } else {
            p.op("3.1.45", to_ksa);
        }
    }
}

/// Applies rules that might replace `cli~` with `caN`.
fn maybe_replace_cli_with_can(p: &mut Prakriya, i: usize) {
    if !has_cli(p, i) {
        return;
    }

    let ni = |t: &Term| t.has_u_in(&["Ric", "RiN"]);
    let shri_dru_sru = |t: &Term| t.has_text_in(&["Sri", "dru", "sru"]);
    let to_can = replace_with(i + 1, "caN");

    if p.has_tag(T::Kartari) && p.has(i, |t| ni(t) || shri_dru_sru(t)) {
        p.op("3.1.48", to_can);
    } else if p.has(i, |t| t.has_u("kamu~\\")) {
        p.op("3.1.48.v1", to_can);
    } else if p.has(i, |t| t.has_text_in(&["De", "Svi"])) {
        p.op_optional("3.1.49", to_can);
    }
    // TODO: 3.1.50 - 3.1.51
}

fn maybe_replace_cli_with_an(p: &mut Prakriya, i: usize) {
    if !has_cli(p, i) {
        return;
    }

    let to_an = replace_with(i + 1, "aN");
    if p.has(i, |t| t.has_u("asu~") || t.has_text_in(&["vac", "KyA"])) {
        p.op("3.1.52", to_an);
    } else if p.has(i, |t| t.has_text_in(&["lip", "sic", "hve"])) {
        let mut skip = false;
        if p.has(i + 2, |t| t.has_tag(T::Atmanepada)) {
            if p.is_allowed("3.1.54") {
                p.step("3.1.54");
                skip = true;
            } else {
                p.decline("3.1.54")
            }
        }
        if !skip {
            p.op("3.1.53", to_an);
        }
    }

    // Ensure no substitution has already occurred (e.g. for Svi which can be
    // matched by 3.1.49 above).
    let to_an = replace_with(i + 1, "aN");
    let jr_stambhu = [
        "jF", "stanB", "mruc", "mluc", "gruc", "gluc", "glunc", "Svi",
    ];
    if p.has(i + 2, |t| t.has_tag(T::Parasmaipada) && has_cli(p, i)) {
        if p.has(i, |t| t.has_text_in(&["sf", "SAs", "f"])) {
            p.op("3.1.56", to_an);
        } else if p.has(i, |t| t.has_tag(T::irit)) {
            p.op_optional("3.1.57", to_an);
        } else if p.has(i, |t| t.has_text_in(&jr_stambhu)) {
            p.op_optional("3.1.58", to_an);
        } else if p.has(i, |t| {
            t.has_text_in(&["kf", "mf", "df", "ruh"]) && p.has_tag(T::Chandasi)
        }) {
            p.op("3.1.59", to_an);
        }
    }
}

fn maybe_replace_cli_with_cin(p: &mut Prakriya, i: usize) {
    if !has_cli(p, i) {
        return;
    }

    let to_cin = replace_with(i + 1, "ciR");
    if p.has(i + 2, |t| t.has_u("ta")) {
        if p.has(i, |t| t.text == "pad") {
            p.op("3.1.60", to_cin);
        } else if p.has(i, |t| {
            t.has_text_in(&["dIp", "jan", "buD", "pUr", "tAy", "pyAy"])
        }) {
            p.op_optional("3.1.61", to_cin);
        }
    }
    // TODO: 3.1.62 - 3.1.66
}

fn maybe_replace_cli_with_sic(p: &mut Prakriya, i: usize) {
    if has_cli(p, i) {
        p.op("3.1.44", |p| op::upadesha_no_it(p, i + 1, "si~c"));
    }
}

/// Applies the vikarana rules for luN (3.1.43 - 3.1.66).
fn add_lun_vikarana(p: &mut Prakriya) {
    p.op("3.1.43", add_vikarana("cli~"));

    let n = p.terms().len();
    assert!(n >= 3);
    let i = n - 3;

    maybe_replace_cli_with_ksa(p, i);
    maybe_replace_cli_with_can(p, i);
    maybe_replace_cli_with_an(p, i);
    maybe_replace_cli_with_cin(p, i);
    maybe_replace_cli_with_sic(p, i);
}

fn add_kr_after_am_pratyaya(p: &mut Prakriya) {
    let mut kf = Term::make_dhatu("qukf\\Y", 8, 10);
    kf.set_text("kf");
    kf.add_tag(T::Dhatu);

    let i_tin = p.terms().len() - 1;
    p.insert_before(i_tin, kf);
    p.step("3.1.40")
}

fn maybe_add_am_pratyaya_for_lit(p: &mut Prakriya) {
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    if p.has(i, |t| t.text == "kAs" || t.has_tag(T::Pratyaya)) {
        p.op("3.1.35", add_aam);
    } else if p.has(i, |t| !f::is_eka_ac(t) && !t.has_text_in(&["jAgf", "UrRu"])) {
        // jAgf is handled separately below.
        p.op("3.1.35.v1", add_aam);
    } else if p.has(i, |t| t.has_adi(&*IC) && f::is_guru(t) && !t.has_u("fCa~")) {
        p.op("3.1.36", add_aam);
    } else if p.has(i, |t| t.has_text_in(&["day", "ay", "As"])) {
        p.op("3.1.37", add_aam);
    } else if p.has(i, |t| {
        t.has_text_in(&["uz", "jAgf"]) || (t.text == "vid" && t.gana == Some(2))
    }) {
        let mut aam = Term::make_upadesha("Am");
        aam.add_tags(&[T::Pratyaya]);
        if let Some(i) = p.find_last(T::Dhatu) {
            p.insert_after(i, aam);
        }
        let used = p.op_optional("3.1.38", add_aam);
        if used {
            if p.has(i, |t| t.text == "vid") {
                // vid does not go through guNa.
                p.set(i, |t| t.add_tag(T::FlagGunaApavada));
            }
        } else {
            return;
        }
    } else if p.has(i, |t| {
        t.has_text_in(&["BI", "hrI", "hu"]) || t.has_u("quBf\\Y")
    }) {
        let add_sluvat_am = |p: &mut Prakriya| {
            let mut aam = Term::make_upadesha("Am");
            aam.add_tags(&[T::Pratyaya, T::Slu]);
            if let Some(i) = p.find_last(T::Dhatu) {
                p.insert_after(i, aam);
            }
        };
        if !p.op_optional("3.1.39", add_sluvat_am) {
            return;
        }
    } else {
        return;
    }

    add_kr_after_am_pratyaya(p);
}

fn maybe_add_am_pratyaya_for_lot(p: &mut Prakriya) {
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    let is_lot = match p.terms().last() {
        Some(t) => t.has_lakshana("lo~w"),
        None => false,
    };

    if p.has(i, |t| t.text == "vid" && t.gana == Some(2) && is_lot) {
        let added_am = p.op_optional("3.1.41", add_aam);

        if added_am {
            // Derive by nipAtana
            p.set(i, |t| t.add_tag(T::FlagGunaApavada));
            add_kr_after_am_pratyaya(p);
        }
    }
}

fn add_sarvadhatuka_vikarana(p: &mut Prakriya) {
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    if !p.has_tag(T::Kartari) {
        p.op("3.1.67", add_vikarana("yak"));
        return;
    }

    // Optional cases
    let stanbhu_stunbhu = ["sta\\nBu~", "stu\\nBu~", "ska\\nBu~", "sku\\nBu~", "sku\\Y"];
    let mut gana_4_declined = false;
    if p.has(i, |t| {
        t.has_text_in(&[
            "BrAS", "BlAS", "Bram", "kram", "klam", "tras", "truw", "laz",
        ])
    }) {
        let applied = p.op_optional("3.1.70", add_vikarana("Syan"));

        // Needed to make 3.1.69 available to roots like Bram
        if !applied && p.has(i, |t| t.gana == Some(4)) {
            gana_4_declined = true;
        }
    // TODO: anupasarga
    } else if p.has(i, |t| t.has_u("yasu~")) {
        p.op_optional("3.1.71", add_vikarana("Syan"));
    } else if p.has(i, |t| t.has_u("akzU~")) {
        p.op_optional("3.1.75", add_vikarana("Snu"));
    } else if p.has(i, |t| t.has_u("takzU~")) {
        p.op_optional("3.1.76", add_vikarana("Snu"));
    } else if p.has(i, |t| t.has_u_in(&stanbhu_stunbhu)) {
        p.op_optional("3.1.82", add_vikarana("Snu"));
    }

    if p.find_first(T::Vikarana) != None {
        return;
    }

    if p.has(i, |t| t.gana == Some(4) && !gana_4_declined) {
        p.op("3.1.69", add_vikarana("Syan"));
    } else if p.has(i, |t| t.gana == Some(5)) {
        p.op("3.1.73", add_vikarana("Snu"));
    } else if p.has(i, |t| t.text == "Sru") {
        p.op("3.1.74", |p| {
            p.set(i, |t| t.set_text("Sf"));
            add_vikarana("Snu")(p);
        });
    } else if p.has(i, |t| t.gana == Some(6)) {
        p.op("3.1.77", add_vikarana("Sa"));
    } else if p.has(i, |t| t.gana == Some(7)) {
        p.op("3.1.78", |p| {
            p.set(i, |t| t.add_tag(T::Snam));
            p.set(i, op::mit("na"));
        });
    } else if p.has(i, |t| t.gana == Some(8) || t.has_u("qukf\\Y")) {
        p.op("3.1.79", add_vikarana("u"));
    } else if p.has(i, |t| t.has_u_in(&["Divi~", "kfvi~"])) {
        p.op("3.1.80", |p| {
            p.set(i, op::antya("a"));
            add_vikarana("u")(p);
        });
    } else if p.has(i, |t| t.gana == Some(9)) {
        p.op("3.1.81", add_vikarana("SnA"));
    } else {
        p.op("3.1.68", add_vikarana("Sap"));
    }
}

fn maybe_sic_lopa_before_parasmaipada(p: &mut Prakriya, i: usize, i_vikarana: usize, i_tin: usize) {
    if !p.has(i_tin, |t| t.has_tag(T::Parasmaipada)) {
        return;
    }

    let do_luk = |p: &mut Prakriya, code| p.op(code, op::t(i_vikarana, op::luk));
    if p.has(i, |t| t.has_text_in(&["GrA", "De", "So", "Co", "so"])) {
        let code = "2.4.78";
        // De takes luk by 2.4.77, so 2.4.78 allows aluk.
        if p.has(i, |t| t.text == "De") {
            if p.is_allowed(code) {
                p.step(code);
                return;
            } else {
                p.decline(code);
            }
        } else {
            // The other roots avoid luk by default, so 2.4.78 allows luk.
            if p.is_allowed(code) {
                do_luk(p, code);
                return;
            } else {
                p.decline(code);
            }
        }
    }

    let gati_stha = |t: &Term| {
        (t.text == "gA" && t.gana == Some(2))
            || t.text == "sTA"
            || t.has_tag(T::Ghu)
            || (t.text == "pA" && t.gana == Some(1))
            || t.text == "BU"
    };

    // Run only if aluk was not used above.
    if p.has(i, gati_stha) {
        do_luk(p, "2.4.77");
    }
}

fn maybe_sic_lopa_for_tanadi_atmanepada(
    p: &mut Prakriya,
    i: usize,
    i_vikarana: usize,
    i_tin: usize,
) {
    let tanadi = p.has(i, |t| t.has_text_in(TAN_ADI));
    let tathasoh = p.has(i_tin, |t| t.has_text_in(&["ta", "TAs"]));
    if tanadi && tathasoh {
        p.op_optional("2.4.79", op::t(i_vikarana, op::luk));
    }
}

/// For certain roots && gaNas, delete the vikaraNa.
/// (2.4.72 - 2.4.82)
fn vikarana_lopa(p: &mut Prakriya) {
    // TODO: extend this to other pratyayas -- should properly be pratyaya_lopa
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };
    let i_vikarana = match p.find_first(T::Vikarana) {
        Some(i) => i,
        None => return,
    };
    let i_tin = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return,
    };
    assert!(i + 1 == i_vikarana);

    let vikarana_u = p.get(i_vikarana).unwrap().text.to_string();

    if vikarana_u == "Sap" {
        if p.has(i, |t| t.gana == Some(2)) {
            p.op("2.4.72", op::t(i_vikarana, op::luk));
        } else if p.has(i, |t| t.gana == Some(3)) {
            p.op("2.4.75", op::t(i_vikarana, op::slu));
        }
    } else if vikarana_u == "si~c" {
        maybe_sic_lopa_before_parasmaipada(p, i, i_vikarana, i_tin);
        maybe_sic_lopa_for_tanadi_atmanepada(p, i, i_vikarana, i_tin);
    }
}

pub fn run(p: &mut Prakriya) -> Result<(), Box<dyn Error>> {
    let tin = match p.terms().last() {
        Some(t) => t,
        None => return Ok(()),
    };

    if tin.has_lakshana_in(&["lf~w", "lf~N", "lu~w"]) {
        if tin.has_lakshana_in(&["lf~w", "lf~N"]) {
            p.op("3.1.33", add_vikarana("sya"));
        } else {
            p.op("3.1.33", add_vikarana("tAsi~"));
        }
    } else if tin.has_lakshana("lu~N") {
        add_lun_vikarana(p);
    } else if tin.has_lakshana("li~w") {
        maybe_add_am_pratyaya_for_lit(p);
    } else if tin.has_tag(T::Sarvadhatuka) {
        if tin.has_lakshana("lo~w") {
            // Just for vidāṅkurvantu, etc.
            maybe_add_am_pratyaya_for_lot(p);
        }
        add_sarvadhatuka_vikarana(p);
    }

    if let Some(i_vikarana) = p.find_first(T::Vikarana) {
        vikarana_lopa(p);
        // Run it-samjna-prakarana only after the lopa phase is complete.
        if p.has(i_vikarana, |t| !t.text.is_empty()) {
            it_samjna::run(p, i_vikarana)?;
        }
    }

    // HACK for gAN gatau (bhvAdi). The long A should be handled early because
    // it blocks `AtmanepadezvanataH` && `Ato GitaH`.
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return Ok(()),
    };
    if p.has(i, |t| t.text == "gA") && p.has(i + 1, |t| t.text == "a") {
        p.set(i + 1, |t| t.text = CompactString::from("".to_string()));
        p.step("6.1.101")
    }

    Ok(())
}
