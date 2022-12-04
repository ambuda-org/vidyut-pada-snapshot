use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

type Sound = char;

lazy_static! {
    static ref SUTRAS: Vec<Sutra> = create_shiva_sutras();
    static ref SOUND_PROPS: FxHashMap<Sound, Uccarana> = create_sound_props();
    static ref HAL: SoundSet = s("hal");
}

pub struct SoundSet([u8; 256]);

impl SoundSet {
    pub fn new(string: &str) -> Self {
        let mut res = SoundSet([0; 256]);
        for c in string.chars() {
            res.0[c as usize] = 1;
        }
        res
    }

    pub fn contains_char(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }

    pub fn contains_opt(&self, o: Option<char>) -> bool {
        if let Some(c) = o {
            self.contains_char(c)
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        // Use Paninian order.
        for c in "aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh".chars() {
            if self.contains_char(c) {
                ret.push(c);
            }
        }
        ret
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SoundMap([u8; 256]);

impl SoundMap {
    pub fn new() -> Self {
        Self([0; 256])
    }

    pub fn insert(&mut self, key: Sound, val: Sound) {
        self.0[key as usize] = val as u8;
    }

    pub fn contains(&self, key: Sound) -> bool {
        self.0[key as usize] != 0
    }

    pub fn get(&self, key: Sound) -> Option<Sound> {
        match self.0[key as usize] {
            0 => None,
            c => Some(c as char),
        }
    }
}

impl Default for SoundMap {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Pattern {
    /// Returns whether this `Master` includes the given sound.
    fn matches(&self, s: Sound) -> bool;
}

impl Pattern for char {
    fn matches(&self, s: Sound) -> bool {
        *self == s
    }
}

impl Pattern for SoundSet {
    fn matches(&self, s: Sound) -> bool {
        self.contains_char(s)
    }
}

impl Pattern for &SoundSet {
    fn matches(&self, s: Sound) -> bool {
        self.contains_char(s)
    }
}

struct Sutra {
    sounds: String,
    it: char,
}

impl Sutra {
    fn new(sounds: &str, it: char) -> Self {
        Sutra {
            sounds: sounds.to_string(),
            it,
        }
    }
}

fn create_shiva_sutras() -> Vec<Sutra> {
    vec![
        Sutra::new("aiu", 'R'),
        Sutra::new("fx", 'k'),
        Sutra::new("eo", 'N'),
        Sutra::new("EO", 'c'),
        Sutra::new("hyvr", 'w'),
        Sutra::new("l", 'R'),
        Sutra::new("YmNRn", 'm'),
        Sutra::new("JB", 'Y'),
        Sutra::new("GQD", 'z'),
        Sutra::new("jbgqd", 'S'),
        Sutra::new("KPCWTcwt", 'v'),
        Sutra::new("kp", 'y'),
        Sutra::new("Szs", 'r'),
        Sutra::new("h", 'l'),
    ]
}

fn create_sound_props() -> FxHashMap<Sound, Uccarana> {
    fn flatten_multi<T: Copy>(data: Vec<(SoundSet, T)>) -> FxHashMap<Sound, Vec<T>> {
        let mut mapping = FxHashMap::default();
        for (ks, v) in data {
            for k in ks.to_string().chars() {
                mapping.entry(k).or_insert_with(Vec::new).push(v);
            }
        }
        mapping
    }

    fn flatten<T: Copy>(data: Vec<(SoundSet, T)>) -> FxHashMap<Sound, T> {
        let mut mapping = FxHashMap::default();
        for (ks, v) in data {
            for k in ks.to_string().chars() {
                mapping.insert(k, v);
            }
        }
        mapping
    }

    let mut sthana = flatten_multi(vec![
        (s("a ku~ h H"), Sthana::Kantha),
        (s("i cu~ y S"), Sthana::Talu),
        (s("f wu~ r z"), Sthana::Murdha),
        (s("x tu~ l s"), Sthana::Danta),
        (s("u pu~"), Sthana::Oshtha),
        (s("e E"), Sthana::KanthaTalu),
        (s("o O"), Sthana::KanthaOshtha),
        (s("v"), Sthana::DantaOshtha),
    ]);
    for k in s("Yam M").to_string().chars() {
        sthana
            .entry(k)
            .or_insert_with(Vec::new)
            .push(Sthana::Nasika);
    }

    let ghosha = flatten(vec![
        (s("ac haS M"), Ghosha::Ghoshavat),
        (s("Kar H"), Ghosha::Aghosha),
    ]);
    let prana = flatten(vec![
        (s("ac yam jaS car M"), Prana::Alpaprana),
        (s("K G C J W Q T D P B h"), Prana::Mahaprana),
    ]);
    let prayatna = flatten(vec![
        (s("yaR Sar"), Prayatna::Ishat),
        (s("ac h"), Prayatna::Vivrta),
        (s("Yay"), Prayatna::Sprshta),
    ]);

    let mut res = FxHashMap::default();
    for k in s("al H M").to_string().chars() {
        let sthana = match sthana.get(&k) {
            Some(s) => s.clone(),
            None => Vec::new(),
        };

        res.insert(
            k,
            Uccarana {
                sthana,
                ghosha: *ghosha.get(&k).unwrap_or(&Ghosha::Aghosha),
                prana: *prana.get(&k).unwrap_or(&Prana::Alpaprana),
                prayatna: *prayatna.get(&k).unwrap_or(&Prayatna::Vivrta),
            },
        );
    }

    res
}

pub fn is_hrasva(c: Sound) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'f' | 'x')
}

pub fn is_dirgha(c: Sound) -> bool {
    matches!(c, 'A' | 'I' | 'U' | 'F' | 'X' | 'e' | 'E' | 'o' | 'O')
}

pub fn is_guna(c: Sound) -> bool {
    matches!(c, 'a' | 'e' | 'o')
}

pub fn is_ac(c: Sound) -> bool {
    lazy_static! {
        static ref AC: SoundSet = s("ac");
    }
    AC.contains_char(c)
}

pub fn is_hal(c: Sound) -> bool {
    HAL.contains_char(c)
}

pub fn is_samyogadi(text: &str) -> bool {
    let mut chars = text.chars();
    HAL.contains_opt(chars.next()) && HAL.contains_opt(chars.next())
}

pub fn is_samyoganta(text: &str) -> bool {
    let mut chars = text.chars().rev();
    HAL.contains_opt(chars.next()) && HAL.contains_opt(chars.next())
}

pub fn to_guna(s: Sound) -> Option<&'static str> {
    let res = match s {
        'i' | 'I' => "e",
        'u' | 'U' => "o",
        'f' | 'F' => "ar",
        'x' | 'X' => "al",
        _ => return None,
    };
    Some(res)
}

pub fn to_vrddhi(s: Sound) -> Option<&'static str> {
    let res = match s {
        'a' | 'A' => "A",
        'i' | 'I' => "E",
        'u' | 'U' => "O",
        'f' | 'F' => "Ar",
        'x' | 'X' => "Al",
        'e' | 'E' => "E",
        'o' | 'O' => "O",
        _ => return None,
    };
    Some(res)
}

