#![allow(non_snake_case)]
pub use crate::shared::reta_program_types::{dedup_preserve_order_i64, PairStr, Program, SpaltenTyp};
use std::collections::BTreeSet;
impl Program {
    pub fn init_spalten_arten_python_like(&mut self) {
        self.spaltenArtenKey_SpaltennummernValue.clear();
        for neg in 0..=1usize {
            for i in 0..12usize {
                self.spaltenArtenKey_SpaltennummernValue.insert((neg, i), BTreeSet::new());
            }
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "paraMainDict={} paraDict={} dataDict0={} dataDict3={} kombi1={} kombi2={} newTable={} argvWithoutProgram={:?} beginDone={} runDone={} hoechsteZeile={} tableGenerated={} relitableRows={} RowsLen={} cliErrors={} sideParas={:?} resultingTableRows={} allesParameters={} spaltenNumbers={} ifPrint={} rowRangeLen={} shellRowsAmount={} shellWidth={} finallyDisplayLines={} ifZeilenSetToInf={} tables={} numlen={} old2Rows={} newerTable={} finallyDisplayLinesByChunks={} rowsOfcombi={} oldRows={} newerRows={} oldTable={} generatedSpaltenParameter={} generatedSpaltenParameterExact={} generatedSpaltenParameterTags={} allEquColumns={} finallyDisplayTable={} bigParamaeter={:?} obZeilenBereicheAngegeben={} breiteHasBeenOnceZero={} breiteORbreiten={} spaltenreihenfolgeundnurdiese={:?} puniverseprims={} generRows={} rowsAsNumbersNot={} rowsOfcombiNot={} htmlOrBBcode={} spaltenArtenKeys={}",
            self.paraMainDict.len(),
            self.paraDict.len(),
            self.dataDicts[0].len(),
            self.dataDicts[3].len(),
            self.kombiReverseDict.len(),
            self.kombiReverseDict2.len(),
            self.newTable,
            self.argvWithoutProgram,
            self.allImportantBeginThingsDone,
            self.runDone,
            self.hoechsteZeile,
            self.tableGenerated,
            self.relitable.len(),
            self.RowsLen,
            self.cliErrors.len(),
            self.sideParas,
            self.__resultingTable.len(),
            self.allesParameters,
            self.spaltenNumbers.len(),
            self.ifPrint,
            self.rowsRangeLen,
            self.shellRowsAmount,
            self.shellWidth,
            self.finallyDisplayLines.len(),
            self.ifZeilenSetToInf,
            self.tables.len(),
            self.numlen,
            self.old2Rows.len(),
            self.newerTable.len(),
            self.finallyDisplayLinesByChunks.len(),
            self.rowsOfcombi.len(),
            self.oldRows.len(),
            self.newerRows.len(),
            self.oldTable.len(),
            self.generatedSpaltenParameter.len(),
            self.generatedSpaltenParameter_Exact.len(),
            self.generatedSpaltenParameter_Tags.len(),
            self.allEquColumns.len(),
            self.finallyDisplayTable.len(),
            self.bigParamaeter,
            self.obZeilenBereicheAngegeben,
            self.breiteHasBeenOnceZero,
            self.breiteORbreiten,
            self.spaltenreihenfolgeundnurdiese,
            self.puniverseprims.len(),
            self.generRows.len(),
            self.rowsAsNumbersNot.len(),
            self.rowsOfcombiNot.len(),
            self.htmlOrBBcode,
            self.spaltenArtenKey_SpaltennummernValue.len()
        )
    }
}
