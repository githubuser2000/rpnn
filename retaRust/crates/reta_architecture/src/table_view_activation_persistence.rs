//! Persistence bridge for guarded table-view activation stores.
//!
//! Stage 45 made activation stores line-oriented and parseable.  This module
//! connects those stores to the existing dependency-free `PersistenceStore` so
//! a safe activation can be stored, loaded, audited and cached without changing
//! the visible output path.  It is intentionally still backend-agnostic: the
//! same morphism can later be backed by SQLite, a file, or FFI payloads.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::persistence::{
    PersistedRecord, PersistenceStore, stable_digest_text,
};
use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::table_view_activation_store::{
    TableViewActivationStore, TableViewActivationStorePolicy, activation_store_for_cli_args, parse_activation_store_text,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPersistencePolicy {
    pub store_policy: TableViewActivationStorePolicy,
    pub persist_store_text: bool,
    pub parse_after_load: bool,
    pub record_audit_event: bool,
    pub cache_latest_digest: bool,
    pub include_store_preview: bool,
    pub preview_limit: usize,
    pub store_kind: String,
    pub cache_prefix: String,
}

impl Default for TableViewActivationPersistencePolicy {
    fn default() -> Self {
        Self {
            store_policy: TableViewActivationStorePolicy::default(),
            persist_store_text: true,
            parse_after_load: true,
            record_audit_event: true,
            cache_latest_digest: true,
            include_store_preview: true,
            preview_limit: 8,
            store_kind: "table_view_activation_store".to_string(),
            cache_prefix: "table_view_activation_store:latest".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPersistenceReport {
    pub class: String,
    pub morphism: String,
    pub status: String,
    pub store_kind: String,
    pub store_name: String,
    pub source_transaction_id: Option<String>,
    pub source_line_count: usize,
    pub source_record_count: usize,
    pub source_validation_status: String,
    pub source_text_checksum: u64,
    pub source_text_digest: String,
    pub section_record: Option<PersistedRecord>,
    pub audit_record: Option<PersistedRecord>,
    pub cache_record: Option<PersistedRecord>,
    pub loaded_text_digest: Option<String>,
    pub loaded_matches_source: bool,
    pub parse_ready: bool,
    pub parse_validation_status: Option<String>,
    pub parse_failed_guards: Vec<String>,
    pub store_preview: Vec<String>,
    pub universal_property: String,
}

impl TableViewActivationPersistenceReport {
    pub fn is_ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPersistenceSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub store_tables: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationPersistenceBundle;

impl TableViewActivationPersistenceBundle {
    pub fn snapshot(&self) -> TableViewActivationPersistenceSnapshot {
        TableViewActivationPersistenceSnapshot {
            class: "TableViewActivationPersistenceSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_persistence.persist_store_text".to_string(),
                "table_view_activation_persistence.load_store_text".to_string(),
                "table_view_activation_persistence.parse_loaded_store".to_string(),
                "table_view_activation_persistence.record_audit_event".to_string(),
                "table_view_activation_persistence.cache_latest_digest".to_string(),
            ],
            store_tables: vec![
                "local_sections".to_string(),
                "audit_events".to_string(),
                "cache_entries".to_string(),
            ],
            universal_property:
                "persisted_activation_store_is_ready_only_when_load_parse_and_hash_match_the_source_store".to_string(),
        }
    }

    pub fn persist_store(
        &self,
        activation_store: &TableViewActivationStore,
        fallback_legacy_lines: &[String],
        current_transaction_id: Option<&str>,
        persistence: &mut PersistenceStore,
        policy: &TableViewActivationPersistencePolicy,
    ) -> TableViewActivationPersistenceReport {
        persist_activation_store_to_persistence(
            activation_store,
            fallback_legacy_lines,
            current_transaction_id,
            persistence,
            policy,
        )
    }

    pub fn persist_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        switch_config: &ArchitectureSwitchConfig,
        persistence: &mut PersistenceStore,
        policy: &TableViewActivationPersistencePolicy,
    ) -> TableViewActivationPersistenceReport {
        activation_persistence_for_cli_args(
            args,
            legacy_lines,
            switch_config,
            persistence,
            policy,
        )
    }
}

pub fn bootstrap_table_view_activation_persistence() -> TableViewActivationPersistenceBundle {
    TableViewActivationPersistenceBundle
}

pub fn activation_persistence_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    switch_config: &ArchitectureSwitchConfig,
    persistence: &mut PersistenceStore,
    policy: &TableViewActivationPersistencePolicy,
) -> TableViewActivationPersistenceReport {
    let activation_store = activation_store_for_cli_args(
        args,
        legacy_lines,
        switch_config,
        &policy.store_policy,
    );
    let current_transaction_id = activation_store.latest_transaction_id.as_deref();
    persist_activation_store_to_persistence(
        &activation_store,
        legacy_lines,
        current_transaction_id,
        persistence,
        policy,
    )
}

pub fn persist_activation_store_to_persistence(
    activation_store: &TableViewActivationStore,
    fallback_legacy_lines: &[String],
    current_transaction_id: Option<&str>,
    persistence: &mut PersistenceStore,
    policy: &TableViewActivationPersistencePolicy,
) -> TableViewActivationPersistenceReport {
    let store_name = activation_store
        .latest_transaction_id
        .clone()
        .unwrap_or_else(|| "activation-store-without-transaction".to_string());
    let source_text_digest = stable_digest_text(&activation_store.store_text);
    let store_preview = if policy.include_store_preview {
        activation_store
            .store_text
            .lines()
            .take(policy.preview_limit)
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let mut context = BTreeMap::new();
    context.insert("transaction_id".to_string(), store_name.clone());
    context.insert(
        "validation_status".to_string(),
        activation_store.validation.status.clone(),
    );
    context.insert(
        "record_count".to_string(),
        activation_store.record_count.to_string(),
    );
    context.insert(
        "line_count".to_string(),
        activation_store.line_count.to_string(),
    );
    context.insert("source_digest".to_string(), source_text_digest.clone());
    if let Some(chain_hash) = activation_store.latest_chain_hash {
        context.insert("latest_chain_hash".to_string(), chain_hash.to_string());
    }

    let section_record = if policy.persist_store_text {
        Some(persistence.persist_section(
            policy.store_kind.clone(),
            store_name.clone(),
            activation_store.store_text.clone(),
            Some(context),
        ))
    } else {
        None
    };

    let loaded_text = section_record
        .as_ref()
        .and_then(|record| persistence.load_section(&record.digest))
        .map(|section| section.payload_json.clone());
    let loaded_text_digest = loaded_text
        .as_ref()
        .map(|text| stable_digest_text(text));
    let loaded_matches_source = loaded_text_digest
        .as_ref()
        .map(|digest| digest == &source_text_digest)
        .unwrap_or(!policy.persist_store_text);

    let parse_report = if policy.parse_after_load {
        loaded_text.as_ref().map(|text| {
            parse_activation_store_text(
                text,
                fallback_legacy_lines,
                current_transaction_id.or(activation_store.latest_transaction_id.as_deref()),
                &policy.store_policy,
            )
        })
    } else {
        None
    };
    let parse_ready = parse_report
        .as_ref()
        .map(|report| report.validation.is_ready())
        .unwrap_or(!policy.parse_after_load);
    let parse_validation_status = parse_report
        .as_ref()
        .map(|report| report.validation.status.clone());
    let parse_failed_guards = parse_report
        .as_ref()
        .map(|report| report.validation.failed_guards.clone())
        .unwrap_or_default();

    let audit_record = if policy.record_audit_event {
        Some(persistence.record_audit_event(
            "table_view_activation_store.persist",
            store_name.clone(),
            format!(
                "transaction={store_name};status={};source_digest={source_text_digest};loaded_matches={loaded_matches_source};parse_ready={parse_ready}",
                activation_store.validation.status.as_str()
            ),
        ))
    } else {
        None
    };

    let cache_record = if policy.cache_latest_digest {
        Some(persistence.cache_put(
            format!("{}:{store_name}", policy.cache_prefix),
            source_text_digest.clone(),
        ))
    } else {
        None
    };

    let status = if activation_store.validation.is_ready()
        && loaded_matches_source
        && parse_ready
        && section_record.is_some() == policy.persist_store_text
    {
        "ready"
    } else {
        "blocked"
    };

    TableViewActivationPersistenceReport {
        class: "TableViewActivationPersistenceReport".to_string(),
        morphism: "table_view_activation_persistence.persist_store_text".to_string(),
        status: status.to_string(),
        store_kind: policy.store_kind.clone(),
        store_name,
        source_transaction_id: activation_store.latest_transaction_id.clone(),
        source_line_count: activation_store.line_count,
        source_record_count: activation_store.record_count,
        source_validation_status: activation_store.validation.status.clone(),
        source_text_checksum: activation_store.text_checksum,
        source_text_digest,
        section_record,
        audit_record,
        cache_record,
        loaded_text_digest,
        loaded_matches_source,
        parse_ready,
        parse_validation_status,
        parse_failed_guards,
        store_preview,
        universal_property:
            "persist_load_parse_is_identity_for_ready_activation_stores_and_blocks_on_store_drift".to_string(),
    }
}

pub fn continuum_m_activation_persistence_smoke() -> TableViewActivationPersistenceReport {
    let activation_store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
    let mut persistence = PersistenceStore::default();
    persist_activation_store_to_persistence(
        &activation_store,
        &activation_store
            .journal
            .records
            .last()
            .map(|record| record.selected_lines.clone())
            .unwrap_or_default(),
        activation_store.latest_transaction_id.as_deref(),
        &mut persistence,
        &TableViewActivationPersistencePolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_persistence_roundtrips_ready_store() {
        let activation_store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
        let mut persistence = PersistenceStore::default();
        let report = persist_activation_store_to_persistence(
            &activation_store,
            &activation_store
                .journal
                .records
                .last()
                .map(|record| record.selected_lines.clone())
                .unwrap_or_default(),
            activation_store.latest_transaction_id.as_deref(),
            &mut persistence,
            &TableViewActivationPersistencePolicy::default(),
        );
        assert!(report.is_ready(), "{report:?}");
        assert!(report.loaded_matches_source);
        assert!(report.parse_ready);
        assert!(report.section_record.is_some());
        assert!(report.audit_record.is_some());
        assert!(report.cache_record.is_some());
    }

    #[test]
    fn activation_persistence_detects_bad_loaded_text() {
        let activation_store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
        let mut persistence = PersistenceStore::default();
        let mut policy = TableViewActivationPersistencePolicy::default();
        policy.record_audit_event = false;
        policy.cache_latest_digest = false;
        let mut context = BTreeMap::new();
        context.insert("source_digest".to_string(), "tampered".to_string());
        let record = persistence.persist_section(
            policy.store_kind.clone(),
            "tampered".to_string(),
            activation_store.store_text.replace("H\t", "X\t"),
            Some(context),
        );
        let loaded_text = persistence
            .load_section(&record.digest)
            .map(|section| section.payload_json.clone())
            .unwrap();
        let parse_report = parse_activation_store_text(
            &loaded_text,
            &[],
            activation_store.latest_transaction_id.as_deref(),
            &policy.store_policy,
        );
        assert!(!parse_report.validation.is_ready());
    }
}
