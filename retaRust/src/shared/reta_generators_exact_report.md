# Reta-Generatorspalten: vollständige Inventarisierung und Rust-Referenz

Diese Datei ist direkt aus den Python- und Rust-Quelldateien des hochgeladenen Projekts abgeleitet. Ziel ist eine bitgenaue Transcompilierungs-Referenz in reta-Architektur.

## Gefundene Generator-Familien

- `generated1`: 50 Einträge
- `boolAndTupleSet1`: 8 Einträge
- `generated2`: 13 Einträge
- `metakonkret`: 12 Einträge
- `gebroUni1`: 1 Einträge
- `gebrGal1`: 1 Einträge
- `gebrEmo1`: 1 Einträge
- `gebrGroe1`: 1 Einträge
- `concat1`: dynamische Prim-/Concat-Generatorfamilie, nicht als fixe `paraNdataMatrix`-Liste gespeichert
- `kombi2`: zweite Kombi-Familie, CSV-basiert, keine eigentliche Generatorfunktion

## Exakte Python-Aufrufreihenfolge der Generatorspalten

1. `readConcatCsv` — reta.py:1460-1497 — CSV-basierte Generator-/Concat-Erweiterungen 1..9
2. `concatVervielfacheZeile` — libs/lib4tables_concat.py:410-496 — Vielfache-Spalte
3. `concatModallogik` — libs/lib4tables_concat.py:497-853 — Modallogik aus generated1
4. `concatPrimCreativityType` — libs/lib4tables_concat.py:282-324 — Prim-Kreativität/Typ
5. `concatGleichheitFreiheitDominieren` — libs/lib4tables_concat.py:214-247 — Gleichheit/Freiheit/Dominieren
6. `concatGeistEmotionEnergieMaterieTopologie` — libs/lib4tables_concat.py:248-281 — Geist/Emotion/Energie/Materie/Topologie
7. `concatMondExponzierenLogarithmusTyp` — libs/lib4tables_concat.py:325-409 — Mond/Exponenz/Logarithmus/Typ
8. `concat1RowPrimUniverse2` — libs/lib4tables_concat.py:1421-2009 — generated2-Textgeneratoren
9. `concat1PrimzahlkreuzProContra` — libs/lib4tables_concat.py:975-1420 — Primzahlkreuz pro/contra
10. `concatLovePolygon` — libs/lib4tables_concat.py:97-131 — Liebe/Sternpolygon
11. `spalteFuerGegenInnenAussenSeitlichPrim` — libs/lib4tables_concat.py:2743-2863 — boolAndTupleSet1
12. `spalteMetaKontretTheorieAbstrakt_etc_1` — libs/lib4tables_concat.py:2010-2025 — metakonkret
13. `createSpalteGestirn` — libs/tableHandling.py:1456-1517 — Gestirn-Spalte

## Familie `generated1` (50 Einträge)

