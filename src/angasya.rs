//! angasya
//! =======
//! (6.4.1 - end of 7.4)
//!
//! Rules that modify the sounds and terms in an aṅga.
//!
//! This section of the text is massive, so we break it down into several smaller prakaraṇas.

use crate::abhyasasya;
use crate::asiddhavat;
use crate::constants::Tag as T;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::s;
use crate::sup_adesha;
use crate::term::{Term, TermView};

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

    let to_at = |t: &mut Term| t.text = t.text.replace('J', "at");
    let to_ant = |t: &mut Term| t.text = t.text.replace('J', "ant");

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

/// Applies rules that replace the jha-pratyaya with the appropriate sounds.
/// (7.1.1 - 7.1.7)
pub fn try_pratyaya_adesha(p: &mut Prakriya) {
    let i = p.terms().len() - 1;
    let t = &p.terms()[i];

    if t.has_text(&["yu~", "vu~"]) {
        if t.text == "yu~" {
            p.op("7.1.1", op::t(i, op::text("ana")));
        } else {
            p.op("7.1.1", op::t(i, op::text("aka")));
        }
    } else if t.has_adi(&s("P Q K C G")) {
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
    } else if p.has(i, |t| t.has_tag(T::Tin) && t.has_text(&["tu", "hi"])) {
        // N is to block pit-guNa, not for replacement of the last letter.
        p.op_optional("7.1.35", |p| op::upadesha(p, i, "tAta~N"));
    }
}

/*
fn pratyaya_adesha(p: Prakriya):
    """Rules that substitute the pratyaya.

    (7.1.1 - 7.1.35)
    """
    tin = p.terms[-1]
    if not tin.all(T.TIN):
        return

    ps = [t for t in p.terms[:-1] if t.text]
    prev = ps[-1]

    if prev.antya == "A" and tin.u == "Ral":
        op.upadesha("7.1.34", p, tin, "O")

    // Run 3.1.83 here because it has no clear place otherwise.
    if prev.u == "SnA" and tin.text == "hi" and ps[-2].antya in s("hal"):
        op.upadesha("3.1.83", p, prev, "SAnac")
*/

