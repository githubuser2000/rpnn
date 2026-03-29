use std::collections::HashSet;

// Hilfsfunktion: Simuliert BereichToNumbers2 für den Test
// In der echten Implementierung würdest du die richtige Rust-Version verwenden
pub fn simulate_bereich_to_numbers2(input: &str) -> HashSet<i64> {
    let mut result = HashSet::new();
    
    // Einfache Simulation: nimmt durch Komma getrennte Zahlen
    for part in input.split(',') {
        if let Ok(num) = part.trim().parse::<i64>() {
            if num > 0 {
                result.insert(num);
            }
        }
    }
    
    result
}

// Erweiterte Bereichsverarbeitung
pub fn parse_number_range(input: &str) -> HashSet<i64> {
    let mut numbers = HashSet::new();
    
    for part in input.split(',') {
        let trimmed = part.trim();
        
        // Einzelne Zahl
        if let Ok(num) = trimmed.parse::<i64>() {
            if num > 0 {
                numbers.insert(num);
            }
            continue;
        }
        
        // Versuche Bereichsnotation (z.B. "1-5")
        if let Some(range) = try_parse_range(trimmed) {
            numbers.extend(range);
        }
    }
    
    numbers
}

// Versucht einen Bereich wie "1-5" zu parsen
fn try_parse_range(input: &str) -> Option<Vec<i64>> {
    if let Some(pos) = input.find('-') {
        let start_str = &input[..pos];
        let end_str = &input[pos+1..];
        
        if let (Ok(start), Ok(end)) = (start_str.parse::<i64>(), end_str.parse::<i64>()) {
            if start > 0 && end >= start {
                return Some((start..=end).collect());
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simulate_bereich_to_numbers2() {
        let result = simulate_bereich_to_numbers2("12,8,12,5");
        assert_eq!(result.len(), 3); // Duplikate werden entfernt
        assert!(result.contains(&12));
        assert!(result.contains(&8));
        assert!(result.contains(&5));
    }
    
    #[test]
    fn test_parse_number_range() {
        let result = parse_number_range("1-5,8,10");
        assert_eq!(result.len(), 7); // 1,2,3,4,5,8,10
        assert!(result.contains(&1));
        assert!(result.contains(&5));
        assert!(result.contains(&8));
        assert!(result.contains(&10));
    }
    
    #[test]
    fn test_try_parse_range() {
        assert_eq!(try_parse_range("1-5"), Some(vec![1, 2, 3, 4, 5]));
        assert_eq!(try_parse_range("10-12"), Some(vec![10, 11, 12]));
        assert_eq!(try_parse_range("5-5"), Some(vec![5]));
        assert_eq!(try_parse_range("abc"), None);
        assert_eq!(try_parse_range("5-1"), None); // Ungültiger Bereich
    }
}