1. `Menschliches` / `Moral` → `vec![Tuple(vec![Int(216), Int(221)])]`
2. `Menschliches` / `Sinn_des_Lebens` → `vec![Tuple(vec![Int(181), Int(182)])]`
3. `Menschliches` / `Egoismus` → `vec![Tuple(vec![Int(66), Int(67)])]`
4. `Menschliches` / `Liebe` → `vec![Tuple(vec![Int(121), Int(122)])]`
5. `Grundstrukturen` / `Liebe_(7)` → `vec![Tuple(vec![Int(121), Int(122)])]`
6. `Eigenschaften_n` / `Weisheit_etc` → `vec![Tuple(vec![Int(40), Int(41)])]`
7. `Eigenschaften_n` / `Dein_Recht_bekommen` → `vec![Tuple(vec![Int(291), Int(292)])]`
8. `Eigenschaften_n` / `unterlegen_überlegen` → `vec![Tuple(vec![Int(380), Int(381)])]`
9. `Eigenschaften_n` / `Ehrlichkeit_und_Streit` → `vec![Tuple(vec![Int(375), Int(376)])]`
10. `Eigenschaften_1/n` / `Würdig` → `vec![Tuple(vec![Int(373), Int(374)])]`
11. `Eigenschaften_1/n` / `Regel_vs_Ausnahme` → `vec![Tuple(vec![Int(371), Int(372)])]`
12. `Eigenschaften_1/n` / `Werte` → `vec![Tuple(vec![Int(360), Int(361)])]`
13. `Eigenschaften_1/n` / `Gutartigkeits-Egoismus` → `vec![Tuple(vec![Int(362), Int(363)])]`
14. `Eigenschaften_1/n` / `Reflektieren_Erkenntnis-Erkennen` → `vec![Tuple(vec![Int(364), Int(365)])]`
15. `Eigenschaften_1/n` / `Vertrauen_wollen` → `vec![Tuple(vec![Int(366), Int(367)])]`
16. `Eigenschaften_n` / `einklinken_vertrauen_anprangern` → `vec![Tuple(vec![Int(368), Int(369)])]`
17. `Eigenschaften_1/n` / `Ausrichten_Einrichten` → `vec![Tuple(vec![Int(358), Int(359)])]`
18. `Eigenschaften_1/n` / `Toleranz_Respekt_Akzeptanz_Willkommen` → `vec![Tuple(vec![Int(62), Int(63)])]`
19. `Eigenschaften_n` / `familiebrauchen` → `vec![Tuple(vec![Int(279), Int(280)])]`
20. `Eigenschaften_n` / `ego` → `vec![Tuple(vec![Int(277), Int(278)])]`
21. `Eigenschaften_n` / `Selbstsucht_Ichsucht_etc` → `vec![Tuple(vec![Int(274), Int(275)])]`
22. `Eigenschaften_n` / `Forschen_Erfinden_Einklinken` → `vec![Tuple(vec![Int(258), Int(259)])]`
23. `Eigenschaften_n` / `Kooperation_vs_Arsch` → `vec![Tuple(vec![Int(245), Int(246)])]`
24. `Eigenschaften_n` / `Liebe_usw` → `vec![Tuple(vec![Int(247), Int(248)])]`
25. `Eigenschaften_n` / `Selbstlosigkeit_Ichlosigkeit_etc` → `vec![Tuple(vec![Int(238), Int(239)])]`
26. `Eigenschaften_n` / `variationsreich_eintönig` → `vec![Tuple(vec![Int(236), Int(237)])]`
27. `Eigenschaften_n` / `Zuneigung_Abneigung` → `vec![Tuple(vec![Int(199), Int(200)])]`
28. `Menschliches` / `ehrlich_vs_höflich` → `vec![Tuple(vec![Int(224), Int(225)])]`
29. `Eigenschaften_n` / `ehrlich_vs_höflich` → `vec![Tuple(vec![Int(224), Int(225)])]`
30. `Eigenschaften_n` / `Tragweite` → `vec![Tuple(vec![Int(211), Int(212)])]`
31. `Eigenschaften_n` / `wertvoll` → `vec![Tuple(vec![Int(186), Int(187)])]`
32. `Eigenschaften_n` / `Götter_Propheten_Familien_Freunde` → `vec![Tuple(vec![Int(184), Int(185)])]`
33. `Eigenschaften_n` / `sanft_vs_hart` → `vec![Tuple(vec![Int(161), Int(162)]), Tuple(vec![Int(159), Int(160)])]`
34. `Eigenschaften_n` / `vereinen_vs_verbinden` → `vec![Tuple(vec![Int(133), Int(134)])]`
35. `Eigenschaften_n` / `gut_böse_lieb_schlecht` → `vec![Tuple(vec![Int(38), Int(39)])]`
36. `Eigenschaften_n` / `Sinn_und_Zweck_des_Lebens` → `vec![Tuple(vec![Int(181), Int(182)])]`
37. `Eigenschaften_n` / `Zeit_vs_Raum` → `vec![Tuple(vec![Int(49), Int(50)])]`
38. `Eigenschaften_n` / `egalitär_vs_autoritär` → `vec![Tuple(vec![Int(163), Int(164)])]`
39. `Eigenschaften_n` / `Meinungen_und_Ruf` → `vec![Tuple(vec![Int(60), Int(61)])]`
40. `Eigenschaften_n` / `Meinungsintelligenz` → `vec![Tuple(vec![Int(151), Int(152)])]`
41. `Eigenschaften_n` / `Sittlichkeit` → `vec![Tuple(vec![Int(179), Int(180)])]`
42. `Eigenschaften_n` / `Führung` → `vec![Tuple(vec![Int(173), Int(174)])]`
43. `Eigenschaften_n` / `Durchleuchten` → `vec![Tuple(vec![Int(177), Int(178)])]`
44. `Eigenschaften_n` / `Fördern_Sensiblisieren_und_Gedeihen` → `vec![Tuple(vec![Int(175), Int(176)])]`
45. `Eigenschaften_n` / `Überheblichkeit` → `vec![Tuple(vec![Int(171), Int(172)])]`
46. `Eigenschaften_n` / `Polung_der_Liebe` → `vec![Tuple(vec![Int(121), Int(122)])]`
47. `Eigenschaften_n` / `Egoismus_vs_Altruismus` → `vec![Tuple(vec![Int(66), Int(67)])]`
48. `Eigenschaften_n` / `kausal` → `vec![Tuple(vec![Int(110), Int(111)])]`
49. `Eigenschaften_n` / `Gleichheit` → `vec![Tuple(vec![Int(192), Int(193)])]`
50. `Eigenschaften_n` / `Überleben` → `vec![Tuple(vec![Int(194), Int(195)])]`

