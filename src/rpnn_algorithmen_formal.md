# 📘 Formale Algorithmen-Dokumentation (`rpnn/src`)

## 🧭 Typenmodell

```rust
type Table = (Headers, Data)

type Headers = Vec<String>
type Data    = Vec<Row>
type Row     = Vec<String>

type ColumnIndex = usize
type Indices = Vec<ColumnIndex>
```

---

## 🔁 Gesamtpipeline

```math
Output = Render ∘ Chunk ∘ Generate ∘ Project ∘ Resolve (Input)
```

---

## 1. 🔍 Spaltenauflösung

### Typ

```rust
resolve : Query → Indices
```

### Definition

```math
R(q) =
    E(q),                wenn E(q) ≠ ∅
    F(q),                wenn E(q) = ∅ ∧ F(q) ≠ ∅
    G(q),                sonst
```

---

## 2. 📐 Projektion

### Typ

```rust
project : Table × Indices → Table
```

### Definition

```math
Headers' = [Headers[i] | i ∈ I]

Data' = [
    [row[i] | i ∈ I]
    for row ∈ Data
]
```

---

## 3. ⚙️ Generatoren

### Typ

```rust
Generator = Vec<String> → String
```

```rust
generate : Table × Indices × Generator → Table
```

### Definition

```math
new_column[j] = f([row[i] for i ∈ I])

Headers' = Headers ⊕ [new_header]

Data'[j] = Data[j] ⊕ [new_column[j]]
```

---

## 4. 🔢 Bereichsparser

### Typ

```rust
parse_range : String → Indices
```

### Definition

```math
parse_range(a-b) = {a, a+1, ..., b}
```

---

## 5. 📦 Chunking

### Typ

```rust
chunk : (Columns × Widths × MaxWidth) → Vec<Chunk>
```

### Definition

Greedy-Partitionierung:

```math
∑ width(c_i) ≤ MaxWidth
```

---

## 6. 📏 Breitenfunktion

### Typ

```rust
width : Table → Vec<usize>
```

### Definition

```math
width(i) = max(
    len(header_i),
    max(len(row_i))
)
```

---

## 7. 🧾 CSV-Funktion

```rust
parse_csv : File → Table
```

---

## 8. 🖨️ Rendering

```rust
render : Table × Widths → String
```

---

## 🧠 Algebraische Struktur

Das System ist eine Komposition von Morphismen:

```math
Table → Table → Table → Layout → String
```

---

## 🔗 Eigenschaften

- Deterministisch
- Funktional (Generatoren)
- Monoton (Erweiterung durch neue Spalten)
- Stabil (Projektion erhält Ordnung)
