/*!
atidesha (1.2.1 - 1.2.17)
=========================
*/

use crate::constants::Tag as T;
use crate::dhatu_gana as gana;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref F: SoundSet = s("f");
    static ref IK: SoundSet = s("ik");
    static ref JHAL: SoundSet = s("Jal");
    static ref HAL: SoundSet = s("hal");
}

pub fn is_kutadi(t: &Term) -> bool {
    // Check number explicitly because some roots are duplicated within tudAdi
    // but outside this gana (e.g. juq).
    t.has_gana(6) && t.has_u_in(&gana::KUT_ADI)
}

fn run_before_attva_at_index(p: &mut Prakriya, i: usize) {
    let n = match p.view(i + 1) {
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

    let n = p.view(i + 1).unwrap();
    let last = p.terms().last().unwrap();
    let lin_or_sic = last.has_lakshana("li~N") || n.has_u("si~c");
    if last.has_tag(T::Atmanepada) && lin_or_sic && n.has_adi(&*JHAL) {
        let t = &p.terms()[i];
        let is_dhatu = t.has_tag(T::Dhatu);
        let i_n = n.end();
        let is_ik_halanta = t.has_upadha(&*IK) && t.has_antya(&*HAL);
        if is_dhatu && is_ik_halanta {
            p.op_term("1.2.11", i_n, op::add_tag(T::kit));
        } else if is_dhatu && t.has_antya(&*F) {
            p.op_term("1.2.12", i_n, op::add_tag(T::kit));
        }
    }
}

/// Runs most atidesha rules.
pub fn run_before_attva(p: &mut Prakriya) {
    for i in 0..p.terms().len() {
        run_before_attva_at_index(p, i);
    }
}

/// Runs atidesha rules that must follow rule 6.1.45 (Adeca upadeze 'ziti).
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
pub fn run_after_attva(p: &mut Prakriya) {
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return,
    };
    let n = match p.view(i + 1) {
        Some(n) => n,
        None => return,
    };
    let i_tin = p.terms().len() - 1;

    let stha_ghu = p.has(i, |t| t.text == "sTA" || t.has_tag(T::Ghu));
    if stha_ghu || p.has(i_tin, f::atmanepada) && n.has_u("si~c") {
        let i_n_end = n.end();
        p.op("1.2.17", |p| {
            p.set(i, op::antya("i"));
            p.set(i_n_end, op::add_tag(T::kit));
        });
    }
}
