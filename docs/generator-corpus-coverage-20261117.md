# Whole-corpus generator coverage, 2026-11-17

The latest native-adapter pass unlocks **63 additional complete cards** across
the full frozen corpus. **62 compile against the actual native API**; Nine Lives
also generates but needs the `Incarnation` counter registered in Cantrip.

Across the complete adapter series since schema 192 adoption, **113 new cards**
generate, of which **112 native-type-check**. Two previously accepted outputs,
Samite Ministration and Shadowbane, are now explicitly held for conditional or
modified prevention life gain. The net diagnostic-free increase is **111**.

| Revision | Complete cards without generator diagnostics |
|---|---:|
| Adapter-series baseline `4db7b5d7c9` | 29,086 |
| Before the latest pass `3495f5bbb6` | 29,134 |
| Current generator, including the callback correction | 29,197 |

The input is the same frozen 20261001.1 population for all three revisions:
33,529 records, of which 33,457 are cards. The other 72 are 48 records without
faces and 24 token definitions. No new Oracle snapshot was imported. The
comparison uses the same published schema 192 IR, Oracle text, dependencies and
native API fingerprint. Multi-face cards count once and must pass as a whole.
Each diagnostic-free output was parsed as Rust and checked for a root CARD.
The 112 newly unlocked modules were additionally compiled together against the
actual `cantrip-api` and `cantrip-mechanics` crates. This does not certify runtime
scenarios or the correctness of every reference. Glyph of Reincarnation's
separate reference hold remains visible in the early-set audit.

The full-corpus API check exposed a shared prevention callback mismatch missed
by the earlier smoke examples: a printed `PreventionDef::then` takes before/after
proposals, whereas a granted prevention takes a slice of actual prevented parts.
The generator now keeps those contracts separate. Printed follow-ups still run
when prevention is prohibited; their event amount remains the original proposed
damage, and an explicit performed-prevention receipt separately reads the actual
amount prevented. New Phantom Centaur, Gloom Surgeon and Anti-Venom fixtures
exercise that distinction. All 117 generator tests, 50 native smoke modules and 15
affected Source checks pass; all 112 eligible full-corpus gain modules type-check.

The [machine-readable report](../tools/oracle-codegen/data/corpus-coverage-20261117.json)
contains revision hashes, input hashes, exact card lists, the one counter
prerequisite and both newly held cards. Full comparison rows, historical sources
and native compiler receipts are under
`/mnt/vizier/ice-age-reassessment-20261117/corpus-comparison/`.
