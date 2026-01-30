# generate_rust.py
import re

def parse_line(line):
    """Parse a line from your CSV format"""
    line = line.strip()
    if not line:
        return None
    
    # Split by semicolons
    parts = line.split(';')
    if len(parts) < 3:
        print(f"Warning: Line has less than 3 parts: {line}")
        return None
    
    # Parse main categories (first part in parentheses)
    main_part = parts[0].strip()
    if main_part.startswith('(') and main_part.endswith(')'):
        main_categories = [cat.strip().strip("'") for cat in main_part[1:-1].split(',')]
    else:
        main_categories = [main_part.strip("'")]
    
    # Parse sub categories (second part, comma separated)
    sub_part = parts[1].strip()
    sub_categories = [cat.strip() for cat in sub_part.split(',')]
    
    # Parse IDs (third part in brackets)
    ids_part = parts[2].strip()
    if ids_part.startswith('[') and ids_part.endswith(']'):
        ids_str = ids_part[1:-1].strip()
        if ids_str:
            try:
                # Handle complex cases with quotes and strings
                ids = []
                for item in ids_str.split(','):
                    item = item.strip()
                    if item.isdigit():
                        ids.append(int(item))
            except:
                ids = []
        else:
            ids = []
    else:
        ids = []
    
    return main_categories, sub_categories, ids

def generate_rust_code(input_file, output_file):
    with open(input_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    rust_lines = []
    rust_lines.append("use std::collections::{HashMap, HashSet};\n")
    rust_lines.append("pub fn create_category_map() -> HashMap<&'static str, HashMap<&'static str, Vec<u32>>> {")
    rust_lines.append("    let mut main_to_sub = HashMap::new();\n")
    rust_lines.append("    let data = vec![")
    
    data_count = 0
    for line in lines:
        result = parse_line(line)
        if result:
            main_categories, sub_categories, ids = result
            if main_categories and sub_categories:
                # Format main categories
                main_str = 'vec![' + ', '.join([f'"{cat}"' for cat in main_categories]) + ']'
                
                # Format sub categories  
                sub_str = 'vec![' + ', '.join([f'"{cat}"' for cat in sub_categories]) + ']'
                
                # Format IDs
                ids_str = 'vec![' + ', '.join([str(id) for id in ids]) + ']'
                
                rust_lines.append(f"        ({main_str}, {sub_str}, {ids_str}),")
                data_count += 1
    
    rust_lines.append("    ];")
    rust_lines.append(f"    // Total: {data_count} data entries")
    rust_lines.append("")
    rust_lines.append("    for (main_categories, sub_categories, ids) in data {")
    rust_lines.append("        for &main_cat in &main_categories {")
    rust_lines.append("            for &sub_cat in &sub_categories {")
    rust_lines.append("                insert_entry(&mut main_to_sub, main_cat, sub_cat, ids.clone());")
    rust_lines.append("            }")
    rust_lines.append("        }")
    rust_lines.append("    }")
    rust_lines.append("")
    rust_lines.append("    main_to_sub")
    rust_lines.append("}")
    rust_lines.append("")
    rust_lines.append("fn insert_entry(")
    rust_lines.append("    main_to_sub: &mut HashMap<&'static str, HashMap<&'static str, Vec<u32>>>,")
    rust_lines.append("    main_category: &'static str,")
    rust_lines.append("    sub_category: &'static str,")
    rust_lines.append("    new_ids: Vec<u32>")
    rust_lines.append(") {")
    rust_lines.append("    let main_entry = main_to_sub")
    rust_lines.append("        .entry(main_category)")
    rust_lines.append("        .or_insert_with(HashMap::new);")
    rust_lines.append("")
    rust_lines.append("    let existing_ids = main_entry")
    rust_lines.append("        .entry(sub_category)")
    rust_lines.append("        .or_insert_with(Vec::new);")
    rust_lines.append("")
    rust_lines.append("    let mut all_ids: HashSet<u32> = existing_ids.iter().cloned().collect();")
    rust_lines.append("    for &id in &new_ids {")
    rust_lines.append("        all_ids.insert(id);")
    rust_lines.append("    }")
    rust_lines.append("")
    rust_lines.append("    let mut sorted_ids: Vec<u32> = all_ids.into_iter().collect();")
    rust_lines.append("    sorted_ids.sort();")
    rust_lines.append("    *existing_ids = sorted_ids;")
    rust_lines.append("}")
    rust_lines.append("")
    rust_lines.append("fn main() {")
    rust_lines.append("    let category_map = create_category_map();")
    rust_lines.append("    ")
    rust_lines.append("    println!(\"=== STATISTIK ===\");")
    rust_lines.append("    println!(\"Hauptkategorien: {}\", category_map.len());")
    rust_lines.append("    ")
    rust_lines.append("    let mut total_entries = 0;")
    rust_lines.append("    let mut total_ids = 0;")
    rust_lines.append("    ")
    rust_lines.append("    for (main_cat, sub_map) in &category_map {")
    rust_lines.append("        total_entries += sub_map.len();")
    rust_lines.append("        for ids in sub_map.values() {")
    rust_lines.append("            total_ids += ids.len();")
    rust_lines.append("        }")
    rust_lines.append("    }")
    rust_lines.append("    ")
    rust_lines.append("    println!(\"Einträge: {}\", total_entries);")
    rust_lines.append("    println!(\"ID-Referenzen: {}\", total_ids);")
    rust_lines.append("    ")
    rust_lines.append("    // Beispielabfragen")
    rust_lines.append("    println!(\"\\n=== BEISPIELE ===\");")
    rust_lines.append("    ")
    rust_lines.append("    if let Some(sub_map) = category_map.get(\"Menschliches\") {")
    rust_lines.append("        println!(\"'Menschliches' hat {} Unterkategorien\", sub_map.len());")
    rust_lines.append("        if let Some(ids) = sub_map.get(\"menschliches\") {")
    rust_lines.append("            println!(\"  - 'menschliches': {} IDs\", ids.len());")
    rust_lines.append("        }")
    rust_lines.append("    }")
    rust_lines.append("}")
    
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write('\n'.join(rust_lines))
    
    print(f"Generated {output_file} with {data_count} data entries")

if __name__ == "__main__":
    # Anpassen an deine Dateinamen
    input_file = "coordinatesColumnsFirstReliTable.csv"
    output_file = "columnCategories_complete.rs"
    
    generate_rust_code(input_file, output_file)
    print(f"\nKompilieren mit:")
    print(f"rustc {output_file} -o columnCategories")
    print(f"./columnCategories")
