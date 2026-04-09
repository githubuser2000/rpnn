use crate::doc_tools::markdown_reader;
use crate::runtime::I18nExact;
use indexmap::IndexMap;

pub const PYTHON_SOURCE__GENERATE4README_FALLBACK: &str = r#"Python source is expected at python_reference/libs/generate4readme.py.
This Rust fallback only avoids a build-time include failure when the file was not copied.
"#;

pub fn python_source_generate4readme() -> String {
    let candidate = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("python_reference")
        .join("libs")
        .join("generate4readme.py");
    match std::fs::read_to_string(candidate) {
        Ok(text) => text,
        Err(_) => PYTHON_SOURCE__GENERATE4README_FALLBACK.to_string(),
    }
}

const EN_HEADER: &str = r#"Main program is reta or reta.py.
More convenient is retaPrompt, which is still available with presets as rp and rpl.

User manual:
There are 4 main parameters.
**Important: the secondary parameters must be placed directly after the correct main parameter, otherwise they have no effect and no other main parameter may be placed in between!**
Main parameters start with a minus -.
Secondary parameters start with 2 minus --.

# main parameter

## -debug
    * has no secondary parameters, is only relevant and interesting for me as a programmer

## -lines

    * --all
"#;

const DE_HEADER: &str = r#"Hauptprogramm ist reta oder reta.py
Bequemer ist retaPrompt, was es mit Voreinstellungen noch als rp und rpl gibt.

Bedienungsanleitung:
Es gibt 4 Hauptparameter.
**Wichtig: die Nebenparameter müssen direkt hinter dem richtigen Hauptparamter stehen, sonst wirken sie
nicht und dazwischen darf kein anderer Hauptparameter stehen!**
Hauptparameter beginnen mit einem Minus -.
Nebenparameter beginnen mit 2 Minus --.

# Hauptparameter
Besser die Readme aus Markdown mit einem Markdown-Leseprogramm lesen!

## -debug
    *    hat keine Nebenparameter, ist nur für mich als Programmierer relevant und interesssant

## -zeilen

    * --alles
"#;

const EN_FOOTER: &str = r#"

## -output
    * --nocolor
    * --kind=
        * (only one allowed)
        * shell,html,csv,markdown,bbcode
    * --onetable
    * --columnorderandonlythese=
        * 3,5,1
        * i.e. from e.g. 5 columns the 3rd, then 5th and 1st should be displayed first and the others not!
    * --no_blank_contents
        * This makes that rows are not output, which contain only a minus or question mark or otherwise almost no information.
    * --noheadings
        * Headings are not output.
    * --no_numbering
        * Number of line and number of numbering several lines do not become displayed.
"#;

const DE_FOOTER: &str = r#"

## -ausgabe
    * --keinefarbe
    * --art=
        * (nur eine erlaubt)
        * shell,html,csv,markdown,bbcode
    * --einetabelle
    * --spaltenreihenfolgeundnurdiese=
        * 3,5,1
    * --keineleereninhalte
        * Zeilen werden nicht ausgegeben, wenn sie fast nur Minus oder Fragezeichen enthalten.
    * --keineueberschriften
        * Überschriften werden nicht ausgegeben.
    * --keinenummerierung
        * Zeilennummern und Nummerierungen werden nicht ausgegeben.
"#;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LanguageMode {
    German,
    English,
}

impl LanguageMode {
    fn from_argv(argv: &[String]) -> Self {
        if argv
            .iter()
            .any(|arg| arg == "-language=english" || arg == "-language=englisch")
        {
            Self::English
        } else {
            Self::German
        }
    }
}

pub fn main_like_python(argv: &[String]) -> i32 {
    let language = LanguageMode::from_argv(argv);

    if argv.iter().any(|arg| arg == "--render-retaprompt") {
        match markdown_reader::retaprompt_hilfe_text() {
            Ok(text) => {
                print!("{text}");
                return 0;
            }
            Err(err) => {
                eprintln!("retaprompt-readme konnte nicht gelesen werden: {err}");
                return 1;
            }
        }
    }
    if argv.iter().any(|arg| arg == "--show-python-source") {
        print!("{}", python_source_generate4readme());
        return 0;
    }

    if argv.iter().any(|arg| arg == "--render-reta") {
        match markdown_reader::reta_hilfe_text() {
            Ok(text) => {
                print!("{text}");
                return 0;
            }
            Err(err) => {
                eprintln!("reta-readme konnte nicht gelesen werden: {err}");
                return 1;
            }
        }
    }

    print_header(language);
    print_parameter_inventory(language);
    0
}

fn print_header(language: LanguageMode) {
    match language {
        LanguageMode::German => println!("{DE_HEADER}"),
        LanguageMode::English => println!("{EN_HEADER}"),
    }
}

fn print_parameter_inventory(language: LanguageMode) {
    let i18n = I18nExact::from_python_evaluated_shapes();
    let grouped = group_parameter_names_like_python(&i18n);

    for (main_name, secondary_names) in grouped {
        println!("## -{main_name}");
        for (parameter_name, data_names) in secondary_names {
            let has_entries = !data_names.is_empty();
            println!(
                "    * --{}{} ",
                parameter_name,
                if has_entries { "=" } else { "" }
            );
            if has_entries {
                println!("        * {}", data_names.join(","));
            }
        }
    }

    match language {
        LanguageMode::English => {
            println!();
            println!();
            println!("## -combination");
            println!("    * --galaxy=");
            println!("        * {}", flatten_combination_map(&i18n.kombiParaNdataMatrix).join(","));
            println!("    * --universe=");
            println!("        * {}", flatten_combination_map(&i18n.kombiParaNdataMatrix2).join(","));
        }
        LanguageMode::German => {
            println!();
            println!();
            println!("## -kombination");
            println!("    * --galaxie=");
            println!("        * {}", flatten_combination_map(&i18n.kombiParaNdataMatrix).join(","));
            println!("    * --universum=");
            println!("        * {}", flatten_combination_map(&i18n.kombiParaNdataMatrix2).join(","));
            println!("{DE_FOOTER}");
        }
    }

    if matches!(language, LanguageMode::English) {
        println!("{EN_FOOTER}");
    }
}

fn group_parameter_names_like_python(
    i18n: &I18nExact,
) -> IndexMap<String, Vec<(String, Vec<String>)>> {
    let mut grouped: IndexMap<String, Vec<(String, Vec<String>)>> = IndexMap::new();

    for entry in &i18n.paraNdataMatrix {
        let position = entry
            .parameterMainNames
            .get(1)
            .or_else(|| entry.parameterMainNames.first())
            .cloned()
            .unwrap_or_default();

        let thing = entry
            .parameterNames
            .get(1)
            .or_else(|| entry.parameterNames.first())
            .cloned()
            .unwrap_or_default();

        let data_names = entry
            .datas
            .iter()
            .flat_map(|values| values.iter())
            .filter_map(|value| match value {
                crate::runtime::PyAtom::Str(text) => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();

        grouped.entry(position).or_default().push((thing, data_names));
    }

    grouped
}

fn flatten_combination_map(map: &IndexMap<i64, Vec<String>>) -> Vec<String> {
    map.values().flat_map(|values| values.iter().cloned()).collect()
}
