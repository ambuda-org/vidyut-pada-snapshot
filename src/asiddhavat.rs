/*!
asiddhavat
==========
(6.4.22 - 6.4.175 [end])

Rules in the *asiddhavat* rules do not interfere with each other. That is, if
a rule A would ordinary block a rule B, both are allowed to apply if they are
defined within this section.

*asiddhavat* rules are within the scope of the *aNgasya* adhikAra. For details,
see the `angasya` module.
*/

use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref HAL: SoundSet = s("hal");
    static ref JHAL: SoundSet = s("Jal");
    static ref MAHAPRANA: SoundSet = s("K G C J W Q T D P B");
}

/// Runs rules conditioned on a following knit ArdhadhAtuka suffix.
///
/// (6.4.63 - 6.4.69)
fn run_kniti_ardhadhatuka(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    let n = p.view(i + 1)?;

    let kniti_ardha = n.any(&[T::kit, T::Nit]) && n.has_tag(T::Ardhadhatuka);

    if kniti_ardha && dhatu.has_u("dI\\N") && n.has_adi(&*AC) {
        p.op("6.4.63", |p| op::insert_agama(p, i, "yu~w"));
        // No change to `n` index (`i + ``) needed since `yu~w` is an agama and will will be
        // included in `n`.
    } else if dhatu.has_antya('A') && n.has_adi(&*AC) && (kniti_ardha || f::is_it_agama(n.first()?))
    {
        p.op_term("6.4.64", i, op::antya(""));
    } else if dhatu.has_antya('A') && kniti_ardha {
        let ghu_ma = dhatu.has_tag(T::Ghu)
            || dhatu.has_text_in(&["mA", "sTA", "gA", "sA"])
            || dhatu.has_u("o~hA\\k")
            || (dhatu.has_u("pA\\") && dhatu.has_gana(1));
        if n.has_u("yat") {
            p.op_term("6.4.65", i, op::antya("I"));
        } else if n.has_adi(&*HAL) && ghu_ma {
            if n.has_lakshana("li~N") {
                p.op_term("6.4.67", i, op::antya("e"));
            } else {
                p.op_term("6.4.66", i, op::antya("I"));
            }
        } else if f::is_samyogadi(dhatu) {
            // HACK: skip dhatus with agama. `k` indicates a following agama.
            let next = p.get(i + 1)?;
            if next.all(&[T::Agama, T::kit]) {
                return None;
            }

            if n.has_u("lyap") {
                p.step("6.4.69");
            } else if n.has_lakshana("li~N") {
                p.op_optional("6.4.68", op::t(i, op::antya("e")));
            }
        }
    }

    Some(())
}

/// Returns whether the given slice ends in a samyoga.
fn is_samyogapurva(slice: &[Term]) -> bool {
    let mut num_hal = 0_u8;
    for t in slice.iter().rev() {
        for c in t.text.chars().rev() {
            if HAL.contains_char(c) {
                num_hal += 1;
                if num_hal >= 2 {
                    return true;
                }
            } else {
                return false;
            }
        }
    }
    false
}

/// Runs rules conditioned on a following `kit` or `Nit` suffix.
///
/// (6.4.98 - 6.4.126)
fn try_run_kniti(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !n.any(&[T::kit, T::Nit]) {
        return None;
    }

    if anga.has_text_in(&["gam", "han", "jan", "Kan", "Gas"]) && n.has_adi(&*AC) && !n.has_u("aN") {
        p.op_term("6.4.98", i, op::upadha(""));
    } else if (anga.has_text("hu") || anga.has_antya(&*JHAL)) && n.last()?.has_text("hi") {
        // TODO: why `end` here??
        p.op_term("6.4.101", n.end(), op::text("Di"));
    } else if anga.has_u("ciR") {
        p.op_term("6.4.104", n.start(), op::luk);
    } else if anga.has_antya('a') && n.first()?.has_text("hi") {
        p.op_term("6.4.105", n.start(), op::luk);
    } else if anga.has_antya('u') && !is_samyogapurva(&p.terms()[..i]) && n.first()?.has_text("hi")
    {
        p.op_term("6.4.106", n.start(), op::luk);
    }

    Some(())
}

