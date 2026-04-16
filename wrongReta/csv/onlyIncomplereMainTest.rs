#!/usr/bin/env python3
"""
Generiert automatisch den vollständigen Rust-Code aus der CSV-Datei.
Korrigierte Version mit richtigen Format-Strings.
"""

import re
import sys

def parse_csv_line(line):
    """Parse eine Zeile aus dem CSV-Format."""
    line = line.strip()
    if not line:
        return None
    
    # Entferne Zeilennummern am Anfang (falls vorhanden)
    line = re.sub(r'^\d+\s*', '', line)
    
    # Split by semicolons - genau 3 Teile sollten es sein
    parts = line.split(';')
    if len(parts) < 3:
        return None
    
    # Erster Teil: Hauptkategorien in Klammern
    main_part = parts[0].strip()
    
    # Zweiter Teil: Unterkategorien (kann leer sein)
    sub_part = parts[1].strip()
    
    # Dritter Teil: IDs in eckigen Klammern
    ids_part = parts[2].strip()
    
    # Parse Hauptkategorien
    main_categories = []
    if main_part.startswith('(') and main_part.endswith(')'):
        # Entferne äußere Klammern
        inner = main_part[1:-1].strip()
        # Split by commas, handle quotes
        tokens = re.split(r"',\s*'", inner.replace("('", "").replace("')", ""))
        main_categories = [t.strip("' ") for t in tokens if t.strip("' ")]
    else:
        # Einzelne Kategorie ohne Klammern
        main_categories = [main_part.strip("' ")]
    
    # Parse Unterkategorien
    sub_categories = []
    if sub_part:
        # Einfacher split by comma
        sub_categories = [cat.strip() for cat in sub_part.split(',') if cat.strip()]
    
    # Parse IDs
    ids = []
    if ids_part.startswith('[') and ids_part.endswith(']'):
        content = ids_part[1:-1].strip()
        if content:
            try:
                # Einfacher Ansatz: finde alle Zahlen
                number_pattern = r'\b\d+\b'
                matches = re.findall(number_pattern, content)
                ids = [int(m) for m in matches]
            except:
                ids = []
    
    return main_categories, sub_categories, ids

