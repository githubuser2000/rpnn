# -*- coding: utf-8 -*-

"""Runtime/helper layer extracted from the legacy words monolith."""

from .words_matrix import *

def classify(mod):
    if mod == 0:
        return _("ja")
    elif mod == 1:
        return _("Gegenteil")
    elif mod == 2:
        return _("ähnlich")
    elif mod == 3:
        return _("entferntes Gegenteil")
    elif mod == 4:
        return _("entfernt ähnlich")


class tableHandling:
    parameterName: dict = {"kombination": _("kombination")}
    art = ausgabeArt
    into = {
        "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)": _(
            "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)"
        ),
        "Wichtigstes_zum_gedanklich_einordnen": _(
            "Wichtigstes_zum_gedanklich_einordnen"
        ),
        "Zweitwichtigste": _("Zweitwichtigste"),
        "berufe": _("berufe"),
        "intelligenz": _("intelligenz"),
        "tiere": _("tiere"),
        "Kombination_(Universum_und_Galaxie)_(14_mit_15)": _(
            "Kombination_(Universum_und_Galaxie)_(14_mit_15)"
        ),
    }
    gestirnGrossschrift = {
        "Gestirn": _("Gestirn"),
        ", und außerdem ": _(", und außerdem "),
        "Mond (Potenzen)": _("Mond (Potenzen)"),
        "Sonne": _("Sonne"),
        "Sonne (keine Potenzen)": _("Sonne (keine Potenzen)"),
        "Planet (2*n)": _("Planet (2*n)"),
        "wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht": _(
            "wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht"
        ),
    }


