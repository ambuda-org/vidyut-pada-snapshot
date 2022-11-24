//! vikarana
//! ========
//! (3.1.33 - 3.1.90)
//! 
//! Rules that add various intermediate suffixes, called **vikaraṇas**, between the
//! dhātu and the tiṅ ending. Roughly, we can split these rules into four groups:
//! 
//! - rules for lr̥t, lr̥ṅ, luṭ, and leṭ (3.1.33 - 3.1.34)
//! - rules for ām-pratyaya (3.1.35 - 3.1.42)
//! - rules for luṅ (3.1.43 - 3.1.67)
//! - rules for sārvadhātuka pratyayas (3.1.68 - 3.1.90), which includes laṭ, loṭ,
//!   laṅ, and vidhi-liṅ.
//! 
//! (āśīr-liṅ and liṭ do not use any vikaraṇas.)

// The it-prakarana is applied at the very end, since there might be various
// substitutions by lopa that block the prakarana.

use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::operations as op;
use crate::dhatu_gana::{PUSH_ADI, DYUT_ADI, TAN_ADI}; 
use crate::prakriya::Prakriya;
use crate::sounds::s;
use crate::term::Term;

fn add_vikarana(v: &str) -> Fn(&mut Prakriya) {
    |p| {
        let mut vikarana = Term::make_upadesha(v);
        vikarana.add_tags(&[T::Pratyaya, T::Vikarana]);
        if let Some(i) = p.find_last(T::Dhatu) {
            p.insert_after(i, vikarana);
        }
    }
}

fn add_aam() -> Fn(&mut Prakriya) {
    |p| {
        let mut aam = Term::make_upadesha(v);
        aam.add_tags(&[T::Pratyaya]);
        if let Some(i) = p.find_last(T::Dhatu) {
            p.insert_after(i, aam);
        }
    }
}

/// Apply the vikarana rules for luN (3.1.43 - 3.1.66)
fn maybe_add_lun_vikarana(p: &mut Prakriya) {
    p.op("3.1.43", op::add_vikarana("cli~"));

    dhatu, cli, tin = p.terms[-3:]

    // The vArttika doesn't say this specifically, but the commentator examples
    // imply that this holds only for parasmaipada.
    if dhatu.text in {"spfS", "mfS", "kfz", "tfp", "dfp"} and tin.any(T.PARASMAIPADA):
        if op.optional(op.upadesha_no_it, "3.1.44.v1", p, cli, "si~c"):
            return

    shal_igupadha_anit = (
        dhatu.antya in s("Sal")
        and dhatu.upadha in s("ik")
        // iT hasn't been added yet, so check for "U" (veT) and anudAtta (aniT).
        and (dhatu.all(T.ANUDATTA) or dhatu.all("U"))
    )
    jr_stambhu = {"jF", "stanB", "mruc", "mluc", "gruc", "gluc", "glunc", "Svi"}

    // Takes priority over shala igupadha
    pushadi_dyutadi_ldit = (
        (dhatu.u in PUSH_ADI and dhatu.gana == 4)
        or (dhatu.u in DYUT_ADI and dhatu.gana == 1)
        or dhatu.all("x")
    )
    if tin.all(T.PARASMAIPADA) and cli.u == "cli~" and pushadi_dyutadi_ldit:
        op.upadesha_no_it("3.1.55", p, cli, "aN")

    } else if  shal_igupadha_anit:
        if dhatu.text == "dfS":
            p.step("3.1.47")
        } else if  dhatu.text == "Sliz" and dhatu.gana == 4:
            op.optional(op.upadesha_no_it, "3.1.46", p, cli, "ksa")
        else:
            if dhatu.all("U"):
                // Needed if we use "ksa" with a veT root.
                if p.allow(T.F_ANIT_KSA):
                    p.add_tags(T.F_ANIT_KSA)
                    op.upadesha_no_it("3.1.45", p, cli, "ksa")
                else:
                    p.add_tags(T.F_SET_SIC)
                    p.decline(T.F_ANIT_KSA)
            else:
                op.upadesha_no_it("3.1.45", p, cli, "ksa")

    shri_dru_sru = dhatu.text in ("Sri", "dru", "sru")
    if p.all(T.KARTARI) and (dhatu.u in ("Ric", "RiN") or shri_dru_sru):
        op.upadesha_no_it("3.1.48", p, cli, "caN")
    } else if  dhatu.u == "kamu~\\":
        op.upadesha_no_it("3.1.48.v1", p, cli, "caN")
    } else if  dhatu.text in ("De", "Svi"):
        op.optional(op.upadesha_no_it, "3.1.49", p, cli, "caN")
    // TODO: 3.1.50 - 3.1.51
    } else if  dhatu.u == "asu~" or dhatu.text in {"vac", "KyA"}:
        op.upadesha_no_it("3.1.52", p, cli, "aN")
    } else if  dhatu.text in {"lip", "sic", "hve"}:
        skip = False
        if tin.all(T.ATMANEPADA):
            if p.allow("3.1.54"):
                p.step("3.1.54")
                skip = True
            else:
                p.decline("3.1.54")
        if not skip:
            op.upadesha_no_it("3.1.53", p, cli, "aN")

    // Ensure no substitution has already occurred (e.g. for Svi which can be
    // matched by 3.1.49 above).
    if tin.all(T.PARASMAIPADA) and cli.u == "cli~":
        if dhatu.text in {"sf", "SAs", "f"}:
            op.upadesha_no_it("3.1.56", p, cli, "aN")
        } else if  dhatu.all("ir"):
            op.optional(op.upadesha_no_it, "3.1.57", p, cli, "aN")
        } else if  dhatu.text in jr_stambhu:
            op.optional(op.upadesha_no_it, "3.1.58", p, cli, "aN")
        } else if  dhatu.text in {"kf", "mf", "df", "ruh"} and p.all(T.CHANDASI):
            op.upadesha_no_it("3.1.59", p, cli, "aN")

    // TODO: ciN (3.1.60 - 3.1.66)
    if tin.u == "ta":
        if dhatu.text == "pad":
            op.upadesha_no_it("3.1.60", p, cli, "ciR")
        } else if  dhatu.text in {"dIp", "jan", "buD", "pUr", "tAy", "pyAy"}:
            op.optional(op.upadesha_no_it, "3.1.61", p, cli, "ciR")

    // Base case
    if cli.u == "cli~":
        op.upadesha_no_it("3.1.44", p, cli, "si~c")
}

