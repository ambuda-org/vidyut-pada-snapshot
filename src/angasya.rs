//! angasya
//! =======
//! (6.4.1 - end of 7.4)
//!
//! Rules that modify the sounds and terms in an aṅga.
//!
//! This section of the text is massive, so we break it down into several smaller prakaraṇas.

use crate::abhyasasya;
use crate::asiddhavat;
use crate::char_view::{char_rule, get_at, set_at, xy};
use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::sup_adesha;
use crate::term::{Term, TermView};
use compact_str::CompactString;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref HRASVA: SoundSet = SoundSet::new("aiufx");
    static ref OSHTHYA: SoundSet = s("pu~ v");
    static ref FF: SoundSet = s("f");
    static ref IK: SoundSet = s("ik");
    static ref HAL: SoundSet = s("hal");
    static ref JHAL: SoundSet = s("Jal");
    static ref YANY: SoundSet = s("yaY");
    static ref I_U: SoundSet = s("i u");
    static ref PHA_DHA_KHA_CHA_GHA: SoundSet = s("P Q K C G");
}

fn op_antya_guna(t: &mut Term) {
    if let Some(a) = t.antya() {
        if let Some(sub) = al::to_guna(a) {
            op::antya(sub)(t);
            t.add_tag(T::FlagGuna);
        }
    }
}

fn maybe_rule(p: &mut Prakriya, rule: Rule) -> Option<Rule> {
    if p.is_allowed(rule) {
        Some(rule)
    } else {
        p.decline(rule);
        None
    }
}

/// Applies rules that replace an initial "J" in a pratyaya with the appropriate sounds.
/// (7.1.3 - 7.1.7)
fn maybe_do_jha_adesha(p: &mut Prakriya, i: usize) {
    if !p.has(i, |t| t.has_adi('J')) {
        return;
    }
    let b = match p.terms()[..i]
        .iter()
        .enumerate()
        .rev()
        .find(|(_, t)| !t.text.is_empty())
    {
        Some((index, _)) => index,
        None => return,
    };

    let to_at = |t: &mut Term| t.text = CompactString::from("at") + &t.text[1..];
    let to_ant = |t: &mut Term| t.text = CompactString::from("ant") + &t.text[1..];

    if p.has(b, f::tag(T::Abhyasta)) {
        p.op("7.1.4", op::t(i, to_at));
    } else if p.has(b, |t| !t.has_antya('a')) && p.has(i, f::atmanepada) {
        p.op("7.1.5", op::t(i, to_at));
    } else {
        p.op("7.1.3", op::t(i, to_ant));
    }

    if p.has(i, f::atmanepada) {
        let insert_rut = |p: &mut Prakriya| p.insert_after(b, Term::make_agama("ru~w"));

        if p.has(b, f::u("SIN")) {
            insert_rut(p);
            p.step("7.1.6");
        } else if p.has(b, |t| t.has_u("vida~") && t.gana == Some(2)) {
            p.op_optional("7.1.7", insert_rut);
        }
    }
}

/// Applies rules that replace one or more sounds in a pratyaya.
///
/// Usually, these sounds are it letters ("J") or otherwise aupadeshika (e.g. "yu").
/// Examples: Bava + Ji -> Bavanti, kar + yu -> karaNa.
///
/// (7.1.1 - 7.1.35 + 3.1.83)
pub fn try_pratyaya_adesha(p: &mut Prakriya) {
    let len = p.terms().len();
    if len <= 2 {
        return;
    }

    let i = len - 1;
    let t = &p.terms()[i];
    if t.has_text_in(&["yu~", "vu~"]) {
        if t.text == "yu~" {
            p.op("7.1.1", op::t(i, op::text("ana")));
        } else {
            p.op("7.1.1", op::t(i, op::text("aka")));
        }
    } else if t.has_adi(&*PHA_DHA_KHA_CHA_GHA) {
        let sub = match t.adi().unwrap() {
            'P' => "Ayan",
            'Q' => "ey",
            'K' => "In",
            'C' => "Iy",
            'G' => "in",
            _ => panic!("Unexpected"),
        };
        p.op("7.1.2", op::t(i, op::adi(sub)));
    } else if t.has_adi('J') {
        maybe_do_jha_adesha(p, i);
    // -tAt substitution needs to occur early because it conditions samprasarana.
    } else if p.has(i - 1, |t| t.has_antya('A')) && t.has_u("Ral") {
        op::upadesha_v2("7.1.34", p, i, "O");
    } else if p.has(i, |t| t.has_tag(T::Tin) && t.has_text_in(&["tu", "hi"])) {
        // N is to block pit-guNa, not for replacement of the last letter.
        p.op_optional("7.1.35", |p| op::upadesha(p, i, "tAta~N"));
    }

    // Run 3.1.83 here because it has no clear place otherwise.
    // TODO: is there a better place for this?
    if i > 2 {
        let t = &p.terms()[i];
        if p.has(i - 2, |t| t.has_antya(&*HAL)) && p.has(i - 1, f::u("SnA")) && t.text == "hi" {
            op::upadesha_v2("3.1.83", p, i - 1, "SAnac");
        }
    }
}

fn can_use_guna_or_vrddhi(anga: &Term, n: &TermView) -> bool {
    // 1.1.6 dIdhI-vevI-iTAm
    let didhi_vevi_itam =
        anga.has_u_in(&["dIDIN", "vevIN"]) || (anga.has_u("iw") && anga.has_tag(T::Agama));
    // 1.1.5 kNiti ca
    let kniti = n.any(&[T::kit, T::Nit]);
    // Other prohibitions
    let blocked = anga.any(&[T::FlagAtLopa, T::FlagGunaApavada]);
    let is_pratyaya = n.has_tag(T::Pratyaya);

    !didhi_vevi_itam && !kniti && !blocked && is_pratyaya

    // Otherwise, 1.1.3 iko guNavRddhI
}

