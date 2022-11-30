/*!
tripadi
=======
(8.2.1 - end of 8.4)

The final three pādas of the Ashtadhyayi are called the **tripādi**. The tripādi generally contains
sandhi rules and other miscellaneous sound change rules.

The tripādi uses a different rule selection mechanism from the rest of the Ashtadhyayi: whereas the
rest of the text selects rules based on their priority and allows a rule to apply if it has scope,
the tripādi applies rules in order and will never "go back" to apply an earlier rule.
*/

use crate::char_view::{char_rule, set_at, xy};
use crate::constants::Tag as T;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds as al;
use crate::sounds::{map_sounds, s, SoundMap, SoundSet};
use crate::term::Term;
use crate::term::TermView;
use lazy_static::lazy_static;

lazy_static! {
    static ref INKU: SoundSet = s("iR2 ku~");
    static ref JHAL: SoundSet = s("Jal");
    static ref JHASH: SoundSet = s("JaS");
    static ref KHAR: SoundSet = s("Kar");
    static ref JHAL_TO_CAR: SoundMap = map_sounds("Jal", "car");
    static ref JHAL_TO_JASH: SoundMap = map_sounds("Jal", "jaS");
    static ref JHAL_TO_JASH_CAR: SoundMap = map_sounds("Jal", "jaS car");
    static ref IK: SoundSet = s("ik");
    static ref YAY: SoundSet = s("yay");
    static ref HAL: SoundSet = s("hal");
}

/// Runs rules for lopa of the final `n` of a prAtipadika.
/// Example: rAjan + Bis -> rAjaBis.
///
/// (8.2.7 - 8.2.8)
fn try_na_lopa(p: &mut Prakriya) {
    let i = match p.find_last(T::Sup) {
        Some(i) => i,
        None => return,
    };
    let i_anga = i - 1;

    if p.has(i_anga, |t| t.has_antya('n') && t.has_tag(T::Pratipadika)) && p.has(i, f::empty) {
        if p.has(i, |t| t.has_tag(T::Sambuddhi) || t.has_u("Ni")) {
            p.step("8.2.8");
        } else {
            p.op_term("8.2.7", i_anga, op::antya(""));
        }
    }
}

/// Runs rules that change r to l.
/// Example: girati -> gilati.
///
/// (8.2.18 - 8.2.20)
fn try_ra_to_la(p: &mut Prakriya) {
    let do_ra_la = |t: &mut Term| t.text = t.text.replace('f', "x").replace('r', "l");

    for i in 0..p.terms().len() {
        let n = i + 1;
        // HACK to exclude kfpa (cur-gana root).
        if p.has(i, |t| t.text.starts_with("kfp") && !t.has_u("kfpa")) {
            p.op("8.2.18", op::t(i, do_ra_la));
        } else if p.has(i, f::u("gF")) && p.has(n, f::u("yaN")) {
            p.op("8.2.20", op::t(i, do_ra_la));
        } else if p.has(i, |t| t.has_u("gF") && t.gana == Some(6))
            && p.has(n, |t| t.has_adi(&s("ac")))
        {
            // TODO: where is it specified that this is only for gF/girati?
            p.op("8.2.21", op::t(i, do_ra_la));
        }
    }
}