/*
    prev = p.terms[index - 1] if index > 0 else None
    if c.antya == "u" and c.all(T.PRATYAYA):
        if prev and prev.text in ("kar", "kur"):
            if n.adi in s("m v"):
                op.luk("6.4.108", p, c)
            } else if  n.adi in s("y"):
                op.luk("6.4.109", p, c)
        } else if  n.adi in s("m v") and not samyogapurva:
            op.optional(op.antya, "6.4.107", p, c, "")

    sarvadhatuka = n.all(T.SARVADHATUKA)
    if sarvadhatuka:
        // Must come before 6.4.111
        if (c.u == "asa~" or c.all(T.GHU)) and n.terms[-1].u == "hi":
            for t in p.terms:
                if t.any(T.ABHYASA):
                    t.text = ""
            op.antya("6.4.119", p, c, "e")

        if c.all("Snam"):
            // TODO: unsafe?
            c.text = c.text.replace("na", "n")
            p.step("6.4.111")
        // Match on the upadesha so we don't include asu~ (asyati).
        } else if  c.u == "asa~":
            c.text = c.text.replace("a", "")
            p.step("6.4.111")

        } else if  c.u == "SnA" or c.all(T.ABHYASTA):
            if c.text == "daridrA" and n.adi in s("hal"):
                op.antya("6.4.114", p, c, "i")
            } else if  c.u == "YiBI\\" and n.adi in s("hal"):
                op.optional(op.antya, "6.4.115", p, c, "i")
            } else if  c.antya == "A":
                p.debug("aa")
                if c.u == "o~hA\\k" and n.adi in s("hal"):
                    if n.adi == "y":
                        op.antya("6.4.118", p, c, "")
                    else:
                        do = True
                        if n.text == "hi":
                            if op.optional(op.antya, "6.4.117", p, c, "A"):
                                do = False
                        if do:
                            op.optional(op.antya, "6.4.116", p, c, "i")
                    p.debug("A", c.u, n.text)
                } else if  n.adi in s("hal") and not c.all("ghu"):
                    op.antya("6.4.113", p, c, "I")
                else:
                    op.antya("6.4.112", p, c, "")
*/

/// Run rules that replace the dhatu's vowel with e and apply abhyasa-lopa.
/// Example: `la + laB + e` -> `leBe`
fn try_et_adesha_and_abhyasa_lopa_for_it(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    if !dhatu.all(&[T::Dhatu, T::Abhyasta]) {
        return None;
    }
    let abhyasa = p.get(i - 1)?;
    if !abhyasa.has_tag(T::Abhyasa) {
        return None;
    }
    let n = p.view(i + 1)?;

    let kniti = n.any(&[T::kit, T::Nit]);
    let thali_seti = n.get(0)?.has_u("iw") && n.get(1)?.has_u("Tal");
    if !(kniti || thali_seti) {
        return None;
    }

    let op_et_abhyasa_lopa = |p: &mut Prakriya| {
        p.set(i, op::upadha("e"));
        p.set(i - 1, op::lopa);
    };

    if dhatu.text == "daB" && dhatu.has_u("danBu~") {
        p.op("6.4.120", op_et_abhyasa_lopa);
    } else if dhatu.has_u("tF") || dhatu.has_text_in(&["Pal", "Baj", "trap"]) {
        p.op("6.4.122", op_et_abhyasa_lopa);
    } else if dhatu.has_text("SraT") && dhatu.has_u("SranTa~") {
        p.op("6.4.122.v1", op_et_abhyasa_lopa);
    } else if dhatu.has_text("graT") {
        // TODO: attested, but can't find the rule for it.
        p.op("???", op_et_abhyasa_lopa);
    } else if dhatu.has_text("rAD") {
        p.op_optional("6.4.123", op_et_abhyasa_lopa);
    } else if dhatu.has_u("jF") || dhatu.has_text_in(&["Bram", "tras"]) {
        p.op_optional("6.4.124", op_et_abhyasa_lopa);
    } else if dhatu.has_u_in(gana::PHAN_ADI) {
        p.op_optional("6.4.125", op_et_abhyasa_lopa);
    } else if dhatu.has_text_in(&["Sas", "dad"]) || dhatu.has_adi('v') || dhatu.has_tag(T::FlagGuna)
    {
        // No change.
        p.step("6.4.126")
    } else {
        let is_eka_hal_madhya =
            dhatu.text.len() == 3 && dhatu.has_adi(&*HAL) && dhatu.has_antya(&*HAL);
        let is_a = dhatu.has_upadha('a');
        let is_lit = n.has_lakshana("li~w");
        // Aspirated consonants become usaspirated in the tripAdi, which hasn't run
        // yet at this stage in the derivation. So, also "look ahead" and check for
        // aspirated consonants.
        let is_anadeshadi = abhyasa.adi() == dhatu.adi() && !abhyasa.has_adi(&*MAHAPRANA);

        if is_eka_hal_madhya && is_a && is_lit && is_anadeshadi {
            if kniti {
                // `la laB e` -> `leBe`
                p.op("6.4.120", op_et_abhyasa_lopa);
            } else {
                // `Sa Sak i Ta` -> `SekiTa`
                p.op("6.4.121", op_et_abhyasa_lopa);
            }
        }
    }

    Some(())
}