/// Runs rules that replace an anga's vowel with its corresponding vrddhi.
/// Example: kf + i + ta -> kArita
fn try_vrddhi_adesha(p: &mut Prakriya, i: usize) {
    if p.has(i, f::tag(T::FlagGunaApavada)) {
        return;
    }
    let n = match p.view(i + 1) {
        Some(v) => v,
        None => return,
    };

    if p.has(i, f::text("mfj")) && !n.any(&[T::kit, T::Nit]) {
        p.op_term("7.2.114", i, op::text("mArj"));
    } else {
        try_nnit_vrddhi(p, i);
    }
}

/// Runs rules for vrddhi conditioned on following Nit-Yit.
///
/// (7.2.115 - 7.3.35)
fn try_nnit_vrddhi(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    let n = p.view(i + 1)?;

    if !n.any(&[T::Yit, T::Rit]) || !can_use_guna_or_vrddhi(anga, &n) {
        // Allow RiN even though it is Nit and will be excluded by `can_use_guna_or_vrddhi`.
        if !n.has_u("RiN") {
            return None;
        }
    }

    let anga = p.get(i)?;
    let n = p.view(i + 1)?;
    if anga.has_text_in(&["jan", "vaD"]) && (anga.has_u("ciR") || n.has_tag(T::Krt)) {
        // Declined for `ajani` etc.
        p.step("7.3.35");
    } else if anga.has_antya(&*AC) {
        // The use of "acaH" in 7.2.115 indicates that we should ignore "iko guNavRddhI" which
        // ordinarily restricts vrddhi to ik vowels only. By ignoring this restriction, we can
        // correctly generate `vye -> vivyAya` etc.
        let sub = al::to_vrddhi(anga.antya().unwrap()).unwrap();
        p.op_term("7.2.115", i, op::antya(sub));
    } else if anga.has_upadha('a') {
        p.op_term("7.2.116", i, op::upadha("A"));
    }

    Some(())
}

/// Runs rules that replace an anga's vowel with its corresponding guna.
/// Example: buD + a + ti -> boDati
fn try_guna_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get(i)?;
    if anga.has_tag(T::Agama) {
        return None;
    }

    let n = p.view(i + 1)?;
    if !can_use_guna_or_vrddhi(anga, &n) {
        return None;
    }

    let is_sarva_ardha = n.any(&[T::Sarvadhatuka, T::Ardhadhatuka]);
    let piti_sarvadhatuke = n.all(&[T::pit, T::Sarvadhatuka]);
    let is_ik = anga.has_antya(&*IK);

    // HACK: Asiddhavat, but this blocks guna.
    // TODO: move this to asiddhavat && add no_guna tag.
    if anga.has_text("guh") && n.has_adi(&*AC) {
        p.op_term("6.4.89", i, op::upadha("U"));
    } else if anga.has_u_in(&["Divi~", "kfvi~"]) {
        // Per commentary on 3.1.81, make an exception for dhinv and kRNv.
    } else if anga.has_u("mid") && n.has_tag(T::Sit) {
        p.op_term("7.3.82", i, |t| {
            t.text.replace_range(.., "med");
            t.add_tag(T::FlagGuna);
        });
    } else if is_ik && n.has_u("jus") {
        p.op_term("7.3.83", i, op_antya_guna);
    } else if is_sarva_ardha {
        let vi_cin_nal = n.get(0)?.has_u_in(&["kvip", "ciN", "Ral"]);

        // Exceptions
        if anga.has_tag(T::Abhyasta) && piti_sarvadhatuke && n.has_adi(&*AC) {
            // e.g. nenijAma
            p.step("7.3.87")
        } else if anga.has_text_in(&["BU", "sU"]) && n.has_tag(T::Tin) && piti_sarvadhatuke {
            // e.g. aBUt
            // TODO: broken due to `vu~k`-Agama throwing off `n`.
            p.step("7.3.88");
        } else if anga.has_antya('u') && n.has_adi(&*HAL) && piti_sarvadhatuke {
            let sub = al::to_vrddhi(anga.antya()?)?;
            if anga.has_u("UrRu") {
                if f::is_aprkta(n.last()?) {
                    p.op_term("7.3.91", i, op_antya_guna);
                } else {
                    // If vrddhi is declined, UrRu will take guna by 7.3.84 below.
                    p.op_optional("7.3.90", op::t(i, op::antya(sub)));
                }
            } else if n.has_tag(T::Luk) {
                p.op_term("7.3.89", i, op::antya(sub));
            };
        }

        // Main guna rules.
        let anga = p.get(i)?;
        let n = p.get(i + 1)?;
        if anga.has_text("jAgf") && !vi_cin_nal && !n.has_tag(T::Nit) {
            p.op_term("7.3.85", i, |t| {
                op::antya("ar")(t);
                t.add_tag(T::FlagGuna);
            });
        } else if anga.has_upadha(&*HRASVA) {
            // Main rules.
            // TODO: puganta
            let sub = al::to_guna(anga.upadha()?)?;
            p.op_term("7.3.86", i, |t| {
                op::upadha(sub)(t);
                t.add_tag(T::FlagGuna);
            });
        } else if is_ik {
            p.op_term("7.3.84", i, op_antya_guna);
        }
    }

    Some(())
}