fn try_samyoganta_and_sa_lopa(p: &mut Prakriya) {
    /*
    """Final samyoga. (8.2.23 - 8.2.29)"""

    // Exception to 8.2.23.
    sk = s("s k").regex
    hal = s("hal").regex
    jhal = s("Jal").regex

    // TODO: jhal case
    pattern = f"({sk}+){hal}+"
    jhal = f"{jhal}"

    // jhalo jhali
    view = StringView(p.terms)
    lopa_offset = 0
    for match in re.finditer(f"{jhal}(s){jhal}", view.text):
        start, end = match.span(1)
        view.delete_span(start - lopa_offset, end - lopa_offset)
        p.step("8.2.26")

    // saMst can be handled only with difficulty. For details, see the
    // commentary in the mAdhavIya-dhAtuvRtti:
    //
    // https://archive.org/details/237131938MadhaviyaDhatuVrtti/page/n434/mode/1up
    view = StringView(p.terms)
    vtext = view.text
    // We find all matches at the beginning of the loop. If multiple sa-lopas
    // apply, then each lopa will cause a frame shift that will affect later
    // sa-lopas. As a quick hack, manage this with `offset` so that the deletion
    // indices are always properly aligned.
    lopa_offset = 0
    for match in re.finditer(pattern + f"({jhal}|$)", vtext):
        can_apply = True
        if "sanst" in vtext:
            // Apply the rule only if the change would not affect "sanst."
            offset = 0
            for t in p.terms:
                offset += len(t.text)
                if t.text == "sanst":
                    break
            if match.span(1)[0] <= offset:
                // rule would apply to "sanst" -- block.
                can_apply = False
        if can_apply:
            start, end = match.span(1)
            view.delete_span(start - lopa_offset, end - lopa_offset)
            p.step("8.2.29")
            lopa_offset += 1
    */

    for i in 0..p.terms().len() {
        let n = i + 1;
        if p.terms().get(n).is_none() {
            break;
        }

        if p.has(i, |t| t.has_antya('r')) && p.has(n, |t| t.text == "s") && n == p.terms().len() - 1
        {
            p.op_term("8.2.24", i, op::adi(""));
        } else if p.has(i, |t| t.has_antya('s')) && p.has(i + 1, |t| t.has_adi('D')) {
            // Per kAzikA, applies only to s of si~c. But this seems to cause
            // problems e.g. for tAs + Dve.
            p.op_term("8.2.25", i, op::antya(""));
        }
    }

    /*

    // hrasvAd aGgAt
    for i, c in enumerate(p.terms):
        try:
            n = p.terms[i + 1]
            n2 = p.terms[i + 2]
        except IndexError:
            break
        if (
            c.antya in sounds.HRASVA
            and n.text == "s"
            and n2.adi in s("Jal")
            and not c.any(T.AGAMA)
        ):
            op.lopa("8.2.27", p, n)

    for i, _ in enumerate(p.terms[:-2]):
        x, y, z = p.terms[i : i + 3]
        if x.u == "iw" and y.u == "si~c" and z.u == "Iw":
            op.lopa("8.2.28", p, y)

            // sic-lopa is siddha with respect to prior rules (8.2.3 vArttika)
            z.text = ""
            // HACK: x should always have text at this point. Temp workaround.
            if x.text:
                op.antya("6.1.101", p, x, "I")
    */

    char_rule(
        p,
        |p, text, i| al::is_samyoganta(text) && i == text.len() - 1,
        |p, _, i| {
            set_at(p, i, "");
            p.step("8.3.24");
            true
        },
    );
}

/// Runs rules that change the final "h" of a dhatu.
/// Example: muh + ta -> mugdha.
///
/// (8.2.31 - 8.2.35)
fn try_ha_adesha(p: &mut Prakriya) {
    lazy_static! {
        static ref JHAL: SoundSet = s("Jal");
    }

    // TODO: implement padAnta
    // By a vArttika, this applies only at term boundaries.
    let druha_muha = &["dru\\ha~", "mu\\ha~", "zRu\\ha~", "zRi\\ha~"];

    for i in 0..p.terms().len() {
        let is_dhatu = p.has(i, f::tag(T::Dhatu));
        let jhali = p.has(i + 1, |t| JHAL.contains_opt(t.adi()));
        let ante = i == p.terms().len() - 1;

        if jhali || ante {
            if is_dhatu {
                let dhatu = &p.terms()[i];
                if dhatu.has_u_in(druha_muha) {
                    p.op_optional("8.2.33", |p| p.set(i, op::antya("G")));
                } else if dhatu.has_u("Ra\\ha~^") {
                    p.op_term("8.2.34", i, op::antya("D"));
                } else if dhatu.text == "Ah" {
                    p.op_term("8.2.35", i, op::antya("T"));
                } else if dhatu.has_adi('d') {
                    p.op_term("8.2.35", i, op::antya("G"));
                }
            }
            // If no change was made, use the default.
            if p.has(i, |t| t.has_antya('h')) {
                p.op_term("8.2.31", i, op::antya("Q"));
            }
        }
    }
}

