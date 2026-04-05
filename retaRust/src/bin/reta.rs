use indexmap::IndexMap;
use std::collections::BTreeSet;
use reta_transcompilation_phase5_more_real::runtime::*;

#[allow(non_snake_case)]
pub struct Program {
    pub argv: Vec<String>,
    pub tables: TablesPlaceholder,
    pub breiteHasBeenOnceZero: bool,
    pub breiteORbreiten: bool,
    pub mainParaCmds: IndexMap<String, Option<i64>>,
    pub paraDict: IndexMap<(String, String), Vec<PyCollection>>,
    pub spaltenArtenKey_SpaltennummernValue: IndexMap<(usize, usize), BTreeSet<PyAtom>>,
    pub i18n: I18nPlaceholder,
    pub kombiReverseDict: IndexMap<String, PyAtom>,
    pub kombiReverseDict2: IndexMap<String, PyAtom>,
}

#[allow(non_snake_case)]
impl Program {
    pub fn new(argv: Vec<String>) -> Self {
        let i18n = I18nPlaceholder::demo();
        Self {
            argv,
            tables: TablesPlaceholder::new(),
            breiteHasBeenOnceZero: false,
            breiteORbreiten: false,
            mainParaCmds: IndexMap::new(),
            paraDict: IndexMap::new(),
            spaltenArtenKey_SpaltennummernValue: IndexMap::new(),
            i18n,
            kombiReverseDict: IndexMap::new(),
            kombiReverseDict2: IndexMap::new(),
        }
    }

    pub fn ordered_distinct_first_items(&self) -> Vec<String> {
        let mut out: Vec<String> = vec![];
        for (k, _) in self.paraDict.keys() {
            if !out.iter().any(|x| x == k) {
                out.push(k.clone());
            }
        }
        out
    }

    pub fn ordered_distinct_second_items(&self) -> Vec<String> {
        let mut out: Vec<String> = vec![];
        for (_, v) in self.paraDict.keys() {
            if !out.iter().any(|x| x == v) {
                out.push(v.clone());
            }
        }
        out
    }

    pub fn resultingSpaltenFromTuple(
        &mut self,
        tupl: Vec<PyCollection>,
        neg: &str,
        paraValue: Option<String>,
        befehlName: Option<String>,
    ) {
        for (i, mut eineSpaltenArtmitSpaltenNummern) in tupl.into_iter().enumerate() {
            if eineSpaltenArtmitSpaltenNummern.is_list_or_tuple()
                && eineSpaltenArtmitSpaltenNummern.len() > 0
            {
                if let Some(converted) = eineSpaltenArtmitSpaltenNummern.to_set_if_first_bool_or_nested() {
                    eineSpaltenArtmitSpaltenNummern = PyCollection::Set(converted);
                }
            }

            let gebrochen_befehl = if i == 2 {
                match &befehlName {
                    Some(name) => matches!(
                        name.as_str(),
                        "Multiplikationen"
                            | "gebrochenuniversum"
                            | "gebrochenuniversum2"
                            | "gebrochengalaxie"
                            | "gebrochengalaxie2"
                            | "gebrochenemotion"
                            | "gebrochenemotion2"
                            | "gebrochengroesse"
                            | "gebrochengroesse2"
                    ),
                    None => false,
                }
            } else {
                false
            };

            if i == 2 && (eineSpaltenArtmitSpaltenNummern.is_list_or_tuple() || gebrochen_befehl) {
                panic!("UNTRANSCOMPILIERT: gebrochen-Branch in resultingSpaltenFromTuple");
            } else if paraValue.as_deref() == Some(self.i18n.beschrieben_wort.as_str())
                && matches!(befehlName.as_deref(), Some("primvielfache"))
            {
                let key = (neg.len(), 2usize);
                self.spaltenArtenKey_SpaltennummernValue
                    .entry(key)
                    .or_insert_with(BTreeSet::new)
                    .insert(PyAtom::Int(2));
            } else {
                if let Some(as_set) = eineSpaltenArtmitSpaltenNummern.as_set() {
                    let key = (neg.len(), i);
                    let target = self.spaltenArtenKey_SpaltennummernValue.entry(key).or_insert_with(BTreeSet::new);
                    for x in as_set {
                        target.insert(x);
                    }
                }
            }
        }
    }

    pub fn spalten_removeDoublesNthenRemoveOneFromAnother(&mut self) {
        let half_len = self.spaltenArtenKey_SpaltennummernValue.len() / 2;
        for el2Type in 0..half_len {
            let left_key = (0usize, el2Type);
            let right_key = (1usize, el2Type);
            let left_now = self.spaltenArtenKey_SpaltennummernValue.get(&left_key).cloned().unwrap_or_default();
            let right_now = self.spaltenArtenKey_SpaltennummernValue.get(&right_key).cloned().unwrap_or_default();
            let intersection: BTreeSet<PyAtom> = left_now.intersection(&right_now).cloned().collect();
            let entry = self.spaltenArtenKey_SpaltennummernValue.entry(left_key).or_insert_with(BTreeSet::new);
            for item in intersection {
                entry.remove(&item);
            }
        }
        for el2Type in 0..half_len {
            let left_key = (0usize, el2Type);
            let right_key = (1usize, el2Type);
            let popped = self.spaltenArtenKey_SpaltennummernValue.shift_remove(&right_key).unwrap_or_default();
            let entry = self.spaltenArtenKey_SpaltennummernValue.entry(left_key).or_insert_with(BTreeSet::new);
            for item in popped {
                entry.remove(&item);
            }
        }
    }

    pub fn breiteBreitenSysArgvPara(&mut self, cmd: &str, _neg: &str) -> bool {
        if let Some(value) = cmd.strip_prefix("breite=") {
            if let Ok(num) = value.parse::<i64>() {
                self.tables.textWidth = num;
                self.breiteHasBeenOnceZero = num == 0;
                self.breiteORbreiten = true;
                return true;
            }
        }
        if let Some(value) = cmd.strip_prefix("breiten=") {
            let mut out = Vec::new();
            for part in value.split(',') {
                if let Ok(num) = part.parse::<i64>() {
                    out.push(num);
                } else {
                    return false;
                }
            }
            self.tables.breitenn = out;
            self.breiteORbreiten = true;
            return true;
        }
        false
    }

