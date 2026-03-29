# 📘 Erweiterte formale Algorithmen-Dokumentation (`rpnn/src`)

## Ziel

Dieses Dokument beschreibt die Kernalgorithmen des Systems in einer formaleren Sprache, näher an Typentheorie, Datenfluss, Invarianten und rust-artigen Signaturen.

---

# 1. Grundlegende Typen

## 1.1 Primitive Struktur

```rust
pub type Cell = String;
pub type Row = Vec<Cell>;
pub type Headers = Vec<String>;
pub type Data = Vec<Row>;
pub type ColumnIndex = usize;
pub type Width = usize;
```

## 1.2 Tabellentyp

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    pub headers: Headers,
    pub data: Data,
}
```

Semantisch:

```math
Table = (H, D)
```

mit

- \( H = [h_0, h_1, \dots, h_{m-1}] \)
- \( D = [r_0, r_1, \dots, r_{n-1}] \)
- \( r_j = [c_{j,0}, c_{j,1}, \dots, c_{j,m-1}] \)

---

# 2. Globale Invarianten

## 2.1 Rechteck-Invariante

Jede Zeile hat genau so viele Zellen wie es Header gibt:

```math
\forall r \in D:\quad |r| = |H|
```

Rust-artig:

```rust
pub fn is_rectangular(table: &Table) -> bool {
    table.data.iter().all(|row| row.len() == table.headers.len())
}
```

## 2.2 Indexgültigkeit

Für jede verwendete Spaltenmenge \( I \subseteq \mathbb{N} \) gilt:

```math
\forall i \in I:\quad i < |H|
```

## 2.3 Ordnungsstabilität

Projektionen und Chunking verändern die Reihenfolge der explizit ausgewählten Spalten nicht, sondern erhalten sie.

---

# 3. Gesamtpipeline als Komposition

Die Hauptverarbeitung kann als Komposition von Transformationen modelliert werden:

```math
Output = Render \circ Chunk \circ Generate \circ Project \circ Resolve (Input)
```

Typisiert:

```rust
resolve  : Input -> ColumnSelection
project  : Table × ColumnSelection -> Table
generate : Table × GeneratorPlan -> Table
chunk    : Table × LayoutConfig -> Vec<TableChunk>
render   : Vec<TableChunk> × RenderConfig -> String
```

---

# 4. Spaltenauflösung

## 4.1 Datentypen

```rust
pub enum Query {
    ExactName(String),
    CategoryPair { ober: String, unter: String },
    ExplicitIndices(Vec<usize>),
    Generated(String),
}
```

```rust
pub type ColumnSelection = Vec<ColumnIndex>;
```

## 4.2 Formales Modell

Die Auflösung ist eine priorisierte Funktion:

```math
Resolve(q) =
\begin{cases}
E(q) & \text{falls } E(q) \neq \emptyset \\
F(q) & \text{falls } E(q)=\emptyset \land F(q)\neq\emptyset \\
G(q) & \text{sonst}
\end{cases}
```

Dabei sind:

- \( E \): exakte Auflösung
- \( F \): sekundäre / erweiterte Auflösung
- \( G \): generatorbasierte Auflösung

## 4.3 Rust-artige Signatur

```rust
pub fn resolve_columns(table: &Table, query: &Query) -> Vec<ColumnIndex>;
```

## 4.4 Algorithmische Form

```text
resolve_columns(table, query):
    r1 = exact_match(table, query)
    if r1 is not empty:
        return r1

    r2 = extended_match(table, query)
    if r2 is not empty:
        return r2

    r3 = generated_match(table, query)
    return r3
