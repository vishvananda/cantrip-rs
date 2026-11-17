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

## Declaration, history and cost adapters

Camouflage uses the native blocker-declaration replacement, leaving block
legality to the core. Its pile procedure respects each creature’s block capacity,
permits empty piles and assigns each pile to a distinct attacker at random.
Rasputin uses the first tap/untap transition this turn, rejects a newly entered
object and preserves phased-out identity. Its counter ceiling is a state-based
action table, not a cap on placement. This self-orientation query does not supply
Power Surge’s historical board snapshot.

Derelor uses the native colored mana cost increase, keeping color-word changes
separate from the printed mana symbol. Night Soil chooses a single graveyard
while paying the cost. Unsupported set constraints, counts and self-exile from a
graveyard activation remain gaps.

The original 43 now have **11 cleanly generated / 32 awaiting adapters**, plus
Library of Leng. **90 generator tests** and **20 native interface examples** pass.
Focused Source cost/early-set tests pass; the full Source sweep is in progress.
The old Tangle snapshot now includes the prevention defaults required by the
expanded native struct; those were the only differences in its CI failure.

## Block costs, retargeting and pending-event adapters

Hipparion uses the native mana block-tax callback. Reflecting Mirror keeps its
single-target and targets-you conditions while delegating the restricted change
to native target legality. Soul Burn restricts only X and counts actual black mana
from `SpentOnX`, not the whole payment or as-though colors. Repeatable activation
identity, delayed bodies and snow provenance are deliberately not inferred from
that spell receipt. Freyalise’s Winds records counter removal on the same native
untap proposal, restricted to the permanent controller’s untap step; unmatched
proposal-side instructions remain explicit gaps.

The original 43 are now **15 cleanly generated / 28 awaiting adapters**, plus
Library of Leng. **95 generator tests** and **24 native interface examples** pass.
The preceding full Source sweep had 606 passes, 12 ignored and one stale Alabaster
Leech expectation; that is fixed and all 46 tests in the five affected targets
pass (one ignored). Tangle’s exact snapshot also passes with the new defaults.

## Damage shields and scoped replacements

Errant Minion and Power Leak use the native damage-with-prevention interface;
prevention remains a shield, not subtraction from the damage amount. An arbitrary
mana payment uses repeated {1} payments, not an {X} cost, preserving restrictions
on mana that can pay only an X cost. Runesword uses the native regeneration ban
on an open set. The native API distinguishes object-scoped replacements with explicit `to` and
`into` fields from `ReplacementRule` history/global effects; lifetime details are
covered in the following batch.

Whippoorwill’s legacy named rule represents both prevention and redirection bans:
its Source is `DamageDealtToItCantBePreventedOrRedirected`. Both native lasting
rules are emitted. A move of the watched dying creature follows the actual card
produced by that death through `EventDestination`; reads of the old event subject
are unchanged, and the effect never chases later zone moves. The adapter rewrite
is limited to known battlefield-to-graveyard watches and stops at nested future
abilities. No Source, reference or owned schema was changed.

The original 43 are now **19 cleanly generated / 24 awaiting adapters**, plus
Library of Leng. **99 generator tests**, including generation of every native
fixture, pass. **28 actual modules** compile against the native API. The checker
previously accepted diagnostic-only comments as empty modules; it now rejects
those, and a deliberately unsupported Word of Command record verifies failure.
The current 28 fixtures all pass the strict check. All 28 focused Source tests
pass (one ignored), including delayed source/subject and legacy rule profiles.

## Draw-step rules, cancelable delays and post-move instructions

Fasting uses the optional native draw-step replacement, with its benefit in the
chosen skip callback. Nafs Asp retains the damaged player, makes one delayed
ability and a matching special action, and expires that payment opportunity when
the next draw step begins. Unsupported deadline/cost/capture layouts stay gaps.
Land Equilibrium binds the **putting player** (`put_by`), not the incoming
permanent’s controller, and sacrifices after the battlefield arrival. Unmodeled
new-object receipts and granted/mixed post-move callbacks are refused.

Ordinary object-specific entry replacements use `Replacement` with empty `to`
and `into`, preserving their lifetime when the object leaves. Captured history
rules such as Runesword use `ReplacementRule`, which intentionally outlives that
object. Semester’s End is now a native fixture for the former contract.

