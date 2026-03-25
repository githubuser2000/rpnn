# 📘 Algorithmen-Dokumentation (`rpnn/src`)

## 🧭 Überblick

Das System implementiert eine mehrstufige tabellarische Transformationspipeline:

CLI-Input → Spaltenauflösung → Datenprojektion → Generatoren → Chunking → Rendering

---

## 1. 🔍 Spaltenauflösung

### Algorithmus

resolve_columns(query):
    result = exact_match(query)
    if result ≠ ∅:
        return result

    result = fuzzy_match(query)
    if result ≠ ∅:
        return result

    result = generator_match(query)
    return result

---

## 2. 📐 Tabellenprojektion

project_table(headers, data, indices):
    new_headers = [headers[i] for i in indices]

    new_data = []
    for row in data:
        new_row = [row[i] for i in indices]
        new_data.append(new_row)

    return (new_headers, new_data)

---

## 3. ⚙️ Generator-Transformation

generate_column(data, base_indices, f):
    new_column = []

    for row in data:
        inputs = [row[i] for i in base_indices]
        value = f(inputs)
        new_column.append(value)

    return new_column

append_column(headers, data, new_header, new_column):
    headers.append(new_header)

    for i in range(len(data)):
        data[i].append(new_column[i])

---

## 4. 🔢 Bereichsparser

parse_range(input):
    split input at '-'

    start = parse_int(left)
    end   = parse_int(right)

    if start ≤ end:
        return [start, ..., end]

---

## 5. 📦 Chunking

chunk_columns(columns, widths, max_width):
    chunks = []
    current_chunk = []
    current_width = 0

    for i in range(len(columns)):
        w = widths[i]

        if current_width + w > max_width:
            chunks.append(current_chunk)
            current_chunk = []
            current_width = 0

        current_chunk.append(columns[i])
        current_width += w

    if current_chunk ≠ ∅:
        chunks.append(current_chunk)

    return chunks

---

## 6. 📏 Spaltenbreiten

Auto:
width[i] = max(len(header[i]), max(len(row[i])))

Override:
width[i] = constant

---

## 7. 🧾 CSV Parsing

parse_csv(file):
    headers = normalize(read_first_line())
    data = []

    for line in file:
        row = parse_line(line)
        data.append(row)

    return (headers, data)

---

## 8. 🖨️ Rendering

render_table(headers, data, widths):
    print_row(headers)
    for row in data:
        print_row(row)

---

## 🧠 Gesamtform

Output = Render ∘ Chunk ∘ Generate ∘ Project ∘ Resolve (Input)