fn am_pratyaya_lit(p: Prakriya):
    _, dhatu = p.find_last(T.DHATU)
    la = p.terms[-1]

    if dhatu.text == "kAs" or dhatu.all(T.PRATYAYA):
        _add_am("3.1.35", p)
    // jAgf is handled separately below.
    } else if  not f.is_eka_ac(dhatu) and dhatu.text not in {"jAgf", "UrRu"}:
        _add_am("3.1.35.v1", p)
    } else if  dhatu.adi in s("ic") and f.is_guru(dhatu) and dhatu.u != "fCa~":
        _add_am("3.1.36", p)
    } else if  dhatu.text in {"day", "ay", "As"}:
        _add_am("3.1.37", p)
    } else if  dhatu.text in {"uz", "jAgf"} or (dhatu.text == "vid" and dhatu.gana == 2):
        if op.optional(_add_am, "3.1.38", p):
            if dhatu.text == "vid":
                // vid does not go through guNa.
                dhatu.add_tags(T.F_GUNA_APAVADA)
        else:
            return
    } else if  dhatu.text in {"BI", "hrI", "hu"} or dhatu.u == "quBf\\Y":
        am = Term.make_upadesha("Am")
        am.add_tags(T.PRATYAYA, T.SLU)
        if not op.optional(_add_am, "3.1.39", p, am):
            return
    else:
        return

    // "Am" added.
    // TODO: qukf//Y?
    kf = Term.make_dhatu("kf", 8)
    kf.add_tags(T.DHATU)
    p.terms = p.terms[:-1] + [kf, p.terms[-1]]
    p.step("3.1.40")


fn am_pratyaya_lot(p: Prakriya):
    _, dhatu = p.find_last(T.DHATU)
    la = p.terms[-1]

    if dhatu.text == "vid" and dhatu.gana == 2 and la.any("lo~w"):
        if op.optional(_add_am, "3.1.41", p):
            // Derive by nipAtana
            dhatu.add_tags(T.F_GUNA_APAVADA)
            kf = Term.make_dhatu("qukf\\Y", 8)
            kf.text = "kf"
            kf.add_tags(T.DHATU)
            p.terms = p.terms[:-1] + [kf, p.terms[-1]]
            p.step("3.1.40")


