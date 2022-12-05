/// Runs rules that perform `dvitva` (doubling) on the dhAtu.
use crate::constants::Tag as T;
use crate::filters as f;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds as al;
use crate::sounds::{s, SoundSet};
use crate::term::Term;
use compact_str::CompactString;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref NDR: SoundSet = s("n d r");
}

fn do_dvitva(rule: Rule, p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get(i)?;
    let next = p.get(i + 1)?;

    if al::is_ac(dhatu.adi()?) && al::is_hal(dhatu.antya()?) && next.has_u_in(&["Ric", "RiN"]) {
        // Case 1a: starts with vowel + a following Ni-pratyaya as part of the vowel.
        //
        // Example:
        // -  anj + i -> an + ji + j
        let i_ni = i + 1;
        let i_third = i + 2;
        let dhatu = p.get(i)?;
        let ni = p.get(i_ni)?;

        let mut text = CompactString::from("");
        text.push_str(&dhatu.text[1..]);
        text.push_str(&ni.text);
        let mut third = Term::make_text(&text);

        // 6.1.3 na ndrAH saMyogAdayaH
        while f::is_samyogadi(&third) && NDR.contains_char(third.adi()?) {
            third.set_adi("");
        }

        // The structure here is workaround for a Rust compile issue.
        if ni.has_u("Ric") {
            third.set_u("Ric");
        } else {
            third.set_u("RiN");
        }
        third.add_tag(T::Dhatu);

        // Update old text so that downstream abhyasa rules like `k -> c` can take effect
        // correctly.
        /*
        p.set(i+1, |t| t.set_text(&third.text));
        // HACK: ugly way of decreasing the dhatu's length.
        for i in 0..third.text.len()-1 {
            p.set(i, |t| t.set_antya(""));
        }
        */

        p.insert_after(i_ni, third);
        p.op_term("6.1.4", i_ni, op::add_tag(T::Abhyasa));

        p.op("6.1.5", |p| {
            p.set(i, op::add_tag(T::Abhyasta));
            p.set(i_ni, op::add_tag(T::Abhyasta));
            p.set(i_third, op::add_tag(T::Abhyasta));
        });
    } else if f::is_eka_ac(dhatu) || al::is_hal(dhatu.adi()?) {
        // TODO: correctly double jAgR
        p.insert_before(i, Term::make_text(&dhatu.text));
        p.step(rule);

        let i_abhyasa = i;
        let i_dhatu = i + 1;
        p.op_term("6.1.4", i_abhyasa, op::add_tag(T::Abhyasa));

        p.set(i_abhyasa, |t| t.add_tag(T::Abhyasta));
        p.set(i_dhatu, |t| t.add_tag(T::Abhyasta));
        if p.has(i_dhatu + 1, |t| t.has_u_in(&["Ric", "RiN"])) {
            p.set(i_dhatu + 1, |t| t.add_tag(T::Abhyasta));
        }
        p.step("6.1.5")
    } else {
    }

    Some(())
}

/*
fn _double(rule: str, p: Prakriya, dhatu: Term, i: int) -> Term:

    } else if  eka_ac(dhatu) or dhatu.adi in s("hal"):
        // TODO: correctly double jAgR
        abhyasa = Term.make_term(dhatu.text)
        op.insert_before(rule, p, dhatu, abhyasa)
        op.samjna("6.1.4", p, abhyasa, T.ABHYASA)

        abhyasa.add_tags(T.ABHYASTA)
        dhatu.add_tags(T.ABHYASTA)
        if p.terms[i + 2].u in ("Ric", "RiN"):
            p.terms[i + 2].add_tags(T.ABHYASTA)
        p.step("6.1.5")
    else:
        // Create 3 terms:
        // 1. the dhatu without the abhyasa
        // 2. the abhyasa
        // 3. the doubled portion

        // 6.1.2 ajAder dvitIyasya
        // 6.1.3 na ndrAH saMyogAdayaH
        third = Term.make_term(dhatu.text[1:])
        while f.samyogadi(third) and third.adi in {"n", "d", "r"}:
            third.text = third.text[1:]
        third.u = dhatu.u
        third.add_tags(T.DHATU)

        // Ru -> nu for UrRu
        if dhatu.text == "UrRu":
            third.text = "nu"

        abhyasa = Term.make_term(third.text)
        abhyasa.add_tags(T.ABHYASA)
        dhatu.text = dhatu.text[: -len(third.text)]

        op.insert_after(None, p, dhatu, abhyasa)
        op.insert_after(rule, p, abhyasa, third)
        op.samjna("6.1.4", p, abhyasa, T.ABHYASA)

        dhatu.add_tags(T.ABHYASTA)
        third.add_tags(T.ABHYASTA)
        abhyasa.add_tags(T.ABHYASTA)
        if p.terms[i + 3].u in ("Ric", "RiN"):
            p.terms[i + 3].add_tags(T.ABHYASTA)
        p.step("6.1.5")
*/

pub fn run(p: &mut Prakriya) -> Option<()> {
    // Select !pratyaya to avoid sanAdi, which are also labeled as Dhatu.
    let i =
        p.find_last_where(|t| t.has_tag(T::Dhatu) && !t.has_tag_in(&[T::Abhyasta, T::Pratyaya]))?;

    let jaksh_adi = &["jakz", "jAgf", "daridrA", "cakAs", "SAs", "dIDI", "vevI"];
    if p.has(i, |t| t.has_text_in(jaksh_adi)) {
        // These are termed abhyasta, but they can still undergo dvitva because
        // the rules below are conditioned specifically on "anabhyAsasya" ("not having an abhyasa")
        // from 6.1.8.
        p.op_term("6.1.6", i, op::add_tag(T::Abhyasta));
    }

    let n = p.get(i + 1)?;
    if p.terms().last()?.has_lakshana("li~w") {
        let dhatu = p.get(i)?;
        // kAshikA:
        //   dayateḥ iti dīṅo grahaṇaṃ na tu daya dāne ityasya.
        //   digyādeśena dvirvacanasya bādhanam iṣyate.
        if dhatu.has_u("de\\N") {
            p.op_term("7.4.9", i, op::text("digi"));
        } else {
            do_dvitva("6.1.8", p, i);
        }
    } else if n.has_u_in(&["san", "yaN"]) {
        do_dvitva("6.1.9", p, i);
    } else if n.has_tag(T::Slu) {
        do_dvitva("6.1.10", p, i);
    } else if p.find_next_where(i, |t| t.has_u("caN")).is_some() {
        // `last()` to avoid `it`.
        do_dvitva("6.1.11", p, i);
    }

    Some(())
}