// 1.1.48 UkAlojjhrasvadIrghaplutaH
pub fn to_hrasva(s: Sound) -> Option<Sound> {
    let res = match s {
        'a' | 'A' => 'a',
        'i' | 'I' => 'i',
        'u' | 'U' => 'u',
        'f' | 'F' => 'f',
        'x' | 'X' => 'x',
        'e' | 'E' => 'i',
        'o' | 'O' => 'u',
        _ => return None,
    };
    Some(res)
}

// 1.1.48 UkAlojjhrasvadIrghaplutaH
pub fn to_dirgha(s: Sound) -> Option<Sound> {
    let res = match s {
        'a' | 'A' => 'A',
        'i' | 'I' => 'I',
        'u' | 'U' => 'U',
        'f' | 'F' => 'F',
        'x' | 'X' => 'X',
        'e' => 'e',
        'E' => 'E',
        'o' => 'o',
        'O' => 'O',
        _ => return None,
    };
    Some(res)
}

fn pratyahara(s: &str) -> SoundSet {
    let first = s.as_bytes()[0] as char;

    let use_second_n = s.ends_with("R2");
    let it = if use_second_n {
        'R'
    } else {
        s.as_bytes()[s.len() - 1] as char
    };

    let mut started = false;
    let mut saw_first_n = false;
    let mut res = String::new();

    for sutra in SUTRAS.iter() {
        for sound in sutra.sounds.chars() {
            if first == sound {
                started = true;
            }
            if started {
                res.push(sound);

                // Add long vowels, which are not explictly included in the
                // Shiva Sutras.
                if is_hrasva(sound) {
                    res.push(to_dirgha(sound).expect("should be ac"));
                }
            }
        }

        if started && it == sutra.it {
            if use_second_n && !saw_first_n {
                saw_first_n = true;
            } else {
                break;
            }
        }
    }

    assert!(!res.is_empty(), "Could not parse pratyahara `{s}`");
    SoundSet::new(&res)
}

