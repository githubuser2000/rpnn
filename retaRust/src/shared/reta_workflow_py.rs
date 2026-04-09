#![allow(non_snake_case)]

use crate::shared::reta_program_types::Program;
use crate::shared::words_py::Words;

impl Program {
    pub fn workflowEverything(&mut self, argv: Vec<String>, words: &Words) -> Vec<Vec<String>> {
        let (RowsLen, paramLines, paramLinesNot, relitable, rowsAsNumbers) =
            self.bringAllImportantBeginThings(argv, words);

        self.RowsLen = RowsLen;
        self.relitable = relitable.clone();
        self.rowsAsNumbers = rowsAsNumbers.clone();

        let (finallyDisplayLinesEarly, _newTableEarly, _numlenEarly, _rowsRangeEarly, _old2newTableEarly) = self.prepare4out_py(
            paramLines.clone(),
            paramLinesNot.clone(),
            self.relitable.clone(),
            self.rowsAsNumbers.clone(),
        );
        let mut zeilenliste: Vec<i64> = finallyDisplayLinesEarly
            .iter()
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        zeilenliste.sort_unstable();
        self.lastLineNumber = zeilenliste.last().copied().unwrap_or(0);

        self.apply_concat_generators_py();

        if self.helpPage() {
            self.__resultingTable = vec![];
            return vec![];
        }

        if self.cliErrors.len() > 0 {
            self.__resultingTable = vec![];
            return vec![];
        }

        let (finallyDisplayLines, mut newTable, numlen, rowsRange, old2newTable): (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) = self.prepare4out_py(
            paramLines,
            paramLinesNot,
            self.relitable.clone(),
            self.rowsAsNumbers.clone(),
        );

        if self.rowsOfcombi.len() > 0 {
            self.combiTableWorkflow();
        }
        if self.rowsOfcombi2.len() > 0 {
            self.combiTableWorkflow();
        }

        newTable = self.onlyThatColumns_py(newTable, self.spaltenreihenfolgeundnurdiese.clone());
        self.newTable = newTable.len() > 0;
        self.finallyDisplayLines = finallyDisplayLines.clone();
        self.numlen = numlen;
        let _old2newTable = old2newTable.clone();

        let out: Vec<Vec<String>> = self.cliOut_py(finallyDisplayLines, newTable.clone(), numlen, rowsRange);
        self.tableGenerated = self.newTable || !out.is_empty();
        self.__resultingTable = out.clone();
        self.addResultingTableToTables();
        self.setOld2Rows();
        self.setNewerTable();
        self.setOldRows();
        self.setNewerRows();
        self.setRowsOfcombi();
        self.setOldTable();
        self.setGeneratedSpaltenParameter();
        self.setAllEquColumns();
        self.setFinallyDisplayTable();
        out
    }

    pub fn combiTableWorkflow(&mut self) {
        self.tableGenerated = self.tableGenerated || self.newTable;
    }

    pub fn run(&mut self, words: &Words) {
        if !self.__runAlles {
            self.__resultingTable = self.workflowEverything(self.argv.clone(), words);
        }
        self.invertAlles();
        self.printOrStoreLines();
        self.runDone = true;
    }

    pub fn runAllesLikePythonInit(&mut self, _words: &Words) {
    }
}