## Familie `boolAndTupleSet1` (8 Einträge)

1. `Wichtigstes_zum_verstehen` / `Zweitwichtigste` → `vec![Tuple(vec![Int(10)])]`
2. `Primzahlwirkung` / `Universum_Strukturalien_Transzendentalien` → `vec![Tuple(vec![Int(5)])]`
3. `Primzahlwirkung` / `Richtung_als_Richtung` → `vec![Tuple(vec![NoneValue])]`
4. `Primzahlwirkung` / `Galaxieabsicht` → `vec![Tuple(vec![Int(10)])]`
5. `Primzahlwirkung` / `Absicht_Reziproke_Galaxie` → `vec![Tuple(vec![Int(42)])]`
6. `Primzahlwirkung` / `Universum_Reziproke` → `vec![Tuple(vec![Int(131)])]`
7. `Primzahlwirkung` / `Dagegen-Gegentranszendentalie` → `vec![Tuple(vec![Int(138)])]`
8. `Primzahlwirkung` / `neutrale_Gegentranszendentalie` → `vec![Tuple(vec![Int(202)])]`

## Familie `generated2` (13 Einträge)

1. `Wichtigstes_zum_verstehen` / `Motive_Sternpolygone` → `vec![Str("primMotivStern")]`
2. `Grundstrukturen` / `nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)` → `vec![Str("primzahlkreuzprocontra")]`
3. `Pro_Contra` / `Primzahlkreuz_pro_contra` → `vec![Str("primzahlkreuzprocontra")]`
4. `Bedeutung` / `Primzahlkreuz_pro_contra` → `vec![Str("primzahlkreuzprocontra")]`
5. `Primvielfache` / `Motive_gleichförmige_Polygone` → `vec![Str("primMotivGleichf")]`
6. `Primvielfache` / `Struktur_gleichförmige_Polygone` → `vec![Str("primStrukGleichf")]`
7. `Primvielfache` / `Motive_Sternpolygone` → `vec![Str("primMotivStern")]`
8. `Primvielfache` / `Struktur_Sternpolygone` → `vec![Str("primStrukStern")]`
9. `Primvielfache` / `Motiv_Sternpolygon_gebrochen-rational` → `vec![Str("primMotivSternGebr")]`
10. `Primvielfache` / `Struktur_Sternpolyon_gebrochen-rational` → `vec![Str("primStrukSternGebr")]`
11. `Primvielfache` / `Motiv_gleichförmige_Polygone_gebrochen-rational` → `vec![Str("primMotivGleichfGebr")]`
12. `Primvielfache` / `Struktur_gleichförmige_Polygone_gebrochen-rational` → `vec![Str("primStrukGleichfGebr")]`
13. `Primvielfache` / `beschrieben` → `vec![Str("PrimCSV")]`

