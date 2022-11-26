use lazy_static::lazy_static;
use std::collections::HashMap;

type Sound = char;

lazy_static! {
    static ref SUTRAS: Vec<Sutra> = create_shiva_sutras();
    static ref SOUND_PROPS: HashMap<Sound, Uccarana> = create_sound_props();
}

struct Sutra {
    sounds: String,
    it: char,
}

impl Sutra {
    fn new(sounds: &str, it: char) -> Self {
        Sutra { sounds: sounds.to_string(), it }
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

fn create_sound_props() -> HashMap<Sound, Uccarana> {
    fn flatten_multi<T: Copy>(data: Vec<(SoundSet, T)>) -> HashMap<Sound, Vec<T>> {
        let mut mapping = HashMap::new();
        for (ks, v) in data {
            for k in ks.iter() {
                mapping.entry(k).or_insert_with(Vec::new).push(v);
            }
        }
        mapping
    }

    fn flatten<T: Copy>(data: Vec<(SoundSet, T)>) -> HashMap<Sound, T> {
        let mut mapping = HashMap::new();
        for (ks, v) in data {
            for k in ks.iter() {
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
    for k in s("Yam M").iter() {
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

    let mut res = HashMap::new();
    for k in s("al H M").iter() {
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

pub struct SoundSet {
    string: String,
}

impl SoundSet {
    pub fn new(string: &str) -> Self {
        SoundSet {
            string: string.to_string(),
        }
    }

    pub fn from_vec(sounds: Vec<Sound>) -> Self {
        let string = sounds.iter().collect();
        SoundSet {
            string,
        }
    }

    pub fn contains(&self, s: &str) -> bool {
        self.string.contains(s)
    }

    pub fn contains_char(&self, c: Sound) -> bool {
        self.string.contains(c)
    }

    pub fn contains_opt(&self, o: Option<char>) -> bool {
        if let Some(c) = o {
            self.contains_char(c)
        } else {
            false
        }
    }

    pub fn into_string(self) -> String {
        self.string
    }

    pub fn iter(&self) -> std::str::Chars {
        self.string.chars()
    }
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
    let mut res = vec![];

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
    SoundSet::from_vec(res)
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

pub fn s(terms: &str) -> SoundSet {
    let mut ret = String::new();
    let ak = ["a", "A", "i", "I", "u", "U", "f", "F", "x", "X"];

    for term in terms.split_whitespace() {
        if term.ends_with("u~") || ak.contains(&term) {
            let first = term.chars().next().unwrap();
            ret += &savarna(first).string;
        } else if term.len() == 1 {
            ret += term;
        } else {
            ret += &pratyahara(term).string;
        }
    }

    SoundSet::new(&ret)
}

pub fn map_sounds(xs: &str, ys: &str) -> HashMap<Sound, Sound> {
    let xs = s(xs);
    let ys = s(ys);

    let mut mapping = HashMap::new();
    for x in xs.iter() {
        let x_props = SOUND_PROPS.get(&x).unwrap();

        // The best sound has the minimal distance.
        let best_y = ys
            .iter()
            .min_by_key(|y| SOUND_PROPS.get(y).unwrap().distance(x_props))
            .unwrap();

        let d = x_props.distance(SOUND_PROPS.get(&best_y).unwrap());
        println!("The closest sound to {x} is {best_y} with distance {d}.");
        mapping.insert(x, best_y);
    }

    mapping
}

pub fn is_hrasva(c: Sound) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'f' | 'x')
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

pub fn to_guna(s: Sound) -> &'static str {
    match s {
        'i' | 'I' => "e",
        'u' | 'U' => "o",
        'f' | 'F' => "ar",
        'x' | 'X' => "al",
        _ => panic!("Invalid guna sound {s}"),
    }
}

pub fn to_vrddhi(s: Sound) -> &'static str {
    match s {
        'a' | 'A' => "A",
        'i' | 'I' => "E",
        'u' | 'U' => "O",
        'f' | 'F' => "Ar",
        'x' | 'X' => "Al",
        'e' | 'E' => "E",
        'o' | 'O' => "O",
        _ => panic!("Invalid vrddhi sound {s}"),
    }
}

// 1.1.48 UkAlojjhrasvadIrghaplutaH
fn to_hrasva(s: Sound) -> Option<Sound> {
    let res = match s {
        'a' | 'A' => 'a',
        'i' | 'I' => 'i',
        'u' | 'U' => 'u',
        'f' | 'F' => 'f',
        'x' | 'X' => 'x',
        'e' | 'E' => 'i',
        'o' | 'O' => 'u',
        _ => panic!("Invalid hrasva sound {s}"),
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
        _ => return None
    };
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_s() {
        let tests = vec![
            ("ac", "aAiIuUfFxXeEoO"),
            ("iR", "iIuU"),
            ("iR2", "iIuUfFxXeEoOhyvrl"),
            ("yaR", "yrvl"),
            ("hal", "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh"),
            ("Yam", "YmNRn"),
            ("Sar", "Szs"),
            ("a", "aA"),
            ("e", "e"),
            ("ku~", "kKgGN"),
            ("cu~", "cCjJY"),
            ("i cu~", "iIcCjJY"),
            ("a ku~ h H", "aAkKgGNhH"),
        ];
        for (input, expected) in tests {
            let expected: HashSet<Sound> = expected.chars().collect();
            let actual: HashSet<Sound> = s(input).string.chars().collect();
            assert_eq!(actual, expected, "input: `{input}`");
        }
    }

    #[test]
    fn test_map_sounds_jhal_jhash() {
        let actual = map_sounds("Jal", "jaS");
        let expected: HashMap<Sound, Sound> = vec![
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
        ]
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_map_sounds_kuh_cu() {
        let actual = map_sounds("ku~ h", "cu~");
        let expected: HashMap<Sound, Sound> = vec![
            ('k', 'c'),
            ('K', 'C'),
            ('g', 'j'),
            ('G', 'J'),
            ('N', 'Y'),
            ('h', 'J'),
        ]
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();

        assert_eq!(expected, actual);
    }
}