def generate_rust_code(input_filename, output_filename):
    """Generiert den Rust-Code aus der CSV-Datei."""
    
    print(f"📖 Lese Datei: {input_filename}")
    
    try:
        with open(input_filename, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except FileNotFoundError:
        print(f"❌ Datei nicht gefunden: {input_filename}")
        print("Bitte stelle sicher, dass die Datei im aktuellen Verzeichnis existiert.")
        return
    
    print(f"📊 Verarbeite {len(lines)} Zeilen...")
    
    rust_code = []
    
    # 1. Header
    rust_code.append("use std::collections::{HashMap, HashSet};")
    rust_code.append("")
    
    # 2. Main function
    rust_code.append("pub fn create_category_map() -> HashMap<&'static str, HashMap<&'static str, Vec<u32>>> {")
    rust_code.append("    let mut main_to_sub = HashMap::new();")
    rust_code.append("    ")
    rust_code.append("    // ALLE DATEN AUS DER CSV")
    rust_code.append("    let data = vec![")
    
    # 3. Daten aus CSV parsen und hinzufügen
    data_entries = []
    for i, line in enumerate(lines):
        result = parse_csv_line(line)
        if result:
            main_cats, sub_cats, ids = result
            
            if not main_cats or not sub_cats:
                continue
            
            # Formatieren für Rust
            main_cats_str = 'vec![' + ', '.join([f'"{cat}"' for cat in main_cats]) + ']'
            sub_cats_str = 'vec![' + ', '.join([f'"{cat}"' for cat in sub_cats]) + ']'
            ids_str = 'vec![' + ', '.join([str(id) for id in ids]) + ']'
            
            data_entries.append((main_cats_str, sub_cats_str, ids_str))
    
    # Füge alle Datenzeilen hinzu
    for i, (main_str, sub_str, ids_str) in enumerate(data_entries):
        rust_code.append(f"        ({main_str}, {sub_str}, {ids_str}),")
    
    rust_code.append("    ];")
    rust_code.append(f"    // Insgesamt {len(data_entries)} Datenzeilen")
    rust_code.append("    ")
    
    # 4. Verarbeitungslogik
    rust_code.append("    // Verarbeite alle Daten")
    rust_code.append("    for (main_categories, sub_categories, ids) in data {")
    rust_code.append("        for &main_cat in &main_categories {")
    rust_code.append("            for &sub_cat in &sub_categories {")
    rust_code.append("                insert_entry(&mut main_to_sub, main_cat, sub_cat, ids.clone());")
    rust_code.append("            }")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("    ")
    rust_code.append("    main_to_sub")
    rust_code.append("}")
    rust_code.append("")
    
    # 5. insert_entry Funktion
    rust_code.append("fn insert_entry(")
    rust_code.append("    main_to_sub: &mut HashMap<&'static str, HashMap<&'static str, Vec<u32>>>,")
    rust_code.append("    main_category: &'static str,")
    rust_code.append("    sub_category: &'static str,")
    rust_code.append("    new_ids: Vec<u32>")
    rust_code.append(") {")
    rust_code.append("    let main_entry = main_to_sub")
    rust_code.append("        .entry(main_category)")
    rust_code.append("        .or_insert_with(HashMap::new);")
    rust_code.append("    ")
    rust_code.append("    let existing_ids = main_entry")
    rust_code.append("        .entry(sub_category)")
    rust_code.append("        .or_insert_with(Vec::new);")
    rust_code.append("    ")
    rust_code.append("    // Kombiniere IDs und entferne Duplikate")
    rust_code.append("    let mut all_ids: HashSet<u32> = existing_ids.iter().cloned().collect();")
    rust_code.append("    for &id in &new_ids {")
    rust_code.append("        all_ids.insert(id);")
    rust_code.append("    }")
    rust_code.append("    ")
    rust_code.append("    // Zurück zu Vec, sortieren")
    rust_code.append("    let mut sorted_ids: Vec<u32> = all_ids.into_iter().collect();")
    rust_code.append("    sorted_ids.sort();")
    rust_code.append("    *existing_ids = sorted_ids;")
    rust_code.append("}")
    rust_code.append("")
    
    # 6. Korrigierte main() Funktion MIT RICHTIGEN FORMAT-STRINGS
    rust_code.append("fn main() {")
    rust_code.append("    println!(\"🚀 Generiere Kategorie-Map...\");")
    rust_code.append("    ")
    rust_code.append("    let category_map = create_category_map();")
    rust_code.append("    ")
    rust_code.append("    println!(\"✅ Map erstellt!\");")
    rust_code.append("    println!(\"📊 Statistik:\");")
    rust_code.append("    println!(\"   - Hauptkategorien: {}\", category_map.len());")
    rust_code.append("    ")
    rust_code.append("    let mut total_subcategories = 0;")
    rust_code.append("    let mut total_id_references = 0;")
    rust_code.append("    ")
    rust_code.append("    for (main_cat, sub_map) in &category_map {")
    rust_code.append("        total_subcategories += sub_map.len();")
    rust_code.append("        for ids in sub_map.values() {")
    rust_code.append("            total_id_references += ids.len();")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("    ")
    rust_code.append("    println!(\"   - Unterkategorien: {}\", total_subcategories);")
    rust_code.append("    println!(\"   - ID-Referenzen: {}\", total_id_references);")
    rust_code.append("    ")
    rust_code.append("    // Beispiele anzeigen")
    rust_code.append("    println!(\"\\n🔍 Beispiele:\");")
    rust_code.append("    ")
    
    # Füge Beispielabfragen für einige Hauptkategorien hinzu
    example_categories = ["Menschliches", "Universum", "Religionen", "Grundstrukturen", "Galaxie"]
    
    for cat in example_categories:
        rust_code.append(f"    if let Some(sub_map) = category_map.get(\"{cat}\") {{")
        rust_code.append(f"        println!(\"   - '{{}}' hat {{}} Unterkategorien\", \"{cat}\", sub_map.len());")
        
        # Handle unterschiedliche Schreibweisen für Selbstreferenz
        if cat == "Universum":
            rust_code.append(f"        if let Some(ids) = sub_map.get(\"universum\") {{")
        elif cat == "Galaxie":
            rust_code.append(f"        if let Some(ids) = sub_map.get(\"galaxie\") {{")
        elif cat == "Religionen":
            rust_code.append(f"        if let Some(ids) = sub_map.get(\"religionen\") {{")
        else:
            rust_code.append(f"        if let Some(ids) = sub_map.get(\"{cat.lower()}\") {{")
            
        rust_code.append(f"            println!(\"     → Selbstreferenz: {{}} IDs\", ids.len());")
        rust_code.append("        }")
        rust_code.append("    }")
    
    rust_code.append("    ")
    rust_code.append("    // Detaillierte Analyse der Top 5 Hauptkategorien")
    rust_code.append("    println!(\"\\n🏆 Top 5 Hauptkategorien:\");")
    rust_code.append("    let mut categories: Vec<(&str, usize)> = category_map")
    rust_code.append("        .iter()")
    rust_code.append("        .map(|(k, v)| (*k, v.len()))")
    rust_code.append("        .collect();")
    rust_code.append("    categories.sort_by(|a, b| b.1.cmp(&a.1));")
    rust_code.append("    ")
    rust_code.append("    for (i, (cat, count)) in categories.iter().take(5).enumerate() {")
    rust_code.append("        println!(\"   {{}}. {{}} ({{}} Unterkategorien)\", i + 1, cat, count);")
    rust_code.append("    }")
    rust_code.append("    ")
    rust_code.append("    // IDs für häufigste Kombination")
    rust_code.append("    println!(\"\\n📋 Beispiel-Daten:\");")
    rust_code.append("    if !categories.is_empty() {")
    rust_code.append("        let top_cat = categories[0].0;")
    rust_code.append("        if let Some(sub_map) = category_map.get(top_cat) {")
    rust_code.append("            let mut subs: Vec<(&str, &Vec<u32>)> = sub_map")
    rust_code.append("                .iter()")
    rust_code.append("                .map(|(k, v)| (*k, v))")
    rust_code.append("                .collect();")
    rust_code.append("            subs.sort_by(|a, b| b.1.len().cmp(&a.1.len()));")
    rust_code.append("            ")
    rust_code.append("            if !subs.is_empty() {")
    rust_code.append("                let (sub_name, ids) = subs[0];")
    rust_code.append("                println!(\"   {{}} → {{}}: {{}} IDs\", top_cat, sub_name, ids.len());")
    rust_code.append("                if !ids.is_empty() {")
    rust_code.append("                    let sample: Vec<_> = ids.iter().take(5).collect();")
    rust_code.append("                    println!(\"     Beispiel-IDs: {{:?}}\", sample);")
    rust_code.append("                    if ids.len() > 5 {")
    rust_code.append("                        println!(\"     ... und {{}} weitere\", ids.len() - 5);")
    rust_code.append("                    }")
    rust_code.append("                }")
    rust_code.append("            }")
    rust_code.append("        }")
    rust_code.append("    }")
    rust_code.append("}")
    
    # Schreibe die Datei
    try:
        with open(output_filename, 'w', encoding='utf-8') as f:
            f.write('\n'.join(rust_code))
        
        print(f"✅ Rust-Code generiert: {output_filename}")
        print(f"📈 {len(data_entries)} von {len(lines)} Zeilen erfolgreich geparst")
        
        # Zeige eine kurze Statistik
        print("\n📊 Parsing-Statistik:")
        print(f"   - Gelesene Zeilen: {len(lines)}")
        print(f"   - Erfolgreich geparst: {len(data_entries)}")
        print(f"   - Erfolgsrate: {(len(data_entries)/len(lines))*100:.1f}%")
        
        if len(data_entries) < len(lines):
            print(f"   - Übersprungen: {len(lines) - len(data_entries)}")
        
        print(f"\n⚙️  Kompilieren mit:")
        print(f"   rustc {output_filename} -o kategorien")
        print(f"\n▶️  Ausführen mit:")
        print(f"   ./kategorien")
        
    except Exception as e:
        print(f"❌ Fehler beim Schreiben der Datei: {e}")

def main():
    """Hauptfunktion."""
    print("=" * 60)
    print("📁 CSV zu Rust-Konverter - FORMAT-STRING KORRIGIERT")
    print("=" * 60)
    
    # Dateinamen
    input_file = "coordinatesColumnsFirstReliTable.csv"
    output_file = "columnCategories_perfect.rs"
    
    # Falls andere Dateinamen übergeben wurden
    if len(sys.argv) > 1:
        input_file = sys.argv[1]
    if len(sys.argv) > 2:
        output_file = sys.argv[2]
    
    print(f"Eingabe:  {input_file}")
    print(f"Ausgabe:  {output_file}")
    print("-" * 60)
    
    generate_rust_code(input_file, output_file)
    
    print("=" * 60)
    print("✅ Fertig! Jetzt mit korrekten Format-Strings!")
    print("=" * 60)

if __name__ == "__main__":
    main()