/// (8.2.76 - 8.2.79)
fn try_lengthen_dhatu_vowel(p: &mut Prakriya, i: usize) {
    if !p.has(i, f::dhatu) {
        return;
    }

    let is_rv = |opt| match opt {
        Some(c) => c == 'r' || c == 'v',
        None => false,
    };
    let is_ik = |opt| match opt {
        Some(c) => al::is_hrasva(c) && IK.contains_char(c),
        None => false,
    };
    let is_hal = |opt| match opt {
        Some(c) => al::is_hal(c),
        None => false,
    };
    let before_upadha = |t: &Term| t.text.chars().rev().nth(2);

    // TODO: bha
    let dhatu = &p.terms()[i];
    if dhatu.has_text_in(&["kur", "Cur"]) {
        p.step("8.2.79");
    } else if is_ik(dhatu.upadha()) && is_rv(dhatu.antya()) {
        let upadha = dhatu.upadha().expect("");
        let upadha_to_dirgha = |t: &mut Term| {
            let sub = al::to_dirgha(upadha).unwrap().to_string();
            op::set_upadha(&t.text, &sub);
        };
        if p.has(i + 1, |t| HAL.contains_opt(t.adi())) {
            p.op_term("8.2.77", i, upadha_to_dirgha);
        } else {
            p.op_term("8.2.76", i, upadha_to_dirgha);
        }
    } else if is_ik(before_upadha(dhatu)) && is_rv(dhatu.upadha()) && is_hal(dhatu.antya()) {
        p.op("8.2.78", |p| {
            let dhatu = &p.terms()[i];
            let n = dhatu.text.len();
            let pre_upadha = before_upadha(dhatu).unwrap();
            let sub = al::to_dirgha(pre_upadha).unwrap().to_string();
            p.set(i, |t| {
                t.text = String::from(&t.text[..n - 3]) + &sub + &t.text[n - 2..]
            });
        });
    }
}

fn per_term_1b(p: &mut Prakriya, i: usize) {
    let n = p.view(i + 1);
    let is_padanta = n.map(|x| x.is_padanta()).unwrap_or(true);
    if p.has(i, |t| t.has_antya('s')) && is_padanta {
        p.op_term("8.2.66", i, op::antya("ru~"));
    }

    try_lengthen_dhatu_vowel(p, i);

    // 8.3.15
    // TODO: next pada
    let n = p.view(i + 1);
    let has_ru = p.has(i, |t| t.text.ends_with("ru~") || t.has_antya('r'));
    if has_ru && is_padanta {
        p.op_term("8.3.15", i, |t| {
            if let Some(p) = t.text.strip_suffix("ru~") {
                t.text = p.to_owned() + "H";
            } else if let Some(p) = t.text.strip_suffix('r') {
                t.text = p.to_owned() + "H";
            }
        });
    }

    /*
    c = p.terms[index]
    try:
        n = [u for u in p.terms[index + 1 :] if u.text][0]
    except IndexError:
        n = None

    vrascha = {
    "o~vrascU~",
        "Bra\\sja~^",
        "sf\\ja~\\",
        "sf\\ja~",
        "mfjU~",
        "ya\\ja~^",
        "rAj",
        "BrAjf~\\",
    }

    jhali_ante = not n or n.adi in s("Jal")
    if (c.u in vrascha or c.antya in s("C S")) and jhali_ante:
        if c.text.endswith("tC"):
            // TODO: seems implied, not sure.
            c.text = c.text[:-2] + "z"
            p.step("8.2.36")
        else:
            op.antya("8.2.36", p, c, "z")

    if c.antya in s("cu~") and (not n or n.adi in s("Jal")):
        mapping = sounds.map_sounds(s("cu~"), s("ku~"))
        op.antya("8.2.30", p, c, mapping[c.antya])

    sdhvoh = n and (n.adi == "s" or n.all(T.PRATYAYA) and n.u.startswith("Dv"))
    basho_bhash = sounds.map_sounds_s("baS", "Baz")
    if c.adi in basho_bhash and c.antya in s("JaS") and sdhvoh:
        op.adi("8.2.37", p, c, basho_bhash[c.adi])
    */

    // Exclude the following from 8.2.39 so that the corresponding rules aren't
    // vyartha:
    // - c for 8.2.30 (coH kuH)
    // - S for 8.2.36 (vraSca-Brasja-...-Ca-SAM zaH)
    // - s for 8.2.66 (sasajuSo ruH)
    // - h for 8.2.31 (ho QaH)
    let c = &p.terms()[i];
    let n = p.view(i + 1);
    let is_padanta = n.map(|x| x.is_padanta()).unwrap_or(true);
    if c.has_antya(&*JHAL) && !c.has_antya(&s("c S s h")) && is_padanta {
        let key = c.antya().unwrap();
        let sub = JHAL_TO_JASH.get(&key).unwrap();
        p.op_term("8.2.39", i, op::antya(&sub.to_string()));
    }

    /*
    if c.all(T.DHATU) and c.u != "quDA\\Y":
        // TODO: abhyasa
        if c.antya in s("Jaz") and n and n.adi in s("t T"):
            op.adi("8.2.40", p, n, "D")

    if c.antya in s("z Q") and n.adi == "s":
        op.antya("8.2.41", p, c, "k")

    if c.any(T.DHATU) and c.antya == "m" and n.adi in {"m", "v"}:
        op.antya("8.2.65", p, c, "n")

    // TODO: sajuS

    try:
        rn = p.terms[index + 1]
    except IndexError:
        rn = None
    next_is_last = index + 1 == len(p.terms) - 1
    if c.antya == "s" and next_is_last and rn.text == "" and rn.u == "tip":
        // Exception to general rule 8.2.66 below
        op.antya("8.2.73", p, c, "d")

    } else if c.antya == "s" and (not n or (next_is_last and rn.text == "")):
        op.antya("8.2.66", p, c, "ru~")

    if c.antya in s("s d") and rn and rn.text == "" and rn.u == "sip":
        if c.antya == "s":
            op.optional(op.antya, "8.2.74", p, c, "ru~")
        else:
            op.optional(op.antya, "8.2.75", p, c, "ru~")

    // 8.2.77
    // TODO: sajuS

    // 8.3.15
    // TODO: next pada
    has_ru = c.text.endswith("ru~") or c.text.endswith("r")
    if has_ru and not n:
        c.text = c.text.replace("ru~", "H")
        if c.text.endswith("r"):
            c.text = c.text[:-1] + "H"
        p.step("8.3.15")
    */
}

