+++
title = "Readme für retaPrompt in Markdown"
author = ["tracehugo"]
date = 2022-12-02T00:00:00+01:00
tags = ["reta", "retaprompt", "readme"]
categories = ["Programmieren"]
draft = false
weight = -7
+++

<div class="ox-hugo-toc toc">

<div class="heading">Table of Contents</div>

- [Befehle](#befehle)
    - [Ausgabe-Befehle](#ausgabe-befehle)
    - [mathematisch Ausgabe-Befehle](#mathematisch-ausgabe-befehle)
    - [Zahlenbereiche, Zeilenangaben](#zahlenbereiche-zeilenangaben)
    - [Die Befehle 15\_....](#die-befehle-15-dot-dot-dot-dot)
    - [sonstige Befehle](#sonstige-befehle)
    - [Speicher-Befehle](#speicher-befehle)
- [retaPrompt starten](#retaprompt-starten)

</div>
<!--endtoc-->


## Befehle {#befehle}
Besser die Readme Markdown mit einem Markdown-Leseprogramm lesen!

-   "help" oder "hilfe" gibt diese Hilfe hier aus.
-   "befehle" gibt die Liste der möglichen Befehle von ReTaPrompt aus.
-   "kurzbefehle" gibt die Liste der möglichen Befehle aus einem Buchstaben von ReTaPrompt aus.
-   "-h" oder "-help" nicht in retaPrompt als Befehl, sondern davor als Programm-Start-Argument für retaPrompt, gibt aus, welche Programm-Start-Argumente für ReTa-Prompt möglich sind.

### Ausgabe-Befehle {#ausgabe-befehle}

-   der befehl reta ist wie, als würde man nicht retaPrompt bzw. rp bzw. rpl starten sondern den CLI-Befehl reta.
    reta -h gibt die Hilfe von reta aus. Der Befehl "reta", mit seinen Parametern, kann nicht mit den anderen Ausgabe-Befehlen vermischt werden.
-   Zeilenangaben sind z.B. eine Zahl oder ein Zahlenbereich wie 2-5 oder diese Dinge mit Komma getrennt wie z.B. 1,3-5,20
    Für den Befehl "u" und  "a", also "universum" und "absicht", kann man auch Brüche angeben, wie 2/3,4/5,1/2.
-   Diese Ausgabe-Befehle lassen sich in einer Eingabe-Zeile kombinieren.
-   "a" bzw. "absicht" bzw. "motive" oder "motiv" gibt die eine Spalte der intrinsischen Absichten der Sternpolygone aus, zusammen mit einer Zeilenangabe für n oder 1/n oder n/m aus, z.B. a1/2
-   "u" bzw. "universum" gibt die eine Spalte der universellen Strukturalien bzw. Transzendentalien der Sternpolygone aus, zusammen mit einer Zeilenangabe, für n oder 1/n oder n/m aus, z.B. u1/2
-   "t" bzw. "thomas" gibt die eine Spalte des Thomasevangliums aus was den intrinsischen Absichten der Sternpolygone in kodierter Form entspricht, zusammen mit einer Zeilenangabe
-   "G" bzw. "geist" gibt den universellen Geist aus, für n oder 1/n aus, z.B. G1/2
-   "B" bzw. "bewusstsein" gibt das Bewusstsein aus, für n oder 1/n aus, z.B. B2
-   "E" bzw. "emotion" gibt die Gefühle für n oder 1/n oder n/m aus, z.B. E1/2,3/2
-   "I" bzw. "impulse" gibt die Impulse für n oder 1/n aus, z.B. I1/2
-   "W" bzw. "wirklichkeit" gibt die Wirklichkeiten für n oder 1/n aus, z.B. W1/2
-   "groesse" gibt die Strukturgröße für n oder 1/n oder n/m aus
-   "komplex" gibt die Komplexitätstufen für n oder 1/n aus
-   "ee" macht, dass keine Überschriften angezeigt werden.
-   "kugeln" bzw. "kreise" gibt die kugeln kreise für n aus, z.B. kugeln 7,14
-   "freiheit" bzw. "gleichheit" gibt entsprechendes für n oder 1/n aus
-   "v" bzw. "einzeln" bzw. "vielfache" bewirkt in Ausgabe-Befehlen außer "reta", dass deren Zeilenangaben z.B. 7 nicht nur Zeile 7 meinen, sondern alle vielfacher dieser Zeilengaben auch, also auch 14,21, usw.
-   "einzeln" ist bei Kurzbefehlen der Standard: Dass Zeilenangaben nicht Vielfache meinen, sondern einzelne Zeilen.
-   "mond" gibt, zusammen mit einer Zeilenangabe, Informationen über Gestirne aus: wie Monde, Planeten, Sonnen
-   "alles" gibt, zusammen mit einer Zeilenangabe einfach alle Spalten aus. Das dauert.
-   "primzahlkreuz" gibt, zusammen mit einer Zeilenangabe die Spalten des Primzahlkreuz-Algorithmusses aus. Diese stehen im Zusammenhang mit den Spalten über Geist (15). Das dauert.
-   "r" bzw. "richtung" gibt, zusammen mit einer Zeilenangabe, die Spalten an, die ausgeben, inwiefern eine Zeile pro außen, pro innen, pro seitlich, gegen seitlich funktioniert.
-   Befehl "u" und "a", also "universum" und "absicht", gibt auch das Reziproke der angebenen Brüche aus, z.B. bei 2/3,3/4 auch 3/2,4/3.
-   Einige der Kurzbefehle aus Buchstaben wie "a" oder "u" lassen sich auch ohne Leerzeichen dazwischen als Befehl verwenden. Beispiel: statt "a u 1,2" geht auch "au1,2".
-   Kurzbefehl "e" macht, dass Zeilen mit fast keiner Information nicht angezeigt werden, also Zeilen mit nur einem Minus oder Fragezeichen
-   Der Befehl "abstand", zusammen 2 weiteren Angaben durch Leerzeichen getrennt, einer Zahl und zwei Zahlen zwischen denen unmittelbar ein Bindestrich steht, z.B. "abstand 7 17-25": berechnet einfach die Subtraktion zwischen der Zahl 7 und dem Zahlenbereich 17 bis 25.
-   der Befehl "leeren" macht den Bildschirm frei
-   Man kann auch Python Regexe verwenden: Befehl '2 r"absi"' ergibt zusammmen': '2 absichten absicht'. Sehr viel komplizierte Dinge sind möglich! Diese Regex-Angabe ist auch möglich vor und hinter fast allen Gleichheitszeichen, also Paramter mit Wert jeweils. Die Angabe des Regex innerhalb r"" gibt es auch getrennt einmal jeweils immer bei mehreren Paramterwerten, die mit Komma getrennt werden.
-   Statt eines Regex funktioniert das Sternchen vor oder hinter einem Gleichheitszeichen, um alles zu meinen. Vor dem Gleichheitszeichen bei -spalten funktioniert auch --*=. Dann funktioniert Autocomplete für das, was dahinter kommen kann für -spalten.

### mathematisch Ausgabe-Befehle {#mathematisch-ausgabe-befehle}

-   "prim" bzw. "primfaktorzerlegung" gibt die Primfaktoren einer angegeben Zahl aus.
-   "multis" bzw. gibt alle Multiplikationen zweier Zahlen aus, die zur angegeben Zahl führen.
-   "abc" oder "abcd" gibt, zusammen mit einer Angabe von Buchstaben aus, welcher Zahl welcher Buchstabe entspricht.
-   "p" bzw. mulpri" bedeutet, dass beide Befehle gemeint sind: "prim" und "multis".
-   "modulo" gibt für eine Zahl die Reste bei Divisionen aus.


### Zahlenbereiche, Zeilenangaben {#zahlenbereiche-zeilenangaben}

Zeilenangaben oder Zahlenbereiche für andere Angaben können sein (ohne Anführungszeichen):

-   "2" das wäre dann Zeile 2
-   "3-5" das wäre Zeile 3,4,5
-   "4,9" das wäre Zeile 4 und 9
-   "2,3-5" das wäre Zeile 2,3,4,5
-   "5+2+4" das wären die Nachbarn der 5 mit Abstand 2 und 4, also 1,3,7,9
-   "10+2" Als Angabe, wenn die Vielfacher gemeint wären (Es sind dann immer vielfacher hier bei "v10+2" [und nicht nur bei Eingaben für explizit vielfacher, sondern dann überall]), dann wäre das nicht nur 8 und 12, sondern auch 18 und 22, und 28 und 32, usw.
-   In der Readme von reta, statt retaPrompt, wird das nochmal erklärt
-  5,v20-22 meint Zeile 5 und außerdem alle Vielfacher von 20,21,22, also z.B. 40,42,44
-  -20,v10 meint alle Vielfacher von 10 ohne die 20 dabei
-  mit Brüchen funktioniert diese v Syntax genau auch
-  "1/2,3" meint Zeile 3 für Sternpolygone n und 1/2 meint hier Zeile 2 für gleichförmige Polygone 1/n
-  "1/2-3/3" zieht sozusagen ein Quadrat und meint damit die Brüche 1/2,2/2,3/2,1/3,2/3,3/3, also den Bereich zwichen den Brüchen.
-  "4/5+2/2" wählt den Bruch 2/3,2/7,6/3,6/7: bildet also den Abstand 2 hin und zurück für den Zähler und Nenner
-  Die Syntax, bei der ein Minus und Plus vorkommt, lässt sich zusammen auch kombinieren. Für Ganz-Zahlwerte wie für Brüche.
-  Diese Syntax kann mehrfach hintereinader eingesetzt werden, wenn sie mit Kommas voneinader getrennt wird. Innerhalb dieser Kommas dürfen entweder Brüche oder ganze Zahlen als Bereichsangabe enthalten sein und dadurch zusammen angegeben werden durch viele Kommas.
-  Diese Syntax für Brüche und ganze Zahlen funktioniert mit dem "v" oder "vielfache" Befehl und mit dieser Minus Syntax, welche für das Herausnehmen von Bereichen da ist.
-   Beispiel
    -  Die 7 ist das Gute und die 6 der Wert und beides ergibt das Wohl: 7-6+1 müssten dann die Antagonisten davon sein (Das ist nicht nur 5 und 8, sondern auch 6 und 7 noch mal dummerweise.). Da muss noch 7-6 abgezogen werden und gesucht sind auch die Vielfacher, also sind "v7-6+1,v-6-7" alle Antagonisten vom "Wohl".
-   Die Angabe von "R" oder "range" bewirkt, dass stattdessen die Zählungen anstelle der Zeilen gesetzt werden.
-   Statt Zahlenbereiche sind auch Python Generatoren möglich wie: {n*2+1 for n in range(3)}
    - ein Minus vor dem Generator bewirkt, dass der Bereich entfernt werden soll
    "-[5 * n for n in range(5)],19-21" entfernt die "20", sodass das Ergebnis "19,21" ist.
-   Statt Zahlenbereiche sind auch Python Rechnungen möglich wie: [2*3]
-   "invertieren" wählt die Nachbarszeilen

### Die Befehle 15\_.... und EIG..... {#die-befehle-15-dot-dot-dot-dot}

-   Die Befehle (die nur wie Zahlen aussehen, aber Befehle sind), die mit 15\_ beginnen, bilden zusammen eine Baumstruktur und sind Ausgabe-Befehle der Grundstrukuren und des Geistes (15), wie "u" oder "a" Ausgabe-Befehle sind. Eine Zeilenangabe wird benötigt. Dann kann etwas ausgeben werden.
-   Die Befehle, die mit EIG ... anfangen handeln zusammen mit einer Zeilenangabe betreffen die Eigenschaften von Sternpolygonen und gleichförmigen Polygonen.

### sonstige Befehle {#sonstige-befehle}

-   "q" oder ":q" oder "exit" oder "quit" oder "ende" beendet ReTaPrompt.
-   "shell", mit Anweisungen dahinter, führt gewöhnliche Shellbefehle aus.
-   "python", mit Anweisungen dahinter, führt gewöhnliche Python-Befehle aus.
-   "math", mit Anweisungen dahinter, führt gewöhnliche Python-Rechen-Befehle, wie 1+1 oder 2\*\*3, aus.
-   "loggen" schaltet das Logging der ReTaPrompt Befehls-History an und "nichtloggen" schaltet diese aus. Wenn das loggen angeschaltet ist, dann kann man Befehle aus der Vergangenheit her holen, diese ggf. editieren, und dann ausführen.
-   "w" bzw. "teiler" bedeutet, dass bei Ausgabebefehlen nicht nur die entsprechende Zeile ausgegeben werden soll, sondern auch die Zeilen der Teiler der Zeilen der Zeilenangabe. Beispielsweise bei der Zeilenangabe 12, auch die Zeilen 2,3,4,6,12. Zeile 1 wird dann nie ausgegeben.
    Wenn dazu noch "v" als Befehl dazu kommt, dann werden von allen diesen Teilern auch die Vielfache als Zeilen ausgegeben. Das wird dann viel.


### Speicher-Befehle {#speicher-befehle}

-   Speichern
    -   "S" bzw. "BefehlSpeichernDanach" speichert den als nächstes eingegeben Befehl ab."S" ein weiteres mal ausgeführt, fügt dieser Speicherung weitere Befehle hinzu.
    -   "s" bzw. "BefehlSpeichernDavor" speichert den davor eingebene Befehl ab. Es kann bisher immer nur der letzte Befehl als eine Sache abgespeichert werden.
        Befehl "s" oder "S", mehrmals ausgeführt, addieren Befehlseingaben.
    -   Werden diese 2 bzw. 4 Speicherbefehle ("s" und "S") zusammen mit anderen Befehlen eingegeben, so werden die anderen Befehle sofort gespeichert, ohne dass diese davor oder danach hätten eingegeben werden müssen.
-   Ausgabe
    -   Wenn man dann in der Befehlseingabe einen Befehl oder Teile eines Befehles eingibt, dann kombinieren sich der gespeicherte Befehl mit dieser Befehlseingabe.
    -    Beispielsweise hat man den Befehl a ohne Zeilenangabe gespeichert. Wenn man dann eine Zeilenangabe eingibt, z.B. 2, dann ist das der Befehl "a 2". Auf diese Art kann man schneller Befehle eingeben.
    Normalerweise, ohne Speicherung, kann man in der Befehlseingabe ausschließlich eine Zeilenangabe tippen und damit sind das die Befehle w a t p.
    -   "o" bzw. "BefehlSpeicherungAusgeben" gibt den gespeicherten Befehl aus. Einfach Enter tippen tut das auch.
    -   Es möglich einen vollständigen reta Befehl (statt eines Kurzbefehles) zu speichern, also z.B. `reta -spalten --licht` und dann bei der Nächsten Befehlseingabe kann man eine Zeilenangabe machen, z.B. 4,7-10 und dann werden von den Spalten über Licht Zeilen 4,7,8,9,10 ausgegeben. So kann man schneller alles tippen. Außerdem lässt sich das mit den Kurzbefehlen w und v Kombinieren, also kann man tippen "3,7-10 v w".
        - Umgekehrt geht das auch: Man speichert eine Zeilenangabe und kann nach der Speicherung einen reta Befehl angeben, ohne Zeilenangabe, und automatisch wird dann die Zeile für den reta Befehl ausgeben. So kann man mehrere RetaBefehle angeben, ohne jedes Mal neu die Zeile für diesen angeben zu müssen.
-  Löschen
    -   "l" bzw. "BefehlSpeicherungLöschen" eröffnet eine Auswahl, was an gespeicherten Befehlen gelöscht werden soll.
    Zahlenbereiche können angegeben werden oder die entsprechenden Zeichen, die gelöscht werden sollen.


## retaPrompt starten {#retaprompt-starten}

-   retaPrompt starten mit Parameter -vi für ViMode (Ansonsten gelten Emacs-Tastenkürzel.),
-   beenden mit q, exit, quit und
-   Hilfe aufrufen mit h oder help oder hilfe,
-   rp (statt retaPrompt zu starten) ist retaPrompt mit vi mode, rpl ist retaPrompt mit vi mode und aktiviertem logging bei Programmstart und mit weniger hilfreichen Informationsausgaben.
-   retaPrompt Parameter -log aktiviert logging bei Programmstart.
-   retaPrompt Parameter -language=english bewirkt, dass alle Befehle in Englisch sind. Die Inhalte sind bisher noch in deutsch.
-   "-befehl" bewirkt, dass bis zum letzten Programmparameter retaPrompt Befehl nur ein RetaPrompt-Befehl ausgeführt wird.
-   "-e" bewirkt, dass bei allen Befehlen das 'e' Kommando bzw. 'keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar' jedes mal verwendet wird - außer wenn der erste Befehl reta war, weil dieser anders funktioniert
Besser die Readme Markdown mit einem Markdown-Leseprogramm lesen!
