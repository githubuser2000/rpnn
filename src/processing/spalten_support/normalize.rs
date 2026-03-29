pub fn normalize_category_key(s: &str) -> String {
    s.trim().to_string()
}

pub fn is_primzahlkreuz_pro_contra_request(ober: &str, unter: &str) -> bool {
    let ober = normalize_category_key(ober);
    let unter = normalize_category_key(unter);

    matches!(ober.as_str(), "Bedeutung" | "bedeutung" | "Pro_Contra" | "ProContra" | "procontra" | "Universum" | "universum")
        && matches!(unter.as_str(), "Primzahlkreuzprocontra" | "primzahlkreuzprocontra" | "Primzahlkreuz" | "primzahlkreuz")
}
