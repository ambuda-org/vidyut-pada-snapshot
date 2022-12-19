/*!
Most of the arguments to `Ashtadhyayi`s core functions are strongly typed enums.
*/
use crate::constants::Tag;
use std::str::FromStr;

/// The prayoga of some tinanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Prayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// bhAve prayoga generally produces the same forms as karmani prayoga.
    Bhave,
}
impl Prayoga {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Kartari => Tag::Kartari,
            Self::Karmani => Tag::Karmani,
            Self::Bhave => Tag::Bhave,
        }
    }
    /// Returns a simple human-readable string that represents the enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Kartari => "kartari",
            Self::Karmani => "karmani",
            Self::Bhave => "bhave",
        }
    }
}
impl FromStr for Prayoga {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "kartari" => Self::Kartari,
            "karmani" => Self::Karmani,
            "bhave" => Self::Bhave,
            &_ => return Err("Could not parse Prayoga"),
        };
        Ok(res)
    }
}

/// The person of some tinanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Purusha {
    /// The third person.
    Prathama,
    /// The second person.
    Madhyama,
    /// The first person.
    Uttama,
}
impl Purusha {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Prathama => Tag::Prathama,
            Self::Madhyama => Tag::Madhyama,
            Self::Uttama => Tag::Uttama,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prathama => "prathama",
            Self::Madhyama => "madhyama",
            Self::Uttama => "uttama",
        }
    }
}
impl FromStr for Purusha {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "prathama" => Self::Prathama,
            "madhyama" => Self::Madhyama,
            "uttama" => Self::Uttama,
            &_ => return Err("Could not parse Purusha"),
        };
        Ok(res)
    }
}

/// The number of some tinanta or subanta.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}
impl Vacana {
    pub(crate) fn as_tag(&self) -> Tag {
        match self {
            Self::Eka => Tag::Ekavacana,
            Self::Dvi => Tag::Dvivacana,
            Self::Bahu => Tag::Bahuvacana,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eka => "eka",
            Self::Dvi => "dvi",
            Self::Bahu => "bahu",
        }
    }
}
impl FromStr for Vacana {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "eka" => Self::Eka,
            "dvi" => Self::Dvi,
            "bahu" => Self::Bahu,
            &_ => return Err("Could not parse Vacana"),
        };
        Ok(res)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vibhakti {
    /// The first vibhakti. Sometimes called the *nominative case*.
    Prathama,
    /// The second vibhakti. Sometimes called the *accusative case*.
    Dvitiya,
    /// The third vibhakti. Sometimes called the *instrumental case*.
    Trtiya,
    /// The fourth vibhakti. Sometimes called the *dative case*.
    Caturthi,
    /// The fifth vibhakti. Sometimes called the *ablative case*.
    Panchami,
    /// The sixth vibhakti. Sometimes called the *genitive case*.
    Sasthi,
    /// The seventh vibhakti. Sometimes called the *locative case*.
    Saptami,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lakara {
    /// Describes action in the present tense. Ssometimes called the *present indicative*.
    Lat,
    /// Describes unwitnessed past action. Sometimes called the *perfect*.
    Lit,
    /// Describes future action after the current day. Sometimes called the *periphrastic future*.
    Lut,
    /// Describes general future action. Sometimes called the *simple future*.
    Lrt,
    Let,
    /// Describes commands. Sometimes called the *imperative*.
    Lot,
    /// Describes past action before the current day. Sometimes called the *imperfect*.
    Lan,
    /// Describes options, potential action, etc. Sometimes called the *optative*.
    VidhiLin,
    /// Describes wishes and prayers. Sometimes called the *benedictive*.
    AshirLin,
    /// Describes general past action. Sometimes called the *aorist*.
    Lun,
    /// Describes past counterfactuals ("would not have ..."). Sometimes called the *conditional*.
    Lrn,
}

impl Lakara {
    pub fn as_str(&self) -> &'static str {
        match self {
            Lakara::Lat => "lat",
            Lakara::Lit => "lit",
            Lakara::Lut => "lut",
            Lakara::Lrt => "lrt",
            Lakara::Let => "let",
            Lakara::Lot => "lot",
            Lakara::Lan => "lan",
            Lakara::VidhiLin => "vidhi-lin",
            Lakara::AshirLin => "ashir-lin",
            Lakara::Lun => "lun",
            Lakara::Lrn => "lrn",
        }
    }

    /// Returns whether or not this lakara is Nit.
    pub(crate) fn is_nit(&self) -> bool {
        matches![
            self,
            Lakara::Lan | Lakara::AshirLin | Lakara::VidhiLin | Lakara::Lun | Lakara::Lrn
        ]
    }

    /// Returns whether or not this lakara will be termed sArvadhAtuka.
    pub(crate) fn is_sarvadhatuka(&self) -> bool {
        matches!(self, Lakara::Lat | Lakara::Lot | Lakara::Lan | Lakara::VidhiLin)
    }

    /// Returns whether or not this lakara will be termed ArdhadhAtuka.
    pub(crate) fn is_ardhadhatuka(&self) -> bool {
        !self.is_sarvadhatuka()
    }
}
impl FromStr for Lakara {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "lat" => Self::Lat,
            "lit" => Self::Lit,
            "lut" => Self::Lut,
            "lrt" => Self::Lrt,
            "let" => Self::Let,
            "lot" => Self::Lot,
            "lan" => Self::Lan,
            "vidhi-lin" => Self::VidhiLin,
            "ashir-lin" => Self::AshirLin,
            "lun" => Self::Lun,
            "lrn" => Self::Lrn,
            &_ => return Err("Could not parse La"),
        };
        Ok(res)
    }
}
