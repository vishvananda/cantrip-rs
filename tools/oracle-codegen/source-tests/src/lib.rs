//! Test-only adapter. Production generator dependencies contain no Source parser.
pub use mtgish_source::*;
pub mod cantrip {
    pub use cantrip_oracle_codegen::cantrip::*;
    use cantrip_oracle_codegen::{corpus::Result, input::LayoutContext};
    use mtgish_source::{ir::UnifiedCard, mtg_types::OracleCard};
    use std::{collections::BTreeMap, path::Path, sync::atomic::{AtomicU64, Ordering}};
    fn context(source: &OracleCard) -> std::result::Result<LayoutContext, Diagnostic> {
        mtgish_source::artifacts::record(source)
            .and_then(|record| Ok(serde_json::from_value(record["context"].clone())?))
            .map_err(|error| Diagnostic {path:"layout".into(),feature:"invalid producer context".into(),message:error.to_string()})
    }
    pub fn whole(source:&OracleCard,card:&UnifiedCard)->std::result::Result<Whole,Diagnostic> {
        cantrip_oracle_codegen::cantrip::whole(&context(source)?,card)
    }
    pub fn whole_printed(source:&OracleCard,card:&UnifiedCard,oracle:Option<&BTreeMap<String,String>>)->std::result::Result<Whole,Diagnostic> {
        cantrip_oracle_codegen::cantrip::whole_printed(&context(source)?,card,oracle)
    }
    pub fn whole_module(source:&OracleCard,card:&UnifiedCard)->std::result::Result<String,Diagnostic> {
        cantrip_oracle_codegen::cantrip::whole_module(&context(source)?,card)
    }
    pub fn whole_gaps(source:&OracleCard,card:&UnifiedCard)->Vec<Diagnostic> {
        match context(source) {Ok(context)=>cantrip_oracle_codegen::cantrip::whole_gaps(&context,card),Err(error)=>vec![error]}
    }
    struct Temporary(std::path::PathBuf);
    impl Drop for Temporary {fn drop(&mut self){let _=std::fs::remove_file(&self.0);}}
    fn with_ir<T>(source:&Path,run:impl FnOnce(&Path)->Result<T>)->Result<T> {
        static NEXT:AtomicU64=AtomicU64::new(0);
        let file=Temporary(std::env::temp_dir().join(format!("cantrip-source-ir-{}-{}.jsonl",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed))));
        mtgish_source::artifacts::export(source,&file.0)?;
        run(&file.0)
    }
    pub fn modules_named(input:&Path,names:&[String])->Result<Vec<(String,String)>> {
        with_ir(input,|ir|cantrip_oracle_codegen::cantrip::modules_named(ir,names))
    }
    pub fn modules_printed(input:&Path,oracle:&Path,names:&[String])->Result<Vec<(String,String)>> {
        with_ir(input,|ir|cantrip_oracle_codegen::cantrip::modules_printed(ir,oracle,names))
    }
    pub mod planned {
        pub use cantrip_oracle_codegen::cantrip::planned::*;
        use super::*;
        pub fn tracked(input:&Path,oracle:&Path,output:&Path)->Result<usize> {
            with_ir(input,|ir|cantrip_oracle_codegen::cantrip::planned::tracked(ir,oracle,output))
        }
        pub fn tracked_cached(input:&Path,oracle:&Path,output:&Path,cache:Option<&Path>)->Result<usize> {
            with_ir(input,|ir|cantrip_oracle_codegen::cantrip::planned::tracked_cached(ir,oracle,output,cache))
        }
        pub fn lane_rows(input:&Path,oracle:&Path,output:&Path,cache:Option<&Path>)->Result<usize> {
            with_ir(input,|ir|cantrip_oracle_codegen::cantrip::planned::lane_rows(ir,oracle,output,cache))
        }
    }
}