/// Runs rules that are conditioned on a following Sit-pratyaya.
fn try_shiti(p: &mut Prakriya) {
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    if !p.has(i, f::dhatu) {
        return;
    }
    let i_n = match p.find_next_where(i, |t| !t.text.is_empty()) {
        Some(i) => i,
        None => return,
    };

    let sham_adi = &[
        "Samu~", "tamu~", "damu~", "Sramu~", "Bramu~", "kzamU~", "klamu~", "madI~",
    ];

    let pa_ghra = &[
        "pA\\", "GrA\\", "DmA\\", "zWA\\", "mnA\\", "dA\\R", "df\\Si~r", "f\\", "sf\\", "Sa\\dx~",
        "za\\dx~",
    ];

    let piba_jighra = &[
        "piba", "jiGra", "Dama", "tizWa", "mana", "yacCa", "paSya", "fcCa", "DO", "SIya", "sIda",
    ];

    let anga = &p.terms()[i];
    let n = &p.terms()[i_n];
    if !n.has_tag(T::Sit) {
        return;
    }
    let last = p.terms().last().unwrap();

    if anga.has_antya('o') && n.has_u("Syan") {
        p.op_term("7.3.71", i, op::antya(""));
    } else if anga.has_u_in(sham_adi) && n.has_u("Syan") && anga.has_gana(4) {
        // Check ganas to avoid `Bramu~ anavasTAne` (BrAmyati).
        p.op_term("7.3.74", i, op::upadha("A"));
    } else if anga.has_text_in(&["zWiv", "klam"]) {
        // TODO: A-cam
        p.op_term("7.3.75", i, |t| {
            match t.text.as_str() {
                "zWiv" => t.text = CompactString::from("zWIv"),
                "klam" => t.text = CompactString::from("klAm"),
                _ => (),
            };
        });
    } else if anga.has_text("kram") && last.has_tag(T::Parasmaipada) {
        p.op_term("7.3.76", i, op::text("krAm"));
    } else if anga.has_u_in(&["izu~", "ga\\mx~", "ya\\ma~"]) {
        p.op_term("7.3.77", i, op::antya("C"));
    } else if anga.has_u_in(pa_ghra) && !anga.has_gana(2) && !anga.has_gana(3) {
        // Check ganas above to avoid `pA rakzaRe` (pAti), `f gatO` (iyarti)
        let to_piba_jighra = |p: &mut Prakriya| {
            let anga = &p.terms()[i];
            if let Some(needle) = &anga.u {
                if let Some(sub) = op::yatha(needle, pa_ghra, piba_jighra) {
                    p.op_term("7.3.78", i, op::text(sub));
                } else {
                    panic!("No match found for `{}`", anga.text);
                }
            }
        };
        if anga.has_u("sf\\") {
            // sartervegitāyāṃ gatau dhāvādeśam icchanti। anyatra sarati, anusarati
            // ityeva bhavati. (kAzikA)
            p.op_optional("7.3.78", to_piba_jighra);
        } else {
            p.op("7.3.78", to_piba_jighra);
        }
    } else if anga.has_u_in(&["jYA\\", "janI~\\"]) {
        p.op_term("7.3.79", i, op::text("jA"));
    } else if anga.has_u_in(gana::PU_ADI) && (anga.has_gana(5) || anga.has_gana(9)) {
        // All of these dhatus end in vowels.
        p.op_term("7.3.80", i, |t| {
            t.find_and_replace_text("U", "u");
            t.find_and_replace_text("F", "f");
            t.find_and_replace_text("I", "i");
        });
    }
}

/// Runs rules that add nu~m to the base.
///
/// Example: jaBate -> jamBate
///
/// (7.1.58 - 7.1.83)
fn try_add_num_agama(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;

    let last = p.terms().last()?;
    if last.has_text("Am") && last.has_tag(T::Sup) {
        let i_last = p.terms().len() - 1;
        let i_anga = i_last - 1;
        let anga = p.get(i_last - 1)?;
        if anga.has_tag(T::Sarvanama) {
            p.op("7.1.52", |p| op::insert_agama_before(p, i_last, "su~w"));
            it_samjna::run(p, i_last).ok()?;
        } else if anga.has_text("tri") {
            p.op_term("7.1.53", i_anga, op::text("traya"));
        } else if f::is_hrasva(anga) {
            p.op("7.1.54", |p| op::insert_agama_before(p, i_last, "nu~w"));
            it_samjna::run(p, i_last).ok()?;
        } else if anga.has_tag(T::Sat) || anga.has_text("catur") {
            p.op("7.1.55", |p| op::insert_agama_before(p, i_last, "nu~w"));
            it_samjna::run(p, i_last).ok()?;
        }
    }

    // 7.1.58 (idito nuM dhAtoH) is in `dhatu_karya`, so we skip it here.

    let anga = &p.terms()[i];
    let n = p.view(i + 1)?;
    if anga.has_u_in(gana::MUC_ADI) && n.has_u("Sa") {
        p.op_term("7.1.59", i, op::mit("n"));
    } else if anga.has_u_in(gana::TRMPH_ADI) && n.has_u("Sa") {
        p.op_term("7.1.59.v1", i, op::mit("n"));
    } else if anga.has_text_in(&["masj", "naS"]) && n.has_adi(&*JHAL) {
        p.op_term("7.1.60", i, op::mit("n"));
    }

    let anga = &p.terms()[i];
    let n = p.view(i + 1)?;
    let liti = n.has_lakshana("li~w");
    if n.has_adi(&*AC) {
        if anga.has_u_in(&["ra\\Da~", "jaBI~\\"]) {
            if anga.has_u("ra\\Da~") && f::is_it_agama(n.first().unwrap()) && !liti {
                p.step("7.1.62");
            } else {
                p.op_term("7.1.61", i, op::mit("n"));
            }
        } else if anga.has_u("ra\\Ba~\\") && !n.has_u("Sap") && !liti {
            p.op_term("7.1.63", i, op::mit("n"));
        } else if anga.has_u("qula\\Ba~\\z") && !n.has_u("Sap") && !liti {
            // TODO: 7.1.65 - 7.1.69
            p.op_term("7.1.64", i, op::mit("n"));
        }
    }

    let n = p.view(i + 1)?;
    if n.has_tag(T::Sarvanamasthana) {
        let anga = p.view(i)?;
        if anga.any(&[T::udit, T::fdit]) && !anga.has_tag(T::Dhatu) {
            p.op_term("7.1.70", i, op::mit("n"));
        } else if anga.has_tag(T::Napumsaka) && (n.has_adi(&*JHAL) || n.has_adi(&*AC)) {
            p.op_term("7.1.72", i, op::mit("n"));
        } else if anga.has_antya(&*IK) && n.has_adi(&*AC) && n.has_tag(T::Vibhakti) {
            p.op_term("7.1.73", i, op::mit("n"));
        }
    }

    Some(())
}

