use reedline::DefaultCompleter;

pub const RP_META_COMMANDS: &[&str] = &[
    "help",
    "hilfe",
    "befehle",
    "kurzbefehle",
    "q",
    ":q",
    "exit",
    "quit",
    "ende",
    "leeren",
    "clear",
    ":ui",
    ":preview",
    ":history",
    ":mode vi",
    ":mode emacs",
    "shell",
    "reta",
];

pub const RETA_MAINS: &[&str] = &[
    "-zeilen",
    "-spalten",
    "-kombination",
    "-ausgabe",
    "-debug",
    "-h",
    "-help",
];

pub const RETA_SIDE_PARAMETERS: &[&str] = &[
    "--vorhervonausschnitt=",
    "--vorhervonausschnittteiler",
    "--zaehlung=",
    "--zeit=",
    "--alles",
    "--typ=",
    "--vielfachevonzahlen=",
    "--primzahlen=",
    "--primzahlvielfache=",
    "--nachtraeglichneuabzaehlung=",
    "--nachtraeglichneuabzaehlungvielfache=",
    "--breite=",
    "--breiten=",
    "--art=",
    "--onetable",
    "--justtext",
    "--nocolor",
    "--keinenummerierung",
    "--keineueberschriften",
    "--spaltenreihenfolgeundnurdiese=",
    "--galaxie=",
    "--universum=",
    "--*=",
];

pub fn completion_vocabulary() -> Vec<String> {
    RP_META_COMMANDS
        .iter()
        .chain(RETA_MAINS.iter())
        .chain(RETA_SIDE_PARAMETERS.iter())
        .map(|s| (*s).to_string())
        .collect()
}

pub fn build_default_completer() -> Box<DefaultCompleter> {
    Box::new(DefaultCompleter::new_with_wordlen(
        completion_vocabulary(),
        2,
    ))
}

pub fn candidates_for_prefix(prefix: &str) -> Vec<String> {
    let lower = prefix.to_lowercase();
    completion_vocabulary()
        .into_iter()
        .filter(|candidate| candidate.to_lowercase().contains(&lower))
        .collect()
}