/*
/// Runs rules conditioned on a following ardhadhatuka suffix.
///
/// (6.4.46 - 6.4.70)
fn ardhadhatuke(p: &mut Prakriya, i: usize) {
    c = p.terms[index]
    n = TermView.make(p, index)
    if not n or not n.any(T.ARDHADHATUKA) {
        return
    }

    // HACK to avoid abhyasa-at-lopa
    if c.all(T.ABHYASA) {
        return
    }

    if c.text == "Brasj" {
        op.optional(op.text, "6.4.47", p, c, "Barj")

    } else if  c.antya == "a":
        op.antya("6.4.48", p, c, "")
        c.add_tags(T.F_AT_LOPA)
        // TODO: remove P F_AT_LOPA
        p.add_tags(T.F_AT_LOPA)
    }
}


fn run_dirgha(p: Prakriya):
    """6.4.2 - 6.4.19"""

    sup = p.terms[-1]
    if not sup.all(T.SUP):
        return
    anga = p.terms[-2]

    has_num = False
    if anga.u == "nu~w":
        anga = p.terms[-3]
        has_num = True

    if sup.text == "Am" and has_num:
        if anga.text in {"tisf", "catasf"}:
            p.step("6.4.3")
        } else if  anga.text == "nf":
            op.optional(op.antya, "6.4.4", p, anga, sounds.dirgha(anga.antya))
        } else if  anga.antya == "n":
            op.upadha("6.4.5", p, anga, sounds.dirgha(anga.upadha))
        } else if  anga.antya in s("ac"):
            op.antya("6.4.2", p, anga, sounds.dirgha(anga.antya))

    } else if  sup.any(T.SARVANAMASTHANA) and not sup.any(T.SAMBUDDHI):
        tr_exclude = {"pitf", "pitar", "jAmAtf", "jAmAtar", "BrAtf", "BrAtar"}
        if anga.antya == "n":
            op.upadha("6.4.8", p, anga, sounds.dirgha(anga.upadha))
        // TODO: restrict
        } else if  (
            anga.antya == "f" or anga.text.endswith("ar")
        ) and anga.text not in tr_exclude:
            op.upadha("6.4.11", p, anga, sounds.dirgha(anga.upadha))


fn antya_nalopa(p: Prakriya, index):
    """Rules that delete the final n of a term.

    (6.4.37 - )
    """

    c = p.terms[index]
    n = TermView.make(p, index)
    if not n:
        return

    n.u = n.terms[0].u

    anudatta_tanadi_van = c.all(T.ANUDATTA) or c.u in TAN_ADI or c.text == "van"
    jhali = n.adi in s("Jal")
    kniti = f.is_knit(n)

    if c.text in {"jan", "san", "Kan"}:
        // jan + Syan should always be jAyate.
        if (n.adi == "y" and kniti) and not (c.text == "jan" and n.u == "Syan"):
            op.optional(op.antya, "6.4.38", p, c, "A")
        } else if  (jhali and kniti) or n.u == "san":
            op.antya("6.4.37", p, c, "A")

    } else if  c.text == "tan" and n.u == "yak":
        op.optional(op.antya, "6.4.39", p, c, "A")

    } else if  c.antya in s("Yam") and anudatta_tanadi_van and jhali and kniti:
        if n.u == "lyap":
            op.optional(op.antya, "6.4.37", p, c, "")
        else:
            op.antya("6.4.37", p, c, "")
*/