/// Runs rules that can introduce an `Iw`-agama.
/// Example: bru -> bravIti
///
/// (7.3.93 - 7.3.99)
///
/// Skipped: 7.3.97 ("bahulam chandasi")
/// TODO: 7.3.99 - 100
pub fn iit_agama(p: &mut Prakriya) -> Option<()> {
    let i_sarva = p.find_last(T::Sarvadhatuka)?;
    if i_sarva == 0 {
        return None;
    };
    let i = i_sarva - 1;

    let anga = p.get(i)?;
    let n = p.view(i_sarva)?;

    if n.has_adi(&*HAL) && n.has_tag(T::Sarvadhatuka) {
        let piti = n.has_tag(T::pit);
        let mut rule = None;
        if anga.has_text("brU") && piti {
            rule = Some("7.3.93");
        } else if anga.has_u("yaN") && piti {
            rule = maybe_rule(p, "7.3.94");
        } else if anga.has_u_in(&["tu\\", "ru", "zwu\\Y", "Sam", "ama~"]) {
            rule = maybe_rule(p, "7.3.95");
        } else if f::is_aprkta(n.last()?) {
            if anga.has_u_in(&["asa~", "si~c"]) {
                rule = Some("7.3.96");
            } else if anga.has_u_in(&["rud", "svap", "Svas", "praR", "jakz"]) {
                rule = Some("7.3.98");
            }
        }

        if let Some(rule) = rule {
            p.op(rule, |p| op::insert_agama_after(p, i, "Iw"));
            it_samjna::run(p, i + 1).ok()?;
        }
    }

    Some(())
}

/// Runs rules conditioned on a following sarvadhatuka affix.
///
/// Example: `labh + Ate -> labh + Iyte (-> labhete)`
///
/// (7.2.76 - 7.2.81)
fn try_sarvadhatuke(p: &mut Prakriya) {
    let i = match p.find_last(T::Tin) {
        Some(i) => i,
        None => return,
    };

    if !p.has(i, f::sarvadhatuka) {
        return;
    }

    if p.has(i, f::lakshana("li~N")) {
        // At this stage, all liN verbs will have an Agama (such as yAsu~w) between the
        // dhatu/vikarana and the tin-pratyaya.
        let i_anga = i - 2;
        let i_agama = i - 1;

        if !p.has(i_agama, f::tag(T::Agama)) {
            return;
        }

        let contains_s = |t: &Term| t.text.contains('s');
        if p.has(i_agama, contains_s) || p.has(i, contains_s) {
            p.op("7.2.79", |p| {
                let agama = &mut p.terms_mut()[i_agama];
                agama.text.retain(|c| c != 's');

                let tin = &mut p.terms_mut()[i];
                if tin.has_antya('s') {
                    tin.text.retain(|c| c != 's');
                    tin.text += "s";
                } else {
                    tin.text.retain(|c| c != 's');
                }
            });
        }

        // yAs -> yA due to 7.2.79 above.
        if p.has(i_anga, |t| t.has_antya('a')) && p.has(i_agama, f::text("yA")) {
            p.op_term("7.2.80", i_agama, op::text("Iy"));
        }
    }

    // TODO: not sure where to put this. Not lin.
    if p.has(i - 1, |t| t.has_antya('a')) && p.has(i, |t| t.has_adi('A') && t.has_tag(T::Nit)) {
        p.op_term("7.2.81", i, op::adi("Iy"));
    }
}