fn try_vrddhi_adesha(p: &mut Prakriya, i: usize) {
    if p.has(i, f::tag(T::FlagGunaApavada)) {
        return;
    }
    let n = match p.view(i) {
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
fn try_nnit_vrddhi(p: &mut Prakriya, i: usize) {
    /*
    if not (n and n.any("Y", "R") and n.all(T.PRATYAYA)):
        return

    if c.any(T.F_AT_LOPA, T.F_GUNA_APAVADA):
        // at-lopa prevents vrddhi
        return

    // HACK
    n.u = n.terms[0].u

    if not (f.can_use_guna(c, n) or n.u == "RiN"):
        return

    na = None
    if c.text in {"jan", "vaD"} and (n.u == "ciR" or n.all(T.KRT)):
        na = "7.3.35"

    if na:
        p.step(na)
    // By "acaH" we should ignore iko guNavRddhI (vye -> vivyAya)
    } else if c.antya in s("ac"):
        op.antya("7.2.115", p, c, sounds.vrddhi(c.antya))
    } else if c.upadha == "a":
        op.upadha("7.2.116", p, c, "A")
    */
}

fn try_guna_adesha(p: &mut Prakriya, i: usize) {
    if p.has(i, f::tag(T::Agama)) {
        return;
    }
    if p.has(i, |t| t.any(&[T::FlagAtLopa, T::FlagGunaApavada])) {
        return;
    }

    let n = match TermView::new(p.terms(), i + 1) {
        Some(n) => n,
        None => return,
    };

    let is_sarva_ardha = n.any(&[T::Sarvadhatuka, T::Ardhadhatuka]);
    /*
    let can_guna = f.can_use_guna(c, n)
    let piti_sarvadhatuke = n.all("p", T.SARVADHATUKA)

    if (
        c.antya == "u"
        && n.terms[0].any("luk")
        && n.adi in s("hal")
        && piti_sarvadhatuke
        && can_guna
    ) {
        if c.u == "UrRuY" {
            if p.allow("7.3.90") {
                p.step("7.3.90")
            } else {
                p.decline("7.3.90")
                op.antya("7.3.89", p, c, sounds.vrddhi(c.antya))
            }
        } else {
            op.antya("7.3.89", p, c, sounds.vrddhi(c.antya))
        }

    if p.has(i, f::text("mid" && n.any("S") {
        c.add_tags(T.F_GUNA)
        op.text("7.3.82", p, c, "med")

    } else if n.first_non_empty.u == "jus" && c.antya in s("ik") {
        c.add_tags(T.F_GUNA)
        op.antya("7.3.83", p, c, sounds.guna(c.antya))

    } else if p.has(i, f::text("tfnah" && n.adi in s("hal") && piti_sarvadhatuke {
        op.mit("7.3.92", p, c, "i")

    // General case
    } else if can_guna && sarva_ardha && c.antya in s("ac") {
    */
    let can_guna = |opt: Option<char>| opt.map(|c| al::to_guna(c).is_some()).unwrap_or(false);
    if is_sarva_ardha {
        if p.has(i, f::text("jAgf"))
            && !n.slice()[0].has_u_in(&["kvip", "ciN"])
            && !n.has_tag(T::Nit)
        {
            p.op("7.3.85", |p| {
                p.set(i, op::add_tag(T::FlagGuna));
                p.set(i, op::antya("ar"));
            });
        } else if p.has(i, f::text_in(&["BU", "sU"])) && n.all(&[T::Tin, T::Sarvadhatuka, T::pit]) {
            p.step("7.3.88");
        } else if p.has(i, |t| can_guna(t.antya())) {
            let guna = al::to_guna(p.terms()[i].antya().unwrap()).unwrap();
            p.op("7.3.84", |p| {
                p.set(i, op::add_tag(T::FlagGuna));
                p.set(i, op::antya(guna));
            });
        }
    }

    /*
    // puganta-laghu-upadha (TODO: puk)
    } else if can_guna && sarva_ardha && c.upadha in sounds.HRASVA {
    */
    // HACK: Asiddhavat, but this blocks guna.
    // TODO: move this to asiddhavat && add no_guna tag.
    let n = TermView::new(p.terms(), i + 1).unwrap();
    if p.has(i, f::text("guh")) && s("ac").contains_opt(n.adi()) {
        p.op_term("6.4.89", i, op::upadha("U"));
    } else if p.has(i, f::u_in(&["Divi~", "kfvi~"])) {
        // Per commentary on 3.1.81, make an exception for dhinv and kRNv.
    } else if p.has(i, f::tag(T::Abhyasta))
        && n.all(&[T::pit, T::Sarvadhatuka])
        && s("ac").contains_opt(n.adi())
    {
        // e.g. nenijAma
        p.step("7.3.87")
    } else if p.has(i, |t| {
        can_guna(t.upadha()) && al::is_hrasva(t.upadha().unwrap())
    }) {
        let upadha = p.terms()[i].upadha().unwrap();
        p.op("7.3.86", |p| {
            let guna = al::to_guna(upadha).unwrap();
            p.set(i, op::add_tag(T::FlagGuna));
            p.set(i, op::upadha(guna));
        });
    }
    /*
    }
    */
}

/*
fn shiti(p: Prakriya, index: int):
    c = p.terms[index]
    ns = [u for u in p.terms[index + 1 :] if u.text]
    if not ns:
        return
    n = ns[0]

    if not n.all("S"):
        return

    if c.antya == "o" and n.u == "Syan":
        op.antya("7.3.71", p, c, "")

    sham_adi = (
        "Samu~",
        "tamu~",
        "damu~",
        "Sramu~",
        "Bramu~",
        "kzamU~",
        "klamu~",
        "madI~",
    )
    // Need gana check to avoid including Bramu~ from bhU-gaNa
    if c.u in sham_adi and n.u == "Syan" and c.gana == 4:
        c.text = c.text.replace("a", "A")
        p.step("7.3.74")

    pa_ghra = (
        "pA\\",
        "GrA\\",
        "DmA\\",
        "zWA\\",
        "mnA\\",
        "dA\\R",
        "df\\Si~r",
        "f\\",
        "sf\\",
        "Sa\\dx~",
        "za\\dx~",
    )

    // 7.3.78
    if c.all(T.DHATU) and c.u in pa_ghra and c.gana not in {2, 3}:
        piba_jighra = (
            "piba",
            "jiGra",
            "Dama",
            "tizWa",
            "mana",
            "yacCa",
            "paSya",
            "fcCa",
            "DO",
            "SIya",
            "sIda",
        )
        mapping = dict(zip(pa_ghra, piba_jighra))
        // sartervegitāyāṃ gatau dhāvādeśam icchanti। anyatra sarati, anusarati
        // ityeva bhavati. (kAzikA)
        if c.u == "sf\\":
            p.op_optional(op.text, "7.3.78", p, c, mapping[c.u])
        else:
            op.text("7.3.78", p, c, mapping[c.u])

    } else if c.u in ("jYA\\", "janI~\\"):
        op.text("7.3.79", p, c, "jA")
    } else if c.u in PU_ADI and c.gana in {5, 9}:
        c.text = c.text.replace("U", "u").replace("F", "f").replace("I", "i")
        p.step("7.3.80")

    // TODO: A-cam
    if c.text in ("zWiv", "klam"):
        c.text = op.yatha(c.text, ("zWiv", "klam"), ("zWIv", "klAm"))
        p.step("7.3.75")
    } else if p.has(i, f::text("kram" and p.terms[-1].all("parasmaipada"):
        op.text("7.3.76", p, c, "krAm")
    } else if c.u in ("izu~", "ga\\mx~", "ya\\ma~"):
        op.antya("7.3.77", p, c, "C")


fn num_agama(p: Prakriya, index: int):
    """Rules that add nu~m to the base.

    (7.1.58 - 7.1.83)

    :param p:
    :param index:
    """
    c = p.terms[index]
    n = TermView.make_pratyaya(p, index)
    if not n:
        return

    last = p.terms[-1]
    if last.text == "Am":
        prev = p.terms[-2]
        if prev.all(T.SARVANAMA):
            op.insert_agama_before("7.1.52", p, last, "su~w")
        } else if prev.text == "tri":
            op.text("7.1.53", p, "traya")
        // TODO: nadI, Ap
        } else if f.is_hrasva(prev):
            op.insert_agama_before("7.1.54", p, last, "nu~w")
        } else if prev.any(T.SAT) or prev.text == "catur":
            op.insert_agama_before("7.1.55", p, last, "nu~w")

    // 7.1.58 (idito nuM dhAtoH) is in `dhatu_karya`

    if c.u in MUC_ADI and n.terms[0].u == "Sa":
        op.mit("7.1.59", p, c, "n")
    } else if c.u in TRMPH_ADI and n.terms[0].u == "Sa":
        op.mit("7.1.59.v1", p, c, "n")
    } else if c.text in ("masj", "naS") and n.adi in s("Jal"):
        op.mit("7.1.60", p, c, "n")

    liti = n.any("li~w")
    if n.adi in s("ac"):
        if c.u in ("ra\\Da~", "jaBI~\\"):
            if c.u == "ra\\Da~" and f.is_it_agama(n.terms[0]) and not liti:
                p.step("7.1.62")
            else:
                op.mit("7.1.61", p, c, "n")
        } else if c.u == "ra\\Ba~\\" and n.terms[0].u != "Sap" and not liti:
            op.mit("7.1.63", p, c, "n")
        } else if c.u == "qula\\Ba~\\z" and n.terms[0].u != "Sap" and not liti:
            // TODO: 7.1.65 - 7.1.69
            op.mit("7.1.64", p, c, "n")

    if n.any(T.SARVANAMASTHANA):
        // TODO: aYc
        if c.any("u", "f") and not c.any(T.DHATU):
            op.mit("7.1.70", p, c, "n")
        if c.any(T.NAPUMSAKA) and n.adi in s("Jal ac"):
            op.mit("7.1.72", p, c, "n")
        if c.any in s("ik") and n.adi in s("ac") and n.any(T.VIBHAKTI):
            op.mit("7.1.73", p, c, "n")


fn iit_agama(p: Prakriya):
    for index, _ in enumerate(p.terms):
        c = p.terms[index]
        n = TermView.make_pratyaya(p, index)

        if not c.text or not n:
            continue

        // Prevent loops
        // TODO: find a cleaner solution to this.
        if c.u == "Iw":
            return

        sarva = n.all(T.SARVADHATUKA)
        hali = n.adi in s("hal")
        if sarva and hali:
            piti = n.all("p")
            if piti and p.has(i, f::text("brU":
                // i + 1 to skip luk
                op.insert_agama_after("7.3.93", p, index + 1, "Iw")
            } else if piti and c.u == "yaN":
                p.op_optional(op.insert_agama_after, "7.3.94", p, index, "Iw")
            } else if c.u in ("tu\\", "ru", "zwu\\Y", "Sam", "ama~"):
                p.op_optional(op.insert_agama_after, "7.3.95", p, index, "Iw")
            } else if c.u in {"asa~", "si~c"} and f.is_aprkta(n):
                op.insert_agama_after("7.3.96", p, index, "Iw")

    _, dhatu = p.find_last(T.DHATU)
    if not dhatu:
        return
    last = p.terms[-1]
    if dhatu.u in {"rud", "svap", "Svas", "praR", "jakz"} and f.is_aprkta(last):
        if p.allow("7.3.99"):
            op.insert_agama_after("7.3.96", p, index, "Iw")
        else:
            p.decline("7.3.99")
            op.insert_agama_after("7.3.98", p, index, "Iw")
*/

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
                agama.text = agama.text.replace('s', "");

                let tin = &mut p.terms_mut()[i];
                if tin.has_antya('s') {
                    tin.text = tin.text.replace('s', "") + "s";
                } else {
                    tin.text = tin.text.replace('s', "");
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

fn run_for_each_2(p: &mut Prakriya, index: usize) {
    /*
    terms = p.terms

    c = terms[index]
    if not c.text:
        return
    n = TermView.make_pratyaya(p, index)

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

    try_tas_asti_lopa(p, index);

    /*
    if c.u in ("dIDIN", "vevIN") and n.adi in s("i u"):
        op.antya("7.4.53", p, c, "")

    // Must occur before guna and after 7.3.77 (gam -> gacC).
    samhitayam_tuk(p)

    // Vrddhi takes priority over guna. For example, Nic is Ardhadhatuka (guna)
    // and Nit (vrddhi), but it will cause vrddhi if possible.
    */
    try_vrddhi_adesha(p, index);
    try_guna_adesha(p, index);

    /*
    // TODO: 7.4.23-4
    if n:
        final_f_and_dirgha(p, index)
    */
}

/*
fn samhitayam_tuk(p: Prakriya):
    view = StringView(p.terms)
    vtext = view.text

    for match in re.finditer("[aiufx](C)", vtext):
        index = match.span(1)[0]
        term = view.term_for_index(index)
        if term.any(T.ABHYASA):
            // tena cicchadatuḥ, cicchiduḥ ityatra tukabhyāsasya grahaṇena na
            // gṛhyate iti halādiḥśeṣeṇa na nivartyate
            // -- kAzikA
            pass
        else:
            view[match.span(1)[0]] = "tC"
            p.step("6.1.73")

    match = re.search("[AIUFXeEoO](C)", vtext)
    if match:
        view[match.span(1)[0]] = "tC"
        p.step("6.1.75")


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

fn dhatu_rt_adesha(p: &mut Prakriya, i: usize) {
    if !p.has(i, f::tag(T::Dhatu)) {
        return;
    }

    if p.has(i, |t| t.has_antya('F')) {
        if p.has(i, f::upadha("pu~ v")) {
            p.op_term("7.1.102", i, op::antya("ur"));
        } else {
            p.op_term("7.1.100", i, op::antya("ir"));
        }
    }
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
        if p.has(i, ends_in_a) && s("yaY").contains_opt(n.adi()) {
            p.op_term("7.3.101", i, op::antya("A"));
        }
    } else if n.has_tag(T::Sup) {
        if p.has(i, ends_in_a) {
            if n.has_tag(T::Bahuvacana) && s("Jal").contains_opt(n.adi()) {
                p.op_term("7.3.103", i, op::antya("e"));
            } else if s("yaY").contains_opt(n.adi()) {
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

/*
fn optional_rule(rule: str, p: Prakriya):
    if p.allow(rule):
        return rule
    else:
        p.decline(rule)
        return None
*/

/// Runs rules that cause vrddhi of `sic`-pratyaya.
///
/// sic-vrddhi applies only for parasmaipada endings. This function must follow `it_agama` due to
/// 7.2.4.
///
/// (7.2.1 - 7.2.7)
fn try_sic_vrddhi(p: &mut Prakriya) {
    /*
    try:
        i, dhatu = p.find_last(T.DHATU)
        if not dhatu:
            return

        tin = p.terms[-1]

        x, y = p.terms[i + 1 : i + 3]
        if x.u == "iw":
            it, sic = x, y
        else:
            it = None
            sic = x
    except (IndexError, ValueError):
        return

    if not (dhatu and sic.u == "si~c" and tin.all(T.PARASMAIPADA)):
        return

    // A dhatu followed by ArdhadhAtuka has its final `a` deleted by 6.4.48.
    // But in the context of the rules below, we should ignore the effect of
    // 6.4.48 per 1.1.57 (acaH parasmin pUrvavidhau) and cause no changes for
    // such roots. (Motivating examples: agopAyIt, avadhIt)
    if p.all(T.F_AT_LOPA):
        return

    // 1.2.1 -- skip vrddhi for these sounds
    // HACK: check only sic, atidesha should not apply to it.
    if (it and it.any("N")) or sic.any("N"):
        return

    if dhatu.upadha == "a" and dhatu.antya in s("l r"):
        // apavAda to 7.2.7 below, so run this first.
        op.upadha("7.2.2", p, dhatu, sounds.vrddhi(dhatu.upadha))
        return

    block_rule = None
    // TODO: don't add hack for tug-Agama. Should reorder.
    if it:
        // TODO: other cases
        if (
            dhatu.antya in s("h m y")
            or dhatu.text in {"kzaR", "Svas", "jAgf", "Svi"}
            or dhatu.all("e")
        ):
            block_rule = "7.2.5"
        } else if dhatu.text == "UrRu":
            block_rule = optional_rule("7.2.6", p)
        } else if dhatu.adi in s("hal") and dhatu.upadha == "a" and dhatu.antya != "C":
            block_rule = optional_rule("7.2.7", p)
        // Base case
        } else if dhatu.antya in s("hal"):
            block_rule = "7.2.4"

    if block_rule:
        p.step(block_rule)
        return

    if dhatu.antya in s("ac"):
        op.antya("7.2.1", p, dhatu, sounds.vrddhi(dhatu.antya))
    } else if f.samyoganta(dhatu):
        // 7.2.3 applies to the final vowel generally, even if samyoganta
        text = dhatu.text
        if text[-3] in s("ac"):
            dhatu.text = text[:-3] + sounds.vrddhi(text[-3]) + text[-2:]
        else:
            // e.g. "mansj", "pracC"
            dhatu.text = dhatu.text.replace("a", "A")
        p.step("7.2.3")
    else:
        op.upadha("7.2.3", p, dhatu, sounds.vrddhi(dhatu.upadha))
    */
}

/*
fn cani_before_guna(p: Prakriya):
    index, c = p.find_first(T.DHATU)
    if not c:
        return

    try:
        nic = p.terms[index + 1]
        nici = nic.u in ("Ric", "RiN")
    except IndexError:
        nici = False
    try:
        can = p.terms[index + 2]
        cani = can.u == "caN"
    except IndexError:
        cani = False

    // 7.4.7 blocks guna.
    if c.upadha in s("f") and nici and cani:
        if p.allow("7.4.7"):
            c.add_tags(T.F_GUNA_APAVADA)
            op.upadha("7.4.7", p, c, "f")
        else:
            p.decline("7.4.7")

    last = p.terms[-1]
    if c.text in {"SF", "dF", "pF"} and last.any("li~w") and c.gana != 10:
        if p.allow("7.4.12"):
            c.add_tags(T.F_GUNA_APAVADA)
            op.antya("7.4.12", p, c, "f")
        else:
            p.decline("7.4.12")


fn hacky_before_dvitva(p: Prakriya):
    cani_before_guna(p)

    for c in p.terms:
        if c.any(T.DHATU) and c.upadha == "F":
            op.upadha("7.1.101", p, c, "ir")


fn cani_after_guna(p: Prakriya):
    """Rules conditioned on a following caN-pratyaya (luN-vikarana).

    (7.4.1 - 7.4.6)
    """
    index, c = p.find_first(T.DHATU)
    if not c:
        return

    try:
        nic = p.terms[index + 1]
        has_agama = False
        if nic.any(T.AGAMA):
            has_agama = True
            nic = p.terms[index + 2]
            can = p.terms[index + 3]
        else:
            can = p.terms[index + 2]
    except IndexError:
        return

    if nic.u not in ("Ric", "RiN"):
        return
    if can.u != "caN":
        return

    // Ignore 'f' because it is handled by 7.4.7.
    if c.upadha in s("ac") and c.upadha not in s("f"):
        res = sounds.hrasva(c.upadha)
        if c.any(T.F_AT_LOPA) or p.has(i, f::text("SAs" or c.any("f"):
            p.step("7.4.2")
        } else if res != c.upadha:
            op.upadha("7.4.1", p, c, res)
    } else if has_agama and c.antya in s("ac"):
        // HACK for agama
        res = sounds.hrasva(c.antya)
        op.antya("7.4.1", p, c, res)
*/

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

    if p.has(i, |t| t.has_antya(&s("f")) || t.text == "dfS") {
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

fn try_ksa_lopa(p: &mut Prakriya) {
    let i_dhatu = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    let i = i_dhatu + 1;
    let i_tin = i_dhatu + 2;
    if p.has(i, f::u("ksa")) {
        if p.has(i + 1, f::adi("ac")) {
            p.op_term("7.2.72", i, op::antya(""));
        }
        if p.has(i_dhatu, f::text_in(&["duh", "dih", "lih", "guh"])) && p.has(i_tin, f::atmanepada)
        {
            p.op_optional("7.3.73", op::t(i, op::antya("")));
        }
    }
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

fn try_anga_adesha_before_vibhakti(p: &mut Prakriya) {
    let i_sup = match p.find_last(T::Sup) {
        Some(i) => i,
        None => return,
    };
    let i = i_sup - 1;

    if p.has(i, f::text("rE")) && p.has(i, f::adi("hal")) {
        p.op_term("7.2.85", i, op::antya("A"));
    } else if p.has(i, f::text_in(&["yuzmad", "asmad"])) {
        let sup = &p.terms()[i_sup];

        if sup.has_adi(&s("ac")) {
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
}

pub fn run_remainder(p: &mut Prakriya) {
    sup_adesha::run(p);
    try_pratyaya_adesha(p);
    try_anga_adesha_before_vibhakti(p);

    // `try_ksa_lopa` must run before `try_sarvadhatuke` so that at-lopa (since `ksa` ends in `a`)
    // has a chance to take effect and prevent "ato yeyaH" (7.2.80).
    try_ksa_lopa(p);
    try_sarvadhatuke(p);

    /*
    for index, _ in enumerate(p.terms):
        shiti(p, index)

    // Must come before asiddhavat rule 6.4.78 (e.g. "iyarti", ekahalmadhya)
    */
    abhyasasya::run(p);

    for i in 0..p.terms().len() {
        asiddhavat::run_before_guna(p, i);
    }

    /*
        // num-Agama must come after asiddhavat rule 6.2.24, which causes na-lopa.
        num_agama(p, index)
        index += 1
    */

    try_sic_vrddhi(p);

    for index in 0..p.terms().len() {
        /* nau(p, index)
        cajoh_kuh(p, index)
        */
        run_for_each_2(p, index);
    }

    liti(p);
    // Rules for various lun-vikaranas.
    ani(p);

    /*
    // Asiddhavat must run before cani for "Ner aniTi"
    for index, _ in enumerate(p.terms):
        c = p.terms[index]
        if c.text:
            asiddhavat.run_nau(p, index)

    cani_after_guna(p)
    abhyasasya.run_sani_cani(p)
    */

    for index in 0..p.terms().len() {
        /*
        c = p.terms[index]
        if not c.text:
            continue

        asiddhavat.run_after_guna(p, index)
        */
        dhatu_rt_adesha(p, index);
        try_ato_dirgha(p, index);
    }

    // asiddhavat.run_dirgha(p)
}
