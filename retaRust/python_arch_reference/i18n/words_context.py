# -*- coding: utf-8 -*-

"""Context/schema layer extracted from the legacy words monolith."""

from .words_bootstrap import *

sprachen: defaultdict = defaultdict(lambda: "de")
sprachen["english"] = "en"
sprachen["englisch"] = "en"
sprachen["deutsch"] = "de"
sprachen["german"] = "de"
sprachen["vietnamesisch"] = "vn"
sprachen["vietnamese"] = "vn"
sprachen["tiếngviệt"] = "vn"
sprachen["chinesisch"] = "cn"
sprachen["chinese"] = "cn"
sprachen["中國人"] = "cn"
sprachen["koreanisch"] = "kr"
sprachen["korean"] = "kr"
sprachen["한국인"] = "kr"

sprachen2: defaultdict = defaultdict(lambda: "messages")
sprachen2["english"] = "messages"
sprachen2["englisch"] = "messages"
sprachen2["deutsch"] = "messages"
sprachen2["german"] = "messages"
sprachen2["vietnamesisch"] = "vn"
sprachen2["vietnamese"] = "vn"
sprachen2["tiếngviệt"] = "vn"
sprachen2["chinesisch"] = "cn"
sprachen2["chinese"] = "cn"
sprachen2["中國人"] = "cn"
sprachen2["koreanisch"] = "kr"
sprachen2["korean"] = "kr"
sprachen2["한국인"] = "kr"


sprachenWahl = ""
sprachenParameterWort = "-language="

flagS = False
for arg in sys.argv:
    if arg[: len(sprachenParameterWort)] == sprachenParameterWort:
        sprachenWahl = arg[len(sprachenParameterWort) :]
        flagS = True
        break
if "-debug" in sys.argv:
    print("Sprachenwahl: {}".format(sprachenWahl))
if flagS and sprachenWahl not in sprachen.keys():
    print(
        "allowed are: {}\nwrong: {}".format(
            str(tuple(sprachen.keys()))[1:-1], sprachenWahl
        )
    )

if len({"deutsch", "german", ""} & {sprachenWahl}) == 0:
    alxp("not german")
    subFolder = sprachen[sprachenWahl]
    sprachenFileName = sprachen2[sprachenWahl]
    i18nPath = os.path.join(os.path.dirname(__file__))
    t = gettext.translation(
        sprachenFileName, localedir=i18nPath, languages=[subFolder], fallback=False
    )
    t.install()
    _ = t.gettext
else:
    alxp("german")
    localedir = os.path.join(os.path.abspath(os.path.dirname(__file__)))
    translate = gettext.translation("nichts", localedir, fallback=True)
    _ = translate.gettext

# sys.path.insert(1, "./..")
Multiplikationen = [(_("Multiplikationen"), "")]
"""
ES FEHLEN NOCH ALLE ''
fertig: in prepare ist nichts
fertig: concat fertig
fertig: center fertig
fertig: lib4tables fertig
fertig: reta.py fertig
nichts drin: enum
nichts drin: multis
nichts drin: grundstruk html
die aus den anderen dateien: nestedcompleter
LibRetaPrompt: größten Teil entnommen

ES FEHLEN NOCH ALLE ''
"""

netzwerkWort = _("netzwerk")
Primzahlkreuz_pro_contra_strs: tuple = (
    "Primzahlkreuz_pro_contra",
    "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)",
)

keineTabellenAusgabe = _("ein Mal kein Tabelleninhalt")
Primzahlkreuz_pro_contra_strs_Fkt: tuple = (
    _("Primzahlkreuz_pro_contra"),
    _("nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)"),
)
Primzahlkreuz_pro_contra_strs_Dict = {
    Primzahlkreuz_pro_contra_strs: Primzahlkreuz_pro_contra_strs_Fkt,
}
gebrochenSpaltenMaximumPlus1: int = 24  # Das ist nicht die Spaltenbreite, sondern wie weit gebrochene Zahlen gehen dürfen bei Zähler und Nenner

