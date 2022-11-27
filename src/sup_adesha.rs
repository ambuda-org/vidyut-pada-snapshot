use crate::constants::Tag as T;
use crate::filters as f;
use crate::operations as op;
use crate::prakriya::Prakriya;
use crate::sounds::s;
use crate::stem_gana as gana;

fn yatha(needle: &str, old: &'static [&str], new: &'static [&str]) -> Option<&'static str> {
    for (i, o) in old.iter().enumerate() {
        if needle == *o {
            return Some(new[i]);
        }
    }
    None
}

/// Tries adesha rules for stems ending in 'a'.
fn try_adanta_adesha(p: &mut Prakriya, i_anga: usize, i: usize) {
    let nasi_ni = &["Nasi~", "Ni"];
    let smat_smin = &["smAt", "smin"];
    let ta_nasi_nas = &["wA", "Nasi~", "Nas"];
    let ina_at_sya = &["ina", "At", "sya"];

    let is_sarvanama = p.has(i_anga, f::tag(T::Sarvanama));
    let sup_u = match &p.terms()[i].u {
        Some(u) => u.to_string(),
        None => "".to_string(),
    };

    if p.has(i, f::text("Bis")) {
        if p.has(i_anga, f::text_in(&["idam", "adas"])) {
            p.step("7.1.11");
        } else {
            p.op_term("7.1.9", i, op::text("Es"));
        }
    } else if is_sarvanama && p.has(i, f::u_in(nasi_ni)) {
        let mut do_sub = true;
        if p.has(i_anga, f::text_in(gana::PURVA_ADI)) {
            if p.is_allowed("7.1.16") {
                p.step("7.1.16");
                do_sub = false
            } else {
                p.decline("7.1.16");
            }
        }
        if do_sub {
            if let Some(sub) = yatha(&sup_u, nasi_ni, smat_smin) {
                p.op_term("7.1.9", i, op::text(sub));
            }
        }
    }

    if p.has(i, |t| t.has_u_in(ta_nasi_nas) && t.has_text(&["A", "as"])) {
        if let Some(sub) = yatha(&sup_u, ta_nasi_nas, ina_at_sya) {
            p.op_term("7.1.12", i, op::text(sub));
        }
    } else if p.has(i, f::u("Ne")) {
        if is_sarvanama {
            p.op_term("7.1.14", i, op::text("smE"));
        } else {
            p.op_term("7.1.13", i, op::text("ya"));
        }
    } else if is_sarvanama && p.has(i, f::u("jas")) {
        p.op("7.1.17", |p| op::upadesha(p, i, "SI"));
    }
}

/// Tries adesha rules for `asmad` and `yuzmad`.
fn try_yusmad_asmad_sup_adesha(p: &mut Prakriya, i_anga: usize, i: usize) {
    if !p.has(i_anga, f::text_in(&["yuzmad", "asmad"])) {
        return;
    }

    let sup = &p.terms()[i];
    if sup.has_u("Nas") {
        p.op_term("7.1.27", i, op::text("a"));
    } else if sup.has_u("Ne") || sup.any(&[T::Prathama, T::V2]) {
        if sup.has_u("Sas") {
            p.step("7.1.29");
        } else {
            p.op_term("7.1.28", i, op::text("am"));
        }
    } else if sup.has_u("Byas") {
        if sup.has_tag(T::V5) {
            p.op_term("7.1.31", i, op::text("at"));
        } else {
            p.op_term("7.1.30", i, op::text("Byam"));
        }
    } else if sup.all(&[T::V5, T::Ekavacana]) {
        p.op_term("7.1.32", i, op::text("at"));
    }
    // TODO: 7.1.33
}

fn try_ni_adesha(p: &mut Prakriya, i_anga: usize, i: usize) {
    if p.has(i, f::u("Ni")) && p.has(i_anga, f::antya("it ut")) {
        p.op_term("7.3.118", i, op::text("O"));
        if p.has(i_anga, f::tag(T::Ghi)) {
            p.op_term("7.3.119", i_anga, op::antya("a"));
        }
    }
    if p.has(i, f::u("wA")) && p.has(i_anga, |t| t.has_tag(T::Ghi) && !t.has_tag(T::Stri)) {
        p.op_term("7.3.120", i, op::text("nA"));
    }
}

/// (7.1.19 - 7.1.32)
pub fn run(p: &mut Prakriya) {
    let i = p.terms().len() - 1;
    if !p.has(i, f::sup) {
        return;
    }
    let i_anga = i - 1;

    try_ni_adesha(p, i_anga, i);

    let is_napumsaka = p.has(i_anga, f::tag(T::Napumsaka));
    let is_jas_shas = p.has(i, f::u_in(&["jas", "Sas"]));

    if p.has(i_anga, f::u_in(&["dAp", "wAp", "cAp"])) && p.has(i, f::text("O")) {
        p.op("7.1.18", |p| op::upadesha(p, i, "SI"));
    } else if is_napumsaka && p.has(i, f::text("O")) {
        p.op("7.1.19", |p| op::upadesha(p, i, "SI"));
    } else if is_napumsaka && is_jas_shas {
        p.op("7.1.20", |p| op::upadesha(p, i, "Si"));
    } else if p.has(i_anga, |t| t.text == "azwA" && t.has_u("azwan")) && is_jas_shas {
        p.op("7.1.21", |p| op::upadesha(p, i, "OS"));
    } else if p.has(i_anga, f::text("zaz")) && is_jas_shas {
        p.op_term("7.1.22", i, op::luk);
    } else if is_napumsaka && p.has(i, f::u_in(&["su~", "am"])) {
        if p.has(i_anga, |t| t.has_antya(&s("a"))) {
            if p.has(i_anga, |t| t.has_text(gana::DATARA_ADI)) {
                p.op_term("7.1.25", i, op::text("adq"));
            } else {
                p.op_term("7.1.24", i, op::text("am"));
            }
        } else {
            p.op_term("7.1.23", i, op::luk);
        }
    } else {
        try_adanta_adesha(p, i_anga, i);
    }

    try_yusmad_asmad_sup_adesha(p, i_anga, i);
}