The original 43 are **22 cleanly generated / 21 awaiting adapters**, plus Library
of Leng. **103 generator tests** and **32 real native modules** pass. Full CI for
the preceding interface batch passed all 607 Source tests (12 ignored). The local
batch-six full sweep found only the old Land Equilibrium expected-gap string;
that is corrected, with all six affected Source tests passing. The later family5
CI failure was another stale Semester’s End field assertion, now corrected to
the fully specified scoped replacement. No engine/card registrations changed.


## Prevention follow-ups and damage redirection

Rock Hydra uses a relative native prevention amount and removes the counters in
its prevention follow-up, including when the damage cannot be prevented.
Silhouette asks `Query::instructing()` about the spell or ability whose instruction
causes the damage; it keeps the protected creature's identity in the effect.
Lava Burst's unconditional printed damage rule emits both native damage callbacks,
restricted to creature recipients. Conditional instructions/presence are not
silently promoted to unconditional card rules.

Jade Monolith and Nova Pentacle retain three separate identities: the chosen
damage source, the protected recipient and, when needed, the new recipient.
Nova Pentacle uses the native opponent target chooser and stored `into` value;
its future callback does not reread the original activation's targets. Personal
Incarnation consumes one point through `DamageComponent::Redirected`, preserving
the original source/combat status within the same event. Larger cumulative
redirection budgets remain explicit gaps rather than emitting a nonexistent
shield variant or moving the entire damage assignment.

Six more original cards are clean: **28 of43**, with **15 awaiting adapters or
reference corrections**, plus Library of Leng. **108 generator tests** and
**39 real native interface modules** pass. The additional new control is Dawn
Elemental, ensuring ordinary static prevention emits the new empty `then` field.
The preceding full CI passed **607 Source tests** (12 ignored).

**Sacred Boon is a reference hold, not an engine gap.** The published Grammar
0.3.4 bundle nests end-step trigger registration inside the shield's callback
and reads an individual prevention occurrence. It must create one delayed
ability when the spell resolves and later read the shield's cumulative prevented
amount. The same defect occurs in Scars of the Veteran. This is reported as
[Oracle Grammar #35](https://github.com/vishvananda/oracle-grammar/issues/35),
with exact published-bundle provenance. MTGish already fixed the Source grammar
in closed #144; no duplicate Source report or upstream pull is needed. The
native Cantrip card already implements the correct cumulative-history behavior.


## Copied mana, historical controllers and special actions

Merseine copies the enchanted object's **full mana cost**, retaining colored,
hybrid and Phyrexian symbols and setting X to zero before reductions. Missing
mana costs remain unpayable, including face-down objects and locked Rooms;
they are not treated as payable zero. The ability keeps its noncontroller
activation permission. Combinations with unsupported cost changes/conditions
stay explicit gaps rather than overwriting an existing callback.

A controller-at-last-block query walks this turn's matching block and control
change records backwards and stops at the last block by the specified Wall.
No matching block yields no player, never the current controller as a fallback.
This removes Glyph of Reincarnation's backend query gap, but the face has a
**separate reference hold**: published IR lacks the died-this-way restriction
on the destruction outcomes. [Oracle Grammar #36](https://github.com/vishvananda/oracle-grammar/issues/36)
includes provenance, a counterexample with exile replacing death, and a request
to check simultaneous return timing. The Source parser already distinguishes
“died this way”; this is a semantic lowering/reference issue, not a new MTGish
source-grammar bug. An existing backend Oracle-text heuristic must not substitute
for the missing typed distinction.

Guardian Angel creates one target-bound native player special action; each
payment creates a separate prevention shield on the stored recipient. This does
not retarget the object or consume the action after one use. Other timing,
cost and capture layouts remain gaps. Melee uses the native block chooser rule
through end of combat, retaining its separate combat-long unblocked trigger.

The original43 now have **32 without generator diagnostics**, **11 with generator
holds**, plus Library of Leng. Two face/reference corrections are tracked
separately: Sacred Boon and Glyph of Reincarnation. Thus 32 compiling generator
outputs must not be read as 32 newly audited Oracle-correct implementations.
**112 generator tests** and **43 real native modules** pass. The preceding full
Source regression sweep passed **607 tests** (12 ignored).
Family-eight focused Source checks additionally passed **37 tests** (one ignored).