# DOPPELT
# spalten: dict = {}
# spalten |= {
#    "breite": _("breite"),
#    "breiten": _("breiten"),
#    "keinenummerierung": _("keinenummerierung"),
# }

primzahlWort = _("Primzahl")

geistWort = _("geist")
emotionWort = _("emotion")
ausgabeParas: dict = {
    "nocolor": _("nocolor"),
    "justtext": _("justtext"),
    "art": _("art"),
    "onetable": _("onetable"),
    "spaltenreihenfolgeundnurdiese": _("spaltenreihenfolgeundnurdiese"),
    "endlessscreen": _("endlessscreen"),
    "endless": _("endless"),
    "dontwrap": _("dontwrap"),
    "breite": _("breite"),
    "breiten": _("breiten"),
    "keineleereninhalte": _("keineleereninhalte"),
    "keinenummerierung": _("keinenummerierung"),
    "keineueberschriften": _("keineueberschriften"),
}
ausgabeParasEqSign: dict = {
    "nocolor": False,
    "justtext": False,
    "art": True,
    "onetable": False,
    "spaltenreihenfolgeundnurdiese": True,
    "endlessscreen": False,
    "endless": False,
    "dontwrap": False,
    "breite": True,
    "breiten": True,
    "keineleereninhalte": False,
    "keinenummerierung": False,
    "keineueberschriften": False,
}

ausgabeParasLen = {key: len(value) for (key, value) in ausgabeParas.items()}
kombiMainParas: dict = {
    "galaxie": _("galaxie"),
    "universum": _("universum"),
}
zeilenParas: dict = {
    "alles": _("alles"),
    "gestern": _("gestern"),
    "heute": _("heute"),
    "hoehemaximal": _("hoehemaximal"),
    "mond": _("mond"),
    "morgen": _("morgen"),
    "nachtraeglichneuabzaehlung": _("nachtraeglichneuabzaehlung"),
    "nachtraeglichneuabzaehlungvielfache": _("nachtraeglichneuabzaehlungvielfache"),
    "oberesmaximum": _("oberesmaximum"),
    "planet": _("planet"),
    "potenzenvonzahlen": _("potenzenvonzahlen"),
    "primzahlvielfache": _("primzahlvielfache"),
    "schwarzesonne": _("schwarzesonne"),
    "sonne": _("sonne"),
    "typ": _("typ"),
    "vielfachevonzahlen": _("vielfachevonzahlen"),
    "vorhervonausschnitt": _("vorhervonausschnitt"),
    "vorhervonausschnittteiler": _("vorhervonausschnittteiler"),
    "zaehlung": _("zaehlung"),
    "zeit": _("zeit"),
    "primzahlen": _("primzahlen"),
    "aussenerste": _("aussenerste"),
    "innenerste": _("innenerste"),
    "aussenalle": _("aussenalle"),
    "innenalle": _("innenalle"),
    "invertieren": _("invertieren"),
    "SonneMitMondanteil": _("SonneMitMondanteil"),
}

zeilenParasLen = {key: len(value) for (key, value) in zeilenParas.items()}
# zeilenParasLenPlus2 = {key: len(value) + 2 for (key, value) in zeilenParas.items()}
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

ausgabeArt: dict = {
    "bbcode": _("bbcode"),
    "html": _("html"),
    "csv": _("csv"),
    "shell": _("shell"),
    "markdown": _("markdown"),
    "emacs": _("emacs"),
    "nichts": _("nichts"),
}
# ausgabeArt2 = {value: key for key, value in ausgabeArt}

wahl16Words: dict = {
    "Multiversalien_(16)": _("Multiversalien_(16)"),
    "P": _("P"),
    # "Meta-Physik-Teilchen_(1)": _("Meta-Physik-Teilchen_(1)"),
}