/// Runs rules that change `n` to `R`.
/// Example: krInAti -> krIRAti.
///
/// (8.2.31 - 8.2.35)
fn try_natva(p: &mut Prakriya) {
    /*
        i, u = p.find_first(T.DHATU)
        if u and (
            (u.u == "kzuBa~" and p.terms[i + 1].u in {"SnA", "SAnac"})
            or (u.u == "tfpa~" and p.terms[i + 1].u == "Snu")
        ):
            p.step("8.4.39")
            return

        // TODO: AG and num
        view = StringView(p.terms)
        between = s("aw ku~ pu~ M").regex
        match = re.search(f"[rzfF]({between}*)n", view.text)

        if match:
            // End of pada
            if match.span(0)[1] == len(view.text):
                p.step("8.4.37")
            else:
                view[match.span(0)[1] - 1] = "R"
                if match.group(1):
                    p.step("8.4.2")
                else:
                    trigger = view[match.span(0)[0]]
                    if trigger in "rz":
                        p.step("8.4.1")
                    else:
                        p.step("8.4.1-v")
    }
    */
}

/// Converts "m" and "n" to the anusvara when a consonant follows.
///
/// Example: Sankate -> SaMkate
fn try_mn_to_anusvara(p: &mut Prakriya) {
    // TODO: a-padAnta
    char_rule(
        p,
        xy(|x, y| (x == 'm' || x == 'n') && JHAL.contains_char(y)),
        |p, _, i| {
            set_at(p, i, "M");
            p.step("8.3.24");
            true
        },
    );
}

/// Runs rules that make a sound mUrdhanya when certain sounds precede.
///
/// Example: `nesyati -> nezyati`
///
/// (8.3.55 - 8.3.119)
fn try_murdhanya(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        let n = i + 1;
        if p.get(n).is_none() {
            return;
        }

        let apadanta = p.has(n, f::not_empty);
        // HACK: don't include Agama.
        let adesha_pratyaya = p.has(n, |t| t.any(&[T::Pratyaya, T::FlagAdeshadi, T::Agama]));
        if p.has(i, |t| t.has_antya(&*INKU)) && p.has(n, f::adi("s")) && adesha_pratyaya && apadanta
        {
            p.op_term("8.3.59", n, op::adi("z"));
        } else if p.has(i, |t| {
            t.has_u_in(&["va\\sa~", "SAsu~", "Gasx~"]) && t.has_upadha(&*INKU)
        }) {
            p.op_term("8.3.60", i, op::antya("z"));
        }
    }
}