/*
fn final_f_and_dirgha(p: Prakriya, index: int):
    c = p.terms[index]
    if not c.text:
        return
    n = TermView.make_pratyaya(p, index)
    if not n:
        return

    kniti = f.is_knit(n)
    akrtsarvadhatukayoh = not n.any(T.SARVADHATUKA, T.KRT)
    shayaklinksu = n.terms[0].u in {"Sa", "yak"} or (
        (p.terms[-1].all("li~N", T.ARDHADHATUKA) and n.adi == "y")
    )
    if c.antya == "f" and shayaklinksu and kniti:
        // nyAsa on 7.4.29:
        //
        //     `ṛ gatiprāpaṇayoḥ` (dhātupāṭhaḥ-936), `ṛ sṛ gatau`
        //     (dhātupāṭhaḥ-1098,1099) - ityetayor bhauvādika-
        //     jauhotyādikayor grahaṇam
        if f.samyogadi(c) or p.has(i, f::text("f":
            op.antya("7.4.29", p, c, "ar")
        else:
            op.antya("7.4.28", p, c, "ri")
    } else if akrtsarvadhatukayoh and (n.terms[0].u == "cvi" or n.adi == "y"):
        if c.antya == "f":
            op.antya("7.4.27", p, c, "rI")
        else:
            if n.terms[0].u == "cvi":
                op.antya("7.4.26", p, c, sounds.dirgha(c.antya))
            else:
                op.antya("7.4.25", p, c, sounds.dirgha(c.antya))


fn nittva(p, index):
    c = p.terms[index]
    if not c.text:
        return
    try:
        n = p.terms[index + 1]
    except IndexError:
        return

    if p.has(i, f::text("go" and n.all(T.SARVANAMASTHANA):
        op.tag("7.1.90", p, n, "R")
    } else if c.antya == "o" and n.all(T.SARVANAMASTHANA):
        op.tag("7.1.90.v1", p, n, "R")
    } else if n.u == "Ral" and n.all(T.UTTAMA):
        if p.allow("7.1.91"):
            n.remove_tags("R")
            p.step("7.1.91")
        else:
            p.decline("7.1.91")
    } else if c.antya == "f" and n.u == "su~" and not n.any(T.SAMBUDDHI):
        op.antya("7.1.94", p, c, "an")
*/

fn try_tas_asti_lopa(p: &mut Prakriya, i: usize) {
    if p.has(i, |t| t.text == "tAs" || f::is_asti(t)) {
        let n = i + 1;
        if p.has(n, |t| t.has_adi('s')) {
            p.op_term("7.4.50", i, op::antya(""));
        } else if p.has(n, |t| t.has_adi('r')) {
            p.op_term("7.4.51", i, op::antya(""));
        } else if p.has(n, |t| t.has_adi('e')) {
            p.op_term("7.4.52", i, op::antya("h"));
        }
    }
}

fn run_for_each_2(p: &mut Prakriya, i: usize) {
    /*
    nittva(p, index)

    if n:
        kniti = n.any("k", "N")
        if c.u == "SIN":
            if kniti and n.adi == "y":
                op.antya("7.4.22", p, c, "ay")
            } else if n.all(T.SARVADHATUKA):
                op.antya("7.4.21", p, c, sounds.guna(c.antya))

    // HACK: check for "dhatu" to avoid processing "yAs"-Agama
    // TODO: don't do this hack.
    if (
        c.antya == "s"
        and c.all(T.DHATU)
        and n
        and n.terms[0].adi == "s"
        and n.all(T.ARDHADHATUKA)
    ):
        op.antya("7.4.49", p, c, "t")
    */

    try_tas_asti_lopa(p, i);

    if let Some(n) = p.view(i + 1) {
        if p.has(i, f::u_in(&["dIDIN", "vevIN"])) && n.has_adi(&*I_U) {
            p.op_term("7.4.53", i, op::antya(""));
        }
    };

    // Must occur before guna and after 7.3.77 (gam -> gacC).
    samhitayam_tuk(p);

    // Vrddhi takes priority over guna. For example, Nic is Ardhadhatuka (guna)
    // and Nit (vrddhi), but it will cause vrddhi if possible.
    try_vrddhi_adesha(p, i);
    try_guna_adesha(p, i);

    /*
    // TODO: 7.4.23-4
    if n:
        final_f_and_dirgha(p, index)
    */
}

fn samhitayam_tuk(p: &mut Prakriya) {
    char_rule(p, xy(|x, y| al::is_hrasva(x) && y == 'C'), |p, _, i| {
        // tena cicchidatuḥ, cicchiduḥ ityatra tukabhyāsasya grahaṇena na
        // gṛhyate iti halādiḥśeṣeṇa na nivartyate
        // -- kAzikA
        if let Some(t) = get_at(p, i) {
            if t.has_tag(T::Abhyasa) {
                return false;
            }
        }

        set_at(p, i + 1, "tC");
        p.step("6.1.73");
        true
    });

    char_rule(p, xy(|x, y| al::is_dirgha(x) && y == 'C'), |p, _, i| {
        set_at(p, i + 1, "tC");
        p.step("6.1.73");
        true
    });
}

/*
fn cajoh_kuh(p: Prakriya, index: int):
    """Conversion of cu~ to ku~ in various contexts.

    (7.3.52 - 7.3.69)
    """
    c = p.terms[index]
    n = TermView.make_pratyaya(p, index)
    if not n:
        return

    // HACK
    n.u = n.terms[0].u

    mapping = {"c": "k", "j": "g", "h": "G"}

    // TODO: do niyama only under the conditions below.
    niyama = None
    if c.adi in s("ku~"):
        niyama = "7.3.59"
    } else if c.text in {"aj", "vraj"}:
        niyama = "7.3.60"
    // TODO: pra-vac
    } else if c.text in {"yaj", "yAc", "ruc", "fc"} and n.u == "Ryat":
        niyama = "7.3.66"
    if niyama:
        p.step(niyama)
        return

    if c.antya in s("c j") and (n.any("G") or n.u == "Ryat"):
        op.antya("7.3.52", p, c, mapping[c.antya])
    } else if p.has(i, f::text("han":
        if n.any("Y", "R") or n.adi == "n":
            op.adi("7.3.54", p, c, "G")
        } else if c.all(T.ABHYASTA):
            op.adi("7.3.55", p, c, "G")
    } else if p.has(i, f::text("hi" and c.all(T.ABHYASTA) and n.u != "caN":
        op.adi("7.3.56", p, c, "G")

    sanlitoh = n.u == "san" or n.all("li~w")
    if p.has(i, f::text("ji" and c.gana == 1 and c.all(T.ABHYASTA) and sanlitoh:
        op.adi("7.3.57", p, c, "g")
    } else if p.has(i, f::text("ci" and c.all(T.ABHYASTA) and sanlitoh:
        p.op_optional(op.adi, "7.3.58", p, c, "k")
*/

