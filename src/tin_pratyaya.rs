//! tin_pratyaya
//! ============
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
use crate::operations as op;
use crate::prakriya::Prakriya;

const TIN_PARA: &[&str] = &["ti", "tas", "Ji", "si", "Tas", "Ta", "mi", "vas", "mas"];
const NAL_ADI: &[&str] = &["Ral", "atus", "us", "Tal", "aTus", "a", "Ral", "va", "ma"];
// const TIN_NAL_MAPPING = dict(zip(TIN_PARA, NAL_ADI))
// TAJHAYOH = {"ta": "eS", "Ja": "irec"}

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
    let tin = if p.has_tag(T::Parasmaipada) {
        find_tin_parasmai(purusha, vacana)
    } else {
        assert!(p.has_tag(T::Atmanepada));
        find_tin_atmane(purusha, vacana)
    };

    if let Some(i) = p.find_last(T::Pratyaya) {
        p.set(i, |t| {
            t.add_tags(&[
                // 1.4.104
                T::Vibhakti,
                T::Tin,
                purusha.as_tag(),
                vacana.as_tag(),
            ]);
        });
        p.rule("3.4.78", |_| true, |p| op::upadesha(p, i, tin));

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
/*

fn jher_jus(p: &mut Prakriya, i: usize, la: La) {
    if !p.has(i, |t| t.has_u("Ji")) {
        return;
    }

    if matches!(la, La::AshirLin | La::VidhiLin) {
        p.op("3.4.108", |p| op::upadesha(p, i, "jus"));
    } else if la.is_nit() {
        let i_dhatu = p.find_last(T::Dhatu);
        prev = [t for t in p.terms[-2::-1] if t.text][0]

        _vid = prev.text == "vid" and prev.gana == 2
        if prev.u == "si~c" or prev.any(T::ABHYASTA) or _vid {
            p.op("3.4.109", |p| op::upadesha(p, i, "jus"));
        } else if prev.antya == "A" and p.terms[-2].u == "si~c" {
            p.op("3.4.110", |p| op::upadesha(p, i, "jus"));
        } else if la == La::Lan {
            if dhatu.text == "dviz":
                op.optional(op.upadesha, "3.4.112", p, la, "jus")
            } else if  prev.antya == "A" and prev.any(T::DHATU):
                op.optional(op.upadesha, "3.4.111", p, la, "jus")
        }
    }
}

fn lut_adesha(p: &mut Prakriya, i_la: usize, la: La) {
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
            p.op("2.4.85", |p| op::upadesha(p, i_la, ending));
        }
    }
}

/// Applies substitutions to the given tin suffix.
///
/// Due to rule 3.4.109 ("sic-abhyasta-vidibhyaH ca"), this should run after dvitva and the
/// insertion of vikaraNas.
pub fn siddhi(p: &mut Prakriya, la: La) {
    let i_dhatu = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };
    let i_la = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return,
    };

    // Special case: handle lut_adesha first.
    lut_adesha(p, i_la, la);

    // Matching for "w" will cause errors because the ending 'iw' has 'w' as an
    // anubandha. So, match the wit-lakAras by name so we can exclude 'iw':
    la_wit = {"la~w", "li~w", "lu~w", "lf~w", "le~w", "lo~w"}
    if la.all(T::ATMANEPADA) and la.any(*la_wit):
        if la.all("li~w") and la.text in TAJHAYOH:
            op.upadesha("3.4.81", p, la, TAJHAYOH[la.text])
        } else if  la.text == "TAs":
            op.upadesha("3.4.80", p, la, "se")
        else:
            op.ti("3.4.79", p, la, "e")

    } else if  la.all("li~w") and la.all(T::PARASMAIPADA):
        op.upadesha("3.4.82", p, la, TIN_NAL_MAPPING[la.text])

    } else if  la.all("la~w") and la.all(T::PARASMAIPADA):
        if dhatu.u == "vida~" and la.text in TIN_PARA:
            op.optional(op.upadesha, "3.4.83", p, la, TIN_NAL_MAPPING[la.text])
        if dhatu.text == "brU" and la.text in TIN_PARA[:5]:
            if p.allow("3.4.84"):
                dhatu.text = "Ah"
                op.upadesha("3.4.84", p, la, TIN_NAL_MAPPING[la.text])
            else:
                p.decline("3.4.84")

    if la.all("lo~w"):
        if la.text == "si":
            la.u = la.text = "hi"
            la.remove_tags("p")
            p.step("3.4.87")

            if p.all(T::CHANDASI):
                op.optional(op.tag, "3.4.88", p, la, "p")

        } else if  la.text == "mi":
            op.text("3.4.89", p, la, "ni")
        } else if  la.antya == "i":
            op.antya("3.4.86", p, la, "u")
        } else if  la.antya == "e":
            last_two = la.text[-2:]

            if la.all(T::UTTAMA) and la.text.endswith("e"):
                op.antya("3.4.93", p, la, "E")

            } else if  last_two in ("se", "ve"):
                if last_two == "se":
                    la.text = la.text[:-2] + "sva"
                else:
                    la.text = la.text[:-2] + "vam"
                p.step("3.4.91")

            else:
                op.antya("3.4.90", p, la, "Am")

        if la.all("uttama"):
            // 3.4.92
            agama = Term.agama("Aw")
            // Add pit to the pratyaya, not the Agama.
            la.add_tags("p")
            p.terms.insert(-1, agama)
            p.step("3.4.92")
            it_samjna.run(p, -2)

    // TODO: 3.4.94 - 3.4.98

    // Switch used below.
    keep_nit = False

    // Must occur before 3.4.100 below
    _jher_jus(p, la)

    // Include lo~w by 3.4.85
    if la.any("lo~w") or f.is_nit_lakara(la):
        // 3.4.101
        tastha = ("tas", "Tas", "Ta", "mi")
        if la.text in tastha:
            la.text = op.yatha(la.text, tastha, ("tAm", "tam", "ta", "am"))
            p.step("3.4.101")

        if la.all(T::PARASMAIPADA):
            if la.all(T::UTTAMA) and la.antya == "s":
                op.antya("3.4.99", p, la, "")
            // lo~w excluded by existence of 3.4.86
            if la.text.endswith("i") and not la.all("lo~w"):
                op.antya("3.4.100", p, la, "")

    if la.all("li~N"):
        if la.all(T::PARASMAIPADA):
            // Add Nit to the pratyaya, not the Agama.
            p.terms.insert(-1, Term.agama("yAsu~w"))
            if p.all(T::ASHIH):
                // Add kit to the pratyaya, not the Agama.
                op.tag("3.4.104", p, la, "k")
            else:
                // Add Nit to the pratyaya, not the Agama.
                op.tag("3.4.103", p, la, "N")
                keep_nit = True

            it_samjna.run(p, -2)
        else:
            p.terms.insert(-1, Term.agama("sIyu~w"))
            p.step("3.4.102")
            it_samjna.run(p, -2)

            if la.u == "Ja":
                op.upadesha("3.4.105", p, la, "ran")
            } else if  la.u == "iw":
                op.upadesha("3.4.106", p, la, "a")

        if "t" in la.text or "T" in la.text:
            la.text = la.text.replace("t", "st").replace("T", "sT")
            p.step("3.4.107")

    // The 'S' of 'eS' is just for sarva-Adeza (1.1.55). If it is kept, it will
    // cause many problems when deriving li~T:: So, remove it here.
    if la.u == "eS":
        la.tags.remove("S")
}
*/