wahl15Words: dict = {
    "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,"
    + Primzahlkreuz_pro_contra_strs[1]: ",".join(
        (
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Geist_(15)"),
            _("Model_of_Hierarchical_Complexity),"),
            Primzahlkreuz_pro_contra_strs_Fkt[1],
        ),
    ),
    "Konkreta_und_Focus_(2)": _("Konkreta_und_Focus_(2)"),
    "Impulse_(5)": _("Impulse_(5)"),
    "Gefühle_(7)": _("Gefühle_(7)"),
    "Modus_und_Sein_(8)": _("Modus_und_Sein_(8)"),
    "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)": _(
        "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)"
    ),
    "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12": ",".join(
        (("Meta-Systeme_(12)"), _("Ordnung_und_Filterung_12_und_1pro12"))
    ),
    "Paradigmen_sind_Absichten_(13)": _("Paradigmen_sind_Absichten_(13)"),
    "Gedanken_sind_Positionen_(17)": _("Gedanken_sind_Positionen_(17)"),
    "Verbundenheiten_(18)": _("Verbundenheiten_(18)"),
    "Triebe_und_Bedürfnisse_(6)": _("Triebe_und_Bedürfnisse_(6)"),
    "Lust_(9)": _("Lust_(9)"),
    "Reflexe_(3),Existenzialien_(3)": ",".join(
        (_("Reflexe_(3)"), _("Existenzialien_(3)"))
    ),
    "Absicht_6_ist_Vorteilsmaximierung": _("Absicht_6_ist_Vorteilsmaximierung"),
    "Absicht_7_ist_Selbstlosigkeit": _("Absicht_7_ist_Selbstlosigkeit"),
    "Absicht_10_ist_Wirklichkeit_erkennen": _("Absicht_10_ist_Wirklichkeit_erkennen"),
    "Absicht_17_ist_zu_meinen": _("Absicht_17_ist_zu_meinen"),
    "Zeit_(4)_als_Wirklichkeit": _("Zeit_(4)_als_Wirklichkeit"),
    "Funktionen_Vorstellungen_(16)": _("Funktionen_Vorstellungen_(16)"),
    "Achtung_(4)": _("Achtung_(4)"),
    "Absicht_1/8": _("Absicht_1/8"),
    "Absicht_1/6_ist_Reinigung_und_Klarheit": _(
        "Absicht_1/6_ist_Reinigung_und_Klarheit"
    ),
    "Reflektion_und_Kategorien_(1/15)": _("Reflektion_und_Kategorien_(1/15)"),
    "Bewusstheit_statt_Bewusstsein_(1)": _("Bewusstheit_statt_Bewusstsein_(1)"),
    "Energie_und_universelle_Eigenschaften_(30)": _(
        "Energie_und_universelle_Eigenschaften_(30)"
    ),
    "Stimmungen_Kombinationen_(14)": _("Stimmungen_Kombinationen_(14)"),
    "Klassen_(20)": _("Klassen_(20)"),
    "Empathie_(37)": _("Empathie_(37)"),
    "Garben_und_Verhalten_nachfühlen(31)": _("Garben_und_Verhalten_nachfühlen(31)"),
    "Verhalten_(11)": _("Verhalten_(11)"),
    "Bedeutung_(10)": _("Bedeutung_(10)"),
    "Themen_(6)": _("Themen_(6)"),
    "Optimierung_(10)": _("Optimierung_(10)"),
    "Attraktionen_(36)": _("Attraktionen_(36)"),
    "Absicht_16_ist_zu_genügen": _("Absicht_16_ist_zu_genügen"),
    "Liebe_(7)": _("Liebe_(7)"),
    "Koalitionen_(10)": _("Koalitionen_(10)"),
    "Ansichten_Standpunkte_(18_17)": _("Ansichten_Standpunkte_(18_17)"),
    "Prinzipien(1/8)": _("Prinzipien(1/8)"),
    "Bestrebungen(1/5)": _("Bestrebungen(1/5)"),
    "Bedingung_und_Auslöser_(1/3)": _("Bedingung_und_Auslöser_(1/3)"),
    "relativer_Zeit-Betrag_(15_10_4_18_6)": _("relativer_Zeit-Betrag_(15_10_4_18_6)"),
    "Zahlenvergleich_(15_18_6)": _("Zahlenvergleich_(15_18_6)"),
    "Leidenschaften_(21)": _("Leidenschaften_(21)"),
    "Erwartungshaltungen_(26)": _("Erwartungshaltungen_(26)"),
    "Extremalien_(19),Ziele_(19)": ",".join((_("Extremalien_(19)"), _("Ziele_(19)"))),
    "universeller_Komperativ_(18→15)": _("universeller_Komperativ_(18→15)"),
    "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)": _(
        "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)"
    ),
    "Sollen_Frage_Vorgehensweise_(1/13)": _("Sollen_Frage_Vorgehensweise_(1/13)"),
    "Fundament_(1/19)": _("Fundament_(1/19)"),
    "abhängige_Verbundenheit_(90)": _("abhängige_Verbundenheit_(90)"),
    "Absicht_13_ist_Helfen": _("Absicht_13_ist_Helfen"),
    "Karte_Filter_und_Unterscheidung_(1/12)": _(
        "Karte_Filter_und_Unterscheidung_(1/12)"
    ),
    "Maßnahmen_39": _("Maßnahmen_(39)"),
}

