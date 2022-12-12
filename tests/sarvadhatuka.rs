use vidyut_gen::arguments::{La, Prayoga, Purusha, Vacana};
use vidyut_gen::ashtadhyayi as A;

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
        let code = format!("{gana}.0001");
        let prakriyas = A::derive_tinantas(
            dhatu,
            &code,
            La::Lat,
            Prayoga::Kartari,
            Purusha::Prathama,
            Vacana::Eka,
            false,
        );
        let mut actual: Vec<_> = prakriyas.iter().map(|t| t.text()).collect();
        let mut padas: Vec<_> = padas.split(',').collect();

        // Expect full equality.
        actual.sort();
        padas.sort();
        assert_eq!(actual, padas);
    }
}