pub fn s(terms: &str) -> SoundSet {
    let mut ret = String::new();
    let ak = ["a", "A", "i", "I", "u", "U", "f", "F", "x", "X"];

    for term in terms.split_whitespace() {
        if term.ends_with("u~") || ak.contains(&term) {
            let first = term.chars().next().unwrap();
            ret += &savarna(first).to_string();
        } else if term.len() == 1 {
            ret += term;
        } else {
            ret += &pratyahara(term).to_string();
        }
    }

    SoundSet::new(&ret)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Sthana {
    Kantha,
    Talu,
    Murdha,
    Danta,
    Oshtha,
    Nasika,
    KanthaTalu,
    KanthaOshtha,
    DantaOshtha,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Ghosha {
    Ghoshavat,
    Aghosha,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Prana {
    Mahaprana,
    Alpaprana,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Prayatna {
    Vivrta,
    Ishat,
    Sprshta,
}

struct Uccarana {
    sthana: Vec<Sthana>,
    ghosha: Ghosha,
    prana: Prana,
    prayatna: Prayatna,
}

impl Uccarana {
    fn distance(&self, other: &Uccarana) -> i32 {
        let mut dist = 0;
        if self.ghosha != other.ghosha {
            dist += 1;
        };
        if self.prana != other.prana {
            dist += 1;
        }
        if self.prayatna != other.prayatna {
            dist += 1;
        }

        let mut sthana_dist = (self.sthana.len() + other.sthana.len()) as i32;
        for s in &self.sthana {
            if other.sthana.contains(s) {
                sthana_dist -= 2;
            }
        }

        dist + sthana_dist
    }
}
pub fn savarna(c: Sound) -> SoundSet {
    let sounds = match c {
        'a' | 'A' => "aA",
        'i' | 'I' => "iI",
        'u' | 'U' => "uU",
        'f' | 'F' | 'x' | 'X' => "fFxX",
        'k' | 'K' | 'g' | 'G' | 'N' => "kKgGN",
        'c' | 'C' | 'j' | 'J' | 'Y' => "cCjJY",
        'w' | 'W' | 'q' | 'Q' | 'R' => "wWqQR",
        't' | 'T' | 'd' | 'D' | 'n' => "tTdDn",
        'p' | 'P' | 'b' | 'B' | 'm' => "pPbBm",
        _ => "",
    };
    SoundSet::new(sounds)
}

pub fn map_sounds(xs: &str, ys: &str) -> SoundMap {
    let xs = s(xs);
    let ys = s(ys);

    let mut mapping = SoundMap::new();
    for x in xs.to_string().chars() {
        let x_props = SOUND_PROPS.get(&x).unwrap();

        // The best sound has the minimal distance.
        let best_y = ys
            .to_string()
            .chars()
            .min_by_key(|y| SOUND_PROPS.get(y).unwrap().distance(x_props))
            .unwrap();
        mapping.insert(x, best_y);
    }

    mapping
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s() {
        let tests = vec![
            ("ac", "aAiIuUfFxXeEoO"),
            ("iR", "iIuU"),
            ("iR2", "iIuUfFxXeEoOyrlvh"),
            ("yaR", "yrlv"),
            ("hal", "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh"),
            ("Yam", "NYRnm"),
            ("Sar", "Szs"),
            ("a", "aA"),
            ("e", "e"),
            ("ku~", "kKgGN"),
            ("cu~", "cCjJY"),
            ("i cu~", "iIcCjJY"),
            ("a ku~ h H", "aAHkKgGNh"),
        ];
        for (input, expected) in tests {
            let actual = s(input).to_string();
            assert_eq!(actual, expected, "input: `{input}`");
        }
    }

    #[test]
    fn test_map_sounds_jhal_jhash() {
        let actual = map_sounds("Jal", "jaS");
        let expected_vec = vec![
            ('J', 'j'),
            ('B', 'b'),
            ('G', 'g'),
            ('Q', 'q'),
            ('D', 'd'),
            ('j', 'j'),
            ('b', 'b'),
            ('g', 'g'),
            ('q', 'q'),
            ('d', 'd'),
            ('K', 'g'),
            ('P', 'b'),
            ('C', 'j'),
            ('W', 'q'),
            ('T', 'd'),
            ('c', 'j'),
            ('w', 'q'),
            ('t', 'd'),
            ('k', 'g'),
            ('p', 'b'),
            ('S', 'j'),
            ('z', 'q'),
            ('s', 'd'),
            ('h', 'g'),
        ];
        let mut expected = SoundMap::new();
        for (k, v) in expected_vec {
            expected.insert(k, v);
        }
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_sounds_kuh_cu() {
        let actual = map_sounds("ku~ h", "cu~");
        let expected_vec = vec![
            ('k', 'c'),
            ('K', 'C'),
            ('g', 'j'),
            ('G', 'J'),
            ('N', 'Y'),
            ('h', 'J'),
        ];
        let mut expected = SoundMap::new();
        for (k, v) in expected_vec {
            expected.insert(k, v);
        }
        assert_eq!(expected, actual);

        assert_eq!(expected, actual);
    }
}
