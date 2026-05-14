//! Prompt interaction controller transcompiled from
//! `python_arch_reference/reta_architecture/prompt_interaction.py`.
//!
//! This is the Rust composition layer that glues session, preparation,
//! execution and nested completion.  It does not run a terminal UI; it produces
//! deterministic one-shot interaction plans for the existing frontends.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::completion_nested::{bootstrap_nested_completion_morphisms, NestedCompletionMorphismBundle};
use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::prompt_execution::{bootstrap_prompt_execution, PromptExecutionBundle, PromptExecutionPlan};
use crate::prompt_language::{custom_split, PromptModus};
use crate::prompt_preparation::{bootstrap_prompt_preparation, PreparedPromptOutput, PromptPreparationBundle};
use crate::prompt_runtime::{bootstrap_prompt_runtime, PromptRuntimeBundle};
use crate::prompt_session::{bootstrap_prompt_session, PromptLoopSetup, PromptSessionBundle, PromptTextState};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptInteractionBundle {
    pub prompt_runtime: PromptRuntimeBundle,
    pub completion_runtime: CompletionRuntimeBundle,
    pub prompt_session: PromptSessionBundle,
    pub prompt_preparation: PromptPreparationBundle,
    pub prompt_execution: PromptExecutionBundle,
    pub nested_completion: NestedCompletionMorphismBundle,
    pub prompt_mode2: PromptModus,
    pub text_dazu0: Vec<String>,
    pub sprachen_wahl: String,
    pub info_log: bool,
    pub befehle: BTreeSet<String>,
    pub befehle2: BTreeSet<String>,
    pub befehle_beenden: BTreeSet<String>,
}

impl PromptInteractionBundle {
    pub fn snapshot(&self) -> PromptInteractionSnapshot {
        PromptInteractionSnapshot {
            class: "PromptInteractionBundle".to_string(),
            prompt_mode2: self.prompt_mode2,
            text_dazu0_len: self.text_dazu0.len(),
            befehle_len: self.befehle.len(),
            befehle2_len: self.befehle2.len(),
            befehle_beenden_len: self.befehle_beenden.len(),
            has_nested_completer: true,
            session_layer: "PromptSessionBundle".to_string(),
            preparation_layer: "PromptPreparationBundle".to_string(),
            execution_layer: "PromptExecutionBundle".to_string(),
        }
    }

    pub fn build_loop_setup(&mut self, argv: &[String]) -> PromptLoopSetup {
        let setup = self.prompt_session.build_loop_setup(argv);
        self.prompt_mode2 = setup.prompt_mode2;
        self.text_dazu0 = setup.text_dazu0.clone();
        self.befehle_beenden = setup.befehle_beenden.clone();
        setup
    }

    pub fn store_prompt(&mut self, chains: Vec<String>, placeholder: &str, text: &str) -> (Vec<String>, PromptTextState) {
        let result = self.prompt_session.store_prompt(chains, placeholder, text, self.prompt_mode2);
        self.prompt_mode2 = result.prompt_mode2;
        self.text_dazu0 = result.text_dazu0.clone();
        (result.chains, result.text_state)
    }

    pub fn delete_before_storage_commands(&self, placeholder: &str, text: &str) -> (String, PromptModus, String) {
        self.prompt_session.delete_before_storage_commands(placeholder, text)
    }

    pub fn apply_storage_output(
        &self,
        pending_output: &[String],
        prompt_mode: PromptModus,
        text_state: PromptTextState,
    ) -> PromptTextState {
        self.prompt_session.apply_storage_output(pending_output, prompt_mode, text_state)
    }

    pub fn prepare_and_plan_one_input(
        &mut self,
        placeholder: &str,
        input: &str,
        prompt_mode: PromptModus,
    ) -> PromptInteractionPlan {
        let text_state = self.prompt_session.new_text_state(input);
        let prepared = self.prompt_preparation.prepare_large_output(
            placeholder,
            prompt_mode,
            self.prompt_mode2,
            PromptModus::Normal,
            input,
            &self.text_dazu0,
        );
        let execution_plan = self.prompt_execution.plan_prompt_execution(&prepared, &text_state);
        let completions = self
            .nested_completion
            .complete(input)
            .into_iter()
            .map(|item| item.text)
            .collect::<Vec<_>>();
        PromptInteractionPlan {
            input: input.to_string(),
            prepared,
            execution_plan,
            completion_preview: completions.into_iter().take(12).collect(),
            resulting_mode: self.prompt_mode2,
            sprachen_wahl: self.sprachen_wahl.clone(),
        }
    }