    pub fn produceAllSpaltenNumbers_phase_01_mainParaCmds_and_scan(&mut self, neg: &str) {
        self.mainParaCmds = self.i18n.main_para_cmds.clone();
        let mut lastMainCmd: i64 = -1;

        for original_cmd in self.argv[1..].iter() {
            let mut cmd = original_cmd.clone();

            if cmd.len() > 1 && cmd.starts_with("-") && !cmd.starts_with("--") {
                let key = cmd[1..].to_string();
                if let Some(Some(value)) = self.mainParaCmds.get(&key) {
                    lastMainCmd = *value;
                } else if key == self.i18n.nichts_wort || key == "nichts" || key == "nothing" {
                } else if cmd.starts_with(&self.i18n.sprachen_parameter_wort)
                    && self.i18n.sprachen.contains(&cmd[self.i18n.sprachen_parameter_wort.len()..].to_string()) {
                } else if cmd.starts_with(&self.i18n.sprachen_parameter_wort)
                    && !self.i18n.sprachen.contains(&cmd[self.i18n.sprachen_parameter_wort.len()..].to_string()) {
                    println!("{}", self.i18n.wrong_lang_sentence);
                    std::process::exit(0);
                } else if neg.len() == 0 {
                    cliout(
                        "Unbekannter Hauptparameter: ".to_string()
                            + &cmd
                            + " | Erlaubt: -"
                            + &self.mainParaCmds.keys().cloned().collect::<Vec<_>>().join(", -"),
                    );
                }
            } else if cmd.starts_with("--") {
                let spalten_key = self.i18n.main_para_cmds.get(&self.tables.spalten_parameter_name).and_then(|x| *x);
                let kombi_key = self.i18n.main_para_cmds.get(&self.tables.kombi_parameter_name).and_then(|x| *x);

                if Some(lastMainCmd) == spalten_key {
                    cmd = cmd[2..].to_string();
                    let eq = cmd.find('=');
                    if self.breiteBreitenSysArgvPara(&cmd, neg) {
                    } else if cmd == self.i18n.keine_num_wort && neg.len() == 0 {
                        self.tables.nummeriere = false;
                    } else if eq.is_some() {
                        self.produceAllSpaltenNumbers_phase_02_eq_branch_parameter_values(neg, &cmd);
                    } else {
                        self.produceAllSpaltenNumbers_phase_03_plain_parameter_branch(neg, &cmd);
                    }
                } else if Some(lastMainCmd) == kombi_key {
                    cmd = cmd[2..].to_string();
                    self.produceAllSpaltenNumbers_phase_04_kombi_branch(neg, &cmd);
                } else if !self.mainParaCmds.values().any(|x| *x == Some(lastMainCmd)) {
                    cliout(
                        self.i18n.cliout7Saetze[0].clone()
                            + &self.i18n.cliout7Saetze[1]
                            + &cmd
                            + &self.i18n.cliout7Saetze[2]
                            + &self.mainParaCmds.keys().cloned().collect::<Vec<_>>().join(" -"),
                    );
                }
            }
        }
    }

