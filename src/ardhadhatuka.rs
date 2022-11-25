//! ardhadhatuka

use crate::constants::Tag as T;
use crate::constants::{La, Purusha, Vacana};
use crate::filters as f;
use crate::operations as op;
use crate::it_samjna;
use crate::term::Term;
use crate::sounds::s;
use crate::prakriya::Prakriya;
use std::error::Error;

/*
fn _causes_guna(n: TermView):
    """Lookahead function for the following rules:

    6.1.50 minātiminotidīṅāṃ lyapi ca
    6.1.51 vibhāṣā līyateḥ
    """
    # Parasmaipada Ashir-liN will use yAsuT-Agama, which is kit.
    if n.all("li~N", T.ARDHADHATUKA, T.PARASMAIPADA):
        return False
    # sArvadhAtukam apit will be Nit.
    if n.all(T.SARVADHATUKA) and not n.all("p"):
        return False
    # apit liT when not after samyoga will be kit.
    # TODO: check for samyoga? But it's not needed for current usage
    if n.all("li~w") and not n.all("p"):
        return False
    # ArdhadhAtuka and other sArvadhAtuka suffixes will cause guna.
    return True
    */


/// Replaces the dhAtu based on the suffix that follows it.
///
// These rules must run before we choose the verb pada.
pub fn dhatu_adesha_before_pada(p: &mut Prakriya, la: La) {
    let i = match p.find_first(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    if la.is_sarvadhatuka() {
        return;
    }

    // KyAY is Yit, which allow parasamipada.
    if p.has(i, |t| t.has_u("ca\\kzi~\\N")) {
        let mut use_khya = true;
        if la == La::Lit {
            if p.is_allowed("2.4.55") {
                use_khya = false
            } else {
                p.decline("2.4.55")
            }
        }
        if use_khya {
            p.op("2.4.54", |p| {
                op::upadesha(p, i, "KyAY");
                // Remove tags set by `ca\kzi~\N` 
                p.set(i, |t| {
                    t.remove_tags(&[T::anudattet, T::Nit]);
                    t.add_tag(T::Anudatta);
                });
                // For anit on `KyAY`.
            });
        }
    }
}

/*
fn dhatu_adesha_before_vikarana(p: Prakriya):
    """Replace the dhAtu based on the following suffix.

    These rules must run before the vikarana is added.
    """
    index, c = p.find_first(T.DHATU)
    n = TermView.make(p, index)

    la = p.terms[-1]
    vidhi_lin = la.any("li~N") and not p.all(T.ASHIH)
    is_sarvadhatuka = la.any("la~w", "lo~w", "la~N") or vidhi_lin
    if is_sarvadhatuka:
        return

    # HACK to make the below readable
    n.u = n.terms[0].u

    if c.text == "ad":
        if n.any("lu~N", "san"):
            op.upadesha("2.4.37", p, c, "Gasx~")
        } else if  n.u in ("GaY", "ap"):
            op.upadesha("2.4.38", p, c, "Gasx~")
        } else if  n.any("li~w"):
            op.optional(op.upadesha, "2.4.40", p, c, "Gasx~")
        } else if  n.u == "lyap" or (n.adi == "t" and n.any("k")):
            op.upadesha("2.4.36", p, c, "jagDi~")

    # Skip 2.4.39 (bahulaM chandasi).

    } else if  c.u == "ve\\Y" and n.any("li~w"):
        op.optional(op.upadesha, "2.4.41", p, c, "vayi~")
    } else if  c.text == "han":
        if n.any("li~N"):
            op.upadesha("2.4.42", p, c, "vaDa")
        } else if  n.any("lu~N"):
            if n.any(T.ATMANEPADA):
                op.optional(op.upadesha, "2.4.44", p, c, "vaDa")
            else:
                op.upadesha("2.4.43", p, c, "vaDa")
    } else if  c.u in {"i\\R", "i\\k"}:
        if c.u == "i\\k":
            p.step("2.4.45.v1")
        if n.any("lu~N"):
            op.upadesha("2.4.45", p, c, "gA")
        } else if  n.u == "Ric":
            op.optional(op.upadesha, "2.4.46", p, c, "gami~")
        } else if  n.u == "san":
            op.optional(op.upadesha, "2.4.47", p, c, "gami~")
    } else if  c.u == "i\\N":
        if n.u == "san":
            op.upadesha("2.4.48", p, c, "gami~")
        } else if  n.any("li~w"):
            op.upadesha("2.4.49", p, c, "gAN")
        } else if  n.any("lu~N", "lf~N"):
            op.optional(op.upadesha, "2.4.50", p, c, "gAN")

    } else if  c.u == "asa~":
        op.upadesha("2.4.52", p, c, "BU")
    } else if  c.u == "brUY":
        # anudAtta to prevent iT
        op.upadesha("2.4.53", p, c, "va\\ci~")
    } else if  c.u == "aja~" and n.u not in ("GaY", "ap"):
        do = True
        if n.u == "lyuw":
            if p.allow("2.4.57"):
                do = False
            else:
                p.decline("2.4.57")
        # vArttika: valAdAvArdhadhAtuke veSyate
        #
        # This vArttika is troublesome and highly constrained. To derive
        # vivAya, we must run in this order:
        #
        #   siddhi --> vArttika --> dvitva
        #
        # But tin-siddhi must follow dvitva for rule 3.4.109. I considered
        # breaking siddhi into two stages -- one for liT, and one for other
        # lakAras -- and that might be worth doing as the program matures.
        # But for now, I don't want to change the entire structure of the
        # program to handle a rare secondary rule like this.
        #
        # As a crude fix, just check for endings that we expect will start with
        # vowels.
        will_yasut = la.all("li~N", T.PARASMAIPADA) and p.any(T.ASHIH)
        is_lit_ajadi = la.all("li~w") and la.adi in s("ac")
        if n.adi in s("val") and not (will_yasut or is_lit_ajadi):
            if p.allow("2.4.56.v2"):
                p.step("2.4.56.v2")
                do = False
            else:
                p.decline("2.4.56.v2")
        if do:
            # aniT-tva comes from anudAtta in upadesha.
            op.upadesha("2.4.56", p, c, "vI\\")


fn dhatu_adesha_after_vikarana(p: Prakriya):
    """
    This code depends on the Ric-vikaraNa being present.
    """
    index, c = p.find_first(T.DHATU)
    n = TermView.make(p, index)

    if not n.any(T.ARDHADHATUKA):
        return

    # HACK to make the below readable
    n.u = n.terms[0].u

    try:
        n2 = p.terms[index + 2]
    except IndexError:
        n2 = None
    if c.u == "i\\N" and n2:
        n2 = p.terms[index + 2]
        if n.u == "Ric" and n2 and n2.u in ("san", "caN"):
            op.optional(op.upadesha, "2.4.50", p, c, "gAN")


fn aa_adesha(p: Prakriya, index: int):
    c = p.terms[index]
    if not c.any(T.DHATU):
        return
    n = TermView.make(p, index)
    if not n:
        return

    # HACK
    n.u = n.terms[0].u

    # Substitution of A for root vowel

    if c.antya in s("ec") and not n.any("S"):
        if c.text == "vye" and n.any("li~w"):
            p.step("6.1.46")
        else:
            op.antya("6.1.45", p, c, "A")
    } else if  c.text in {"sPur", "sPul"} and n.u == "GaY":
        op.upadha("6.1.47", p, c, "A")
    } else if  c.u in {"qukrI\\Y", "i\\N", "ji\\"} and n.u == "Ric":
        op.antya("6.1.48", p, c, "A")
    # TODO: 6.1.49

    # 6.1.50 has a circular dependency:
    #
    # - 6.1.50 comes before dvitva
    # - dvitva comes before tin-siddhi
    # - tin-siddhi changes the application of guNa
    # - guNa affects the application of 6.1.50
    #
    # So, "look ahead" and use this rule only if the suffix will potentially
    # cause guNa. See `_causes_guna` for details.
    ashiti_lyapi = not n.any("S") or n.u == "lyap"
    if c.u in {"mI\\Y", "qu\\mi\\Y", "dI\\N"} and ashiti_lyapi and _causes_guna(n):
        op.antya("6.1.50", p, c, "A")
    } else if  c.text == "lI" and ashiti_lyapi and _causes_guna(n) and c.gana != 10:
        # līyateriti yakā nirdeśo na tu śyanā. līlīṅorātvaṃ vā syādejviṣaye
        # lyapi ca. (SK)
        op.optional(op.antya, "6.1.51", p, c, "A")
    # TODO: 6.1.52 - 6.1.53
    } else if  c.u in {"ciY", "ci\\Y", "sPura~"} and n.u == "Ric":
        if c.text == "sPura~":
            op.optional(op.upadha, "6.1.54", p, c, "A")
        else:
            op.optional(op.antya, "6.1.54", p, c, "A")
    # TODO: 6.1.55 - 6.1.56
    } else if  c.text == "smi" and n.u == "Ric":
        op.optional(op.antya, "6.1.57", p, c, "A")


fn am_agama_for_term(p: Prakriya, index: int):
    c = p.terms[index]
    if not c.any(T.DHATU):
        return
    n = TermView.make(p, index)
    if not n:
        return

    if n.adi in s("Jal") and not n.any("k"):
        if c.text in {"sfj", "dfS"}:
            op.mit("6.1.58", p, c, "a")
        } else if  c.all(T.ANUDATTA) and c.upadha == "f":
            op.optional(op.mit, "6.1.59", p, c, "a")


fn run_before_vikarana(p: Prakriya):
    dhatu_adesha_before_vikarana(p)


fn run_before_dvitva(p: Prakriya):
    """Replace the dhAtu based on the following suffix.

    These rules must run after the vikarana is added and before dvitva.
    """
    dhatu_adesha_after_vikarana(p)

    for i, _ in enumerate(p.terms):
        aa_adesha(p, i)


fn am_agama(p: Prakriya):
    for i, _ in enumerate(p.terms):
        am_agama_for_term(p, i)
*/
