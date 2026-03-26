use crate::if_is_zeilen_angabe::str_as_generator_to_vec_i64;

pub(crate) fn parse_zeilenangabe_zu_bereichen(text: &str) -> Option<Vec<(usize, usize)>> {
    let mut bereiche = Vec::new();

    if let Some(zahlen) = str_as_generator_to_vec_i64(text) {
        for &zahl in &zahlen {
            if zahl >= 0 {
                bereiche.push((zahl as usize, zahl as usize));
            }
        }

        if !bereiche.is_empty() {
            bereiche.sort_by(|a, b| a.0.cmp(&b.0));
            return Some(bereiche);
        }
    }

    let teile: Vec<&str> = text.split(',').collect();

    if teile.len() == 1 {
        let teil = teile[0].trim();

        if teil.contains('-') {
            let bereichs_teile: Vec<&str> = teil.split('-').collect();
            if bereichs_teile.len() == 2 {
                if let (Ok(von), Ok(bis)) = (
                    bereichs_teile[0].trim().parse::<usize>(),
                    bereichs_teile[1].trim().parse::<usize>(),
                ) {
                    bereiche.push((von, bis));
                }
            }
        } else if let Ok(num) = teil.parse::<usize>() {
            bereiche.push((num, num));
        }
    } else {
        for teil in teile {
            let teil_trimmed = teil.trim();

            if teil_trimmed.is_empty() {
                continue;
            }

            if teil_trimmed.contains('-') {
                let bereichs_teile: Vec<&str> = teil_trimmed.split('-').collect();
                if bereichs_teile.len() == 2 {
                    if let (Ok(von), Ok(bis)) = (
                        bereichs_teile[0].trim().parse::<usize>(),
                        bereichs_teile[1].trim().parse::<usize>(),
                    ) {
                        bereiche.push((von, bis));
                        continue;
                    }
                }
            }

            if let Ok(num) = teil_trimmed.parse::<usize>() {
                bereiche.push((num, num));
            }
        }
    }

    if !bereiche.is_empty() {
        bereiche.sort_by(|a, b| a.0.cmp(&b.0));
        Some(bereiche)
    } else {
        None
    }
}
