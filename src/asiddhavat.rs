/*!
asiddhavat (6.4.22 - 6.4.175 [end])
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Rules in the *asiddhavat* rules do not interfere with each other. That is, if
a rule A would ordinary block a rule B, both are allowed to apply if they are
defined within this section.

*asiddhavat* rules are within the scope of the *aNgasya* adhikAra. For details,
see the `angasya` module.
*/

use crate::constants::Tag as T;
use crate::filters as f;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::term::Term;

/*

fn run_kniti_ardhadhatuka(p: Prakriya, index: int):
    c = p.terms[index]
    n = TermView.make(p, index)

    // Run only if followed by kit/Nit Ardhadhatuka
    if not n:
        return

    kniti_ardha = n.any("k", "N") and n.any(T.ARDHADHATUKA)

    // HACK
    n.u = n.terms[0].u

    if c.u == "dI\\N" and n.adi in s("ac"):
        op.insert_agama_after("6.4.63", p, index, "yu~w")

    if (
        c.antya == "A"
        and n.adi in s("ac")
        and (kniti_ardha or f.is_it_agama(n.terms[0]))
    ):
        op.antya("6.4.64", p, c, "")

    // TODO: rule has iwi, but we have aci already. So what's the point?
    ma_stha = {"mA", "sTA", "gA", "sA"}
    if c.antya == "A" and kniti_ardha:
        if n.adi in s("hal") and (
            c.all(T.GHU)
            or c.text in ma_stha
            or c.u == "o~hA\\k"
            or (c.u == "pA\\" and c.gana == 1)
        ):
            if n.all("li~N"):
                op.antya("6.4.67", p, c, "e")
            else:
                op.antya("6.4.66", p, c, "I")
        } else if  f.samyogadi(c):
            dhatu = TermView.make_dhatu(p, index)
            if dhatu and dhatu.antya not in s("ac"):
                // HACK: skip dhatus with agama
                pass
            } else if  n.u == "lyap":
                p.step("6.4.69")
            } else if  n.all("li~N"):
                op.optional(op.antya, "6.4.68", p, c, "e")
        } else if  n.u == "yat":
            op.antya("6.4.65", p, c, "I")


fn run_kniti(p: Prakriya, index):
    """Rules conditioned on a following `kit` or `Nit` suffix.

    (6.4.98 - 6.4.126)
    """
    c = p.terms[index]
    n = TermView.make(p, index)
    assert n.any("k", "N")

    prev = p.terms[index - 1] if index > 0 else None

    // View up to and including current term
    view = StringView(p.terms[: index + 1])
    prefix = view.text[:-1]
    try:
        samyogapurva = view.text[-2] in s("hal") and view.text[-3] in s("hal")
    except IndexError:
        samyogapurva = False

    _gama_hana = {"gam", "han", "jan", "Kan", "Gas"}
    if c.text in _gama_hana and n.adi in s("ac") and n.terms[0].u != "aN":
        op.upadha("6.4.98", p, c, "")
    } else if  (c.text == "hu" or c.antya in s("Jal")) and n.terms[-1].text == "hi":
        op.text("6.4.101", p, n.terms[-1], "Di")
    } else if  c.u == "ciR":
        op.luk("6.4.104", p, n.terms[0])
    } else if  c.antya == "a" and n.terms[0].text == "hi":
        op.luk("6.4.105", p, n.terms[0])
    } else if  c.antya == "u" and not samyogapurva and n.terms[0].text == "hi":
        op.luk("6.4.106", p, n.terms[0])

    n = TermView.make(p, index)
    if not n:
        return

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


fn lit_et(p: Prakriya, index: int):
    """Replacement with e and abhyasa-lopa."""
    c = p.terms[index]
    if not c.all(T.DHATU, T.ABHYASTA):
        return

    abhyasa = p.terms[index - 1]
    if not abhyasa.any(T.ABHYASA):
        // For example, if nuT-Agama separates the abhyAsa from the abhyasta.
        return

    n = TermView.make(p, index)

    ekahalmadhya = len(c.text) == 3 and c.adi in s("hal") and c.antya in s("hal")
    // aspirated consonants become usaspirated in the tripAdi, which hasn't run
    // yet at this stage in the derivation. So, also "look ahead" and check for
    // aspirated consonants.
    anadeshadi = abhyasa.adi == c.adi and abhyasa.adi not in sounds.MAHAPRANA

    kniti = n.any("k", "N")
    try:
        thali_seti = n.terms[0].u == "iw" and n.terms[1].u == "Tal"
    except IndexError:
        thali_seti = False

    if not (kniti or thali_seti):
        return

    if c.text == "daB" and c.u == "danBu~":
        abhyasa.text = ""
        op.upadha("6.4.120.v1", p, c, "e")
    } else if  c.u == "tF" or c.text in {"Pal", "Baj", "trap"}:
        abhyasa.text = ""
        op.upadha("6.4.122", p, c, "e")
    } else if  c.text == "SraT" and c.u == "SranTa~":
        abhyasa.text = ""
        op.upadha("6.4.122.v1", p, c, "e")
    } else if  c.text == "graT":
        abhyasa.text = ""
        // TODO: attested, but can't find the rule for it.
        op.upadha("???", p, c, "e")
    } else if  c.text == "rAD":
        if p.allow("6.4.123"):
            abhyasa.text = ""
            op.upadha("6.4.123", p, c, "e")
        else:
            p.decline("6.4.123")
    } else if  c.u == "jF" or c.text in {"Bram", "tras"}:
        if p.allow("6.4.124"):
            abhyasa.text = ""
            op.upadha("6.4.124", p, c, "e")
        else:
            p.decline("6.4.124")
    } else if  c.u in PHAN_ADI:
        if p.allow("6.4.125"):
            abhyasa.text = ""
            op.upadha("6.4.125", p, c, "e")
        else:
            p.decline("6.4.125")
    // General cases
    } else if  c.text in {"Sas", "dad"} or c.adi == "v" or c.any(T.F_GUNA):
        p.step("6.4.126")
    } else if  ekahalmadhya and c.upadha == "a" and n.all("li~w") and anadeshadi:
        if kniti:
            code = "6.4.120"
        else:
            code = "6.4.121"
        abhyasa.text = ""
        op.upadha(code, p, c, "e")


fn ardhadhatuke(p: Prakriya, index: int):
    """Rules conditioned on a following ardhadhatuka suffix.

    (6.4.46 - 6.4.70)
    """
    c = p.terms[index]
    n = TermView.make(p, index)
    if not n or not n.any(T.ARDHADHATUKA):
        return
    // HACK to avoid abhyasa-at-lopa
    if c.all(T.ABHYASA):
        return

    if c.text == "Brasj":
        op.optional(op.text, "6.4.47", p, c, "Barj")

    } else if  c.antya == "a":
        op.antya("6.4.48", p, c, "")
        c.add_tags(T.F_AT_LOPA)
        // TODO: remove P F_AT_LOPA
        p.add_tags(T.F_AT_LOPA)


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

pub fn run_before_guna(p: &mut Prakriya, i: usize) {
    /*
    c = p.terms[index]
    n = TermView.make(p, index)
    if not n:
        return

    n.u = n.terms[0].u

    if c.all("Snam") and c.upadha == "n":
        op.upadha("6.4.23", p, c, "")

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
    if let Some(n) = p.view(i + 1) {
        if p.has(i, f::text("BU")) && n.has_lakshana_in(&["lu~N", "li~w"]) {
            op::append_agama("6.4.88", p, i, "vu~k");
        }
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
}

/*
fn run_aci(p: Prakriya, index: int):
    """Apply sound changes when a vowel follows.

    6.4.77 - 6.4.100
    """

    c = p.terms[index]
    n = TermView.pratyaya2(p, index)
    if n is None:
        try:
            n = p.terms[index + 1]
        except IndexError:
            n = None

    // All of the rules below condition on a following vowel.
    if not n or n.adi not in s("ac"):
        return

    iyuv = {"i": "iy", "I": "iy", "u": "uv", "U": "uv"}

    try:
        prev = p.terms[index - 1]
    except IndexError:
        prev = None
    if (
        c.u in ("hu\\", "Snu")
        and c.antya == "u"
        and n.all(T.SARVADHATUKA)
        and prev
        and prev.antya not in s("hal")
    ):
        p.debug(prev)
        op.antya("6.4.87", p, c, "v")

    // General case
    if c.antya in iyuv:
        if c.u == "i\\R":
            op.antya("6.4.81", p, c, "y")

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


fn run_nau(p: Prakriya, index: int):
    """Asiddhavat rules conditioned on a following Ri suffix."""

    c = p.terms[index]
    n = TermView.make(p, index)
    if not n:
        return

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

    if c.all("m") and n.terms[0].u == "Ric" and c.upadha in s("ac"):
        op.upadha("6.4.92", p, c, sounds.hrasva(c.upadha))


fn run_after_guna(p: Prakriya, index: int):
    c = p.terms[index]
    n = TermView.make(p, index)
    if not n:
        return

    run_kniti_ardhadhatuka(p, index)
    run_aci(p, index)

    if f.is_knit(n):
        run_kniti(p, index)

    // TODO: fails kniti check because this depends on the last affix, and
    // term view includes only "u" here. So the rule is awkwardly placed
    // here.
    last = p.terms[-1]
    sarva_kniti = last.all(T.SARVADHATUKA) and last.any("k", "N")
    if c.u == "qukf\\Y" and c.text == "kar" and n.adi == "u" and sarva_kniti:
        c.text = "kur"
        p.step("6.4.110")

    lit_et(p, index)

    if n.terms[0].all("q"):
        op.ti("6.4.143", p, c, "")
*/
