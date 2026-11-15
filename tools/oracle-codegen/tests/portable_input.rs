use cantrip_oracle_codegen::{corpus, cantrip, input::Record};
use std::{path::{Path,PathBuf},sync::atomic::{AtomicU64,Ordering}};
struct Files(PathBuf);
impl Files {fn new()->Self {static NEXT:AtomicU64=AtomicU64::new(0);let path=std::env::temp_dir().join(format!("codegen-input-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)));std::fs::create_dir(&path).unwrap();Self(path)}fn path(&self,name:&str)->PathBuf{self.0.join(name)}}
impl Drop for Files {fn drop(&mut self){let _=std::fs::remove_dir_all(&self.0);}}
fn fixture()->PathBuf{Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/records.jsonl")}
#[test]
fn standalone_ir_covers_single_and_multi_face_records_without_a_producer() {
    let cards=corpus::read_cards(&fixture()).unwrap();
    assert_eq!(cards.len(),7);
    let mut order=Vec::new();corpus::map_cards(&fixture(),|line,record|Ok((line,record.identity.names)),|line,row|{assert_eq!(line,row.0);order.push(row.1);Ok(())}).unwrap();
    assert_eq!(order,cards.iter().map(|r|r.identity.names.clone()).collect::<Vec<_>>());
    let bolt=cards.iter().find(|r|r.identity.names==["Lightning Bolt"]).unwrap();
    let code=cantrip::whole_module(&bolt.context,&bolt.card).unwrap();assert!(code.contains("g.damage"));
    for mut record in cards.into_iter().filter(|r|r.card.faces.len()>1) {
        assert!(record.context.primary_face.is_some());
        record.validate().unwrap();
        record.context.primary_face=None;assert!(record.validate().is_err());
        record.context.primary_face=Some("missing face".into());assert!(record.validate().is_err());
    }
}
#[test]
fn rejected_record_never_replaces_previous_generation() {
    let files=Files::new();let input=files.path("input.jsonl");let oracle=files.path("oracle.json");let output=files.path("rows.jsonl");
    let good=std::fs::read_to_string(fixture()).unwrap().lines().next().unwrap().to_owned();
    let mut bad:serde_json::Value=serde_json::from_str(&good).unwrap();
    bad["card"]["unknown_rule"]=serde_json::json!(true);
    std::fs::write(&input,format!("{good}\n{bad}\n")).unwrap();std::fs::write(&oracle,"[]").unwrap();std::fs::write(&output,"previous output").unwrap();
    let error=cantrip::planned::lane_rows(&input,&oracle,&output,None).unwrap_err().to_string();
    assert!(error.contains(":2:"),"{error}");assert!(error.contains("unknown or missing IR fields"),"{error}");
    assert_eq!(std::fs::read_to_string(output).unwrap(),"previous output");
}
#[test]
fn cli_rejects_versions_and_trailing_or_missing_records() {
    let files=Files::new();let input=files.path("input.jsonl");let good=std::fs::read_to_string(fixture()).unwrap().lines().next().unwrap().to_owned();
    for value in [format!("{good} extra"),format!("{good}\n\n"),good.replace("\"format_version\":1","\"format_version\":2")] {
        std::fs::write(&input,value).unwrap();
        assert!(!std::process::Command::new(env!("CARGO_BIN_EXE_cantrip-oracle-codegen")).args(["check-input",input.to_str().unwrap()]).output().unwrap().status.success());
    }
    let _:Record=serde_json::from_str(&good).unwrap();
}
