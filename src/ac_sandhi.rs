//! ac_sandhi
//! =========
//! (6.1.66 - 6.1.101)

use std::error::Error;
use crate::constants::Tag as T;
use crate::filters as f;
use crate::operations as op;
use crate::dhatu_gana::{PUSH_ADI, DYUT_ADI, TAN_ADI}; 
use crate::prakriya::Prakriya;
use crate::sounds::s;
use crate::it_samjna;
use crate::term::Term;

/*
fn sup_sandhi_before_angasya(p: Prakriya) {
    n = p.terms[-1]
    if not n.all(T.SUP) {
        return
    }
    c = p.terms[-2]

    if c.antya == "o" and n.u in {"am", "Sas"} {
        n.text = n.text[1:]
        op.antya("6.1.93", p, c, "A")
    }
}

/// Replaces character `i` of the current prakriya with the given substitute.
fn set_at(p: &mut Prakriya, index: usize, substitute: &str) {
    let cur = 0;
    for t in p.terms() {
        let delta = t.text.len();
        if (cur..=cur+delta).contains(&index) {
            let t_offset = index - cur;
            t.text = String::from(&t.text[..t_offset]) + substitute + &t.text[t_offset+1..];
            return
        } else {
            cur += delta;
        }
    }
}

fn general_vowel_sandhi(p: &mut Prakriya, terms=None) {
    """

    :param terms: optional. Used as a hack to apply ac-sandhi before Ni-lopa
    without also modifying Ni.
    """
    view = StringView(terms or p.terms)
    m = re.search("([a])([aeo])", view.text)
    while m {
        view[m.span(0)[0]] = ""
        p.step("6.1.97")
        m = re.search("([a])([aeo])", view.text)
    }

    a = s("a").regex
    ac = s("ac").regex
    ak = s("ak").regex
    ec = s("ec").regex
    ic = s("ic").regex
    ik = s("ik").regex

    for m in find_all_matches(f"({ec})({ac})", view) {
        mapping = dict(zip(s("ec").items, ("ay", "av", "Ay", "Av")))
        view[m.span(0)[0]] = mapping[m.group(1)]
        p.step("6.1.78")
    }

    m = re.search(f"({ak})({ak})", view.text)
    while m {
        first = m.group(1)
        second = m.group(2)
        if second in sounds.savarna(first) {
            view[m.span(0)[0]] = ""
            view[m.span(1)[0]] = sounds.dirgha(first)
            p.step("6.1.101")
        }

        // Loop but avoid infinite loop, which is possible here due to the
        // savarna check.
        old_start = m.span(0)[0]
        m = re.search(f"({ak})({ak})", view.text)
        if m and m.span(0)[0] == old_start {
            break
        }

    for m in find_all_matches(f"({ik})({ac})", view) {
        mapping = dict(zip("iIuUfFxX", "yyvvrrll"))
        view[m.span(0)[0]] = mapping[m.group(1)]
        p.step("6.1.77")
    }

    m = re.search(f"({a})({ic})", view.text)
    while m {
        first = m.group(1)
        second = m.group(2)
        if second in s("ec") {
            view[m.span(0)[0]] = sounds.vrddhi(second)
            view[m.span(0)[1] - 1] = ""
            p.step("6.1.88")
        else {
            // HACK for trnah
            term = view.term_for_index(m.span(0)[0])
            if term.text == "tfnaih" {
                op.text("6.1.87", p, term, "tfneh")
            } else {
                view[m.span(0)[0]] = sounds.guna(second)
                view[m.span(0)[1] - 1] = ""
                p.step("6.1.87")
            }
        }
        m = re.search(f"({a})({ic})", view.text)
    }
}


/*
fn sup_sandhi_after_angasya(p: &mut Prakriya) {
    // Program cannot model "antAdivacca" so split the rule.
    n = p.terms[-1]
    if not n.all(T.SUP) {
        return
    }

    c = p.terms[-2]

    if c.antya in s("ak") and n.any(T.V1, T.V2) {
        if n.text == "am" {
            op.adi("6.1.107", p, n, "")
        } else if  c.antya in s("a") and n.adi in s("ic") {
            p.step("6.1.104")
        } else if  c.antya in sounds.DIRGHA and (n.adi in s("ic") or n.u == "jas") {
            p.step("6.1.105")
        } else if  n.adi in s("ac") {
            antya = c.antya
            c.text = c.text[:-1]
            op.adi("6.1.102", p, n, sounds.dirgha(antya))

            if n.u == "Sas" and c.all(T.PUM) {
                op.antya("6.1.103", p, n, "n")
            }
        }

    } else if  n.u in {"Nasi~", "Nas"} {
        if c.antya in s("eN") {
            op.adi("6.1.110", p, n, "")
        } else if  c.antya == "f" {
            c.text = c.text[:-1] + "ur"
            op.adi("6.1.110", p, n, "")
        }
    }
}
*/

fn run_for_term(p: &mut Prakriya, index: usize) {
    terms = p.terms
    c = terms[index]

    // Ignore this case if it starts an upadesha, otherwise roots like "vraj"
    // would by vyartha. Likewise for roots ending with 'v'
    // TODO: handle term boundaries more elegantly
    val = s("val")
    s_val = "".join(val.items)
    re_vyor_vali = f"[vy]([{s_val}])"
    if re.search(re_vyor_vali, c.text) and not c.all(T.DHATU) {
        c.text = re.sub(re_vyor_vali, r"\1", c.text)
        p.step("6.1.66")

    try {
        n = [u for u in terms[index + 1 :] if u.text][0]
    except IndexError {
        return

    if c.antya in s("v y") and n.adi in val and not c.all(T.DHATU) {
        op.antya("6.1.66", p, c, "")
    }

    // TODO: NI, Ap
    // Check for Agama to avoid lopa on yAs + t.
    if (
        c.antya in s("hal")
        and n
        and len(n.text) == 1
        and n.u in ("su~", "tip", "sip")
        and not c.all(T.AGAMA)
    ) {
        op.antya("6.1.68", p, n, "")
    }

    if (c.antya in sounds.HRASVA or c.antya in s("eN")) and p.terms[-1].any(
        T.SAMBUDDHI
    ) {
        op.lopa("6.1.69", p, p.terms[-1])
    }

    if c.antya in s("a") and n.text == "us" {
        op.antya("6.1.96", p, c, "")

    // ekaH pUrvapara (6.1.84)

    } else if c.u == "Aw" and n.adi in s("ik") {
        c.text = ""
        op.adi("6.1.90", p, n, sounds.vrddhi(n.adi))
    }
}
*/


fn run_common(p: &mut Prakriya) {
    /*
    for i in 0..p.terms().len() {
        run_for_term(p, i)
    }

    general_vowel_sandhi(p)

    for i, c in enumerate(p.terms) {
        try {
            n = p.terms[i + 1]
        except IndexError {
            continue
        // HACK: duplicate 6.4.92 from the asiddhavat section for ci -> cAy, cap
        if c.all("m") and n.u in {"Ric", "pu~k"} and c.text in {"cAy", "cA"} {
            if c.text == "cA" {
                p.op("6.4.92", op::t(i, op::antya("a")));
            } else {
                p.op("6.4.92", op::t(i, op::upadha("a")));
            }
        }
    }
    */
}


pub fn run(p: &mut Prakriya) {
    run_common(p)
    /*
    sup_sandhi_before_angasya(p)
    sup_sandhi_after_angasya(p)
    */
}