class concat:
    themaWort = _("Thema: ")
    polygon1 = {" der eigenen Strukturgröße (": _(" der eigenen Strukturgröße (")}
    polygon2 = {
        ") auf dich bei gleichförmigen Polygonen": _(
            ") auf dich bei gleichförmigen Polygonen"
        )
    }
    gleichheitFreiheitVergleich: dict = {
        "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert": _(
            "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert"
        ),
        "Dominieren, Unterordnen": _("Dominieren, Unterordnen"),
        "Freiheit": _("Freiheit"),
        "Einschränkung der Freiheit": _("Einschränkung der Freiheit"),
        "Gleichheit": _("Gleichheit"),
        "den anderen überbieten wollen": _("den anderen überbieten wollen"),
        "den anderen unterbieten wollen": _("den anderen unterbieten wollen"),
    }
    energietopologie1 = {
        "eine Denkart": _("eine Denkart"),
        "eine Gefühlsart": _("eine Gefühlsart"),
        "total eine Art, etwas geistig zu erzeugen": _(
            "total eine Art, etwas geistig zu erzeugen"
        ),
        "total eine Art zu erleben": _("total eine Art zu erleben"),
        "total eine Energie-Art": _("total eine Energie-Art"),
        "etwas eine Art zu erleben": _("etwas eine Art zu erleben"),
        "etwas eine Art, etwas geistig zu erzeugen": _(
            "etwas eine Art, etwas geistig zu erzeugen"
        ),
        "wenig eine Art, etwas geistig zu erzeugen": _(
            "wenig eine Art, etwas geistig zu erzeugen"
        ),
        "einigermaßen eine Energie-Art": _("einigermaßen eine Energie-Art"),
        "kaum eine Energie-Art": _("kaum eine Energie-Art"),
        "kaum eine Art, etwas geistig zu erzeugen": _(
            "kaum eine Art, etwas geistig zu erzeugen"
        ),
        "eine Denkart": _("eine Denkart"),
        "eine Gefühlsart": _("eine Gefühlsart"),
        "total eine Art, etwas geistig zu erzeugen": _(
            "total eine Art, etwas geistig zu erzeugen"
        ),
        "total eine Art zu erleben": _("total eine Art zu erleben"),
        "total eine Energie-Art": _("total eine Energie-Art"),
        "etwas eine Art zu erleben": _("etwas eine Art zu erleben"),
        "etwas eine Art, etwas geistig zu erzeugen": _(
            "etwas eine Art, etwas geistig zu erzeugen"
        ),
        "wenig eine Art, etwas geistig zu erzeugen": _(
            "wenig eine Art, etwas geistig zu erzeugen"
        ),
        "einigermaßen eine Energie-Art": _("einigermaßen eine Energie-Art"),
        "kaum eine Energie-Art": _("kaum eine Energie-Art"),
        "kaum eine Art, etwas geistig zu erzeugen": _(
            "kaum eine Art, etwas geistig zu erzeugen"
        ),
    }
    ausgabeString = {
        "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art": _(
            "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art"
        )
    }
    kreaZahl = {
        "Evolutions-Züchtungs-Kreativität": _("Evolutions-Züchtungs-Kreativität"),
        "0. Primzahl 1": _("0. Primzahl 1"),
        "1. Primzahl und Sonnenzahl": _("1. Primzahl und Sonnenzahl"),
        "2. Sonnenzahl, aber keine Primzahl": _("2. Sonnenzahl, aber keine Primzahl"),
        "3. Mondzahl": _("3. Mondzahl"),
    }
    mondExpLog1 = {
        "Mond-Typ eines Sternpolygons": _("Mond-Typ eines Sternpolygons"),
        "Mond-Typ eines gleichförmigen Polygons": _(
            "Mond-Typ eines gleichförmigen Polygons"
        ),
    }

    mondExpLog2 = {"kein Mond": _("kein Mond")}
    # wohl nich nötig zu übersetzen modalA_
    # modalA1 = {"modalS": _("modalS")}
    # modalA2 = {"vervielfachter": _("vervielfachter")}
    # modalA3 = {"i_origS": _("i_origS")}

    modalB = {
        "mittelstark überdurchschnittlich: ": _("mittelstark überdurchschnittlich: "),
        "überdurchschnittlich: ": _("überdurchschnittlich: "),
        "mittelleicht überdurchschnittlich: ": _("mittelleicht überdurchschnittlich: "),
        "sehr: ": _("sehr: "),
        "sehr leicht überdurchschnittlich: ": _("sehr leicht überdurchschnittlich: "),
    }
    modalC = {
        "intrinsisch": _("intrinsisch"),
        "zuerst": _("zuerst"),
        "extrinsisch": _("extrinsisch"),
        "als zweites": _("als zweites"),
    }
    modalD = {
        ", nicht: ": _(", nicht: "),
        " (das alles nicht): ": _(" (das alles nicht): "),
        "extrinsisch": _("extrinsisch"),
        "als zweites": _("als zweites"),
        "intrinsisch": _("intrinsisch"),
        "zuerst": _("zuerst"),
    }

    generiertWort = {"Generiert: ": _("Generiert: ")}
    allesNurBezogenAufSatz = _("Alles nur bezogen auf die selbe Strukturgröße einer ")
    headline1 = "Gegen / pro: Nach Rechenregeln auf Primzahlkreuz und Vielfachern von Primzahlen"
    gegen = {"gegen ": _("gegen ")}
    pro = {"pro ": _("pro ")}
    hineinversetzen = {
        " Darin kann sich die ": _(" Darin kann sich die "),
        " am Besten hineinversetzen.": _(" am Besten hineinversetzen."),
    }
    proIst = {
        "pro dieser Zahl sind: ": _("pro dieser Zahl sind: "),
        "pro dieser Zahl ist ": _("pro dieser Zahl ist "),
    }
    contraIst = {
        " contra dieser Zahl sind: ": _(" contra dieser Zahl sind: "),
        " contra dieser Zahl ist ": _(" contra dieser Zahl ist "),
    }
    hineinversetzenSatz = " - Die Zahlen, die für oder gegen diese Zahlen hier sind, können sich in diese am Besten gedanklich hineinversetzen."
    polygone = {
        "Sternpolygone": _("Sternpolygone"),
        "gleichförmige Polygone": _("gleichförmige Polygone"),
    }

    kombisNamen: dict = {
        "Motiv -> Motiv": _("Motiv -> Motiv"),
        "Motiv -> Strukur": _("Motiv -> Strukur"),
        "Struktur -> Motiv": _("Struktur -> Motiv"),
        "Struktur -> Strukur": _("Struktur -> Strukur"),
    }
    # kombisNamen2: dict = {
    #    "GalGal": _("GalGal"),
    #    "GalUni": _("GalUni"),
    #    "UniGal": _("UniGal"),
    #    "UniUni": _("UniUni"),
    # }

    faktorenbla = {
        ", mit Faktoren aus gebrochen-rationalen Zahlen": _(
            ", mit Faktoren aus gebrochen-rationalen Zahlen"
        )
    }
    genMul = {"generierte Multiplikationen ": _("generierte Multiplikationen ")}
    ausserdem = {", außerdem: ": _(", außerdem: "), "| außerdem: ": _("| außerdem: ")}
    Multiplikationen_ = {"Multiplikationen": _("Multiplikationen")}
    nWichtigste = {
        "Wichtigstes_zum_verstehen": _("Wichtigstes_zum_verstehen"),
        "Viertwichtigste": _("Viertwichtigste"),
    }
    metaOrWhat = {
        "Meta-Thema: ": _("Meta-Thema: "),
        "Konkretes: ": _("Konkretes: "),
        "Meta-": _("Meta-"),
        "Konkret-": _("Konkret-"),
        "Theorie-Thema: ": _("Theorie-Thema: "),
        "Praxis: ": _("Praxis: "),
        "Theorie-": _("Theorie-"),
        "Praxis-": _("Praxis-"),
        "Planungs-Thema: ": _("Planungs-Thema: "),
        "Umsetzungs-Thema: ": _("Umsetzungs-Thema: "),
        "Planung-": _("Planung-"),
        "Umsetzung-": _("Umsetzung-"),
        "Anlass-Thema: ": _("Anlass-Thema: "),
        "Wirkungs-Thema: ": _("Wirkungs-Thema: "),
        "Anlass-": _("Anlass-"),
        "wirkung-": _("wirkung-"),
        "Kraft-Gebung: ": _("Kraft-Gebung: "),
        "Verstärkungs-Thema: ": _("Verstärkungs-Thema: "),
        "Kraft-geben-": _("Kraft-geben-"),
        "Verstärkung-": _("Verstärkung-"),
        "Beherrschung: ": _("Beherrschung: "),
        "Richtung-Thema: ": _("Richtung-Thema: "),
        "beherrschend-": _("beherrschend-"),
        "Richtung-": _("Richtung-"),
    }
    metaKonkret = {
        "Meta": _("Meta"),
        "Theorie": _("Theorie"),
        "Management": _("Management"),
        "ganzheitlich": _("ganzheitlich"),
        "Verwertung, Unternehmung, Geschäft": _("Verwertung, Unternehmung, Geschäft"),
        "regieren, beherrschen": _("regieren, beherrschen"),
        "Konkretes": _("Konkretes"),
        "Praxis": _("Praxis"),
        "verändernd": _("verändernd"),
        "darüber hinaus gehend": _("darüber hinaus gehend"),
        "wertvoll": _("wertvoll"),
        "Richtung": _("Richtung"),
        " für 1/n statt n": _(" für 1/n statt n"),
        " für n": _(" für n"),
    }
    innenAussen = {
        "für innen": _("für innen"),
        "für außen": _("für außen"),
        '"für seitlich und gegen Schwächlinge innen"': _(
            '"für seitlich und gegen Schwächlinge innen"'
        ),
        '"gegen seitlich und für Schwächlinge innen"': _(
            '"gegen seitlich und für Schwächlinge innen"'
        ),
        "für außen": _("für außen"),
    }
    spaltenNamen = OrderedDict(
        {
            "Transzendentalien, Strukturalien, Universum n": _(
                "Transzendentalien, Strukturalien, Universum n"
            ),
            "Galaxie n": _("Galaxie n"),
            "Galaxie 1/n": _("Galaxie 1/n"),
            "Transzendentalien, Strukturalien, Universum 1/n": _(
                "Transzendentalien, Strukturalien, Universum 1/n"
            ),
            "Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n": _(
                "Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n"
            ),
            "neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n": _(
                "neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n"
            ),
            "Richtung-Richtung": _("Richtung-Richtung"),
        }
    )

    primRicht = {"Primzahlwirkung (7, Richtung) ": _("Primzahlwirkung (7, Richtung) ")}

    letztEnd = {"] * letztendlich: ": _("] * letztendlich: ")}

    primVielGen = {
        "Primzahlvielfache, nicht generiert": _("Primzahlvielfache, nicht generiert")
    }
    GalOrUniOrFehler = {
        "Fehler": _("Fehler"),
        "Universum": _("Universum"),
        "Galaxie": _("Galaxie"),
        "Emotion": _("Emotion"),
        "Strukturgroesse": _("Strukturgroesse"),
    }

    multipl = {"Multiplikationen": _("Multiplikationen")}
    notGen = {"Nicht_generiert": _("Nicht_generiert")}


