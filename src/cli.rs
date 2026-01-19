#[derive(Debug, Clone, Copy)]
pub struct TextBereich {
    pub von_zeile: usize,
    pub bis_zeile: usize,
    pub von_spalte: usize,
    pub bis_spalte: usize,
}


pub fn parse_cli_args(args: &[String]) -> (Vec<usize>, Vec<String>) {
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
        let mut bereich = TextBereich {
            von_zeile: 0,
            bis_zeile: 0,
            von_spalte: 0,
            bis_spalte: 0,
        };
        match arg.as_str() {
            "--zeilevon" => {
                println!("Startzeile ausgewählt.");
            }
            "--zeilebis" => {
                println!("Endzeile ausgewählt.");
            }
            _ => {
                println!("Anderes oder unbekanntes Argument: {}", arg);
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
        dash_count_before = dash_count;
    }

    (minuses, params)
}