fn dhatu_rt_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    if !p.has(i, f::tag(T::Dhatu)) {
        return None;
    }

    let dhatu = p.get(i)?;

    if dhatu.has_antya('F') {
        if dhatu.has_upadha(&*OSHTHYA) {
            p.op_term("7.1.102", i, op::antya("ur"));
        } else {
            p.op_term("7.1.100", i, op::antya("ir"));
        }
    }

    Some(())
    // HACK: 7.1.101 is performed before dvitva.
}

/// Runs rules that lengthen the last `a` of the anga when certain suffixes follow.
///
/// Example: `Bava + mi -> BavAmi`
///
/// (7.3.101 - 7.3.111)
fn try_ato_dirgha(p: &mut Prakriya, i: usize) {
    let n = match p.view(i + 1) {
        Some(n) => n,
        None => return,
    };

    let to_guna = |t: &mut Term| {
        let last = al::to_guna(t.antya().unwrap()).unwrap();
        op::antya(last)(t);
    };
    let ends_in_a = |t: &Term| t.has_antya('a');

    if n.has_tag(T::Sarvadhatuka) {
        if p.has(i, ends_in_a) && YANY.contains_opt(n.adi()) {
            p.op_term("7.3.101", i, op::antya("A"));
        }
    } else if n.has_tag(T::Sup) {
        if p.has(i, ends_in_a) {
            if n.has_tag(T::Bahuvacana) && JHAL.contains_opt(n.adi()) {
                p.op_term("7.3.103", i, op::antya("e"));
            } else if YANY.contains_opt(n.adi()) {
                p.op_term("7.3.102", i, op::antya("A"));
            } else if n.slice()[0].text == "os" {
                p.op_term("7.3.104", i, op::antya("e"));
            }
        }

        let c = &p.terms()[i];
        let n = match p.view(i + 1) {
            Some(n) => n,
            None => return,
        };
        if al::is_hrasva(c.antya().unwrap()) && c.antya() != Some('a') {
            if n.has_tag(T::Sambuddhi) {
                p.op_term("7.3.108", i, to_guna);
            } else if n.has_u("jas") {
                p.op_term("7.3.109", i, to_guna);
            } else if p.has(i, |t| t.has_antya('f'))
                && (n.has_u("Ni") || n.has_tag(T::Sarvanamasthana))
            {
                p.op_term("7.3.110", i, to_guna);
            } else if p.has(i, f::tag(T::Ghi)) && n.has_tag(T::Nit) {
                p.op_term("7.3.111", i, to_guna);
            }
        }
    }
}

/// Runs rules that cause vrddhi of `sic`-pratyaya.
///
/// sic-vrddhi applies only for parasmaipada endings. This function must follow `it_agama` due to
/// 7.2.4.
///
/// (7.2.1 - 7.2.7)
fn try_sic_vrddhi(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last(T::Dhatu)?;

    let vikarana = p.view(i + 1).unwrap();
    let (i_it, i_sic) = match vikarana.slice().len() {
        1 => (None, vikarana.start()),
        2 => (Some(vikarana.start()), vikarana.end()),
        _ => return None,
    };
    let i_tin = p.terms().len() - 1;

    let it = if i_it.is_none() {
        None
    } else {
        p.get(i_it.unwrap())
    };
    let sic = p.get(i_sic)?;
    let tin = p.get(i_tin)?;
    if !(sic.has_u("si~c") && tin.has_tag(T::Parasmaipada)) {
        return None;
    }

    // A dhatu followed by ArdhadhAtuka has its final `a` deleted by 6.4.48.
    // But in the context of the rules below, we should ignore the effect of
    // 6.4.48 per 1.1.57 (acaH parasmin pUrvavidhau) and cause no changes for
    // such roots. (Motivating examples: agopAyIt, avadhIt)
    if p.has_tag(T::FlagAtLopa) {
        return None;
    }

    // 1.2.1 -- skip vrddhi for these sounds
    // HACK: check only sic, atidesha should not apply to it.
    if let Some(it) = it {
        if it.has_tag(T::Nit) || sic.has_tag(T::Nit) {
            return None;
        }
    }

    let dhatu = p.get(i)?;
    if dhatu.has_upadha('a') && (dhatu.has_antya('l') | dhatu.has_antya('r')) {
        let sub = al::to_vrddhi(dhatu.upadha()?)?;
        // apavAda to 7.2.7 below, so run this first.
        p.op_term("7.2.2", i, op::upadha(sub));
    }

    let mut block = None;

    let dhatu = p.get(i)?;
    let it = if i_it.is_none() {
        None
    } else {
        p.get(i_it.unwrap())
    };
    // TODO: don't add hack for tug-Agama. Should reorder.
    if it.is_some() {
        // TODO: other cases
        let antya = dhatu.antya()?;
        if "hmy".chars().any(|c| c == antya)
            || dhatu.has_text_in(&["kzaR", "Svas", "jAgf", "Svi"])
            || dhatu.has_tag(T::edit)
        {
            block = Some("7.2.5")
        } else if dhatu.has_text("UrRu") {
            block = maybe_rule(p, "7.2.6")
        } else if dhatu.has_adi(&*HAL) && dhatu.has_upadha('a') && !dhatu.has_antya('C') {
            block = maybe_rule(p, "7.2.7")
        } else if dhatu.has_antya(&*HAL) {
            block = maybe_rule(p, "7.2.4")
        }
    };

    if let Some(c) = block {
        p.step(c);
        return None;
    }

    let dhatu = p.get(i)?;
    if dhatu.has_antya(&*AC) {
        let sub = al::to_vrddhi(dhatu.antya()?)?;
        p.op_term("7.2.1", i, op::antya(sub));
    } else if f::is_samyoganta(dhatu) {
        // 7.2.3 applies to the final vowel generally, even if samyoganta
        let n_3 = dhatu.get(dhatu.text.len() - 3)?;
        p.op_term("7.2.3", i, |t| {
            if AC.contains_char(n_3) {
                let sub = al::to_vrddhi(n_3).unwrap();
                let i = t.text.len() - 3;
                t.text.replace_range(i..=i, sub);
            } else {
                // e.g. "mansj", "pracC"
                t.find_and_replace_text("a", "A");
            }
        });
    } else {
        let sub = al::to_vrddhi(dhatu.upadha()?)?;
        p.op_term("7.2.3", i, op::upadha(sub));
    }

    Some(())
}

