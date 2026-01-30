// retaAusgabe-utils.rs
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};

// Vollständig UTF8-taugliche word_wrap Funktion mit Unicode-Breiten
pub fn word_wrap(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    
    let mut result = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    
    if words.is_empty() {
        return vec!["".to_string()];
    }
    
    let mut current_line = String::new();
    let mut current_width = 0;
    
    for word in words {
        // Berechne Unicode-Breite des Wortes
        let word_width = UnicodeWidthStr::width(word);
        
        // Berechne benötigte Breite (Wort + ggf. Leerzeichen)
        let needed_width = if current_line.is_empty() {
            word_width
        } else {
            word_width + 1 // +1 für Leerzeichen
        };
        
        // Wenn das Wort in die aktuelle Zeile passt
        if current_width + needed_width <= width {
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += 1;
            }
            current_line.push_str(word);
            current_width += word_width;
        } else {
            // Füge aktuelle Zeile zum Ergebnis hinzu
            if !current_line.is_empty() {
                result.push(current_line);
            }
            
            // Starte neue Zeile mit dem aktuellen Wort
            current_line = word.to_string();
            current_width = word_width;
            
            // Wenn ein einzelnes Wort breiter als width ist, muss es geteilt werden
            if word_width > width {
                // Teile das Wort auf Zeichenebene mit Unicode-Breiten
                let mut char_accumulator = String::new();
                let mut char_width = 0;
                
                for ch in word.chars() {
                    let ch_width = UnicodeWidthChar::width(ch).unwrap_or(1);
                    
                    if char_width + ch_width > width {
                        if !char_accumulator.is_empty() {
                            result.push(char_accumulator.clone());
                        }
                        char_accumulator.clear();
                        char_width = 0;
                    }
                    
                    char_accumulator.push(ch);
                    char_width += ch_width;
                }
                
                if !char_accumulator.is_empty() {
                    current_line = char_accumulator;
                    current_width = char_width;
                } else {
                    current_line.clear();
                    current_width = 0;
                }
            }
        }
    }
    
    // Letzte Zeile nicht vergessen
    if !current_line.is_empty() {
        result.push(current_line);
    }
    
    if result.is_empty() {
        result.push("".to_string());
    }
    
    result
}

// Hilfsfunktion für Padding mit Unicode-Unterstützung
pub fn unicode_pad(text: &str, width: usize, align_left: bool) -> String {
    let text_width = UnicodeWidthStr::width(text);
    
    if text_width >= width {
        return text.to_string();
    }
    
    let padding = width - text_width;
    
    if align_left {
        format!("{}{}", text, " ".repeat(padding))
    } else {
        format!("{}{}", " ".repeat(padding), text)
    }
}
