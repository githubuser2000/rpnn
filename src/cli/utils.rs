// src/cli/utils.rs
// Optional: Weitere Hilfsfunktionen
pub(crate) fn parse_einfache_zahl(text: &str) -> Option<usize> {
    text.trim().parse::<usize>().ok()
}

pub(crate) fn extrahiere_zahlen_aus_string(text: &str) -> Vec<usize> {
    text.split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sortiere_und_fasse_zusammen() {
        let bereiche = vec![(5, 8), (1, 3), (10, 12), (2, 4)];
        let erwartet = vec![(1, 8), (10, 12)];
        assert_eq!(sortiere_und_fasse_zusammen(bereiche), erwartet);
    }
    
    #[test]
    fn test_sortiere_und_fasse_zusammen_benachbart() {
        let bereiche = vec![(1, 3), (4, 6), (8, 10)];
        let erwartet = vec![(1, 6), (8, 10)];
        assert_eq!(sortiere_und_fasse_zusammen(bereiche), erwartet);
    }
    
    #[test]
    fn test_sortiere_und_fasse_zusammen_leer() {
        assert_eq!(sortiere_und_fasse_zusammen(vec![]), vec![]);
    }
    
    #[test]
    fn test_parse_einfache_zahl() {
        assert_eq!(parse_einfache_zahl("42"), Some(42));
        assert_eq!(parse_einfache_zahl(" 123 "), Some(123));
        assert_eq!(parse_einfache_zahl("abc"), None);
    }
}