class lib4tables:
    zaehlung = {"Zählung": _("Zählung")}
    nummerier = {"Nummerierung": _("Nummerierung")}
    alles = {"alles": _("alles")}


class retapy:
    beschriebenWort = _("beschrieben")
    nichtsWort = _("nichts")
    cliout1Saetze = (
        _('Der Haupt-Parameter "'),
        _('" existiert hier nicht als Befehl!'),
        _(" Es ist nur möglich: -"),
    )

    keineNumWort = _("keinenummerierung")
    cliout2Saetze = (
        _('Der Unter-Paramaeter "--'),
        _('" existiert, aber nicht mit dem Textwert "'),
        _('". Mögliche Nebenparameter-Textwerte, für diesen Unter-Parameter, sind: "'),
        _('". Stattdessen gibt keine Nebenparameter-Textwerte.'),
    )
    cliout3Saetze = (
        _('Der Unter-Paramaeter "--'),
        _('" mit dem Textwert "'),
        _('" existiert hier nicht als Befehl für Haupt-Parameter'),
        " -" + hauptForNeben["spalten"],
        _(" !"),
        _(" Es ist nur möglich:\n--"),
        "".join((", --", ausgabeParas["breiten"], " --", ausgabeParas["breite"])),
        _("\nmit dem Werten dahinter:\n"),
    )
    cliout4Saetze = (
        _('Der Unter-Parameter "--'),
        _('" existiert hier nicht als Befehl für Haupt-Parameter'),
        " -" + hauptForNeben["spalten"],
        _(
            ", oder dieser Parameter braucht Werte analog wie: \n--unterParameter=Wert1\n"
        ),
        _("Es ist nur möglich: --"),
        ", --" + ausgabeParas["keinenummerierung"],
    )
    kombinationenWort = _("kombinationen")
    cliout5Saetze = (
        _('Die Kombispalte "'),
        _('" existiert so nicht als Befehl. Möglich sind die Parameter für '),
    )
    cliout6Satz = "".join(
        (
            _("kein Unter-Parameter"),
            "--",
            kombiMainParas["galaxie"],
            '= ",',
            _(", oder"),
            ', "--',
            kombiMainParas["universum"],
            '=", ',
            _("angegeben für Hauptparameter"),
            " -" + hauptForNeben["kombination"],
            _(" oder einen nicht zugehörigen Parameter: "),
        )
    )
    cliout7Saetze = (
        _("Es muss ein Hauptparameter, bzw. der richtige, gesetzt sein, damit ein"),
        _(' Nebenparameter, wie möglicherweise: "'),
        _('" ausgeführt werden kann. Hauptparameter sind: -'),
    )
    # breiteParameterWort = _("breite")

    cliout8SatzVersucheParaH = _("Versuche Parameter -h")
    cliout9Saetze = (
        _('Den Neben-Parameter "'),
        _('" gibt es hier nicht für den Hauptparameter "-'),
        _('". Oder ein = fehlt dahinter.'),
        _(" Möglich sind: "),
    )
    cliout10Saetze = (
        _('Den Neben-Parameter "'),
        _('" gibt es hier nicht für den Hauptparameter "-'),
        _('". Oder es fehlt ein = dahinter.'),
    )