fn try_cani_before_guna(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;

    let dhatu = p.get(i)?;
    let is_nici = match p.get(i + 1) {
        Some(t) => t.has_u_in(&["Ric", "RiN"]),
        None => false,
    };
    let is_cani = match p.get(i + 2) {
        Some(t) => t.has_u("caN"),
        None => false,
    };

    // 7.4.7 blocks guna.
    if dhatu.has_upadha(&*FF) && is_nici && is_cani {
        p.op_term("7.4.7", i, |t| {
            op::upadha("f")(t);
            t.add_tag(T::FlagGunaApavada);
        });
    }

    let dhatu = p.get(i)?;
    let last = p.terms().last()?;
    if dhatu.has_text_in(&["SF", "dF", "pF"]) && last.has_lakshana("li~w") && !dhatu.has_gana(10) {
        p.op_term("7.4.12", i, |t| {
            op::antya("f")(t);
            t.add_tag(T::FlagGunaApavada);
        });
    }

    Some(())
}

pub fn hacky_before_dvitva(p: &mut Prakriya) {
    try_cani_before_guna(p);

    for i in 0..p.terms().len() {
        if p.has(i, |t| t.has_tag(T::Dhatu) && t.has_upadha('F')) {
            p.op_term("7.1.101", i, op::upadha("ir"));
        }
    }
}

/// Rules conditioned on a following caN-pratyaya (luN-vikarana).
///
/// (7.4.1 - 7.4.6)
fn try_cani_after_guna(p: &mut Prakriya) -> Option<()> {
    let i = p.find_first(T::Dhatu)?;
    let i_ni = p.find_next_where(i, |t| t.has_u_in(&["Ric", "RiG"]))?;
    let i_can = i_ni + 1;

    if !p.has(i_can, f::u("caN")) {
        return None;
    }

    let dhatu = p.get(i)?;

    // Ignore 'f' because it is handled by 7.4.7.
    if dhatu.has_upadha(&*AC) && !dhatu.has_upadha(&*FF) {
        let sub = al::to_hrasva(dhatu.upadha()?)?;
        if dhatu.any(&[T::FlagAtLopa, T::fdit]) || dhatu.has_text("SAs") {
            p.step("7.4.2");
        } else if dhatu.upadha()? != sub {
            p.op_term("7.4.1", i, op::upadha(&sub.to_string()));
        }
    } else if p.has(i + 1, f::tag(T::Agama)) && dhatu.has_antya(&*AC) {
        // TODO: not sure what this rule is meant to be.
        /*
        let sub = al::to_hrasva(dhatu.antya()?)?;
        op.antya("7.4.1", p, c, res)
        p.op_term("7.4.1", i, op::antya(&sub.to_string()));
        */
    };

    Some(())
}

/// Run rules that condition on a following liT-pratyaya.
///
/// (7.4.9 - 7.4.12)
fn liti(p: &mut Prakriya) {
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return,
    };
    let i_tin = p.terms().len() - 1;
    if !p.has(i, f::lakshana("li~w")) {
        return;
    }

    let do_ar_guna = |t: &mut Term| {
        t.add_tag(T::FlagGuna);
        op::antya("ar")(t);
    };

    let can_guna = p.has(i_tin, |t| !t.has_tag(T::Rit));
    if p.has(i, |t| t.has_antya('f') && f::is_samyogadi(t)) && can_guna {
        p.op_term("7.4.10", i, do_ar_guna);
    } else if p.has(i, |t| t.has_antya('F') || t.has_u_in(&["fCa~", "f\\"])) && can_guna {
        p.op_term("7.4.11", i, do_ar_guna);
    }
}

/// Runs rules conditioned on a following aN-pratyaya (luN-vikarana).
///
/// (7.4.16 - 7.4.20)
fn ani(p: &mut Prakriya) {
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    let n = i + 1;
    if !p.has(n, f::u("aN")) {
        return;
    }

    if p.has(i, |t| t.has_antya(&*FF) || t.text == "dfS") {
        if p.has(i, f::text("dfS")) {
            p.op_term("7.4.16", i, op::text("darS"));
        } else {
            p.op_term("7.4.16", i, op::antya("ar"));
        }
    } else if p.has(i, f::u("asu~")) {
        p.op("7.4.17", |p| {
            p.insert_after(i, Term::make_agama("Tu~k"));
            it_samjna::run(p, i + 1).unwrap();
        });
    } else if p.has(i, f::text("Svi")) {
        p.op_term("7.4.18", i, op::antya("a"));
    } else if p.has(i, f::text("pat")) {
        p.op_term("7.4.19", i, op::mit("p"));
    } else if p.has(i, f::text("vac")) {
        p.op_term("7.4.20", i, op::mit("u"));
    }
}