fn stu_to_scu(c: char) -> Option<&'static str> {
    // FIXME: use char map?
    let res = match c {
        's' => "S",
        't' => "c",
        'T' => "C",
        'd' => "j",
        'D' => "J",
        'n' => "Y",
        _ => return None,
    };
    Some(res)
}

fn stu_to_swu(c: char) -> Option<&'static str> {
    // FIXME: use char map?
    let res = match c {
        's' => "z",
        't' => "w",
        'T' => "W",
        'd' => "q",
        'D' => "Q",
        'n' => "R",
        _ => return None,
    };
    Some(res)
}

fn try_change_stu_to_parasavarna(p: &mut Prakriya) {
    lazy_static! {
        static ref SCU: SoundSet = s("S cu~");
        static ref SWU: SoundSet = s("z wu~");
        static ref STU: SoundSet = s("s tu~");
        static ref TU: SoundSet = s("tu~");
    };
    char_rule(
        p,
        xy(|x, y| {
            (STU.contains_char(x) && SCU.contains_char(y))
                || (SCU.contains_char(x) && STU.contains_char(y))
        }),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = text.as_bytes()[i + 1] as char;
            if x == 'S' {
                p.step("8.4.44");
                false
            } else {
                if STU.contains_char(x) {
                    let sub = stu_to_scu(x).expect("");
                    set_at(p, i, sub);
                } else {
                    let sub = stu_to_scu(y).expect("");
                    set_at(p, i + 1, sub);
                }
                p.step("8.4.40");
                true
            }
        },
    );
    char_rule(
        p,
        xy(|x, y| {
            (STU.contains_char(x) && SWU.contains_char(y))
                || (SWU.contains_char(x) && STU.contains_char(y))
        }),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let y = text.as_bytes()[i + 1] as char;
            if TU.contains_char(x) && y == 'z' {
                p.step("8.4.43");
                false
            } else {
                if STU.contains_char(x) {
                    let sub = stu_to_swu(x).expect("");
                    set_at(p, i, sub);
                } else {
                    let sub = stu_to_swu(y).expect("");
                    set_at(p, i + 1, sub);
                }
                p.step("8.4.41");
                true
            }
        },
    );
}

/*
        // HACK
        for i, u in enumerate(p.terms):
            if c is u:
                break
        n = TermView.make_pratyaya(p, i)
        if not n:
            continue

        if (
            c.antya in s("iR2")
            and not c.any(T.AGAMA)
            and (n.any("lu~N", "li~w") or n.all(T.ARDHADHATUKA, "li~N"))
        ):
            last = n.terms[-1]
            if not (last.adi == "D" or n.text.endswith("zIDvam")):
                continue

            do = True
            if f.is_it_agama(n.terms[0]):
                code = "8.3.79"
                if p.allow(code):
                    p.step(code)
                else:
                    do = False
                    p.decline(code)

            if do:
                last.text = last.text.replace("D", "Q")
                p.step("8.3.78")
*/

/*
/// Run rules for retroflex Dha.
fn dha(p: &mut Prakriya):
    view = StringView(p.terms)
    // Save the text before Dha-lopa for a cleaner comparison below.
    vtext = view.text

    // Placed after 8.4.41, otherwise this is vyartha
    match = re.search(f"([Q])[Q]", view.text)
    if match:
        view[match.span(0)[0]] = ""
        p.step("8.3.13")

        // Placed here, otherwise this is vyartha
        // matches aN (no f, x)
        match = re.search(f"([aAiIuU])[Q]", view.text)
        if match:
            // HACK to check for sah and vah
            if "saQ" in vtext or "sAQ" in vtext or "vaQ" in vtext or "vAQ" in vtext:
                view[match.span(0)[0]] = "o"
                p.step("6.3.112")
            else:
                res = match.group(1)
                view[match.span(0)[0]] = sounds.dirgha(res)
                p.step("6.3.111")
*/