    pub fn storage_command(
        &mut self,
        text_state: &mut PromptTextState,
        prompt_mode: PromptModus,
        chains: Vec<String>,
        pending_output: Vec<String>,
    ) -> PromptStorageDecision {
        let save_after = BTreeSet::from(["S".to_string(), "BefehlSpeichernDanach".to_string()]);
        let save_before = BTreeSet::from(["s".to_string(), "BefehlSpeichernDavor".to_string()]);
        let output_saved = BTreeSet::from(["o".to_string(), "BefehlSpeicherungAusgeben".to_string()]);
        let delete_saved = BTreeSet::from(["l".to_string(), "BefehlSpeicherungLöschen".to_string()]);
        let save_all = save_after.union(&save_before).cloned().collect::<BTreeSet<_>>();
        if text_state.has_without_abc(&save_after) && text_state.liste.len() == 1 {
            return PromptStorageDecision::handled(prompt_mode, PromptModus::Speichern, chains, pending_output);
        }
        if text_state.has_without_abc(&save_before) && text_state.liste.len() == 1 {
            let previous = text_state.befehl_davor.clone();
            let placeholder = text_state.platzhalter.clone();
            let (chains, new_state) = self.store_prompt(chains, &placeholder, &previous);
            *text_state = new_state;
            return PromptStorageDecision::handled(prompt_mode, PromptModus::Normal, chains, pending_output);
        }
        if !text_state.menge.difference(&save_all).next().is_none()
            && text_state.menge.intersection(&save_all).count() == 1
        {
            let storage_text = text_state
                .liste
                .iter()
                .filter(|token| !save_all.contains(*token))
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");
            let placeholder = text_state.platzhalter.clone();
            let (chains, new_state) = self.store_prompt(chains, &placeholder, &storage_text);
            *text_state = new_state;
            text_state.set_liste(Vec::new());
            text_state.set_text("");
            text_state.befehl_davor.clear();
            return PromptStorageDecision::handled(prompt_mode, PromptModus::Normal, chains, pending_output);
        }
        if text_state.has_without_abc(&output_saved) && text_state.liste.len() == 1 {
            return PromptStorageDecision::handled(prompt_mode, PromptModus::SpeicherungAusgaben, chains, pending_output);
        }
        if text_state.has_without_abc(&delete_saved) && text_state.liste.len() == 1 {
            let payload = custom_split(&text_state.platzhalter)
                .into_iter()
                .enumerate()
                .map(|(index, value)| format!("{}:{value}", index + 1))
                .collect::<Vec<_>>();
            return PromptStorageDecision {
                handled: true,
                previous_mode: prompt_mode,
                next_mode: PromptModus::LoeschenSelect,
                chains,
                pending_output: payload,
            };
        }
        PromptStorageDecision {
            handled: false,
            previous_mode: prompt_mode,
            next_mode: prompt_mode,
            chains,
            pending_output,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptInteractionSnapshot {
    pub class: String,
    pub prompt_mode2: PromptModus,
    pub text_dazu0_len: usize,
    pub befehle_len: usize,
    pub befehle2_len: usize,
    pub befehle_beenden_len: usize,
    pub has_nested_completer: bool,
    pub session_layer: String,
    pub preparation_layer: String,
    pub execution_layer: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptInteractionPlan {
    pub input: String,
    pub prepared: PreparedPromptOutput,
    pub execution_plan: PromptExecutionPlan,
    pub completion_preview: Vec<String>,
    pub resulting_mode: PromptModus,
    pub sprachen_wahl: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptStorageDecision {
    pub handled: bool,
    pub previous_mode: PromptModus,
    pub next_mode: PromptModus,
    pub chains: Vec<String>,
    pub pending_output: Vec<String>,
}

impl PromptStorageDecision {
    pub fn handled(
        previous_mode: PromptModus,
        next_mode: PromptModus,
        chains: Vec<String>,
        pending_output: Vec<String>,
    ) -> Self {
        Self {
            handled: true,
            previous_mode,
            next_mode,
            chains,
            pending_output,
        }
    }
}

pub fn bootstrap_prompt_interaction() -> PromptInteractionBundle {
    let completion_runtime = bootstrap_completion_runtime();
    PromptInteractionBundle {
        prompt_runtime: bootstrap_prompt_runtime(),
        completion_runtime: completion_runtime.clone(),
        prompt_session: bootstrap_prompt_session(),
        prompt_preparation: bootstrap_prompt_preparation(),
        prompt_execution: bootstrap_prompt_execution(),
        nested_completion: bootstrap_nested_completion_morphisms(),
        prompt_mode2: PromptModus::Normal,
        text_dazu0: Vec::new(),
        sprachen_wahl: "deutsch".to_string(),
        info_log: false,
        befehle: completion_runtime.start_commands(true).into_iter().collect(),
        befehle2: completion_runtime.befehle2.clone(),
        befehle_beenden: BTreeSet::from(["exit".to_string(), "quit".to_string()]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_input_plan_connects_preparation_execution_and_completion() {
        let mut interaction = bootstrap_prompt_interaction();
        let plan = interaction.prepare_and_plan_one_input("", "reta -ausgabe --art=h", PromptModus::Normal);
        assert!(plan.prepared.tokens.first().is_some_and(|token| token == "reta"));
        assert!(plan.completion_preview.iter().any(|item| item == "html"));
    }
}