fn try_ksa_lopa(p: &mut Prakriya) -> Option<()> {
    let i_dhatu = p.find_last(T::Dhatu)?;
    let i = i_dhatu + 1;
    let i_tin = i_dhatu + 2;

    if p.has(i, f::u("ksa")) {
        if p.has(i + 1, |t| t.has_adi(&*AC)) {
            p.op_term("7.2.72", i, op::antya(""));
        }
        if p.has(i_dhatu, f::text_in(&["duh", "dih", "lih", "guh"])) && p.has(i_tin, f::atmanepada)
        {
            p.op_optional("7.3.73", op::t(i, op::antya("")));
        }
    }

    Some(())
}

/*
fn nau(p: Prakriya, index: int):
    c = p.terms[index]
    n = TermView.make_pratyaya(p, index)
    if not n:
        return

    n.u = n.terms[0].u
    if n.u not in {"Ric", "RiN"}:
        return

    // HACK: avoid adding augments for ajAdi-dvitva.
    if n.any(T.ABHYASA):
        return

    if c.text in {"f", "hrI", "vlI", "rI", "knUy", "kzmAy"} or c.antya == "A":
        op.insert_agama_after_by_term("7.3.36", p, c, "pu~k")
    } else if c.text in {"zA", "DA", "sA", "hvA", "vyA", "pA", "pE"} or c.u == "ve\\Y":
        do = True
        if c.u == "ve\\Y":
            if p.op_optional(op.insert_agama_after_by_term, "7.3.38", p, c, "ju~k"):
                do = False
        if do:
            op.insert_agama_after_by_term("7.3.37", p, c, "yu~k")
    } else if p.has(i, f::text("pA" and c.gana == 2:
        op.insert_agama_after_by_term("7.3.37.v1", p, c, "lu~k")
    // TODO: 7.3.39
    } else if c.text in {"prI", "DU"}:
        // Optional per Haradatta (see commentary on prIY in siddhAnta-kaumudI)
        p.op_optional(op.insert_agama_after_by_term, "7.3.37.v2", p, c, "nu~k")
    // TODO: 7.3.39
    } else if p.has(i, f::text("BI")) {
        p.op_optional(op.insert_agama_after_by_term, "7.3.40", p, c, "zu~k")
    } else if p.has(i, f::text("sPAy")) {
        op.antya("7.3.41", p, c, "v")
    } else if p.has(i, f::text("Sad")) {
        p.op_optional(op.antya, "7.3.42", p, c, "t")
    } else if p.has(i, f::text("ruh")) {
        p.op_optional(op.antya, "7.3.43", p, c, "p")
*/

fn try_anga_adesha_before_vibhakti(p: &mut Prakriya) -> Option<()> {
    let i_sup = p.find_last(T::Sup)?;
    if i_sup == 0 {
        return None;
    }
    let i = i_sup - 1;
    let anga = p.get(i)?;
    let sup = p.get(i_sup)?;

    if anga.has_text("rE") && anga.has_adi(&*HAL) {
        p.op_term("7.2.85", i, op::antya("A"));
    } else if anga.has_text_in(&["yuzmad", "asmad"]) {
        if sup.has_adi(&*AC) {
            p.op_term("7.2.89", i, op::antya("y"));
        } else if !sup.text.is_empty() {
            // FIXME: this is not quite right.
            p.op_term("7.2.86", i, op::antya("A"));
        } else if sup.has_tag(T::V2) {
            p.op_term("7.2.87", i, op::antya("A"));
        } else if sup.all(&[T::V1, T::Dvivacana]) {
            p.op_term("7.2.88", i, op::antya("A"));
        } else {
            p.op_term("7.2.90", i, op::antya(""));
        }
    }

    Some(())
}

pub fn run_remainder(p: &mut Prakriya) {
    sup_adesha::run(p);

    try_anga_adesha_before_vibhakti(p);

    // `try_ksa_lopa` must run before `try_sarvadhatuke` so that at-lopa (since `ksa` ends in `a`)
    // has a chance to take effect and prevent "ato yeyaH" (7.2.80).
    try_ksa_lopa(p);
    try_sarvadhatuke(p);
    try_shiti(p);

    // Must come before asiddhavat rule 6.4.78 (e.g. "iyarti", ekahalmadhya)
    abhyasasya::run(p);

    for i in 0..p.terms().len() {
        asiddhavat::run_before_guna(p, i);
    }

    // num-Agama must come after asiddhavat rule 6.2.24, which causes na-lopa.
    try_add_num_agama(p);

    try_sic_vrddhi(p);

    for index in 0..p.terms().len() {
        // nau(p, index);
        // cajoh_kuh(p, index);
        run_for_each_2(p, index);
    }

    liti(p);
    // Rules for various lun-vikaranas.
    ani(p);

    // Asiddhavat must run before cani for "Ner aniTi"
    asiddhavat::run_for_ni(p);

    try_cani_after_guna(p);
    abhyasasya::run_for_sani_cani(p);

    for index in 0..p.terms().len() {
        asiddhavat::run_after_guna(p, index);
        dhatu_rt_adesha(p, index);
        try_ato_dirgha(p, index);
    }

    asiddhavat::run_dirgha(p);
}
