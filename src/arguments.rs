use crate::constants::Tag;

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
