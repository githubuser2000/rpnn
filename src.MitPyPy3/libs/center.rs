//
// getTextWrapThings – vollständige Spiegelung
//
pub fn getTextWrapThings(max_len: Option<usize>) -> (Option<Html2Text>, Option<Pyphen>) {
    if !env::consts::OS.contains("brython") {
        let width = max_len.unwrap_or(80);

        let html2text = Html2Text::new(width);
        let pyphen = Pyphen::new("de_DE");

        return (Some(html2text), Some(pyphen));
    }
    (None, None)
}

//
// Platzhalter-Strukturen (werden später exakt gefüllt)
//
pub struct Html2Text {
    width: usize,
}
impl Html2Text {
    pub fn new(width: usize) -> Self {
        Self { width }
    }
}

pub struct Pyphen {
    lang: String,
}
impl Pyphen {
    pub fn new(lang: &str) -> Self {
        Self {
            lang: lang.to_string(),
        }
    }
}

//
// shellRowsAmount initialisieren
//
pub fn initShellRowsAmount() {
    unsafe {
        if SHELL_ROWS_AMOUNT == 0 {
            SHELL_ROWS_AMOUNT = 40;
        }
    }
}

//
// outputInfo
//
pub fn outputInfo(text: &str) {
    unsafe {
        if INFO_LOG {
            eprintln!("{}", text);
        }
    }
}

//
// output
//
pub fn output(text: &str) {
    unsafe {
        if OUTPUT {
            println!("{}", text);
        }
    }
}

//
// outputRaw
//
pub fn outputRaw(text: &str) {
    unsafe {
        if OUTPUT {
            print!("{}", text);
        }
    }
}

//
// setInfo
//
pub fn setInfo(v: bool) {
    unsafe {
        INFO_LOG = v;
    }
}

//
// setOutput
//
pub fn setOutput(v: bool) {
    unsafe {
        OUTPUT = v;
    }
}

//
// strAsGeneratorToListOfNumStrs
//
pub fn strAsGeneratorToListOfNumStrs(s: &str) -> Option<Vec<String>> {
    if !s.starts_with('[') || !s.ends_with(']') {
        return None;
    }
    let inner = &s[1..s.len() - 1];
    if inner.is_empty() {
        return Some(vec![]);
    }

    let parts: Vec<String> = inner
        .split(',')
        .map(|x| x.trim().to_string())
        .collect();

    Some(parts)
}

//
// generatorToSortedUniqueIntVec
//
pub fn generatorToSortedUniqueIntVec(gen: &[String]) -> Vec<i64> {
    let mut set = HashSet::new();
    for g in gen {
        if let Ok(v) = g.parse::<i64>() {
            set.insert(v);
        }
    }
    let mut vec: Vec<i64> = set.into_iter().collect();
    vec.sort_unstable();
    vec
}

//
// split_kommata_klammern_sicher
//
pub fn split_kommata_klammern_sicher(text: &str) -> Vec<String> {
    kpattern()
        .split(text)
        .map(|s| s.to_string())
        .collect()
}

//
// parseZeilenAngabe
//
pub fn parseZeilenAngabe(text: &str) -> Vec<i64> {
    let parts = split_kommata_klammern_sicher(text);
    let mut result: Vec<i64> = Vec::new();

    for p in parts {
        if let Some(gen) = strAsGeneratorToListOfNumStrs(&p) {
            result.extend(generatorToSortedUniqueIntVec(&gen));
        } else if let Ok(v) = p.parse::<i64>() {
            result.push(v);
        }
    }

    result
}

//
// ensurePP (entspricht Python Once-Init)
//
pub fn ensurePP() {
    PP.get_or_init(|| ());
}

//
// initMultiplikationen
//
pub fn initMultiplikationen() {
    MULTIPLIKATIONEN.get_or_init(|| {
        i18n::Multiplikationen
            .iter()
            .map(|s| s.to_string())
            .collect()
    });
}

//
// istMultiplikation
//
pub fn istMultiplikation(s: &str) -> bool {
    if let Some(m) = MULTIPLIKATIONEN.get() {
        m.iter().any(|x| x == s)
    } else {
        false
    }
}

//
// getShellRowsAmount
//
pub fn getShellRowsAmount() -> i32 {
    unsafe { SHELL_ROWS_AMOUNT }
}
