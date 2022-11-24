use crate::constants::Tag as T;
use crate::prakriya::Prakriya;
use crate::operations as op;

fn run_for_prakriya(p: &mut Prakriya, i: usize) {
    let add_sarva = op::add_tag(i, T::Sarvadhatuka);
    let add_ardha = op::add_tag(i, T::Ardhadhatuka);

    if p.has(i, |t| t.has_tag(T::Pratyaya)) {
        if p.has(i, |t| t.has_lakshana("li~w")) {
            p.op("3.4.115", add_ardha);
        } else if p.has(i, |t| t.has_lakshana("li~N") && p.has_tag(T::Ashih)) {
            p.op("3.4.116", add_ardha);
        } else if p.has(i, |t| t.any(&[T::Tin, T::Sit])) {
            if !p.has(i, |t| t.has_tag(T::Sarvadhatuka)) {
                p.op("3.4.113", add_sarva);
            }
        } else {
            // Suffixes introduced before "dhAtoH" are not called ArdhadhAtuka.
            // So they will not cause guNa and will not condition iT-Agama.
            if p.has(i, |t| t.has_tag(T::FlagNoArdhadhatuka)) {
                // do nothing
            } else if !p.has(i, |t| t.has_tag(T::Ardhadhatuka)) {
                p.op("3.4.114", add_ardha);
            }
        }
    }
}

pub fn run(p: &mut Prakriya) {
    let n = p.terms().len();
    for i in 0..n {
        run_for_prakriya(p, i);
    }
}