## Familie `metakonkret` (12 Einträge)

1. `Meta_vs_Konkret_(Universum)` / `meta` → `vec![Tuple(vec![Int(2), Int(0)])]`
2. `Meta_vs_Konkret_(Universum)` / `konkret` → `vec![Tuple(vec![Int(2), Int(1)])]`
3. `Meta_vs_Konkret_(Universum)` / `Theorie` → `vec![Tuple(vec![Int(3), Int(0)])]`
4. `Meta_vs_Konkret_(Universum)` / `Praxis` → `vec![Tuple(vec![Int(3), Int(1)])]`
5. `Meta_vs_Konkret_(Universum)` / `Management` → `vec![Tuple(vec![Int(4), Int(0)])]`
6. `Meta_vs_Konkret_(Universum)` / `verändernd` → `vec![Tuple(vec![Int(4), Int(1)])]`
7. `Meta_vs_Konkret_(Universum)` / `ganzheitlich` → `vec![Tuple(vec![Int(5), Int(0)])]`
8. `Meta_vs_Konkret_(Universum)` / `darüber_hinausgehend` → `vec![Tuple(vec![Int(5), Int(1)])]`
9. `Meta_vs_Konkret_(Universum)` / `Unternehmung_Geschäft` → `vec![Tuple(vec![Int(6), Int(0)])]`
10. `Meta_vs_Konkret_(Universum)` / `wertvoll` → `vec![Tuple(vec![Int(6), Int(1)])]`
11. `Meta_vs_Konkret_(Universum)` / `Beherrschen` → `vec![Tuple(vec![Int(7), Int(0)])]`
12. `Meta_vs_Konkret_(Universum)` / `Richtung` → `vec![Tuple(vec![Int(7), Int(1)])]`

## Familie `gebroUni1` (1 Einträge)

1. `gebrochen-rational_Universum_n/m` / `14` → `vec![Str("14"), Str("22"), Str("16"), Str("9"), Str("4"), Str("8"), Str("6"), Str("10"), Str("5"), Str("3"), Str("15"), Str("20"), Str("2"), Str("12"), Str("7")…`

## Familie `gebrGal1` (1 Einträge)

1. `gebrochen-rational_Galaxie_n/m` / `14` → `vec![Str("14"), Str("22"), Str("16"), Str("9"), Str("4"), Str("8"), Str("6"), Str("10"), Str("5"), Str("3"), Str("15"), Str("20"), Str("2"), Str("12"), Str("7")…`

## Familie `gebrEmo1` (1 Einträge)

1. `gebrochen-rational_Gefuehle_n/m` / `14` → `vec![Str("14"), Str("22"), Str("16"), Str("9"), Str("4"), Str("8"), Str("6"), Str("10"), Str("5"), Str("3"), Str("15"), Str("20"), Str("2"), Str("12"), Str("7")…`

## Familie `gebrGroe1` (1 Einträge)

1. `gebrochen-rational_Strukturgroesse_n/m` / `14` → `vec![Str("14"), Str("22"), Str("16"), Str("9"), Str("4"), Str("8"), Str("6"), Str("10"), Str("5"), Str("3"), Str("15"), Str("20"), Str("2"), Str("12"), Str("7")…`