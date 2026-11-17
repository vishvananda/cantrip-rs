#!/usr/bin/env python3
"""Compile generated API examples without building the engine or card registry."""
import argparse
import json
import os
import re
from pathlib import Path
import shutil
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cantrip", type=Path, required=True)
    parser.add_argument("--records", type=Path)
    parser.add_argument("--generator", type=Path)
    args = parser.parse_args()
    tooling = Path(__file__).resolve().parent
    native = args.cantrip.resolve()
    records = (args.records or tooling / "tests/fixtures/native_interface_records.jsonl").resolve()
    generator = (args.generator or tooling / "target/debug/cantrip-oracle-codegen").resolve()
    if not generator.is_file():
        parser.error("build the generator first: cargo build --locked --manifest-path tools/oracle-codegen/Cargo.toml")
    for name in ("cantrip-api", "cantrip-mechanics"):
        if not (native / "crates" / name / "Cargo.toml").is_file():
            parser.error(f"missing {name} in {native}")
    # Keep dependency artifacts across runs; each generated project is isolated
    # and removed even if generation or compilation fails.
    cache = tooling / "target/native-interface-check"
    cache.mkdir(parents=True, exist_ok=True)
    names = [face["name"] for line in records.read_text().splitlines()
             for face in json.loads(line)["card"]["faces"]]
    if len(names) != len(set(names)) or not names:
        parser.error("smoke records must have distinct nonempty face names")
    with tempfile.TemporaryDirectory(prefix="project-", dir=cache) as temporary:
        project = Path(temporary)
        (project / "src").mkdir()
        manifest = ['[package]', 'name="cantrip-generated-interface-check"', 'version="0.0.0"',
                    'edition="2024"', '[workspace]', '[dependencies]']
        for name in ("cantrip-api", "cantrip-mechanics"):
            manifest.append(f'{name}={{path={json.dumps(str(native / "crates" / name))}}}')
        manifest.extend(['[profile.dev]', 'debug=0'])
        (project / "Cargo.toml").write_text("\n".join(manifest) + "\n")
        # Seed exact native dependency versions; Cargo adds this one local
        # package to the temporary lock, leaving the checkout's lock unchanged.
        shutil.copy2(native / "Cargo.lock", project / "Cargo.lock")
        modules = ['#![allow(dead_code,unused_imports,unused_variables,unused_mut)]']
        for index, name in enumerate(names):
            generated = subprocess.check_output([str(generator), "module", str(records), name], text=True)
            if not re.search(r"^pub static CARD:\s*CardDef\s*=", generated, re.MULTILINE):
                raise RuntimeError(f"{name}: generator did not produce a card definition:\n{generated}")
            (project / "src" / f"card_{index}.rs").write_text(generated)
            modules.append(f'pub mod card_{index};')
        (project / "src/lib.rs").write_text("\n".join(modules) + "\n")
        subprocess.run(["cargo", "check", "--manifest-path", str(project / "Cargo.toml"),
                        "--target-dir", str(cache / "target")], check=True,
                       env={**os.environ, "CARGO_INCREMENTAL": "0"})
        print(f"Compiled {len(names)} generated cards against the native API: {', '.join(names)}")


if __name__ == "__main__":
    main()
