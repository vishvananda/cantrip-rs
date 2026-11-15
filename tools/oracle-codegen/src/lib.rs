//! Cantrip backend over the published Oracle IR. No parser or MTGish dependency.
pub use oracle_ir_contract::{battlefield_query, counter, declarations, ir, ir_schema,
    keyword_actions, movement, mode_policies, result_reads, subtype, targets, validate, visit,
    coin_policies, complete, change_lists};
pub use oracle_ir::mtg_types;
pub mod cantrip;
mod cantrip_vocab;
pub mod corpus;
pub mod input;
mod semantics;
mod term_shapes;
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Diagnostic { pub path: String, pub feature: String, pub message: String }
pub use cantrip::{module, gaps};
