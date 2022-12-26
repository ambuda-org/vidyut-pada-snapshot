use compact_str::CompactString;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

fn create_ktva(dhatu: &str, gana: u8) -> Vec<CompactString> {
    let a = Ashtadhyayi::new();
    let dhatu = Dhatu::new(dhatu, gana);
    let args = KrdantaArgs::builder().krt(Krt::ktvA).build().unwrap();

    let prakriyas = a.derive_krdantas(&dhatu, &args);
    prakriyas.iter().map(|p| p.text()).collect()
}

// todo: 7.4.40+ (ti kiti)
#[test]
fn test_ktva() {
    let cases = vec![
        // Basic
        ("BU", 1, vec!["BUtvA"]),
        ("eDa~\\", 1, vec!["eDitvA"]),
        ("qukf\\Y", 8, vec!["kftvA"]),
        ("cura~", 10, vec!["corayitvA"]),
        // Exceptions
        ("va\\ha~^", 1, vec!["UQvA"]),
        // ("zWA\\", 1, vec!["sTitvA"]),
        // ("a\\da~", 2, vec!["jagDvA"]),
        // Samprasarana
        ("va\\ca~", 2, vec!["uktvA"]),
        // 1.2.7
        ("mfqa~", 9, vec!["mfqitvA"]),
        ("mfda~", 9, vec!["mfditvA"]),
        ("guDa~", 9, vec!["guDitvA"]),
        ("kuza~", 9, vec!["kuzitvA"]),
        ("kliSU~", 9, vec!["kliSitvA", "klizwvA"]),
        ("vada~", 1, vec!["uditvA"]),
        ("va\\sa~", 1, vec!["uzitvA"]),
    ];

    for (dhatu, gana, expected) in cases {
        let mut expected = expected.to_vec();
        let mut actual = create_ktva(dhatu, gana);
        expected.sort();
        actual.sort();

        assert_eq!(actual, expected);
    }
}
