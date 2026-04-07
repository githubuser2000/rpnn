use indexmap::IndexMap;
use std::collections::BTreeSet;

use crate::shared::reta_program_types::{dedup_preserve_order_i64, PairStr, Program, SpaltenTyp};
use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

impl Program {
    pub fn workflowEverything(&mut self, argv: Vec<String>, words: &Words) -> Vec<Vec<String>> {
        let (RowsLen, paramLines, paramLinesNot, relitable, rowsAsNumbers) =
            self.bringAllImportantBeginThings(argv, words);

        self.RowsLen = RowsLen;
        self.relitable = relitable.clone();
        self.rowsAsNumbers = rowsAsNumbers.clone();

        if self.helpPage() {
            self.__resultingTable = vec![];
            return vec![];
        }

        if self.cliErrors.len() > 0 {
            self.__resultingTable = vec![];
            return vec![];
        }

        let (finallyDisplayLines, mut newTable, numlen, rowsRange, old2newTable) = self.prepare4out_py(
            paramLines,
            paramLinesNot,
            relitable,
            rowsAsNumbers,
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

        let out = self.cliOut_py(finallyDisplayLines, newTable.clone(), numlen, rowsRange);
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
        self.__runAlles = false;
        self.runDone = false;
        self.allImportantBeginThingsDone = false;
        self.tableGenerated = false;
    }
}
