use vidyut_prakriya::args::{Dhatu, Lakara, Prayoga, Purusha, Vacana};
use vidyut_prakriya::Ashtadhyayi;

const PURUSHA_VACANA: &[(Purusha, Vacana)] = &[
    (Purusha::Prathama, Vacana::Eka),
    (Purusha::Prathama, Vacana::Dvi),
    (Purusha::Prathama, Vacana::Bahu),
    (Purusha::Madhyama, Vacana::Eka),
    (Purusha::Madhyama, Vacana::Dvi),
    (Purusha::Madhyama, Vacana::Bahu),
    (Purusha::Uttama, Vacana::Eka),
    (Purusha::Uttama, Vacana::Dvi),
    (Purusha::Uttama, Vacana::Bahu),
];

/// Tests generating all of the basic forms of BU.
#[test]
fn test_bhu() {
    let lat = vec![
        "Bavati", "BavataH", "Bavanti", "Bavasi", "BavaTaH", "BavaTa", "BavAmi", "BavAvaH",
        "BavAmaH",
    ];
    let lit = [
        "baBUva",
        "baBUvatuH",
        "baBUvuH",
        "baBUviTa",
        "baBUvaTuH",
        "baBUva",
        // TODO: only one from `Nal uttamo vA` ?
        "baBUva/baBUva",
        "baBUviva",
        "baBUvima",
    ];
    let lut = [
        "BavitA",
        "BavitArO",
        "BavitAraH",
        "BavitAsi",
        "BavitAsTaH",
        "BavitAsTa",
        "BavitAsmi",
        "BavitAsvaH",
        "BavitAsmaH",
    ];
    let lrt = [
        "Bavizyati",
        "BavizyataH",
        "Bavizyanti",
        "Bavizyasi",
        "BavizyaTaH",
        "BavizyaTa",
        "BavizyAmi",
        "BavizyAvaH",
        "BavizyAmaH",
    ];
    let lot = [
        "Bavatu/BavatAd/BavatAt",
        "BavatAm",
        "Bavantu",
        "Bava/BavatAd/BavatAt",
        "Bavatam",
        "Bavata",
        "BavAni",
        "BavAva",
        "BavAma",
    ];
    let lan = [
        "aBavad/aBavat",
        "aBavatAm",
        "aBavan",
        "aBavaH",
        "aBavatam",
        "aBavata",
        "aBavam",
        "aBavAva",
        "aBavAma",
    ];
    let ashir_lin = [
        "BUyAd/BUyAt",
        "BUyAstAm",
        "BUyAsuH",
        "BUyAH",
        "BUyAstam",
        "BUyAsta",
        "BUyAsam",
        "BUyAsva",
        "BUyAsma",
    ];
    let vidhi_lin = [
        "Baved/Bavet",
        "BavetAm",
        "BaveyuH",
        "BaveH",
        "Bavetam",
        "Baveta",
        "Baveyam",
        "Baveva",
        "Bavema",
    ];
    let lun = [
        "aBUd/aBUt",
        "aBUtAm",
        "aBUvan",
        "aBUH",
        "aBUtam",
        "aBUta",
        "aBUvam",
        "aBUva",
        "aBUma",
    ];
    let lrn = [
        "aBavizyad/aBavizyat",
        "aBavizyatAm",
        "aBavizyan",
        "aBavizyaH",
        "aBavizyatam",
        "aBavizyata",
        "aBavizyam",
        "aBavizyAva",
        "aBavizyAma",
    ];

    fn test_la(la: Lakara, expected_static: &[&'static str]) {
        let mut actual = Vec::new();
        let mut expected = Vec::new();
        for e in expected_static {
            expected.extend(e.split('/'));
        }

        let a = Ashtadhyayi::new();

        let dhatu = Dhatu::new("BU", 1, None);
        for (purusha, vacana) in PURUSHA_VACANA {
            let prakriyas = a.derive_tinantas(&dhatu, la, Prayoga::Kartari, *purusha, *vacana);
            actual.extend(prakriyas.iter().map(|t| t.text()));
        }

        // Expect full equality.
        actual.sort();
        expected.sort();
        assert_eq!(expected, actual);
    }

    test_la(Lakara::Lat, &lat);
    test_la(Lakara::Lit, &lit);
    test_la(Lakara::Lut, &lut);
    test_la(Lakara::Lrt, &lrt);
    test_la(Lakara::Lot, &lot);
    test_la(Lakara::Lan, &lan);
    test_la(Lakara::AshirLin, &ashir_lin);
    test_la(Lakara::VidhiLin, &vidhi_lin);
    test_la(Lakara::Lun, &lun);
    test_la(Lakara::Lrn, &lrn);
}

/// Tests generating various tinantas in lat-prathama-ekavacana.
#[test]
fn test_lat() {
    let tests = [
        // BU-Adi
        ("BU", 1, "Bavati"),
        ("RI\\Y", 1, "nayati,nayate"),
        ("zaha~\\", 1, "sahate"),
        // ad-Adi
        ("a\\da~", 2, "atti"),
        ("dvi\\za~^", 2, "dvezwi,dvizwe"),
        // juhoti-Adi
        ("hu\\", 3, "juhoti"),
        ("f\\", 3, "iyarti"),
        // div-Adi
        ("divu~", 4, "dIvyati"),
        ("YimidA~", 4, "medyati"),
        // su-Adi
        ("zu\\Y", 5, "sunoti,sunute"),
        // tud-Adi
        ("tu\\da~^", 6, "tudati,tudate"),
        // rudh-Adi
        ("ru\\Di~^r", 7, "ruRadDi,rundDe,runDe"),
        // tan-Adi
        ("tanu~^", 8, "tanoti,tanute"),
        ("qukf\\Y", 8, "karoti,kurute"),
        // cur-Adi
        ("qukrI\\Y", 9, "krIRAti,krIRIte"),
        // cur-Adi
        ("cura~", 10, "corayati,corayate"),
        ("Deka", 10, "Dekayati,Dekayate"),
    ];

    for (dhatu, gana, padas) in tests {
        let a = Ashtadhyayi::new();
        let prakriyas = a.derive_tinantas(
            &Dhatu::new(dhatu, gana, None),
            Lakara::Lat,
            Prayoga::Kartari,
            Purusha::Prathama,
            Vacana::Eka,
        );
        let mut actual: Vec<_> = prakriyas.iter().map(|t| t.text()).collect();
        let mut padas: Vec<_> = padas.split(',').collect();

        // Expect full equality.
        actual.sort();
        padas.sort();
        assert_eq!(actual, padas);
    }
}