class nested:
    galWort = kombiMainParas["galaxie"]
    uniWort = kombiMainParas["universum"]
    artWort = ausgabeParas["art"]
    zeitWort = zeilenParas["zeit"]
    typWort = zeilenParas["typ"]


class retaPrompt:
    infoDebugAktiv = _("Debug Log aktiviert.")
    abstandMeldung = (
        _("der Befehl '")
        + befehle2["abstand"]
        + _(
            "' verlangt mindestens 2 Zahlenangaben, wie '"
        )
        + befehle2["abstand"]
        + " 7 17-25'"
    )
    befehleBeenden = {_("ende"), _("exit"), _("quit"), _("q"), _(":q")}
    befehleWort = {"Befehle": _("Befehle"), "Kurzbefehle": _("Kurzbefehle")}
    promptModeSatz = _("promptmode vorher: {} , {}")
    promptModeSatz2 = _("{}{}{} ergibt sich aus '{}' und ergibt danach reta-Befehl:")
    out1Saetze = (
        _("Dies ('"),
        _(
            "') ist tatsächlich ein Befehl (oder es sind mehrere), aber es gibt nichts auszugeben."
        ),
    )
    out2Satz = _("Das ist kein Befehl! -> '{}'")
    out3Saetze = _(
        'Wenn im Zähler oder Nenner eine 1 ist, so werden davon oft (nicht immer) keine Vielfacher gebildet.\nFür Brüche "n/1=ganze Zahl" gibt es die gewöhnlichen Befehle für ganze Zahlen.\nDas ist eine Design-Entscheidung, die getroffen worden ist.'
    )
    replacements = {
        befehle2["e"]: befehle2[
            "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
        ],
        befehle2["G"]: geistWort,
        befehle2["R"]: befehle2["range"],
        befehle2["a"]: befehle2["absicht"],
        befehle2["B"]: befehle2["bewusstsein"],
        befehle2["E"]: befehle2["emotion"],
        befehle2["u"]: befehle2["universum"],
        befehle2["I"]: befehle2["impulse"],
        befehle2["T"]: befehle2["triebe"],
        befehle2["t"]: befehle2["thomas"],
        befehle2["r"]: befehle2["richtung"],
        befehle2["v"]: befehle2["vielfache"],
        befehle2["h"]: befehle2["help"],
        befehle2["w"]: befehle2["teiler"],
        befehle2["S"]: befehle2["BefehlSpeichernDanach"],
        befehle2["s"]: befehle2["BefehlSpeichernDavor"],
        befehle2["l"]: befehle2["BefehlSpeicherungLöschen"],
        befehle2["o"]: befehle2["BefehlSpeicherungAusgeben"],
        befehle2["W"]: befehle2["wirklichkeit"],
    }
    retaPromptParameter = {
        "vi": _("vi"),
        "log": _("log"),
        "h": _("h"),
        "help": _("help"),
        "e": _("e"),
        "debug": _("debug"),
        "befehl": _("befehl"),
    }

    debugLog = _("Debug Log aktiviert.")
    helptext = "".join(
        (
            _(
                """Erlaube Parameter sind
            -"""
            ),
            retaPromptParameter["vi"],
            _(
                """, für vi mode statt emacs mode,
            -"""
            ),
            "language=",
            _(""",  um eine andere Sprache zu wählen und möglich sind: """),
            str([s for s in sprachen.keys() if s.strip() != ""])[1:-1],
            """
            -""",
            retaPromptParameter["log"],
            _(
                """,  um Logging zu aktivieren,
            -"""
            ),
            retaPromptParameter["debug"],
            _(
                """, um Debugging-Log-Ausgabe zu aktivieren. Das ist nur für Entwickler gedacht.
            -"""
            ),
            retaPromptParameter["befehl"],
            _(
                """ bewirkt, dass bis zum letzten Programmparameter retaPrompt Befehl nur ein RetaPrompt-Befehl ausgeführt wird.
            -"""
            ),
            retaPromptParameter["e"],
            _(" bewirkt, dass bei allen Befehlen das '"),
            befehle2["e"],
            _("' Kommando bzw. '"),
            befehle2["keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"],
            _(
                "' jedes mal verwendet wird - außer wenn der erste Befehl reta war, weil dieser anders funktioniert "
            ),
        )
    )

    wspeichernWort = _("was speichern>")
    wloeschenWort = _("was löschen>")
    reziInfoText = _(
        'Wenn im Zähler oder Nenner eine 1 ist, so werden davon oft (nicht immer) keine Vielfacher gebildet.\nFür Brüche "n/1=ganze Zahl" gibt es die gewöhnlichen Befehle für ganze Zahlen.\nDas ist eine Design-Entscheidung, die getroffen worden ist.'
    )