```

## 4.5 Eigenschaft

Die Funktion ist deterministisch relativ zu:

- Headern
- vorhandenen Kategorien
- Generator-Registry

Formal:

```math
q_1 = q_2 \land table_1 = table_2 \Rightarrow Resolve(table_1, q_1) = Resolve(table_2, q_2)
```

---

# 5. Projektion

## 5.1 Idee

Eine Projektion reduziert eine Tabelle auf eine explizite, geordnete Spaltenauswahl.

## 5.2 Typ

```rust
pub fn project(table: &Table, indices: &[ColumnIndex]) -> Table;
```

## 5.3 Definition

Für \( I = [i_0, i_1, \dots, i_{k-1}] \):

```math
H' = [H[i_0], H[i_1], \dots, H[i_{k-1}]]
```

und für jede Zeile \( r \):

```math
Proj_I(r) = [r[i_0], r[i_1], \dots, r[i_{k-1}]]
```

also insgesamt:

```math
Project((H, D), I) = (H', [Proj_I(r) \mid r \in D])
```

## 5.4 Erhaltungsprinzip

Die Projektion ist ordnungserhaltend bezüglich \( I \):

```math
\text{order}(H') = \text{order induced by } I
```

Sie ist nicht notwendigerweise sortierend, sondern folgt der angegebenen Indexsequenz.

---

# 6. Generatoren

## 6.1 Grundidee

Ein Generator berechnet aus einer Auswahl von Eingabezellen pro Zeile genau eine neue Zelle.

## 6.2 Typ

```rust
pub trait Generator {
    fn eval(&self, inputs: &[&str]) -> String;
}
```

Alternativ funktional:

```rust
pub type GeneratorFn = fn(&[String]) -> String;
```

## 6.3 Formales Modell

Ein Generator ist eine Funktion

```math
g : Cell^k \to Cell
```

also

```math
g(c_0, c_1, \dots, c_{k-1}) = c_{\text{neu}}
```

## 6.4 Zeilenweise Anwendung

Für eine Basisspaltenmenge \( I = [i_0,\dots,i_{k-1}] \) und eine Zeile \( r \):

```math
Input_I(r) = [r[i_0], \dots, r[i_{k-1}]]
```

Dann:

```math
NewCol[j] = g(Input_I(r_j))
```

## 6.5 Integration in Tabelle

Neue Tabelle:

```math
H' = H \mathbin{+\!\!+} [h_{\text{neu}}]
```

```math
D' = [\, r_j \mathbin{+\!\!+} [NewCol[j]] \mid r_j \in D \,]
```

## 6.6 Rust-artige Signatur

```rust
pub fn generate_column<G: Generator>(
    table: &Table,
    base_indices: &[ColumnIndex],
    header: String,
    generator: &G,
) -> Table;
```

## 6.7 Kompositionsgesetz

Generatoren können verkettet werden:

```math
Generate_{g_2}(Generate_{g_1}(T)) = T''
```

Das System verhält sich daher wie eine Pipeline von endlichen tabellarischen Endomorphismen, sofern nur Spalten hinzugefügt werden.

---

# 7. Bereichsparser

## 7.1 Ziel

Stringrepräsentationen wie `1-5` oder allgemein Bereichsnotation in diskrete Indexmengen transformieren.

## 7.2 Typ

```rust
pub fn parse_range(input: &str) -> Vec<usize>;
```

## 7.3 Basismodell

Für ein Intervall \( a-b \):

```math
ParseRange(a-b) =
\begin{cases}
[a, a+1, \dots, b] & \text{falls } a \le b \\
[] & \text{sonst}
\end{cases}
```

## 7.4 Erweiterung auf Listen

Für eine kommaseparierte Syntax:

```math
Parse("1-3,7,10-12") = Parse("1-3") \mathbin{+\!\!+} Parse("7") \mathbin{+\!\!+} Parse("10-12")
```

## 7.5 Algebraische Sicht

Der Parser bildet aus einer kompakten symbolischen Repräsentation eine endliche, geordnete Teilmenge von \( \mathbb{N} \).

---

# 8. Breitenberechnung

## 8.1 Ziel

Für jede Spalte wird eine Darstellungsbreite bestimmt.

## 8.2 Typ

```rust
pub fn compute_widths(table: &Table) -> Vec<Width>;
```

## 8.3 Automatische Breite

Für Spalte \( i \):

```math
w_i = \max \Big( |H[i]|,\ \max_{r \in D} |r[i]| \Big)
```

Dabei steht \( |x| \) für die Textbreite des Strings \( x \).

## 8.4 Konstantes Override

Falls globale Benutzerbreite \( c \) gesetzt ist:

```math
w_i = c \quad \forall i
```

## 8.5 Partielles Override

Sei \( U = [u_0,\dots,u_{p-1}] \) eine partielle Vorgabe. Dann:

```math
w_i =
\begin{cases}
u_i & \text{falls } i < p \\
a_i & \text{sonst}
\end{cases}
```

mit \( a_i \) als automatisch berechneter Breite.

---

# 9. Chunking als Greedy-Partitionierung

## 9.1 Ziel

Eine Folge von Spalten wird in darstellbare Gruppen zerlegt, sodass jede Gruppe innerhalb einer maximalen Breite liegt.

## 9.2 Typen

```rust
pub struct TableChunk {
    pub indices: Vec<ColumnIndex>,
}
```

```rust
pub fn chunk_columns(
    indices: &[ColumnIndex],
    widths: &[Width],
    max_width: Width,
) -> Vec<TableChunk>;
```

## 9.3 Problemform

Gegeben:

- Spaltenfolge \( I = [i_0, \dots, i_{m-1}] \)
- Breiten \( w(i_j) \)
- Kapazität \( M \)

Gesucht:

Eine Zerlegung in Teilfolgen \( C_1,\dots,C_t \), so dass:

```math
\sum_{i \in C_k} w(i) \le M
```

## 9.4 Greedy-Algorithmus

```text
current_chunk = []
current_sum = 0

for col in columns:
    if current_sum + width(col) > M:
        emit current_chunk
        current_chunk = []
        current_sum = 0

    current_chunk.push(col)
    current_sum += width(col)

emit current_chunk if nonempty
```

## 9.5 Eigenschaft

Der Algorithmus ist:

- linear
- stabil in der Spaltenreihenfolge
- lokal greedy
- nicht allgemein global optimal im Sinne minimaler Chunkzahl bei beliebigen Zusatzkostenmodellen

---

# 10. CSV-Parsing und Normalisierung

## 10.1 Typ

```rust
pub fn parse_csv(path: &Path) -> Table;
```

## 10.2 Struktur

Das CSV-Parsing liefert:

1. Header-Zeile
2. Datenzeilen
3. Normalisierung der Header

## 10.3 Header-Normalisierung

Sei \( RawHeaders = [x_0,\dots,x_{m-1}] \).

Gesucht ist eine neue Folge \( H \), so dass:

- keine leeren Namen verbleiben
- keine Duplikate verbleiben

Formell:

```math
\forall i \ne j:\ H[i] \ne H[j]
```

## 10.4 Algorithmus

```rust
pub fn normalize_headers(raw: &[String]) -> Vec<String> {
    use std::collections::HashMap;

    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut out = Vec::new();

    for (i, h) in raw.iter().enumerate() {
        let mut name = if h.is_empty() {
            format!("spalte_{}", i + 1)
        } else {
            h.clone()
        };

        let counter = seen.entry(name.clone()).or_insert(0);
        if *counter > 0 {
            name = format!("{}_{}", name, *counter + 1);
        }
        *counter += 1;
        out.push(name);
    }

    out
}
```

## 10.5 Semantik

Die Normalisierung ist eine kanonische Repräsentationsfunktion:

```math
Normalize : RawHeaders \to Headers
```

mit Eindeutigkeitsgarantie.

---

# 11. Rendering

## 11.1 Ziel

Transformation einer strukturierten Tabelle in einen linearen Textstrom.

## 11.2 Typ

```rust
pub fn render(chunks: &[RenderedChunk]) -> String;
```

## 11.3 Zellenformatierung

```rust
pub fn format_cell(s: &str, width: Width) -> String;
```

Formal:

```math
FormatCell : Cell \times Width \to Cell'
```

mit einer Ausgabe fester Breite.

## 11.4 Zeilenrendering

Für Zeile \( r = [c_0,\dots,c_{k-1}] \) und Breiten \( w_0,\dots,w_{k-1} \):

```math
RenderRow(r) = FormatCell(c_0,w_0) \; || \; FormatCell(c_1,w_1) \; || \dots || \; FormatCell(c_{k-1},w_{k-1})
```

wobei \( || \) Konkatenation mit Spaltentrennung bezeichnet.

## 11.5 Chunkrendering

Ein Chunk ist eine Teilprojektion der Gesamtspalten. Rendering erfolgt chunkweise:

```math
Render(Chunks) = RenderChunk(C_1) \mathbin{+\!\!+} RenderChunk(C_2) \mathbin{+\!\!+} \dots
```

---

# 12. Rust-artige abstrakte API

## 12.1 Gesamte Schicht

```rust
pub trait ColumnResolver {
    fn resolve(&self, table: &Table, query: &Query) -> Vec<ColumnIndex>;
}

pub trait TableProjector {
    fn project(&self, table: &Table, indices: &[ColumnIndex]) -> Table;
}

pub trait ColumnGenerator {
    fn generate(&self, table: &Table) -> Table;
}

pub trait Layouter {
    fn chunk(&self, table: &Table, widths: &[Width], max_width: Width) -> Vec<TableChunk>;
}

pub trait Renderer {
    fn render(&self, table: &Table, chunks: &[TableChunk], widths: &[Width]) -> String;
}
```

---

# 13. Kategorie-theoretische Lesart

Eine grobe strukturelle Interpretation:

- Objekte: Tabellenzustände
- Morphismen: tabellarische Transformationen
- Komposition: sequenzielle Anwendung
- Identität: unveränderte Tabelle

Dann kann man schreiben:

```math
T_0 \xrightarrow{Resolve} S \xrightarrow{Project} T_1 \xrightarrow{Generate} T_2 \xrightarrow{Chunk} L \xrightarrow{Render} O
```

Dabei ist:

- \( T_0, T_1, T_2 \) jeweils Tabellen
- \( S \) eine Spaltenselektion
- \( L \) ein Layoutobjekt
- \( O \) ein Textoutput

Nicht alle Schritte sind Endomorphismen auf demselben Typ, aber ein großer Teil der Pipeline ist strukturerhaltende Transformation auf Tabellen.

---

# 14. Wichtige algorithmische Eigenschaften

## 14.1 Determinismus

Bei gleichen Eingaben entsteht gleicher Output.

## 14.2 Lokalität der Generatoren

Ein zeilenweiser Generator hängt nur von der aktuellen Zeile und den angegebenen Basisspalten ab.

Formal:

```math
NewCol[j] = g(r_j[I])
```

und nicht von \( r_k \) für \( k \ne j \), sofern kein globaler Generator verwendet wird.

## 14.3 Monotonie bei Spaltenerweiterung

Wenn ein Generator nur neue Spalten anhängt:

```math
|H'| = |H| + 1
```

und für jede Zeile:

```math
|r'| = |r| + 1
```

## 14.4 Rechteckerhalt

Jede korrekte Transformation erhält die Rechteck-Invariante.

---

# 15. Zusammenfassung der Typen und Signaturen

```rust
pub struct Table {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

pub enum Query {
    ExactName(String),
    CategoryPair { ober: String, unter: String },
    ExplicitIndices(Vec<usize>),
    Generated(String),
}

pub trait Generator {
    fn eval(&self, inputs: &[&str]) -> String;
}

pub fn resolve_columns(table: &Table, query: &Query) -> Vec<usize>;
pub fn project(table: &Table, indices: &[usize]) -> Table;
pub fn generate_column<G: Generator>(
    table: &Table,
    base_indices: &[usize],
    header: String,
    generator: &G,
) -> Table;
pub fn parse_range(input: &str) -> Vec<usize>;
pub fn compute_widths(table: &Table) -> Vec<usize>;
pub fn chunk_columns(indices: &[usize], widths: &[usize], max_width: usize) -> Vec<TableChunk>;
pub fn render(chunks: &[RenderedChunk]) -> String;
```

---

# Schluss

Die Kernidee des Systems ist nicht bloß „Tabellen anzeigen“, sondern:

1. Spalten semantisch auflösen,
2. Tabellen gezielt projizieren,
3. neue Spalten funktional erzeugen,
4. Layout per Greedy-Partitionierung berechnen,
5. alles in festen Textoutput überführen.

Formal gesehen ist `rpnn` damit eine Pipeline aus diskreten, deterministischen Transformationen auf endlich-dimensionalen Tabellenobjekten.