kugelnKreise = ["kugeln", "kreise"]
ParametersMain: NamedTuple = namedtuple(
    "ParametersMain",
    "wichtigste wichtigste2 religionen galaxie strukturgroesse universum multiversum wirtschaft menschliches procontra licht bedeutung symbole Multiplikationen konzept konzept2 inkrementieren operationen universummetakonkret primzahlwirkung gebrochenuniversum gebrochengalaxie gebrochenemotion gebrochengroesse primvielfache planet strukturenkleinere grundstrukturen teilchen kontinuum herrschaft metachemiemetametaphysik alles",
)

konzeptE = {"konzept": _("konzept"), "konzept2": _("konzept2")}
gebrochenUniGal = {
    "gebrochenuniversum": (
        _("gebrochen-rational_Universum_n/m"),
        _("gebrochenuniversum"),
    ),
    "gebrochengalaxie": (_("gebrochen-rational_Galaxie_n/m"), _("gebrochengalaxie")),
    "gebrochenemotion": (_("gebrochen-rational_Gefuehle_n/m"), _("gebrochenemotion")),
    "gebrochengroesse": (
        _("gebrochen-rational_Strukturgroesse_n/m"),
        _("gebrochengroesse"),
    ),
}
gebrochenUniGalEinzeln = {b for a in gebrochenUniGal.values() for b in a}
ParametersMain: NamedTuple = ParametersMain(
    (
        _("Wichtigstes_zum_verstehen"),
        _("wichtigsteverstehen"),
    ),
    (
        _("Wichtigstes_zum_gedanklich_einordnen"),
        _("wichtigsteeinordnen"),
    ),
    (
        _("Religionen"),
        _("religionen"),
        _("religion"),
    ),
    (
        _("Galaxie"),
        _("galaxie"),
        _("alteschriften"),
        _("kreis"),
        _("galaxien"),
        _("kreise"),
    ),
    (
        _("Größenordnung"),
        _("groessenordnung"),
        _("strukturgroesse"),
        _("strukturgroeße"),
        _("strukturgrösse"),
        _("strukturgröße"),
        _("groesse"),
        _("stufe"),
        _("organisationen"),
    ),
    (
        _("Universum"),
        _("universum"),
        _("transzendentalien"),
        _("strukturalien"),
        _("kugel"),
        _("kugeln"),
        _("ball"),
        _("baelle"),
        _("bälle"),
    ),
    (_("Multiversum"), _("multiversum")),
    (_("Wirtschaft"), _("wirtschaft")),
    (
        _("Menschliches"),
        _("menschliches"),
    ),
    (
        _("Pro_Contra"),
        _("procontra"),
        _("dagegendafuer"),
    ),
    (
        _("Licht"),
        _("licht"),
    ),
    (
        _("Bedeutung"),
        _("bedeutung"),
    ),
    (
        _("Symbole"),
        _("symbole"),
    ),
    [a[0] for a in Multiplikationen],
    (
        _("Eigenschaften_n"),
        _("eigenschaften"),
        _("eigenschaft"),
        konzeptE["konzept"],
        _("konzepte"),
    ),
    (
        _("Eigenschaften_1/n"),
        konzeptE["konzept2"],
        _("konzepte2"),
    ),
    (
        _("Inkrementieren"),
        _("inkrementieren"),
    ),
    (
        _("Operationen"),
        _("operationen"),
    ),
    (
        _("Meta_vs_Konkret_(Universum)"),
        _("universummetakonkret"),
    ),
    (
        _("Primzahlwirkung"),
        _("primzahlwirkung"),
    ),
    gebrochenUniGal["gebrochenuniversum"],
    gebrochenUniGal["gebrochengalaxie"],
    gebrochenUniGal["gebrochenemotion"],
    gebrochenUniGal["gebrochengroesse"],
    (
        _("Multiplikationen"),
        _("multiplikationen"),
    ),
    (_("Planet_(10_und_oder_12)"), _("planet")),
    (
        _("Strukturen_1_bis_9"),
        _("strukturkleinerzehn"),
    ),
    (_("Grundstrukturen"), _("grundstrukturen")),
    (_("Teilchen-Meta-Physik"), _("teilchen")),
    (_("Kontinuum"), _("kontinuum")),
    (_("Herrschaft"), _("macht")),
    (_("MetaMetaPhysik_und_MetaChemie"),),
    (_("alles"),),
)

