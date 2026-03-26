// Hilfsfunktion: strAsGeneratorToListOfNumStrs (vereinfacht für Rust)
pub fn str_as_generator_to_list_of_num_strs(text: &str) -> Option<Vec<String>> {
    if text.is_empty() {
        return None;
    }
    
    let trimmed = text.trim();
    
    // Prüfe auf (a,b,c) oder [a,b,c] oder {a,b,c} Format
    if (trimmed.starts_with('(') && trimmed.ends_with(')')) ||
       (trimmed.starts_with('[') && trimmed.ends_with(']')) ||
       (trimmed.starts_with('{') && trimmed.ends_with('}')) {
        
        let inner = &trimmed[1..trimmed.len()-1];
        let numbers: Result<Vec<i32>, _> = inner.split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect();
            
        match numbers {
            Ok(nums) => {
                let strings: Vec<String> = nums.iter()
                    .map(|n| n.to_string())
                    .collect();
                Some(strings)
            }
            Err(_) => None,
        }
    } else {
        None
    }
}

// Alternative Implementierung mit besserer Fehlerbehandlung
pub fn str_as_generator_to_list_of_num_strs_alt(text: &str) -> Option<Vec<i32>> {
    let trimmed = text.trim();
    
    // Konvertiere (a,b,c) zu [a,b,c]
    let processed = if trimmed.starts_with('(') && trimmed.ends_with(')') {
        format!("[{}]", &trimmed[1..trimmed.len()-1])
    } else {
        trimmed.to_string()
    };
    
    // Prüfe auf Array/Set-Format
    if (processed.starts_with('[') && processed.ends_with(']')) ||
       (processed.starts_with('{') && processed.ends_with('}')) {
        
        let inner = &processed[1..processed.len()-1];
        if inner.trim().is_empty() {
            return Some(Vec::new()); // Leere Menge/Liste
        }
        
        let numbers: Result<Vec<i32>, _> = inner.split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect();
            
        numbers.ok()
    } else {
        None
    }
}
