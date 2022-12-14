//! Structured arguments
use crate::constants::Tag;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Prayoga {
    Kartari,
    Karmani,
    Bhave,
}
impl Prayoga {
    pub fn as_tag(&self) -> Tag {
        match self {
            Self::Kartari => Tag::Kartari,
            Self::Karmani => Tag::Karmani,
            Self::Bhave => Tag::Bhave,
        }
    }
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
            &_ => return Err("Could not parse Prayoga")
        };
        Ok(res)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Purusha {
    Prathama,
    Madhyama,
    Uttama,
}
impl Purusha {
    pub fn as_tag(&self) -> Tag {
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
            &_ => return Err("Could not parse Purusha")
        };
        Ok(res)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vacana {
    Eka,
    Dvi,
    Bahu,
}
impl Vacana {
    pub fn as_tag(&self) -> Tag {
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
            &_ => return Err("Could not parse Vacana")
        };
        Ok(res)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vibhakti {
    Prathama,
    Dvitiya,
    Trtiya,
    Caturthi,
    Panchami,
    Sasthi,
    Saptami,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum La {
    Lat,
    Lit,
    Lut,
    Lrt,
    Let,
    Lot,
    Lan,
    VidhiLin,
    AshirLin,
    Lun,
    Lrn,
}

impl La {
    pub fn as_str(&self) -> &'static str {
        match self {
            La::Lat => "lat",
            La::Lit => "lit",
            La::Lut => "lut",
            La::Lrt => "lrt",
            La::Let => "let",
            La::Lot => "lot",
            La::Lan => "lan",
            La::VidhiLin => "vidhi-lin",
            La::AshirLin => "ashir-lin",
            La::Lun => "lun",
            La::Lrn => "lrn",
        }
    }
    pub fn is_nit(&self) -> bool {
        matches![
            self,
            La::Lan | La::AshirLin | La::VidhiLin | La::Lun | La::Lrn
        ]
    }
    pub fn is_sarvadhatuka(&self) -> bool {
        matches!(self, La::Lat | La::Lot | La::Lan | La::VidhiLin)
    }
    pub fn is_ardhadhatuka(&self) -> bool {
        !self.is_sarvadhatuka()
    }
}
impl FromStr for La {
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
            &_ => return Err("Could not parse La")
        };
        Ok(res)
    }
}