wahl15: dict = {
    #    "_": _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15)"),
    "15": ",".join(
        (
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Geist_(15)"),
            _("Model_of_Hierarchical_Complexity"),
            _("Biologischer_Baum_(15)"),
            _("Teilchen_anderes_Universum"),
            Primzahlkreuz_pro_contra_strs_Fkt[1],
        )
    ),
    "2": _("Konkreta_und_Focus_(2)"),
    "5": _("Impulse_(5)"),
    "7": ",".join((_("Gefühle_(7)"), _("Anführer_Arten_(7)"), _("Erlösung"))),
    "8": ",".join((_("Modus_und_Sein_(8)"), _("Bestrafung"),_("Gewalt"))),
    "10": _("Wirklichkeiten_Wahrheit_Wahrnehmung_(10)"),
    "1"+_("pro")+"30": _("analytische_Ontologie"),
    "12": ",".join((_("Meta-Systeme_(12)"), _("Ordnung_und_Filterung_12_und_1pro12"))),
    "13": _("Paradigmen_sind_Absichten_(13)"),
    "17": _("Gedanken_sind_Positionen_(17)"),
    "18": _("Verbundenheiten_(18)"),
    "6": ",".join((_("Triebe_und_Bedürfnisse_(6)"), _("System"))),
    "9": _("Lust_(9)"),
    "3": _("Reflexe_(3),Existenzialien_(3)"),
    "13_6": _("Absicht_6_ist_Vorteilsmaximierung"),
    "13_7": _("Absicht_7_ist_Selbstlosigkeit"),
    "13_10": _("Absicht_10_ist_Wirklichkeit_erkennen"),
    "13_17": _("Absicht_17_ist_zu_meinen"),
    "10_4": _("Zeit_(4)_als_Wirklichkeit"),
    "16": _("Funktionen_Vorstellungen_(16)"),
    "4": _("Achtung_(4)"),
    "13_1"+_("pro")+"8": _("Absicht_1/8"),
    "13_1"+_("pro")+"6": _("Absicht_1/6_ist_Reinigung_und_Klarheit"),
    "1"+_("pro")+"15": _("Reflektion_und_Kategorien_(1/15)"),
    "1": _("Bewusstheit_statt_Bewusstsein_(1)"),
    "30": _("Energie_und_universelle_Eigenschaften_(30)"),
    "14": _("Stimmungen_Kombinationen_(14)"),
    "14_6": _("Rechnen"),
    "20": _("Klassen_(20)"),
    "37": _("Empathie_(37)"),
    "31": _("Garben_und_Verhalten_nachfühlen(31)"),
    "11": _("Verhalten_(11)"),
    "5_10": _("Bedeutung_(10)"),
    "17_6": _("Themen_(6)"),
    "17_6_10"+_("mit")+"4": _("Optimierung_(10)"),
    "36": _("Attraktionen_(36)"),
    "13_16": _("Absicht_16_ist_zu_genügen"),
    "18_7": _("Liebe_(7)"),
    "18_10": _("Koalitionen_(10)"),
    "18_17": _("Ansichten_Standpunkte_(18_17)"),
    "1"+_("pro")+"8": _("Prinzipien(1/8)"),
    "1"+_("pro")+"5": _("Bestrebungen(1/5)"),
    "1"+_("pro")+"3": _("Bedingung_und_Auslöser_(1/3)"),
    "10_4_18_6": _("relativer_Zeit-Betrag_(15_10_4_18_6)"),
    "18_6": _("Zahlenvergleich_(15_18_6)"),
    "21": _("Leidenschaften_(21)"),
    "26": _("Erwartungshaltungen_(26)"),
    "19": _("Extremalien_(19),Ziele_(19)"),
    "18_15": _("universeller_Komperativ_(18→15)"),
    "18_15_n-vs-1"+_("pro")+"n": _("Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)"),
    "1"+_("pro")+"13": _("Sollen_Frage_Vorgehensweise_(1/13)"),
    "1"+_("pro")+"19": _("Fundament_(1/19)"),
    "90": _("abhängige_Verbundenheit_(90)"),
    "13_13": _("Absicht_13_ist_Helfen"),
    "1"+_("pro")+"12": _("Karte_Filter_und_Unterscheidung_(1/12)"),
    "39": _("Maßnahmen_(39)"),
    "1"+_("pro")+"6": _("innere_Werte_1/6_der_Reinigung_und_Klarheit"),
    "28": _("Lebensbereiche_Problemklassen_(28)"),
    "24": _("Netzwerk"),
    "32": _("mathematisches_Design_(32)"),
    _("gegen")+"5": _("gegen_5"),
    "9_6": ParametersMain.strukturgroesse[0],
    "51": _("Kontroverse_(51)"),
    "13_4": _("Taetigkeiten"),
    "7"+_("mit")+"6": _("Wohlbefinden_(7mit6)"),

}