fn sarvadhatuka_vikarana(p: &mut Prakriya) {
    let i = match p.find_last(T.DHATU) {
        Some(i) => i,
        None => return,
    }

    if !p.has_tag(T::Kartari) {
        p.op("3.1.67", add_vikarana("yak"));
        return
    }

    // Optional cases
    gana_4_declined = False
    if dhatu.text in {"BrAS", "BlAS", "Bram", "kram", "klam", "tras", "truw", "laz"}:
        used_option = op.optional(_add, "3.1.70", p, "Syan")

        // Needed to make 3.1.69 available to roots like Bram
        if (not used_option) and dhatu.gana == 4:
            gana_4_declined = True

    // TODO: anupasarga
    } else if p.has(i, |t| t.has_u("yasu~")) {
        p.op_optional("3.1.71", add_vikarana("Syan"));
    } else if p.has(i, |t| t.has_u("akzU~")) {
        p.op_optional("3.1.75", add_vikarana("Snu"));
    } else if p.has(i, |t| t.has_u("takzU~")) {
        p.op_optional("3.1.76", add_vikarana("Snu"));
    } else if p.has(i, |t| t.has_u_in(&["sta\\nBu~", "stu\\nBu~", "ska\\nBu~", "sku\\nBu~", "sku\\Y"]) {
        p.op_optional("3.1.82", add_vikarana("Snu"));
    }

    _, added_vikarana = p.find_first(T.VIKARANA)
    if added_vikarana {
        return
    }

    if dhatu.gana == 4 and not gana_4_declined {
        p.op("3.1.69", add_vikarana("Syan"))
    } else if dhatu.gana == 5 or dhatu.text == "Sru" {
        if dhatu.text == "Sru":
            dhatu.text = "Sf"
            _add("3.1.74", p, "Snu")
        else:
            _add("3.1.73", p, "Snu")
    } else if dhatu.gana == 6 {
        p.op("3.1.77", add_vikarana("Sa"))
    } else if dhatu.gana == 7 {
        dhatu.add_tags("Snam")
        op.mit("3.1.78", p, dhatu, "na")
    } else if} else if  dhatu.gana == 8 or dhatu.u == "qukf\\Y" {
        _add("3.1.79", p, "u")
    } else if dhatu.u in ("Divi~", "kfvi~") {
        dhatu.text = dhatu.text[:-1] + "a"
        p.op("3.1.80", add_vikarana("u"));
    } else if dhatu.gana == 9 {
        p.op("3.1.81", add_vikarana("SnA"));
    } else {
        p.op("3.1.68", add_vikarana("Sap"));
    }
}


fn optional_rule(rule: str, p: Prakriya):
    if p.allow(rule):
        return rule
    else:
        p.decline(rule)
        return None


fn vikarana_lopa(p: Prakriya):
    """For certain roots and gaNas, delete the vikaraNa.

    (2.4.72 - 2.4.82)
    """

    // TODO: extend this to other pratyayas -- should properly be pratyaya_lopa
    _, dhatu = p.find_last(T.DHATU)
    _, vikarana = p.find_first(T.VIKARANA)
    tin = p.terms[-1]

    if not vikarana:
        return

    if vikarana.u == "Sap":
        if dhatu.gana == 2:
            op.luk("2.4.72", p, vikarana)
        } else if  dhatu.gana == 3:
            op.slu("2.4.75", p, vikarana)
    } else if  vikarana.u == "si~c" and tin.all(T.PARASMAIPADA):
        luk = aluk = None
        if dhatu.text in {"GrA", "De", "So", "Co", "so"}:
            // De takes luk by 2.4.77, so this allows aluk.
            if dhatu.text == "De":
                aluk = optional_rule("2.4.78", p)
            // Other roots avoid luk by default, so this allows luk.
            else:
                luk = optional_rule("2.4.78", p)

        // Run only if aluk was not used above.
        if (not aluk) and (
            dhatu.text in {"sTA", "BU"}
            or (dhatu.text == "gA" and dhatu.gana == 2)
            or (dhatu.text == "pA" and dhatu.gana == 1)
            or dhatu.all(T.GHU)
        ):
            luk = "2.4.77"

        assert not (luk and aluk)
        if luk:
            op.luk(luk, p, vikarana)
        } else if  aluk:
            p.step(aluk)
    } else if  vikarana.u == "si~c" and tin.text in {"ta", "TAs"}:
        if dhatu.u in TAN_ADI:
            op.optional(op.luk, "2.4.79", p, vikarana)


fn run(p: Prakriya):
    tin = p.terms[-1]

    if tin.any("lf~w", "lf~N", "lu~w"):
        if tin.any("lf~w", "lf~N"):
            _add("3.1.33", p, "sya")
        else:
            _add("3.1.33", p, "tAsi~")
    } else if  tin.any("lu~N"):
        lun_vikarana(p)
    } else if  tin.any("li~w"):
        am_pratyaya_lit(p)
    else:
        if tin.any("lo~w"):
            // Just for vidāṅkurvantu, etc.
            am_pratyaya_lot(p)
        if tin.all(T.SARVADHATUKA):
            sarvadhatuka_vikarana(p)

    _, vikarana = p.find_first(T.VIKARANA)
    if vikarana:
        vikarana_lopa(p)
        // Run it-samjna-prakarana only after the lopa phase is complete.
        if vikarana.text:
            it_samjna.run_no_index(p, vikarana)

    // HACK for gAN gatau (bhvAdi). The long A should be handled early because
    // it blocks `AtmanepadezvanataH` and `Ato GitaH`.
    try:
        p.debug(p.terms[-3].text, p.terms[-2].text)
        if p.terms[-3].text == "gA" and p.terms[-2].text == "a":
            p.terms[-2].text = ""
            p.step("6.1.101")
    except IndexError:
        pass