    pub fn produceAllSpaltenNumbers_phase_02_eq_branch_parameter_values(&mut self, neg: &str, cmd: &str) {
        let eq = cmd.find('=').unwrap();
        let befehlName = cmd[..eq].to_string();

        for original_value in cmd[eq + 1..].split(',') {
            let mut oneOfThingsAfterEqSign = original_value.to_string();
            let yes1: bool;
            if oneOfThingsAfterEqSign.len() > 0 && oneOfThingsAfterEqSign.starts_with("-") {
                oneOfThingsAfterEqSign = oneOfThingsAfterEqSign[1..].to_string();
                yes1 = neg == "-";
            } else {
                yes1 = neg.is_empty();
            }

            if !yes1 {
                continue;
            }

            let lookup_key = (befehlName.clone(), oneOfThingsAfterEqSign.clone());
            if let Some(found) = self.paraDict.get(&lookup_key).cloned() {
                self.resultingSpaltenFromTuple(
                    found,
                    neg,
                    Some(oneOfThingsAfterEqSign.clone()),
                    Some(befehlName.clone()),
                );
                continue;
            }

            let store_key = ("=".to_string() + neg, befehlName.clone());
            if !self.paraDict.contains_key(&store_key) {
                self.paraDict.insert(store_key.clone(), vec![]);
            }

            if befehlName == "breite" || befehlName == "breiten" || befehlName == self.i18n.keine_num_wort {
                continue;
            }

            if oneOfThingsAfterEqSign.is_empty() {
                continue;
            }

            if befehlName == "spaltenreihenfolgeundnurdiese" {
                let mut list_items = vec![];
                for x in oneOfThingsAfterEqSign.split(',') {
                    if let Ok(num) = x.parse::<i64>() {
                        list_items.push(PyCollection::Atom(PyAtom::Int(num)));
                    } else {
                        list_items.push(PyCollection::Atom(PyAtom::Str(x.to_string())));
                    }
                }
                self.paraDict.insert(store_key, vec![PyCollection::List(list_items)]);
                continue;
            }

            if matches!(befehlName.as_str(), "art" | "spaltenausgabe" | "ausgabe" | "vorhervonausschnitt" | "nachtraeglichdavonausschnitt") {
                self.paraDict.insert(store_key, vec![PyCollection::Atom(PyAtom::Str(oneOfThingsAfterEqSign.clone()))]);
                continue;
            }

            let nebenParameters: Vec<String> = self.paraDict.keys().map(|(a, _)| a.clone()).collect();
            let nebenparameterWerte: Vec<String> = self.paraDict.keys().map(|(_, b)| b.clone()).collect();

            if nebenParameters.iter().any(|x| x == &befehlName) {
                let mut possibleNebenparameterWert: Vec<String> = vec![];
                for (nebenParameter, nebenparameterWert) in nebenParameters.iter().zip(nebenparameterWerte.iter()) {
                    if nebenParameter == &befehlName {
                        possibleNebenparameterWert.push(nebenparameterWert.clone());
                    }
                }
                let all_empty = possibleNebenparameterWert.iter().all(|p| p.is_empty());
                cliout(
                    self.i18n.cliout2Saetze[0].clone()
                        + &befehlName
                        + &self.i18n.cliout2Saetze[1]
                        + &oneOfThingsAfterEqSign
                        + if !possibleNebenparameterWert.is_empty() && ! all_empty  else 
                );
                cliout(
                    if !possibleNebenparameterWert.is_empty() && !all_empty {
                        self.i18n.cliout2Saetze[0].clone()
                            + &befehlName
                            + &self.i18n.cliout2Saetze[1]
                            + &oneOfThingsAfterEqSign
                            + &self.i18n.cliout2Saetze[2]
                            + &possibleNebenparameterWert.join(",")
                            + """
                    } else {
                        self.i18n.cliout2Saetze[0].clone()
                            + &befehlName
                            + &self.i18n.cliout2Saetze[1]
                            + &oneOfThingsAfterEqSign
                            + &self.i18n.cliout2Saetze[3]
                    }
                );
            } else {
                cliout(
                    self.i18n.cliout3Saetze[0].clone()
                        + &befehlName
                        + &self.i18n.cliout3Saetze[1]
                        + &oneOfThingsAfterEqSign
                        + &self.i18n.cliout3Saetze[2]
                        + &self.ordered_distinct_first_items().join(", --")
                        + &self.i18n.cliout3Saetze[6]
                        + &self.ordered_distinct_second_items().join(",")
                );
            }
        }
    }

    pub fn produceAllSpaltenNumbers_phase_03_plain_parameter_branch(&mut self, neg: &str, cmd: &str) {
        if cmd.len() > 0 && ((cmd.ends_with("-") && neg == "-") != (neg.is_empty() && !cmd.ends_with("-"))) {
            let mut real_cmd = cmd.to_string();
            if real_cmd.ends_with("-") && !neg.is_empty() {
                real_cmd.pop();
            }

            if let Some(found) = self.paraDict.get(&(real_cmd.clone(), "".to_string())).cloned() {
                self.resultingSpaltenFromTuple(found, neg, None, Some(real_cmd));
                return;
            }

            cliout(
                self.i18n.cliout4Saetze[0].clone()
                    + &real_cmd
                    + &self.i18n.cliout4Saetze[1]
                    + &self.ordered_distinct_first_items().join(", --")
                    + &self.i18n.cliout4Saetze[5]
            );
            return;
        }

        let key = ("".to_string() + neg, cmd.to_string());
        if !self.paraDict.contains_key(&key) {
            self.paraDict.insert(key.clone(), vec![]);
        }
        if matches!(cmd, "keineleereninhalte" | "keinenummerierung" | "alles" | "zeit" | "zaehlung" | "zählung" | "nichts" | "debug") {
            return;
        }
        panic!("UNTRANSCOMPILIERT: phase_03 complex plain parameter branch for cmd={} neg={}", cmd, neg);
    }

    pub fn produceAllSpaltenNumbers_phase_04_kombi_branch(&mut self, neg: &str, cmd: &str) {
        let galWort = "--".to_string() + self.i18n.kombi_main_paras.get("galaxie").unwrap() + "=";
        let uniWort = "--".to_string() + self.i18n.kombi_main_paras.get("universum").unwrap() + "=";

        if cmd.starts_with(&galWort[2..]) || cmd.starts_with(&uniWort[2..]) {
            for original_kombi in cmd[cmd.find('=').unwrap() + 1..].split(',') {
                let mut oneKombiSpalte = original_kombi.to_string();
                let yes1: bool;
                if oneKombiSpalte.len() > 0 && oneKombiSpalte.starts_with("-") {
                    oneKombiSpalte = oneKombiSpalte[1..].to_string();
                    yes1 = neg == "-";
                } else {
                    yes1 = neg.is_empty();
                }

                if !yes1 {
                    continue;
                }

                if let Some(found) = self.kombiReverseDict.get(&oneKombiSpalte).cloned() {
                    if cmd.find('=').unwrap() == galWort.len() - 3 {
                        self.resultingSpaltenFromTuple(
                            vec![
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::from([found])),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                            ],
                            neg,
                            None,
                            Some("kombinationen".to_string()),
                        );
                        continue;
                    }
                }

                if let Some(found2) = self.kombiReverseDict2.get(&oneKombiSpalte).cloned() {
                    if cmd.find('=').unwrap() == uniWort.len() - 3 {
                        self.resultingSpaltenFromTuple(
                            vec![
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::new()),
                                PyCollection::Set(BTreeSet::from([found2])),
                            ],
                            neg,
                            None,
                            Some("kombinationen".to_string()),
                        );
                        continue;
                    }
                }

                cliout(
                    self.i18n.cliout5Saetze[0].clone()
                        + &oneKombiSpalte
                        + &self.i18n.cliout5Saetze[1]
                        + &cmd[..cmd.find('=').unwrap() + 1]
                );
            }
            return;
        }

        if neg.is_empty() {
            cliout(self.i18n.cliout6Satz.clone() + cmd);
            return;
        }
    }

    pub fn produceAllSpaltenNumbers_phase_05_finish_and_recursive_neg(&mut self, neg: &str) {
        let breiteIstNull = "--".to_string() + &self.tables.ausgabe_breite + "=0";
        if self.argv.iter().any(|x| x == &breiteIstNull) {
            self.breiteBreitenSysArgvPara(&breiteIstNull[2..], "");
        }
        if neg.is_empty() {
            self.produceAllSpaltenNumbers("-");
            self.spalten_removeDoublesNthenRemoveOneFromAnother();
        }
    }

    pub fn produceAllSpaltenNumbers(&mut self, neg: &str) {
        self.produceAllSpaltenNumbers_phase_01_mainParaCmds_and_scan(neg);
        self.produceAllSpaltenNumbers_phase_05_finish_and_recursive_neg(neg);
    }
}

pub const PYTHON_SOURCE__PRODUCE_ALL_SPALTEN_NUMBERS: &str = r#"    def produceAllSpaltenNumbers(self, neg=""):
        global shellRowsAmount

        # x("ANFANG metakonkret", self.paraDict[(cmd[:eq], "konkret")])

        def resultingSpaltenFromTuple(
            tupl: tuple, neg, paraValue=None, befehlName=None
        ) -> tuple:
            # x("tupl", tupl)
            for i, eineSpaltenArtmitSpaltenNummern in enumerate(tupl):
                """
                Die Variable self.tables.spalteGestirn braucht man gar nicht mehr !!!
                """
                # x(
                #    "eineSpaltenArtmitSpaltenNummernWW",
                #    [i, eineSpaltenArtmitSpaltenNummern],
                # )
                if (
                    type(eineSpaltenArtmitSpaltenNummern) in [list, tuple]
                    && len(eineSpaltenArtmitSpaltenNummern) > 0
                ):
                    if type(eineSpaltenArtmitSpaltenNummern[0]) is bool:
                        eineSpaltenArtmitSpaltenNummern = set(
                            eineSpaltenArtmitSpaltenNummern
                        )
                    elif type(eineSpaltenArtmitSpaltenNummern[0]) in [tuple, list]:
                        eineSpaltenArtmitSpaltenNummern = set(
                            eineSpaltenArtmitSpaltenNummern[0]
                        )
                # x(
                #    "if ",
                #    [
                #        type(eineSpaltenArtmitSpaltenNummern),
                #        [
                #            list,
                #            tuple,
                #            # set,
                #        ],
                #        befehlName,
                #        i18n.gebrochenUniGalEinzeln,
                #        {b for a in i18n.gebrochenUniGal.values() for b in a},
                #    ],
                # )
                if i == 2 && (
                    type(eineSpaltenArtmitSpaltenNummern)
                    in [
                        list,
                        tuple,
                        # set,
                    ]
                    or befehlName in i18n.gebrochenUniGalEinzeln
                ):
                    gebrBefehleDict: dict = {
                        Program.ParametersMain.Multiplikationen[0]: 2,
                        Program.ParametersMain.gebrochenuniversum[0]: 5,
                        Program.ParametersMain.gebrochenuniversum[1]: 5,
                        Program.ParametersMain.gebrochengalaxie[0]: 6,
                        Program.ParametersMain.gebrochengalaxie[1]: 6,
                        Program.ParametersMain.gebrochenemotion[0]: 9,
                        Program.ParametersMain.gebrochenemotion[1]: 9,
                        Program.ParametersMain.gebrochengroesse[0]: 10,
                        Program.ParametersMain.gebrochengroesse[1]: 10,
                    }
                    # x("bli 1", befehlName)
                    # x("bli 2", gebrBefehleDict[befehlName])
                    # x("bli 3", paraValue)
                    # x(
                    #    "bli 4",
                    #    Program.lambdaPrimGalax(paraValue)
                    #    if befehlName == Program.ParametersMain.Multiplikationen[0]
                    #    else Program.lambdaGebrUnivUndGalax(paraValue),
                    # )
                    self.spaltenArtenKey_SpaltennummernValue[
                        len(neg), gebrBefehleDict[befehlName]
                    ] |= (
                        Program.lambdaPrimGalax(paraValue)
                        if befehlName == Program.ParametersMain.Multiplikationen[0]
                        else Program.lambdaGebrUnivUndGalax(paraValue)
                    )
                elif (
                    paraValue == i18nR.beschriebenWort
                    && befehlName in Program.ParametersMain.primvielfache
                ):
                    self.spaltenArtenKey_SpaltennummernValue[(len(neg), 2)] |= {2}
                # elif i not in (5, 6, 9, 10):
                else:
                    try:
                        # x(
                        #    "dazu_T",
                        #    [
                        #        i,
                        #        self.spaltenArtenKey_SpaltennummernValue[(len(neg), i)],
                        #        eineSpaltenArtmitSpaltenNummern,
                        #        neg,
                        #    ],
                        # )
                        self.spaltenArtenKey_SpaltennummernValue[
                            (len(neg), i)
                        ] |= eineSpaltenArtmitSpaltenNummern
                    except TypeError:
                        pass
            return self.spaltenArtenKey_SpaltennummernValue

        def spalten_removeDoublesNthenRemoveOneFromAnother():
            for el2Type in range(
                int(len(self.spaltenArtenKey_SpaltennummernValue) / 2)
            ):
                self.spaltenArtenKey_SpaltennummernValue[(0, el2Type)] -= (
                    self.spaltenArtenKey_SpaltennummernValue[(0, el2Type)]
                    & self.spaltenArtenKey_SpaltennummernValue[(1, el2Type)]
                )
            for el2Type in range(
                int(len(self.spaltenArtenKey_SpaltennummernValue) / 2)
            ):
                self.spaltenArtenKey_SpaltennummernValue[
                    (0, el2Type)
                ] -= self.spaltenArtenKey_SpaltennummernValue.pop((1, el2Type))

        self.mainParaCmds: dict = {
            i18n.mainParaCmds["zeilen"]: 0,
            i18n.mainParaCmds["spalten"]: 1,
            i18n.mainParaCmds[tuple(i18n.tableHandling.parameterName.keys())[0]]: 2,
            i18n.mainParaCmds["ausgabe"]: 3,
            i18n.mainParaCmds["debug"]: None,
            i18n.mainParaCmds["h"]: None,
            i18n.mainParaCmds["help"]: None,
        }
        lastMainCmd: int = -1
        for cmd in self.argv[1:]:
            if len(cmd) > 1 && cmd[0] == "-" && cmd[1] != "-":
                if cmd[1:] in self.mainParaCmds.keys():
                    lastMainCmd = self.mainParaCmds[cmd[1:]]
                elif cmd[1:] in (i18nR.nichtsWort, "nichts", "nothing"):
                    pass
                elif (
                    cmd[: len(i18n.sprachenParameterWort)] == i18n.sprachenParameterWort
                    && cmd[len(i18n.sprachenParameterWort) :] in i18n.sprachen.keys()
                ):
                    pass
                elif (
                    cmd[: len(i18n.sprachenParameterWort)] == i18n.sprachenParameterWort
                    && cmd[len(i18n.sprachenParameterWort) :]
                    not in i18n.sprachen.keys()
                ):
                    print(i18n.wrongLangSentence)
                    exit()
                elif len(neg) == 0:
                    # else:
                    cliout(
                        i18nR.cliout1Saetze[0]
                        + cmd
                        + i18nR.cliout1Saetze[1]
                        + i18nR.cliout1Saetze[2]
                        + str(", -".join(list(self.mainParaCmds.keys())))
                    )
            elif cmd[:2] == "--":
                if lastMainCmd == self.mainParaCmds[i18n.mainParaCmds["spalten"]]:
                    cmd = cmd[2:]
                    eq = cmd.find("=")
                    if self.breiteBreitenSysArgvPara(cmd, neg):
                        pass
                    elif cmd == i18nR.keineNumWort && len(neg) == 0:
                        self.tables.nummeriere = False
                    elif eq != -1:
                        for oneOfThingsAfterEqSign in cmd[eq + 1 :].split(","):
                            if (
                                len(oneOfThingsAfterEqSign) > 0
                                && oneOfThingsAfterEqSign[0] == "-"
                            ):
                                oneOfThingsAfterEqSign = oneOfThingsAfterEqSign[1:]
                                yes1 = True if neg == "-" else False
                            else:
                                yes1 = True if len(neg) == 0 else False
                            if yes1:
                                try:
                                    # x(
                                    #    "tupleQ4_5",
                                    #    [
                                    #        self.paraDict[
                                    #            (cmd[:eq], oneOfThingsAfterEqSign)
                                    #        ][5],
                                    #        oneOfThingsAfterEqSign,
                                    #        cmd[:eq],
                                    #    ],
                                    # )
                                    resultingSpaltenFromTuple(
                                        self.paraDict[
                                            (cmd[:eq], oneOfThingsAfterEqSign)
                                        ],
                                        neg,
                                        oneOfThingsAfterEqSign,
                                        befehlName=cmd[:eq],
                                    )
                                except KeyError:
                                    nebenParameters: list = []
                                    nebenparameterWerte: list = []
                                    for value in self.paraDict.keys():
                                        nebenParameters += [value[0]]
                                        nebenparameterWerte += [value[1]]

                                    if cmd[:eq] in nebenParameters:
                                        possibleNebenparameterWert: list = []
                                        for nebenParameter, nebenparameterWert in zip(
                                            nebenParameters,
                                            nebenparameterWerte,
                                        ):
                                            if nebenParameter == cmd[:eq]:
                                                possibleNebenparameterWert += [
                                                    nebenparameterWert
                                                ]

                                        cliout(
                                            i18nR.cliout2Saetze[0]
                                            + cmd[:eq]
                                            + i18nR.cliout2Saetze[1]
                                            + oneOfThingsAfterEqSign
                                            + (
                                                (i18nR.cliout2Saetze[2])
                                                + (
                                                    ",".join(possibleNebenparameterWert)
                                                    + '"'
                                                )
                                                if (
                                                    len(possibleNebenparameterWert) > 0
                                                    && ! all(
                                                        [
                                                            p == ""
                                                            for p in possibleNebenparameterWert
                                                        ]
                                                    )
                                                )
                                                else i18nR.cliout2Saetze[3]
                                            )
                                        )
                                    else:
                                        cliout(
                                            i18nR.cliout3Saetze[0]
                                            + cmd[:eq]
                                            + i18nR.cliout3Saetze[1]
                                            + oneOfThingsAfterEqSign
                                            + i18nR.cliout3Saetze[2]
                                            + i18nR.cliout3Saetze[3]
                                            + i18nR.cliout3Saetze[4]
                                            + i18nR.cliout3Saetze[5]
                                            + str(
                                                ", --".join(
                                                    tuple(
                                                        OrderedSet(
                                                            key[0]
                                                            for key in self.paraDict.keys()
                                                        )
                                                    )
                                                )
                                            )
                                            + i18nR.cliout3Saetze[6]
                                            + i18nR.cliout3Saetze[7]
                                            + str(
                                                ",".join(
                                                    tuple(
                                                        OrderedSet(
                                                            key[1]
                                                            for key in self.paraDict.keys()
                                                        )
                                                    )
                                                )
                                            )
                                        )

                    else:
                        try:
                            if len(cmd) > 0 && (cmd[-1] == "-" && neg == "-") != (
                                len(neg) == 0 && cmd[-1] != "-"
                            ):
                                if len(cmd) > 0 && cmd[-1] == "-" && len(neg) > 0:
                                    cmd = cmd[:-1]

                                # x("tupleP4_5", self.paraDict[(cmd, "")][5])
                                resultingSpaltenFromTuple(
                                    self.paraDict[(cmd, "")], neg, befehlName=cmd
                                )

                        except KeyError:
                            cliout(
                                i18nR.cliout4Saetze[0]
                                + cmd
                                + i18nR.cliout4Saetze[1]
                                + i18nR.cliout4Saetze[2]
                                + i18nR.cliout4Saetze[3]
                                + i18nR.cliout4Saetze[4]
                                + str(
                                    ", --".join(
                                        tuple(
                                            OrderedSet(
                                                key[0] for key in self.paraDict.keys()
                                            )
                                        )
                                    )
                                )
                                + i18nR.cliout4Saetze[5]
                            )

                elif (
                    lastMainCmd
                    == self.mainParaCmds[self.tables.getCombis.parameterName]
                ):
                    galWort = "--" + i18n.kombiMainParas["galaxie"] + "="
                    uniWort = "--" + i18n.kombiMainParas["universum"] + "="

                    if cmd[: len(galWort)] == galWort or cmd[: len(uniWort)] == uniWort:
                        for oneKombiSpalte in cmd[cmd.find("=") + 1 :].split(","):
                            if len(oneKombiSpalte) > 0 && oneKombiSpalte[0] == "-":
                                oneKombiSpalte = oneKombiSpalte[1:]
                                yes1 = True if neg == "-" else False
                            else:
                                yes1 = True if len(neg) == 0 else False
                            if yes1:
                                try:
                                    resultingSpaltenFromTuple(
                                        (
                                            OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            {
                                                self.kombiReverseDict[oneKombiSpalte],
                                            }
                                            if cmd.find("=") == len(galWort) - 1
                                            else OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            {
                                                self.kombiReverseDict2[oneKombiSpalte],
                                            }
                                            if cmd.find("=") == len(uniWort) - 1
                                            else OrderedSet(),
                                        ),
                                        neg,
                                        befehlName="kombinationen",
                                    )
                                except KeyError:
                                    cliout(
                                        i18nR.cliout5Saetze[0]
                                        + oneKombiSpalte
                                        + i18nR.cliout5Saetze[1]
                                        + cmd[: cmd.find("=") + 1]
                                        + " "
                                        + (
                                            str(
                                                tuple(
                                                    [
                                                        element
                                                        for row in i18n.kombiParaNdataMatrix.values()
                                                        for element in row
                                                    ]
                                                )
                                            )[1:-1]
                                            if cmd[: cmd.find("=")] == galWort[:-1]
                                            else str(
                                                tuple(
                                                    [
                                                        element
                                                        for row in i18n.kombiParaNdataMatrix2.values()
                                                        for element in row
                                                    ]
                                                )
                                            )[1:-1]
                                            if cmd[: cmd.find("=")] == uniWort[:-1]
                                            else ""
                                        )
                                    )

                    elif neg == "":
                        cliout(i18nR.cliout6Satz + str(cmd))
                elif lastMainCmd not in self.mainParaCmds.values():
                    cliout(
                        i18nR.cliout7Saetze[0]
                        + i18nR.cliout7Saetze[1]
                        + cmd
                        + i18nR.cliout7Saetze[2]
                        + " -".join(self.mainParaCmds)
                    )
        breiteIstNull = "".join(("--", i18n.ausgabeParas["breite"], "=0"))
        if breiteIstNull in self.argv:
            self.breiteBreitenSysArgvPara(breiteIstNull[2:], "")
        if len(neg) == 0:
            self.produceAllSpaltenNumbers("-")
            spalten_removeDoublesNthenRemoveOneFromAnother()"#;
pub const PYTHON_SOURCE__LOCAL_RESULTING_SPALTEN_FROM_TUPLE: &str = r#"        def resultingSpaltenFromTuple(
            tupl: tuple, neg, paraValue=None, befehlName=None
        ) -> tuple:
            # x("tupl", tupl)
            for i, eineSpaltenArtmitSpaltenNummern in enumerate(tupl):
                """
                Die Variable self.tables.spalteGestirn braucht man gar nicht mehr !!!
                """
                # x(
                #    "eineSpaltenArtmitSpaltenNummernWW",
                #    [i, eineSpaltenArtmitSpaltenNummern],
                # )
                if (
                    type(eineSpaltenArtmitSpaltenNummern) in [list, tuple]
                    && len(eineSpaltenArtmitSpaltenNummern) > 0
                ):
                    if type(eineSpaltenArtmitSpaltenNummern[0]) is bool:
                        eineSpaltenArtmitSpaltenNummern = set(
                            eineSpaltenArtmitSpaltenNummern
                        )
                    elif type(eineSpaltenArtmitSpaltenNummern[0]) in [tuple, list]:
                        eineSpaltenArtmitSpaltenNummern = set(
                            eineSpaltenArtmitSpaltenNummern[0]
                        )
                # x(
                #    "if ",
                #    [
                #        type(eineSpaltenArtmitSpaltenNummern),
                #        [
                #            list,
                #            tuple,
                #            # set,
                #        ],
                #        befehlName,
                #        i18n.gebrochenUniGalEinzeln,
                #        {b for a in i18n.gebrochenUniGal.values() for b in a},
                #    ],
                # )
                if i == 2 && (
                    type(eineSpaltenArtmitSpaltenNummern)
                    in [
                        list,
                        tuple,
                        # set,
                    ]
                    or befehlName in i18n.gebrochenUniGalEinzeln
                ):
                    gebrBefehleDict: dict = {
                        Program.ParametersMain.Multiplikationen[0]: 2,
                        Program.ParametersMain.gebrochenuniversum[0]: 5,
                        Program.ParametersMain.gebrochenuniversum[1]: 5,
                        Program.ParametersMain.gebrochengalaxie[0]: 6,
                        Program.ParametersMain.gebrochengalaxie[1]: 6,
                        Program.ParametersMain.gebrochenemotion[0]: 9,
                        Program.ParametersMain.gebrochenemotion[1]: 9,
                        Program.ParametersMain.gebrochengroesse[0]: 10,
                        Program.ParametersMain.gebrochengroesse[1]: 10,
                    }
                    # x("bli 1", befehlName)
                    # x("bli 2", gebrBefehleDict[befehlName])
                    # x("bli 3", paraValue)
                    # x(
                    #    "bli 4",
                    #    Program.lambdaPrimGalax(paraValue)
                    #    if befehlName == Program.ParametersMain.Multiplikationen[0]
                    #    else Program.lambdaGebrUnivUndGalax(paraValue),
                    # )
                    self.spaltenArtenKey_SpaltennummernValue[
                        len(neg), gebrBefehleDict[befehlName]
                    ] |= (
                        Program.lambdaPrimGalax(paraValue)
                        if befehlName == Program.ParametersMain.Multiplikationen[0]
                        else Program.lambdaGebrUnivUndGalax(paraValue)
                    )
                elif (
                    paraValue == i18nR.beschriebenWort
                    && befehlName in Program.ParametersMain.primvielfache
                ):
                    self.spaltenArtenKey_SpaltennummernValue[(len(neg), 2)] |= {2}
                # elif i not in (5, 6, 9, 10):
                else:
                    try:
                        # x(
                        #    "dazu_T",
                        #    [
                        #        i,
                        #        self.spaltenArtenKey_SpaltennummernValue[(len(neg), i)],
                        #        eineSpaltenArtmitSpaltenNummern,
                        #        neg,
                        #    ],
                        # )
                        self.spaltenArtenKey_SpaltennummernValue[
                            (len(neg), i)
                        ] |= eineSpaltenArtmitSpaltenNummern
                    except TypeError:
                        pass
            return self.spaltenArtenKey_SpaltennummernValue"#;
pub const PYTHON_SOURCE__LOCAL_SPALTEN_REMOVE_DOUBLES_NTHEN_REMOVE_ONE_FROM_ANOTHER: &str = r#"        def spalten_removeDoublesNthenRemoveOneFromAnother():
            for el2Type in range(
                int(len(self.spaltenArtenKey_SpaltennummernValue) / 2)
            ):
                self.spaltenArtenKey_SpaltennummernValue[(0, el2Type)] -= (
                    self.spaltenArtenKey_SpaltennummernValue[(0, el2Type)]
                    & self.spaltenArtenKey_SpaltennummernValue[(1, el2Type)]
                )
            for el2Type in range(
                int(len(self.spaltenArtenKey_SpaltennummernValue) / 2)
            ):
                self.spaltenArtenKey_SpaltennummernValue[
                    (0, el2Type)
                ] -= self.spaltenArtenKey_SpaltennummernValue.pop((1, el2Type))
"#;
pub const PYTHON_SOURCE__PHASE_01_MAINPARACMDS_AND_SCAN: &str = r#"        self.mainParaCmds: dict = {
            i18n.mainParaCmds["zeilen"]: 0,
            i18n.mainParaCmds["spalten"]: 1,
            i18n.mainParaCmds[tuple(i18n.tableHandling.parameterName.keys())[0]]: 2,
            i18n.mainParaCmds["ausgabe"]: 3,
            i18n.mainParaCmds["debug"]: None,
            i18n.mainParaCmds["h"]: None,
            i18n.mainParaCmds["help"]: None,
        }
        lastMainCmd: int = -1
        for cmd in self.argv[1:]:
            if len(cmd) > 1 && cmd[0] == "-" && cmd[1] != "-":
                if cmd[1:] in self.mainParaCmds.keys():
                    lastMainCmd = self.mainParaCmds[cmd[1:]]
                elif cmd[1:] in (i18nR.nichtsWort, "nichts", "nothing"):
                    pass
                elif (
                    cmd[: len(i18n.sprachenParameterWort)] == i18n.sprachenParameterWort
                    && cmd[len(i18n.sprachenParameterWort) :] in i18n.sprachen.keys()
                ):
                    pass
                elif (
                    cmd[: len(i18n.sprachenParameterWort)] == i18n.sprachenParameterWort
                    && cmd[len(i18n.sprachenParameterWort) :]
                    not in i18n.sprachen.keys()
                ):
                    print(i18n.wrongLangSentence)
                    exit()
                elif len(neg) == 0:
                    # else:
                    cliout(
                        i18nR.cliout1Saetze[0]
                        + cmd
                        + i18nR.cliout1Saetze[1]
                        + i18nR.cliout1Saetze[2]
                        + str(", -".join(list(self.mainParaCmds.keys())))
                    )
            elif cmd[:2] == "--":
                if lastMainCmd == self.mainParaCmds[i18n.mainParaCmds["spalten"]]:
                    cmd = cmd[2:]
                    eq = cmd.find("=")
                    if self.breiteBreitenSysArgvPara(cmd, neg):"#;
pub const PYTHON_SOURCE__PHASE_02_EQ_BRANCH_PARAMETER_VALUES: &str = r#"                        pass
                    elif cmd == i18nR.keineNumWort && len(neg) == 0:
                        self.tables.nummeriere = False
                    elif eq != -1:
                        for oneOfThingsAfterEqSign in cmd[eq + 1 :].split(","):
                            if (
                                len(oneOfThingsAfterEqSign) > 0
                                && oneOfThingsAfterEqSign[0] == "-"
                            ):
                                oneOfThingsAfterEqSign = oneOfThingsAfterEqSign[1:]
                                yes1 = True if neg == "-" else False
                            else:
                                yes1 = True if len(neg) == 0 else False
                            if yes1:
                                try:
                                    # x(
                                    #    "tupleQ4_5",
                                    #    [
                                    #        self.paraDict[
                                    #            (cmd[:eq], oneOfThingsAfterEqSign)
                                    #        ][5],
                                    #        oneOfThingsAfterEqSign,
                                    #        cmd[:eq],
                                    #    ],
                                    # )
                                    resultingSpaltenFromTuple(
                                        self.paraDict[
                                            (cmd[:eq], oneOfThingsAfterEqSign)
                                        ],
                                        neg,
                                        oneOfThingsAfterEqSign,
                                        befehlName=cmd[:eq],
                                    )
                                except KeyError:
                                    nebenParameters: list = []
                                    nebenparameterWerte: list = []
                                    for value in self.paraDict.keys():
                                        nebenParameters += [value[0]]
                                        nebenparameterWerte += [value[1]]

                                    if cmd[:eq] in nebenParameters:
                                        possibleNebenparameterWert: list = []
                                        for nebenParameter, nebenparameterWert in zip(
                                            nebenParameters,
                                            nebenparameterWerte,
                                        ):
                                            if nebenParameter == cmd[:eq]:
                                                possibleNebenparameterWert += [
                                                    nebenparameterWert
                                                ]

                                        cliout(
                                            i18nR.cliout2Saetze[0]
                                            + cmd[:eq]
                                            + i18nR.cliout2Saetze[1]
                                            + oneOfThingsAfterEqSign
                                            + (
                                                (i18nR.cliout2Saetze[2])
                                                + (
                                                    ",".join(possibleNebenparameterWert)
                                                    + '"'
                                                )
                                                if (
                                                    len(possibleNebenparameterWert) > 0
                                                    && ! all(
                                                        [
                                                            p == ""
                                                            for p in possibleNebenparameterWert
                                                        ]
                                                    )
                                                )
                                                else i18nR.cliout2Saetze[3]
                                            )
                                        )
                                    else:
                                        cliout(
                                            i18nR.cliout3Saetze[0]
                                            + cmd[:eq]
                                            + i18nR.cliout3Saetze[1]
                                            + oneOfThingsAfterEqSign
                                            + i18nR.cliout3Saetze[2]
                                            + i18nR.cliout3Saetze[3]
                                            + i18nR.cliout3Saetze[4]
                                            + i18nR.cliout3Saetze[5]
                                            + str(
                                                ", --".join(
                                                    tuple(
                                                        OrderedSet(
                                                            key[0]
                                                            for key in self.paraDict.keys()
                                                        )
                                                    )
                                                )
                                            )
                                            + i18nR.cliout3Saetze[6]
                                            + i18nR.cliout3Saetze[7]
                                            + str(
                                                ",".join(
                                                    tuple(
                                                        OrderedSet(
                                                            key[1]
                                                            for key in self.paraDict.keys()
                                                        )
                                                    )
                                                )
                                            )
                                        )"#;
pub const PYTHON_SOURCE__PHASE_03_PLAIN_PARAMETER_BRANCH: &str = r#"
                    else:
                        try:
                            if len(cmd) > 0 && (cmd[-1] == "-" && neg == "-") != (
                                len(neg) == 0 && cmd[-1] != "-"
                            ):
                                if len(cmd) > 0 && cmd[-1] == "-" && len(neg) > 0:
                                    cmd = cmd[:-1]

                                # x("tupleP4_5", self.paraDict[(cmd, "")][5])
                                resultingSpaltenFromTuple(
                                    self.paraDict[(cmd, "")], neg, befehlName=cmd
                                )

                        except KeyError:
                            cliout(
                                i18nR.cliout4Saetze[0]
                                + cmd
                                + i18nR.cliout4Saetze[1]
                                + i18nR.cliout4Saetze[2]
                                + i18nR.cliout4Saetze[3]
                                + i18nR.cliout4Saetze[4]
                                + str(
                                    ", --".join(
                                        tuple(
                                            OrderedSet(
                                                key[0] for key in self.paraDict.keys()
                                            )
                                        )"#;
pub const PYTHON_SOURCE__PHASE_04_KOMBI_BRANCH: &str = r#"                                    )
                                )
                                + i18nR.cliout4Saetze[5]
                            )

                elif (
                    lastMainCmd
                    == self.mainParaCmds[self.tables.getCombis.parameterName]
                ):
                    galWort = "--" + i18n.kombiMainParas["galaxie"] + "="
                    uniWort = "--" + i18n.kombiMainParas["universum"] + "="

                    if cmd[: len(galWort)] == galWort or cmd[: len(uniWort)] == uniWort:
                        for oneKombiSpalte in cmd[cmd.find("=") + 1 :].split(","):
                            if len(oneKombiSpalte) > 0 && oneKombiSpalte[0] == "-":
                                oneKombiSpalte = oneKombiSpalte[1:]
                                yes1 = True if neg == "-" else False
                            else:
                                yes1 = True if len(neg) == 0 else False
                            if yes1:
                                try:
                                    resultingSpaltenFromTuple(
                                        (
                                            OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            {
                                                self.kombiReverseDict[oneKombiSpalte],
                                            }
                                            if cmd.find("=") == len(galWort) - 1
                                            else OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            OrderedSet(),
                                            {
                                                self.kombiReverseDict2[oneKombiSpalte],
                                            }
                                            if cmd.find("=") == len(uniWort) - 1
                                            else OrderedSet(),
                                        ),
                                        neg,
                                        befehlName="kombinationen",
                                    )
                                except KeyError:
                                    cliout(
                                        i18nR.cliout5Saetze[0]
                                        + oneKombiSpalte
                                        + i18nR.cliout5Saetze[1]
                                        + cmd[: cmd.find("=") + 1]
                                        + " "
                                        + (
                                            str(
                                                tuple(
                                                    [
                                                        element
                                                        for row in i18n.kombiParaNdataMatrix.values()
                                                        for element in row
                                                    ]
                                                )
                                            )[1:-1]
                                            if cmd[: cmd.find("=")] == galWort[:-1]
                                            else str(
                                                tuple(
                                                    [
                                                        element
                                                        for row in i18n.kombiParaNdataMatrix2.values()
                                                        for element in row
                                                    ]
                                                )
                                            )[1:-1]
                                            if cmd[: cmd.find("=")] == uniWort[:-1]
                                            else ""
                                        )
                                    )
"#;
pub const PYTHON_SOURCE__PHASE_05_FINISH_AND_RECURSIVE_NEG: &str = r#"                    elif neg == "":
                        cliout(i18nR.cliout6Satz + str(cmd))
                elif lastMainCmd not in self.mainParaCmds.values():
                    cliout(
                        i18nR.cliout7Saetze[0]
                        + i18nR.cliout7Saetze[1]
                        + cmd
                        + i18nR.cliout7Saetze[2]
                        + " -".join(self.mainParaCmds)
                    )
        breiteIstNull = "".join(("--", i18n.ausgabeParas["breite"], "=0"))
        if breiteIstNull in self.argv:
            self.breiteBreitenSysArgvPara(breiteIstNull[2:], "")
        if len(neg) == 0:
            self.produceAllSpaltenNumbers("-")
            spalten_removeDoublesNthenRemoveOneFromAnother()"#;

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let mut program = Program::new(argv);
    let _ = &mut program;
    eprintln!("reta.rs: mehr reale Direktübernahme für Phase 2-5, inklusive Lookup-Pfaden und Finish-Reihenfolge.");
}
