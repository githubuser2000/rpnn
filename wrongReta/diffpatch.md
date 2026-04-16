# Sicherheit von Diff-Patches vs. direkter Codeänderung

## Kurzfassung

Diff-Patches sind **nicht unsicher per se**, sondern verschieben Risiken:
- **+ Reproduzierbarkeit**
- **+ Auditierbarkeit**
- **– Lokale Kontrollsicherheit**, wenn undiszipliniert eingesetzt

Sicherheit hängt weniger vom Mechanismus *Patch* ab als von **Kontextkontrolle, Disziplin und Tooling**.

---

## 1. Was ein Diff-Patch wirklich ist

Ein Diff-Patch ist keine indirekte Magie, sondern eine **deklarative Transformationsbeschreibung**:

> „Wenn der Code diesen Kontext hat, transformiere ihn so.“

Eigenschaften:
- Der Patch selbst ist **Code**
- Er enthält **implizite Vorbedingungen** (Hunk-Kontext)
- Formal vergleichbar mit einer partiellen Funktion  
  `f : Code → Code`

---

## 2. Sicherheitsdimensionen im Vergleich

### 2.1 Integrität

**Direkte Codeänderung**
- ✔ Exakter Zustand im Repo sichtbar
- ✘ Lokale Abweichungen möglich
- ✘ „Works on my machine“

**Diff-Patch**
- ✔ Minimale, explizite Änderungen
- ✔ Gut signierbar (Hash, GPG)
- ✘ Risiko falscher oder teilweiser Anwendung

**Bewertung:** Patch gewinnt, **wenn Anwendung verifiziert wird**.

---

### 2.2 Reproduzierbarkeit

**Direkt**
- ✘ Entwicklerzustand entscheidend
- ✘ Schwer exakt reproduzierbar

**Patch**
- ✔ Deterministisch
- ✔ Automatisierbar
- ✔ Ideal für CI, Nix, Guix, Debian, Yocto

**Bewertung:** Patch klar überlegen.

---

### 2.3 Angriffsmöglichkeiten

Reale Patch-Risiken:
1. Context-Drift
2. Hunk-Fuzz
3. Patch-Injection
4. Supply-Chain-Manipulation

> Diese Risiken existieren auch bei Pull Requests – dort oft weniger sichtbar.

---

## 3. Die zentrale Sicherheitsfrage

> Wer kontrolliert den **Anwendungskontext** des Patches?

Unsicher:
```bash
patch -p1 < irgendwas.patch