/// Runs rules that convert sounds to their savarna version.
fn try_to_savarna(p: &mut Prakriya) {
    char_rule(
        p,
        xy(|x, y| x == 'M' && YAY.contains_char(y)),
        |p, text, i| {
            let y = text.as_bytes()[i + 1] as char;
            let sub = match y {
                'k' | 'K' | 'g' | 'G' | 'N' => "N",
                'c' | 'C' | 'j' | 'J' | 'Y' => "Y",
                'w' | 'W' | 'q' | 'Q' | 'R' => "R",
                't' | 'T' | 'd' | 'D' | 'n' => "n",
                'p' | 'P' | 'b' | 'B' | 'm' => "m",
                _ => "M",
            };
            set_at(p, i, sub);
            p.step("8.4.58");
            true
        },
    )

    /*
    hal = s("hal").regex
    yam = s("yam").regex
    jhar = s("Jar").regex

    view = StringView(p.terms)
    match = re.search(f"{hal}({yam})({yam})", view.text)
    if match:
        c = match.group(1)
        n = match.group(2)
        if c == n:
            if p.allow("8.4.64"):
                view.delete_span(*match.span(1))
            else:
                p.decline("8.4.64")

    view = StringView(p.terms)
    match = re.search(f"{hal}({jhar})({jhar})", view.text)
    if match:
        c = match.group(1)
        n = match.group(2)
        if n in sounds.savarna(c):
            if p.allow("8.4.65"):
                view.delete_span(*match.span(1))
            else:
                p.decline("8.4.65")
    */
}

fn try_jhal_adesha(p: &mut Prakriya) {
    char_rule(
        p,
        xy(|x, y| JHAL.contains_char(x) && JHASH.contains_char(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_JASH.get(&x).unwrap();
            if x != *sub {
                set_at(p, i, &sub.to_string());
                p.step("8.4.53");
                true
            } else {
                false
            }
        },
    );

    if let Some(i) = p.find_first(T::Abhyasa) {
        let abhyasa = p.get(i).unwrap();
        if JHAL.contains_opt(abhyasa.adi()) {
            let sub = JHAL_TO_JASH_CAR
                .get(&abhyasa.adi().unwrap())
                .unwrap()
                .to_string();
            p.op_term("8.4.54", i, op::adi(&sub));
        }
    }

    /*
    // 8.2.38, but indicated here by use of "dadhas" in the rule.
    sdhvoh = n and (n.adi == "s" or n.all(T.PRATYAYA) and n.u.startswith("Dv"))
    if c.u == "quDA\\Y" and c.text == "D" and (n.adi in s("t T") or sdhvoh):
        prev = p.terms[index - 1]
        prev.text = "Da"
        c.text = "d"
        p.step("8.2.38")
    */

    char_rule(
        p,
        xy(|x, y| JHAL.contains_char(x) && KHAR.contains_char(y)),
        |p, text, i| {
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(&x).unwrap();
            if x != *sub {
                set_at(p, i, &sub.to_string());
                p.step("8.4.55");
                true
            } else {
                false
            }
        },
    );

    char_rule(
        p,
        |_, text, i| {
            let x = text.as_bytes()[i] as char;
            JHAL.contains_char(x) && i == text.len() - 1
        },
        |p, text, i| {
            let code = "8.4.56";
            let x = text.as_bytes()[i] as char;
            let sub = JHAL_TO_CAR.get(&x).unwrap();
            if x != *sub {
                if p.is_allowed(code) {
                    set_at(p, i, &sub.to_string());
                    p.step("8.4.56");
                    true
                } else {
                    p.decline("8.4.56");
                    false
                }
            } else {
                false
            }
        },
    );
}

pub fn run(p: &mut Prakriya) {
    try_na_lopa(p);
    try_ra_to_la(p);
    try_samyoganta_and_sa_lopa(p);
    try_ha_adesha(p);

    for i in 0..p.terms().len() {
        per_term_1b(p, i);
    }

    try_murdhanya(p);
    try_mn_to_anusvara(p);
    try_natva(p);
    try_change_stu_to_parasavarna(p);
    // dha(p);

    try_jhal_adesha(p);
    try_to_savarna(p);
}
