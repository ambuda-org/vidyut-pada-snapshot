use crate::operators as op;
use crate::prakriya::Prakriya;
use crate::tag::Tag as T;

fn run_for_prakriya(p: &mut Prakriya, i: usize) -> Option<()> {
    let add_sarva = op::t(i, op::add_tag(T::Sarvadhatuka));
    let add_ardha = op::t(i, op::add_tag(T::Ardhadhatuka));

    let pratyaya = p.get(i)?;

    if pratyaya.has_tag(T::Pratyaya) {
        if pratyaya.has_lakshana("li~w") {
            p.op("3.4.115", add_ardha);
        } else if pratyaya.has_lakshana("li~N") && p.has_tag(T::Ashih) {
            p.op("3.4.116", add_ardha);
        } else if pratyaya.has_tag_in(&[T::Tin, T::Sit]) {
            if !pratyaya.has_tag(T::Sarvadhatuka) {
                p.op("3.4.113", add_sarva);
            }
        } else {
            // Suffixes introduced before "dhAtoH" are not called ArdhadhAtuka.
            // So they will not cause guNa and will not condition iT-Agama.
            if pratyaya.has_tag(T::FlagNoArdhadhatuka) {
                // do nothing
            } else if !pratyaya.is_empty() && !pratyaya.has_tag(T::Ardhadhatuka) {
                // Check `is_empty` to avoid including luk, etc.
                p.op("3.4.114", add_ardha);
            }
        }
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    let n = p.terms().len();
    for i in 0..n {
        run_for_prakriya(p, i);
    }
}
