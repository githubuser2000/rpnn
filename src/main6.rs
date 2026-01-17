use std::fs::File;                                                          use std::io::{self, BufRead};                                               use std::path::Path;
use std::env;
#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() {
    let hauptParameter = vec!["zeilen1", "zeilen2", "spalten","ausgabe"];
    let zeilenParameter  = vec!["von", "bis", "ohnevon", "ohnebis"];
    let spaltenParameter = vec!["spaltennummer", "universum", "cluster", "galaxie", "kontinuum"];
    let ausgabeParameter = vec!["nichtsleeres", "unfarbig"];

    fn text(s: &str) -> Element {
        Element::Text(String::from(s))
    }

    fn list(elements: Vec<Element>) -> Element {
        Element::Liste(elements)
    }

    // Jetzt viel lesbarer
    let meine_liste = list(vec![
    list(vec![
        text("zeilen"),
            list(vec![
                text("nummer")
            ]),
        ]),
        list(vec![
           text("spalten"),
          list(vec![
                text("nummer")
            ]),
        ]),
    ]);
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);
    let args: Vec<String> = env::args().collect();
    //deep_match(&args, &meine_liste, 0);
    //deep_match_proper(&args, &meine_liste);
    //parse_cli_args(args)
    let (dashes, params) = parse_cli_args(&args);
    // Oder: &args[..]
    // let (dashes, params) = parse_cli_args(&args[..]);
    let pfad = "/data/data/com.termux/files/home/Eigene-Dateien/rpnn/csv/religion.csv";
                                                                                match lese_csv_in_matrix(pfad) {
        Ok(matrix) => {                                                                 println!("csv Datei erfolgreich eingelesen:");                                  //print_recursive(&matrix, 0);
        }                                                                           Err(e) => eprintln!("Fehler beim Lesen der Datei: {}", e),
    }

}

fn print_recursive(el: &Element, tiefe: usize) {
    let einrückung = "  ".repeat(tiefe);
    match el {
        Element::Text(t) => {
            println!("{}- {}", einrückung, t);
        }
        Element::Liste(l) => {
            println!("{}[Liste]:", einrückung);
            for kind in l {
                print_recursive(kind, tiefe + 1);
            }
        }
    }
}
fn parse_cli_args(args: &[String]) -> (Vec<usize>, Vec<String>) {
    let mut minuses = Vec::with_capacity(args.len());
    let mut params = Vec::with_capacity(args.len());
    let mut params2: Vec<String>;
    let mut paramsPerParam: Vec<Vec<String>> = vec![vec![String::new()]];
    let mut dash_count_before = 0;
    
    for (i, arg) in args.iter().enumerate() {
        let mut dash_count = 0;
        
        // Zähle aufeinanderfolgende Minuszeichen am Anfang
        for c in arg.chars() {
            if c == '-' {
                dash_count += 1;
            } else {
                break;
            }
        }
        if let Some(letztes) = paramsPerParam.last_mut() {
            println!("ever");
            if dash_count > dash_count_before {
                letztes.push(arg.clone());
                println!("Argument dazu {}", arg);
            }
            else {
                params2 = Vec::new();
                params2.push(arg.clone());
                letztes.push(arg.clone());
                paramsPerParam.push(params2);
                println!("Argument neu {}", arg);
            }
        }
        
        // Extrahiere Parameter ohne Minuszeichen
        let param = if dash_count > 0 {
            arg.chars().skip(dash_count).collect()
        } else {
            arg.clone()
        };
        
        // LÖSUNG: Zuerst drucken, dann moven
        println!("Argument {}: '{}' → {} Minuszeichen → '{}'", 
                i + 1, arg, dash_count, param);
        
        minuses.push(dash_count);
        params.push(param);  
        dash_count_before=dash_count;
    }
    
    (minuses, params)
}

fn lese_csv_in_matrix(dateipfad: &str) -> io::Result<Element> {
    let datei = File::open(dateipfad)?;
    let reader = io::BufReader::new(datei);
    let mut haupt_liste = Vec::new();                                       
    for zeile_result in reader.lines() {                                            let zeile = zeile_result?;                                                  // Splitte die Zeile am Semikolon
        let spalten: Vec<Element> = zeile                                               .split(';')
            .map(|s| Element::Text(s.trim().to_string()))                               .collect();                                                     
        // Füge die Zeile als Liste zur Hauptliste hinzu
        haupt_liste.push(Element::Liste(spalten));
    }

    Ok(Element::Liste(haupt_liste))
}
