# Oracle IR to Cantrip

Standalone tooling over published `oracle-ir` 0.2.0 and `oracle-ir-contract` 0.2.1,
schema 191. This crate is excluded from the engine workspace. Normal builds do
not depend on MTGish, its grammar, or the block editor.

```sh
cargo build --locked --manifest-path tools/oracle-codegen/Cargo.toml
BIN=tools/oracle-codegen/target/debug/cantrip-oracle-codegen
"$BIN" check-input records.jsonl
"$BIN" rows records.jsonl oracle.json rows.jsonl
"$BIN" module records.jsonl 'Lightning Bolt'
"$BIN" survey records.jsonl gaps.json
"$BIN" candidate records.jsonl oracle.json candidate.jsonl discard-causation
```

`tracked` retains proposed API holes; `rows` is the lane input containing each
card's executable file or first gap. Unsupported engine interfaces remain explicit
and are not registered as playable cards. A successful IR validation is not a
claim of engine support. Additional capability profiles are explicit opt-ins.
`CANTRIP_CODEGEN_THREADS` limits corpus parallelism (`MTGISH_THREADS` remains an
alias). The ordered corpus workers use dedicated large stacks; no recursive IR
representation or integer range changed in the extraction.

## Input boundary

JSONL records use this envelope (every field required):

```json
{
  "format_version": 1,
  "card": {"schema_version": 191, "layout": "Card", "data": [], "faces": []},
  "context": {"primary_face": "Card name", "hand_modifier": 0, "life_modifier": 0},
  "identity": {"layout": "Card", "names": ["Card name"], "source_sha256": "producer hash"},
  "diagnostics": []
}
```

The example illustrates the envelope; an actual card must contain valid owned
IR faces. Unknown/missing IR fields, unsupported record versions, invalid scope
or layout roles fail instead of silently losing data. Multi-face records require
an explicit primary (front/main/left) face because face-array order is not that
contract. Signed Vanguard modifiers are preserved. A producer retains its own
layout label and hash in `identity`; no Source syntax is interpreted here.

The MTGish fork supplies `ast export-ir-records INPUT.ronl OUTPUT.jsonl`.
Other producers can serialize the same envelope from their owned IR and explicit
layout roles. `oracle.json` is separate printed metadata, currently the normalized
MTGish export array (`name` and `text`, grouped by layout); it is not used to parse
or lower rules. Preserve its exact snapshot with the IR when reproducing a run.

## Validation and provenance

```sh
cargo test --locked --manifest-path tools/oracle-codegen/Cargo.toml
python3 -m unittest discover -s tools/card-lane
python3 -m unittest discover -s tools -p 'test_*.py'
```

The `source-tests` subworkspace preserves the former backend regression suite.
It is an explicit cross-producer integration check with a Git-pinned MTGish
lowerer, separate from the normal generator dependency graph. See its README.

Extraction from MTGish `5a0546450b7ad3ba07b583dfb749051b7b763a9b` preserved
all 33,529 generated rows byte for byte, including Rust, gaps and face ordering.
The previously staged discard-causation generator work moved with this code;
it remains behind that capability profile. Its engine packet is separately on
`engine/ice-age-discard-attribution-20261114`, not part of this tooling branch.
Historical generator signatures retain their original names to preserve cached
output and traceability. Snapshot audit data lives in `data/` and `TRACKER.md`.
`data/source_rewrite_inputs.json` freezes the original producer rewrite keys for
the tracker audit; it is provenance data, not a grammar or parsing dependency.
Refresh it explicitly with the producer snapshot rather than reading another checkout.

The original generator is MIT licensed; see the retained LICENSE. Python tools
and lane docs moved without campaign databases, credentials or build caches.
