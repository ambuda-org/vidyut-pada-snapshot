/*!
atidesha (1.2.1 - 1.2.17)
=========================
*/

use crate::constants::Tag as T;
use crate::dhatupatha::is_kutadi;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;

fn run_before_attva_at_index(p: &mut Prakriya, i: usize) {
    let n = match p.view(i) {
        Some(x) => x,
        None => return,
    };
    let add_nit = op::add_tag(T::Nit);
    let add_kit = op::add_tag(T::kit);

    // Must check for `Agama` specifically because of the tiN ending "iw".
    let iti = n.has_u("iw") && n.has_tag(T::Agama);

    let gan_kutadi = p.has(i, |t| t.has_u("gAN") || is_kutadi(t));
    if gan_kutadi && !n.any(&[T::Rit, T::Yit]) {
        p.op_term("1.2.1", i + 1, add_nit);
    } else if p.has(i, f::text("vij")) && iti {
        p.op_term("1.2.2", n.end(), add_nit);
    } else if p.has(i, f::text("UrRu")) && iti {
        p.op_optional("1.2.3", op::t(n.end(), add_nit));
    } else if n.has_tag(T::Sarvadhatuka) && !n.has_tag(T::pit) {
        p.op_term("1.2.4", n.end(), add_nit);
    } else if p.has(i, |t| t.has_tag(T::Dhatu) && !f::is_samyoganta(t))
        && n.has_lakshana("li~w")
        && !n.has_tag(T::pit)
    {
        p.op_term("1.2.5", n.end(), add_kit);
    } else if p.has(i, f::text_in(&["BU", "inD"])) && n.has_lakshana("li~w") {
        p.op_term("1.2.6", n.end(), add_kit);
    } else if n.has_lakshana("li~w") && p.has(i, f::text_in(&["SranT", "granT", "danB", "svanj"])) {
        // TODO: rule seems obligatory; where is optionality defined?
        p.op_optional("1.2.6.v1", op::t(n.end(), add_kit));
    }

    /*
    tin = p.terms[-1]
    if (
        tin.all(T.ATMANEPADA) && (tin.all("li~N") or n.terms[0].u == "si~c")
    ) && n.adi in s("Jal") {
        is_dhatu = c.all(T.DHATU)
        ik_halantat = c.upadha in s("ik") && c.antya in s("hal")

        if is_dhatu && ik_halantat:
            op.tag("1.2.11", p, n.terms[-1], "k")
        } else if is_dhatu && c.antya in s("f"):
            op.tag("1.2.12", p, n.terms[-1], "k")
        }
    }
    */
}

/// Runs rules that apply only if the root ends in long A.
///
/// If we don't use a separate function for these rules, we have a dependency loop:
///
/// 1. iT-Agama --> atidesha & samprasarana
/// 2. atidesha & samprasarana --> Ad-Adesha
/// 3. Ad-Adesha --> iT-Agama (sak ca)
///
/// So we break the loop by doing the following:
///
/// 1. iT-Agama (non-A) --> atidesha & samprasarana (non-A)
/// 2. atidesha & samprasarana (non-A) -> Ad-Adesha
/// 3. Ad-Adesha --> iT-Agama (A)
/// 4. iT-Agama (A) --> atidesha and samprasarana (A)
fn run_after_attva_at_index(p: &mut Prakriya, i: usize) {
    /*
    c = p.terms[index]
    n = TermView.make_pratyaya(p, index)
    if not n {
        return
    }

    tin = p.terms[-1]
    if (
        tin.all(T.ATMANEPADA)
        and n.terms[-1].u == "si~c"
        and (c.text == "sTA" or c.all(T.GHU))
    ) {
        n.terms[-1].add_tags("k")
        op.antya("1.2.17", p, c, "i")
    }
    */
}

/// Runs most atidesha rules.
pub fn run_before_attva(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        run_before_attva_at_index(p, i);
    }
}

/// Runs atidesha rules that must follow rule 6.1.45 (Adeca upadeze 'ziti).
pub fn run_after_attva(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        run_after_attva_at_index(p, i);
    }
}