hauptForNeben: dict = {
    "zeilen": _("zeilen"),
    "spalten": _("spalten"),
    "kombination": _("kombination"),
    "ausgabe": _("ausgabe"),
    "h": _("h"),
    "help": _("help"),
    "debug": _("debug"),
    "nichts": _("nichts"),
}


mainParaCmds: dict = hauptForNeben

# @dataclass
class csvFileNames:
    kombi13 = _("kombi.csv")
    kombi15 = _("kombi-meta.csv")
    religion = _("religion.csv")
    prim = _("primenumbers.csv")
    bruch15 = _("gebrochen-rational-universum.csv")
    bruch13 = _("gebrochen-rational-galaxie.csv")
    bruch7 = _("gebrochen-rational-emotionen.csv")
    burchGroesse = _("gebrochen-rational-strukturgroesse.csv")
    kombi_17_13_15 = _("kombi-gedanken17-absichten13-bewusstsein15.csv")
    kombi_11_15 = _("kombi-meta-systeme.csv")
    kombi_10_15 = _("kombi-universelle-wirklichkeit.csv")
    kreis18 = _("kreisVomTyp18.csv")
    sunMoon = _("sunMoonEtc.csv")
    meaningOfLife = _("meaningOfLife.csv")
    dualitaetenTrinities = _("dualism-trinities-etc.csv")
    bruch7 = _("gebrochen-rational-emotionen.csv")
    bruchStrukGroesse = _("gebrochen-rational-strukturgroesse.csv")


EIGS_N_R = (_("EIGN"), _("EIGR"))


class readMeFileNames:
    reta = _("readme-reta.md")
    retaPrompt = _("readme-retaPrompt.md")
    startFiles = _("readme-startFiles.md")
    developer = _("readme.org")


wrongLangSentence = (
    _("für '-languages=' sind die Paramter-Werte erlaubt: ")
    + str(tuple(sprachen.values()))[1:-1]
)

tomDecodedMotivesLang = {"kr": "kr-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv", "vn": "vn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv", "cn": "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv"}


__all__ = [name for name in globals() if not name.startswith('__')]
