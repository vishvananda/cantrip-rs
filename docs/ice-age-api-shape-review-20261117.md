# Intended gaps versus the interfaces that landed

The original 57-card pause list is the input, not all unsupported cards in the
sets. Forty-three now have native implementations; their original addition
commits are all authored by the maintainer account `jschibel`. Our shared engine
foundation work also landed and remains separately attributed in Git. This audit
cannot identify which agent is currently running.

The checked interfaces below implement the capabilities the old list called for.
The generator's proposed Rust names and table shapes often differ from the
native interfaces. A generator diagnostic containing “engine: no…” is therefore
not current evidence of a missing engine feature without checking the API.

| Capability / cards | Intended requirement | Landed shape and assessment |
|---|---|---|
| Ali from Cairo | Preserve damage, replacing only damage-caused life loss | [7ed832f151](https://github.com/cantrip-rs/cantrip-rs/commit/7ed832f151ad14fc96eae0ab2a3b9db495f6b46e): joint damage life results. The card uses `replacements::DAMAGE_CANT_REDUCE_YOUR_LIFE_BELOW_1`. Matches the requirement; differs from a proposed player-rule backend. |
| Arcum’s Weathervane | Lasting supertype changes in layer 4 | [98b61d030d](https://github.com/cantrip-rs/cantrip-rs/commit/98b61d030d8): `EffectKind::AddSupertypes` / `LoseSupertypes`, preserving other flags. Matches; the generator's hypothetical singular-supertype operations need adapting. |
| Barbarian Guides / Illusionary Presence | Every chosen land subtype, including snow qualification | [1dba6db4d9](https://github.com/cantrip-rs/cantrip-rs/commit/1dba6db4d901ba8a0bee12ff8ff7ed88ba3a78d4) and [d9396299d8](https://github.com/cantrip-rs/cantrip-rs/commit/d9396299d8778da8b852ce464f6aa14e23bc41a2): `Keyword::snow_landwalk_of` / `landwalk_of` over the land-type vocabulary. Matches without requiring a general arbitrary land-filter callback. |
| Camouflage | Replace blocker declaration with player piles and random assignment, while enforcing legality | [1acdbf48e9](https://github.com/cantrip-rs/cantrip-rs/commit/1acdbf48e9), subsequently corrected by [b5217364c6](https://github.com/cantrip-rs/cantrip-rs/commit/b5217364c6): `BlockersInstead` supplies card-local pile construction to the engine's declaration procedure. Same authority boundary; the card owns the particular pile algorithm. |
| Dark Sphere / Forcefield | Compute partial prevention from the pending damage | [4a775c8cea](https://github.com/cantrip-rs/cantrip-rs/commit/4a775c8ceaec11da3e788488411b8fb040d20183): `GrantedPreventionDef.portion` / `PreventionPortion`, with current `DamageEvent` amount. Matches; do not precompute the future amount when activating. |
| Divine Intervention | Know the remover and resulting counter total; draw the game | [c5cf1f8b46](https://github.com/cantrip-rs/cantrip-rs/commit/c5cf1f8b4610c588a6bbd4bb3f55f0517efbeea5): `CountersRemoved { total, by: Option<PlayerId>, ... }` and `Game::draw_game`. Matches. The old exact capability marker expected no `by` field and falsely reported a gap. |
| Night Soil | Exile two creature cards from one chosen graveyard, as a cost | [c5aae8e101](https://github.com/cantrip-rs/cantrip-rs/commit/c5aae8e101cc54c539ed7bacb76d4c4d9f6820cd): `exile_from_a_graveyard_of`, integrated into cost payment. Matches the authority requirement, with a dedicated cost constructor. |
| Ice Cauldron | Recover the last noted paid mana and restrict it to a linked card | [f4abd3c914](https://github.com/cantrip-rs/cantrip-rs/commit/f4abd3c914e90820de197de616efc5ec4574ab1a): `ManaRestriction::Card` plus `history::last_noted_mana` and linked exile history. The note can be derived from activation/resolution history; it need not become a separate central note registry. |
| Soul Burn | Restrict X payment and retain exactly which mana paid X | [3a2c77aebb](https://github.com/cantrip-rs/cantrip-rs/commit/3a2c77aebb777f09b0fc435276eb4665b051930f): `XMana::either` and `Event::SpentOnX`. Matches the stronger requirement; counting all mana spent on the spell would be wrong. |
| Library of Leng, still unimplemented | Recognize effect-caused discards in a live replacement | [f193ac95c4](https://github.com/cantrip-rs/cantrip-rs/commit/f193ac95c4cc2382d6da2c0f0b0ef777cc14cbd3): `Query::effect_controller()`. Simpler than our proposed complete discard-cause interface. The updated generator now produces a natively type-checked module. |
| Psychic Purge, still blocked | Retain the causing player for a trigger's later resolution | The live query above is insufficient: the discard event still has no frozen cause/controller. This remains an engine gap. |

The implementation/code for every original card and its old generator gap strings
is recorded in [the machine-readable audit](../tools/oracle-codegen/data/ice-age-backlog-20261117.json).
These table rows are the interfaces examined in detail, not a claim that all 43
complete cards or their interactions received a new independent scenario review.

## Generator adoption checkpoint

Schema 192, current vocabulary, live discard replacements, counter-removal
attribution/game draws, linked token acquisition/creating ability, first surviving
historical destinations and historical entry-sacrifice counts are adapted.
Explicit acting-player scopes no longer convert choosing one opponent into
processing every opponent or rebind the original ability controller.

Standalone generator tests pass (77); the pinned Source integration suite passes
607 tests; seven generated examples additionally type-check against the actual
native API. Native scenario/runtime coverage is a separate check. New type-check
fixtures and CI cover the interface-shape boundary directly.

This does **not** mean the remaining 42 landed cards already generate cleanly.
Their native implementations are useful specifications for the next adapter
batches: supertypes, landwalk, damage life results, partial prevention and then
more involved casting/payment/combat/history interfaces. Keep generator-only
work separate from the 13 genuinely missing engine capabilities listed in the
[backlog reassessment](ice-age-reassessment-20261117.md).

## First interface-family batch

Added native adapters for the four supertype flags, chosen ordinary/snow landwalk
over all land subtypes, and constant damage-related life floors. Life floors use
the shared joint-life-results replacement callbacks, preserving damage and
lifelink. Shared **basic** land types retain the basic-only lookup; unrestricted
land types use the new full lookup.

Ali from Cairo, Arcum’s Weathervane, Barbarian Guides and Illusionary Presence now
generate without gaps and type-check against the real API. The current smoke
fixture contains **11 examples**, and **80 generator tests** pass. Relevant
Source regressions (8 tests) also pass. This brings the original43 to **5 cleanly
generated / 38 awaiting adapters**, plus the newly ready Library of Leng.
The earlier checkpoint figures above are historical.

## Second interface-family batch

Partial granted prevention now uses the native `PreventionPortion` callback,
reading the pending damage amount when damage happens. Prevented-damage life
gain uses `gain_prevented` in the same damage event; other follow-up effects retain
the separate callback. Optional prevention, fixed per-event allowances and
modified/redirected prevention life gain remain explicit gaps where the current
interface has not been validated.

Dark Sphere and Forcefield now generate without gaps. The original 43 are now
**7 cleanly generated / 36 awaiting adapters**, plus Library of Leng. The fixture
contains **16 natively type-checked examples**; **84 generator tests** and **29
affected Source tests** pass (one ignored). The Source preserved-program count
was updated from 7 clean / 4 gaps to 8 clean / 3 gaps, reflecting the newly
supported native interface.
