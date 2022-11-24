use crate::dhatu_karya;
use crate::prakriya::Prakriya;
use crate::constants::Tag as T;
use crate::sanadi;
use std::error::Error;

pub fn tinanta(dhatu: &str, code: &str, la: &str) -> Result<Prakriya, Box<dyn Error>> {
    let mut p = Prakriya::new();

    p_dhatu_karya::run(&mut p, dhatu, code)?;

    let vidhi_lin = false;//la == "li~N" && !p.all(T::Ashih);
    let is_sarvadhatuka = false;//vidhi_lin || la in {"la~w", "lo~w", "la~N"}

    p_sanadi::run(&mut p, is_sarvadhatuka)?;
    Ok(p)
}