fn try_add_a_agama(p: &mut Prakriya, i: usize) {
    if p.find_last(T::Dhatu).is_none() {
        return;
    };
    let i_tin = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return,
    };

    if !p.has(i_tin, f::lakshana_in(&["lu~N", "la~N", "lf~N"])) {
        return;
    }
    // Dhatu may be multi-part, so insert before abhyasa.
    // But abhyasa may follow main dhatu (e.g. undidizati) --
    // So, use the first match we find.
    let i_start = match p.find_first_where(|t| t.any(&[T::Abhyasa, T::Dhatu])) {
        Some(i) => i,
        None => return,
    };

    // Agama already added in a previous iteration, so return.
    // (To prevent infinite loops)
    if i_start > 0 && p.has(i_start - 1, f::tag(T::Agama)) {
        return;
    }

    if p.has(i, f::adi("ac")) {
        let agama = Term::make_agama("Aw");
        p.insert_before(i, agama);
        p.step("6.4.72");
        it_samjna::run(p, i).unwrap();
    } else {
        let agama = Term::make_agama("aw");
        p.insert_before(i, agama);
        p.step("6.4.71");
        it_samjna::run(p, i).unwrap();
    }
}

pub fn run_before_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    if dhatu.has_tag(T::Snam) && dhatu.upadha().unwrap() == 'n' {
        p.op_term("6.4.23", i, op::upadha(""));
    }

    let dhatu = p.get(i)?;
    let n = p.view(i + 1)?;
    let anidit_hal = !dhatu.has_tag(T::idit) && dhatu.has_antya(&*HAL);
    let is_kniti = n.any(&[T::kit, T::Nit]);

    if anidit_hal && is_kniti && dhatu.has_upadha('n') {
        // ancu gati-pUjanayoH
        if dhatu.has_u("ancu~") {
            let code = "6.4.30";
            if p.is_allowed(code) {
                p.step(code);
                p.op_term("6.4.24", i, op::upadha(""));
            } else {
                p.decline(code)
            }
        } else {
            p.op_term("6.4.24", i, op::upadha(""));
        }
    } else if dhatu.has_text_in(&["danS", "sanj", "svanj"]) && n.has_u("Sap") {
        p.op_term("6.4.25", i, op::upadha(""));
    } else if dhatu.text == "ranj" && n.has_u("Sap") {
        p.op_term("6.4.26", i, op::upadha(""));
    } else if dhatu.text == "SAs" && is_kniti && (n.has_u("aN") || n.has_adi(&*HAL)) {
        p.op_term("6.4.34", i, op::upadha("i"));
    }

    /*
    anidit_hal = (not c.any("i")) and c.antya in s("hal")
    kniti = f.is_knit(n)

    if anidit_hal and kniti and c.upadha == "n":
        do = True
        // ancu gati-pUjanayoH
        if c.u == "ancu~":
            if p.allow("6.4.30"):
                p.step("6.4.30")
                do = False
            else:
                p.decline("6.4.30")
        if do:
            op.upadha("6.4.24", p, c, "")

    } else if  c.text in ("danS", "sanj", "svanj") and n.u == "Sap":
        op.upadha("6.4.25", p, c, "")

    } else if  c.text == "ranj" and n.u == "Sap":
        op.upadha("6.4.26", p, c, "")

    } else if  c.text == "SAs" and kniti and (n.u == "aN" or n.adi in s("hal")):
        op.upadha("6.4.34", p, c, "i")

    antya_nalopa(p, index)

    // Blocked by 7.3.84
    can_guna = n.any(T.SARVADHATUKA, T.ARDHADHATUKA) and not f.is_knit(n)
    */

    try_add_a_agama(p, i);

    // ardhadhatuke(p, index)

    // Must run before guNa
    let n = p.view(i + 1)?;
    if p.has(i, f::text("BU")) && n.has_lakshana_in(&["lu~N", "li~w"]) {
        op::append_agama("6.4.88", p, i, "vu~k");
    }

    /*
    if c.u == "ciR" and n.text == "ta":
        op.luk("6.4.104", p, n.terms[0])

    // 6.4.114 has a vArttika for ArdhadhAtuke:
    } else if  c.u == "daridrA" and n.any(T.ARDHADHATUKA):
        if p.terms[-1].all("lu~N"):
            if p.allow("6.4.114.v2"):
                p.step("6.4.114.v2")
                return
            else:
                p.decline("6.4.114.v2")

        // Should replace just the last sound, but sak-Agama causes issues
        // here.
        // TODO: what is the correct prakriya here?
        op.text("6.4.114.v1", p, c, "daridr")
    */

    Some(())
}

