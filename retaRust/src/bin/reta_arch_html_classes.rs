use std::env;

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        let snapshot = reta_architecture::html_class_catalog_snapshot();
        println!(
            "{{\"record_count\":{},\"unique_column_count\":{},\"text_record_count\":{},\"class_record_count\":{},\"column_744_record_count\":{},\"column_744_text\":{}}}",
            snapshot.record_count,
            snapshot.unique_column_count,
            snapshot.text_record_count,
            snapshot.class_record_count,
            snapshot.column_744_record_count,
            snapshot
                .column_744_text
                .as_ref()
                .map(|value| format!("\"{}\"", json_escape(value)))
                .unwrap_or_else(|| "null".to_string())
        );
        return;
    }

    let column = args.remove(0).parse::<i64>().unwrap_or(744);
    let records = reta_architecture::html_class_records_for_column(column);
    println!("{{\"column_number\":{},\"record_count\":{},\"records\":[{}]}}", column, records.len(),
        records
            .into_iter()
            .map(|record| format!(
                "{{\"row_number\":{},\"tag\":\"{}\",\"class_string\":\"{}\",\"text\":\"{}\"}}",
                record.row_number.map(|v| v.to_string()).unwrap_or_else(|| "null".to_string()),
                json_escape(record.tag),
                json_escape(record.class_string),
                json_escape(record.text),
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
