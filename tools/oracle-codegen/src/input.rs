//! Versioned producer-neutral transport: owned IR, layout roles and provenance.
//! Roles preserve front/left identity without embedding a parser's Source tree.
use crate::{corpus::Identity, ir::UnifiedCard};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutContext {
    pub primary_face: Option<String>,
    pub hand_modifier: i64,
    pub life_modifier: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub format_version: u32,
    pub card: UnifiedCard,
    pub context: LayoutContext,
    pub identity: Identity,
    pub diagnostics: Vec<serde_json::Value>,
}

impl Record {
    pub fn validate(&self) -> Result<(), String> {
        if self.format_version != 1 { return Err("unsupported IR record format".into()); }
        crate::validate::unified(&self.card)?;
        if let Some(name) = &self.context.primary_face {
            if !self.card.faces.iter().any(|face| &face.name == name) {
                return Err("primary face is absent from the IR record".into());
            }
        }
        if self.card.faces.len() > 1 && self.context.primary_face.is_none() {
            return Err("multi-face IR record needs an explicit primary face".into());
        }
        Ok(())
    }
}