// Runs rules that are conditioned on an anga ending in an "i" or "v" sound.
//
// (6.4.77 - 6.4.100)
fn run_for_final_i_or_v(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !n.has_adi(&*AC) {
        return None;
    }

    let yv = ["i", "I", "u", "U"];
    let iy_uv = ["iy", "iy", "uv", "uv"];
    let is_samyogapurva = if i > 0 {
        match p.get(i - 1) {
            Some(t) => t.has_antya(&*HAL),
            None => false,
        }
    } else {
        false
    };

    if anga.has_u_in(&["hu\\", "Snu"]) && n.has_tag(T::Sarvadhatuka) && !is_samyogapurva {
        p.op_term("6.4.87", i, op::antya("v"));
    } else if anga.has_u("i\\R") {
        p.op_term("6.4.81", i, op::antya("y"));
    }

    Some(())
}

/*
    // General case
    if c.antya in iyuv:
        aneka_ac = sum(1 for L in c.text if L in s("ac")) > 1
        samyogapurva = (
            len(c.text) >= 3 and c.text[-3] in s("hal") and c.text[-2] in s("hal")
        )

        if (
            c.all(T.DHATU)
            and c.antya in s("i")
            // HACK to infer "aneka-ac" from abhyasta
            and (aneka_ac or c.all(T.ABHYASTA))
            and not samyogapurva
        ):
            op.antya("6.4.82", p, c, "y")
        } else if  c.text == "strI":
            if n.terms[0].u in ("am", "Sas"):
                if p.allow("6.4.80"):
                    pass
                else:
                    p.decline("6.4.80")
                    op.antya("6.4.79", p, c, iyuv[c.antya])
            else:
                op.antya("6.4.79", p, c, iyuv[c.antya])

        } else if  c.all(T.DHATU) or c.u in ("Snu", "BrU"):
            if c.u == "i\\R":
                op.antya("6.4.81", p, c, "y")
            // Some grammarians include ik in the scope of 6.4.81.
            } else if  c.u == "i\\k":
                op.optional(op.antya, "6.4.81", p, c, "y")

            if c.antya != "y":
                op.antya("6.4.77", p, c, iyuv[c.antya])
        } else if  c.all(T.ABHYASA) and n.adi not in sounds.savarna(c.antya):
            op.antya("6.4.78", p, c, iyuv[c.antya])
*/

/// Runs asiddhavat rules that alter a Ri suffix.
pub fn run_for_ni(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_where(|t| t.has_u_in(&["Ric", "RiN"]))?;
    let c = p.get(i)?;
    let n = p.view(i + 1)?;

    /*
        if (
            c.u in ("Ric", "RiN")
            and not f.is_it_agama(n.terms[0])
            and n.all(T.ARDHADHATUKA)
        ):
            n_text = n.terms[0].text
            if n_text in {"Am", "anta", "Alu", "Ayya", "itnu", "iznu"}:
                op.antya("6.4.55", p, c, "ay")
            else:
                // Apply ac_sandhi before lopa, since later rules depend on this
                // being done (e.g. cayyAt)
                ac_sandhi.general_vowel_sandhi(p, p.terms[index - 1 : index + 1])
                op.antya("6.4.51", p, c, "")
    */

    if c.has_tag(T::mit) && n.has_u("Ric") && c.has_upadha(&*AC) {
        if let Some(sub) = al::to_hrasva(c.upadha()?) {
            p.op_term("6.4.92", i, op::upadha(&sub.to_string()));
        }
    }

    Some(())
}

pub fn run_after_guna(p: &mut Prakriya, i: usize) -> Option<()> {
    run_kniti_ardhadhatuka(p, i);
    run_for_final_i_or_v(p, i);
    try_run_kniti(p, i);

    /*
        // TODO: fails kniti check because this depends on the last affix, and
        // term view includes only "u" here. So the rule is awkwardly placed
        // here.
        last = p.terms[-1]
        sarva_kniti = last.all(T.SARVADHATUKA) and last.any("k", "N")
        if c.u == "qukf\\Y" and c.text == "kar" and n.adi == "u" and sarva_kniti:
            c.text = "kur"
            p.step("6.4.110")

    */
    try_et_adesha_and_abhyasa_lopa_for_it(p, i);

    let n = p.view(i + 1)?;
    if n.has_tag(T::qit) {
        p.op_term("6.4.143", i, op::ti(""));
    }

    Some(())
}