wahl16 = {
    "1":  _("Meta-Physik-Teilchen_(1)"),
    "2": ",".join((
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Model_of_Hierarchical_Complexity"),)),

    "3":  _("Teilchen_anderes_Universum"),
    "5": ",".join((
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Model_of_Hierarchical_Complexity"),_("Biologischer_Baum_(16_->_5)"), _("P5"),)),
    "6": _("Geist_(15)"),
    "15": ",".join((
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Model_of_Hierarchical_Complexity"),)),
    "10": _("Struktur-Wissenschaften_(10)"),
    "16": ",".join(wahl16Words.values()),
    "20": _("Muster-Wissenschaften_(20)"),
}

freiheitGleichheit = ("freiheit", "gleichheit")

gemeinsamkeitenWort = _("Gemeinsamkeiten")

# WICHTIG WICHTIG: die Befehle mit nur einem zeichen dürfen  nur ein Zeichen haben !!!!!!!
befehle2: OrderedDict = OrderedDict({"15_" + a: "15_" + a for a in wahl15.keys()})
befehle2.update({"16_15_" + a: "16_15_" + a for a in wahl15.keys() if a != "15"})
#befehle2.update({"16_10_" + a: "16_15_" + a for a in wahl15.keys() if a != "15"})
#befehle2.update({"16_20_" + a: "16_15_" + a for a in wahl15.keys() if a != "15"})
            #_("Struktur-Wissenschaften_(10)"),
            #_("Muster-Wissenschaften_(20)"),

befehle2.update({"16_" + a: "16_" + a for a in wahl16.keys()})
befehle2.update(
    {
        "invertieren" : _("invertieren"),
        "netzwerk": netzwerkWort,
        "komplex": _("komplex"),
        "ee": _("ee"),
        "groesse": _("groesse"),
        "emotion": emotionWort,
        freiheitGleichheit[0]: _(freiheitGleichheit[0]),
        freiheitGleichheit[1]: _(freiheitGleichheit[1]),
        "kurzbefehle": _("kurzbefehle"),
        "leeren": _("leeren"),
        "kugeln": _("kugeln"),
        "kreise": _("kreise"),
        "mond": _("mond"),
        "reta": _("reta"),
        "absicht": _("absicht"),
        "motiv": _("motiv"),
        "thomas": _("thomas"),
        "universum": _("universum"),
        "impulse": _("impulse"),
        "motive": _("motive"),
        "absichten": _("absichten"),
        "primfaktorenvergleich": _("primfaktorenvergleich"),
        "vielfache": _("vielfache"),
        "einzeln": _("einzeln"),
        "multis": _("multis"),
        "multis3": _("multis3"),
        "modulo": _("modulo"),
        "prim": _("prim"),
        "primfaktorzerlegung": _("primfaktorzerlegung"),
        "prim24": _("prim24"),
        "primfaktorzerlegungModulo24": _("primfaktorzerlegungModulo24"),
        "help": _("HELP"),
        "hilfe": _("hilfe"),
        "abc": _("abc"),
        "abcd": _("abcd"),
        "alles": _("alles"),
        "geist": geistWort,
        "a": _("a"),
        "R": _("R"),
        "range": _("range"),
        "B": _("B"),
        "bewusstsein": _("bewusstsein"),
        "E": _("E"),
        "G": _("G"),
        "u": _("u"),
        "I": _("I"),
        "T": _("T"),
        "W": _("W"),
        "wirklichkeit": _("wirklichkeit"),
        "triebe": _("triebe"),
        "befehle": _("befehle"),
        "t": _("t"),
        "richtung": _("richtung"),
        "r": _("r"),
        "v": _("v"),
        "h": _("h"),
        "p": _("p"),
        "primzahlkreuz": _("primzahlkreuz"),
        "ende": _("ende"),
        "exit": _("exit"),
        "quit": _("quit"),
        "q": _("q"),
        ":q": _(":q"),
        "shell": _("shell"),
        "s": _("s"),
        "math": _("math"),
        "loggen": _("loggen"),
        "nichtloggen": _("nichtloggen"),
        "mulpri": _("mulpri"),
        "python": _("python"),
        "w": _("w"),
        "teiler": _("teiler"),
        "BefehlSpeichernDanach": _("BefehlSpeichernDanach"),
        "S": _("S"),
        "BefehlSpeicherungLöschen": _("BefehlSpeicherungLöschen"),
        "l": _("l"),
        "BefehlSpeicherungAusgeben": _("BefehlSpeicherungAusgeben"),
        "o": _("o"),
        "e": _("e"),
        # "BefehlsSpeicherungsModusAus": _("BefehlsSpeicherungsModusAus"),
        # "x": _("x"),
        "BefehlSpeichernDavor": _("BefehlSpeichernDavor"),
        "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar": _(
            "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
        ),
        "abstand": _("abstand"),
        "abstandPrim": _("abstandPrim"),
    }
)
# KurzLangBefehle sind die Befehle, die mehr als ein Zeichen groß sind und für reta dennoch Abkürzungen sind.

# KurzKurzBefehle müssen auch in Fremdsprachen ein Zeichen groß bleiben!
assert all(
    [len(value) == 1 if len(key) == 1 else True for (key, value) in befehle2.items()]
)


def finde_mehrfache_vorkommen(stringliste):
    # Ein Dictionary erstellen, um die Häufigkeit jedes Strings zu zählen
    haeufigkeiten = {}
    for string in stringliste:
        if string in haeufigkeiten:
            haeufigkeiten[string] += 1
        else:
            haeufigkeiten[string] = 1

    # Einträge filtern, deren Häufigkeit größer als 1 ist
    mehrfach_vorkommende_strings = [
        string for string, haeufigkeit in haeufigkeiten.items() if haeufigkeit > 1
    ]

    return mehrfach_vorkommende_strings


assert len(befehle2.keys()) == len(set(befehle2.keys()))
if len(befehle2.values()) != len(set(befehle2.values())):
    print(finde_mehrfache_vorkommen(befehle2.values()))
assert len(befehle2.values()) == len(set(befehle2.values()))

# WICHTIG WICHTIG: die Befehle mit nur einem zeichen dürfen  nur ein Zeichen haben !!!!!!!
befehle: list = list(befehle2.values())
# ["15" + a for a in wahl15.keys()] + [
#    _("mond"),
#    _("reta"),
#    _("absicht"),
#    _("motiv"),
#    _("thomas"),
#    _("universum"),
#    _("motive"),
#    _("absichten"),
#    _("vielfache"),
#    _("einzeln"),
#    _("multis"),
#    _("modulo"),
#    _("prim"),
#    _("primfaktorzerlegung"),
#    _("prim24"),
#    _("primfaktorzerlegungModulo24"),
#    _("help"),
#    _("hilfe"),
#    _("abc"),
#    _("abcd"),
#    _("alles"),
#    _("a"),
#    _("u"),
#    _("befehle"),
#    _("t"),
#    _("richtung"),
#    _("r"),
#    _("v"),
#    _("h"),
#    _("p"),
#    _("mo"),
#    _("mu"),
#    _("primzahlkreuz"),
#    _("ende"),
#    _("exit"),
#    _("quit"),
#    _("q"),
#    ":q",
#    _("shell"),
#    _("s"),
#    _("math"),
#    _("loggen"),
#    _("nichtloggen"),
#    _("mulpri"),
#    _("python"),
#    _("w"),
#    _("teiler"),
#    _("BefehlSpeichernDanach"),
#    _("S"),
#    _("BefehlSpeicherungLöschen"),
#    _("l"),
#    _("BefehlSpeicherungAusgeben"),
#    _("o"),
#    _("e"),
#    # _("BefehlsSpeicherungsModusAus"),
#    # _("x"),
#    _("BefehlSpeichernDavor"),
#    _("keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"),
#    _("abstand"),
# ]


haupt2neben = {hauptForNeben["zeilen"]: zeilenParas,
               hauptForNeben["ausgabe"]: ausgabeParas,
               hauptForNeben["h"]: [],
               hauptForNeben["help"]: [],
               hauptForNeben["nichts"]: [],
               hauptForNeben["debug"]: [],
               hauptForNeben["spalten"]: [b for a in ParametersMain for b in a],
               hauptForNeben["kombination"]: kombiMainParas}

#haupt2nebenSpalten = {}


# x("ParametersMain", ParametersMain)

organisationWort = _("organisation")
thomasWort = _("thomas")
motivationWort = _("motivation")
komplexWort = _("komplex")
transzendentalienWort = _("transzendentalien")
transzendentaliereziprokeWort = _("transzendentaliereziproke")
verhaeltnisgleicherzahlWort = _("verhaeltnisgleicherzahl")
gestirnWort = _("gestirn")
primzahlkreuzWort = _("primzahlkreuz")
GalaxieabsichtWort = _("Galaxieabsicht")


__all__ = [name for name in globals() if not name.startswith('__')]
