//! Standalone IR-only command line. Input comes from any versioned IR producer.
use cantrip_oracle_codegen::{cantrip, corpus, input::Record};
use std::{io::Write, path::Path};

fn write_json(path: &str, value: &impl serde::Serialize) -> corpus::Result<()> {
    let mut out = corpus::Artifact::create(Path::new(path))?;
    serde_json::to_writer_pretty(&mut out, value)?;
    writeln!(out)?; out.commit()?; Ok(())
}
fn main() -> corpus::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [cmd,input,oracle,output] if cmd=="rows" || cmd=="cantrip-files" => {
            let cache=cantrip::pr_cache::default_dir();
            let n=cantrip::planned::lane_rows(Path::new(input),Path::new(oracle),Path::new(output),cache.as_deref())?;
            eprintln!("{n} IR records written");
        }
        [cmd,input,oracle,output] if cmd=="tracked" || cmd=="cantrip-tracked" => {
            let cache=cantrip::pr_cache::default_dir();
            cantrip::planned::tracked_cached(Path::new(input),Path::new(oracle),Path::new(output),cache.as_deref())?;
        }
        [cmd,input,oracle,output,features @ ..] if cmd=="candidate" || cmd=="cantrip-candidate" => {
            let features: Vec<&str>=features.iter().map(String::as_str).collect();
            cantrip::planned::with_additional_features(&features,||cantrip::planned::tracked(Path::new(input),Path::new(oracle),Path::new(output)))?;
        }
        [cmd,input,names @ ..] if cmd=="module" || cmd=="cantrip-module" => {
            for (name,module) in cantrip::modules_named(Path::new(input),names)? { println!("// ===== {name}\n{module}"); }
        }
        [cmd,input,output] if cmd=="survey" || cmd=="cantrip-gaps" => write_json(output,&cantrip::survey(Path::new(input))?)?,
        [cmd,input] if cmd=="check-input" => {
            let mut n=0;
            corpus::for_each_card(Path::new(input),|_,record:Record|{record.validate()?;n+=1;Ok(())})?;
            eprintln!("validated {n} IR records");
        }
        _ => return Err("usage: cantrip-oracle-codegen rows|tracked|candidate <ir-records.jsonl> <oracle.json> <output.jsonl> [feature...]\n       cantrip-oracle-codegen module <ir-records.jsonl> <name>...\n       cantrip-oracle-codegen survey <ir-records.jsonl> <report.json>\n       cantrip-oracle-codegen check-input <ir-records.jsonl>".into()),
    }
    Ok(())
}
