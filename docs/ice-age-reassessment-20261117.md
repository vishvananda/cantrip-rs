# Ice Age and earlier: remaining backlog reassessment

Audited 2026-11-17 against upstream Cantrip `5441ecd742c2e40e908d0347f56afa520005a89b`. Fork `main` was fast-forwarded and pushed to that revision. Generator changes live on `tools/oracle-codegen-20261117`, based on that same revision.

Of the original paused **57 cards**, **43 now have landed implementations and metadata**, **Library of Leng is newly implementation-ready**, and **13 still require engine authority or event information**. The original exclusions remain: Shahrazad, ante and dexterity cards. Flavor-banned cards remain in scope.

The generator consumes published `oracle-ir` / `oracle-ir-contract` **0.3.0**, schema **192**, associated with Grammar **0.3.4**. MTGish retains its frozen **20261001.1** input: **33,529** validated exports and zero lowering diagnostics. The newer face release is provenance, not a replacement Oracle population.

## Newly ready

**Library of Leng** no longer needs the proposed discard-causation interface. Its replacement is checked while the discard happens, where `Query::effect_controller()` distinguishes an effect from a cost/rules discard. The generator now uses this existing interface behind its detected capability. Optional replacement and unlimited hand size were already available. The generated module compiles against the current engine API. Implementation and positive/negative runtime scenarios remain a separate step.

**Psychic Purge remains blocked**: its trigger must retain the causing effect’s controller until resolution. `Event::Discarded` still contains only the discarding player, card, and undefined flag. A live query at trigger resolution is not frozen attribution.

## Remaining engine work

| Card | Required engine feature |
|---|---|
| Drain Power | Force the target player to activate each land’s mana ability, then transfer the actual emptied mana receipt. See [game.rs](../crates/cantrip-api/src/game.rs). |
| Equinox | Authoritative prediction that a spell would destroy a land; current target legality alone is insufficient. See [card.rs](../crates/cantrip-api/src/card.rs). |
| Field of Dreams | Reveal every player’s library top, including a player without a permanent; existing RevealLibraryTop applies to the source controller. See [card.rs](../crates/cantrip-api/src/card.rs). |
| Ghostly Flame | Treat damage sources as colorless only for damage; retain their colors for other purposes. See [card.rs](../crates/cantrip-api/src/card.rs). |
| Guardian Beast | Prevent new Aura enchantment and control changes while preserving already attached Auras; NoOtherAuras has different semantics. See [card.rs](../crates/cantrip-api/src/card.rs). |
| Illusionary Mask | Preserve the actual X-payment mana and restrict face-down casting to payable creature costs; replace damage assignment, damage reception and becoming tapped with turning face up. See [game.rs](../crates/cantrip-api/src/game.rs). |
| North Star | One-use, this-turn permission to spend mana as any type for one spell’s mana cost only, leaving additional costs unchanged. Existing SpendAnyColorRule covers a whole payment and does not supply this permission. See [card.rs](../crates/cantrip-api/src/card.rs). |
| Power Surge | Authoritative untapped-land snapshot as the turn began, before untapping; TurnBegan records do not carry that board snapshot. See [event.rs](../crates/cantrip-api/src/event.rs). |
| Psychic Purge | Freeze the controller of the discard-causing effect in event/trigger data for later resolution; live effect_controller is insufficient. See [event.rs](../crates/cantrip-api/src/event.rs). |
| Season of the Witch | Historical attack eligibility/opportunities for creatures that did not attack; current legality or attacked_this_turn does not answer “couldn’t attack.” See [history.rs](../crates/cantrip-api/src/history.rs). |
| Sunglasses of Urza | Directed white-as-red mana spending substitution; the unrestricted spend-any-color callback is insufficient. See [card.rs](../crates/cantrip-api/src/card.rs). |
| Wall of Shadows | Determine whether a spell/ability can target only Walls, across its target slots/modes; current selected-target legality is insufficient. See [card.rs](../crates/cantrip-api/src/card.rs). |
| Word of Command | Control a player during this resolution/selected spell resolution, force the selected play, and restrict which mana abilities/resources can pay; next-turn control is insufficient. See [game.rs](../crates/cantrip-api/src/game.rs). |

**North Star:** the old backlog description was stale. The frozen Oracle text grants one spell this turn any-type spending for its mana cost, with additional costs unchanged. Its current generated special-action/PlayWith gap labels are implementation details, not a faithful description of the required permission.

The current North Star reference also has a contract/lowering gap: it models
this permission as a hand-only special casting action and does not identify
mana-cost-only payment scope. Reported as
[Oracle Grammar #34](https://github.com/vishvananda/oracle-grammar/issues/34).
That is separate from the 987 already-amended reference records below.

## Landed since the old backlog

- [Ali from Cairo](../crates/cantrip-cards/src/cards/ali_from_cairo.rs)
- [Arcum's Weathervane](../crates/cantrip-cards/src/cards/arcums_weathervane.rs)
- [Barbarian Guides](../crates/cantrip-cards/src/cards/barbarian_guides.rs)
- [Camouflage](../crates/cantrip-cards/src/cards/camouflage.rs)
- [City in a Bottle](../crates/cantrip-cards/src/cards/city_in_a_bottle.rs)
- [Dance of the Dead](../crates/cantrip-cards/src/cards/dance_of_the_dead.rs)
- [Dark Sphere](../crates/cantrip-cards/src/cards/dark_sphere.rs)
- [Derelor](../crates/cantrip-cards/src/cards/derelor.rs)
- [Divine Intervention](../crates/cantrip-cards/src/cards/divine_intervention.rs)
- [Drought](../crates/cantrip-cards/src/cards/drought.rs)
- [Errant Minion](../crates/cantrip-cards/src/cards/errant_minion.rs)
- [Fasting](../crates/cantrip-cards/src/cards/fasting.rs)
- [Firestorm Phoenix](../crates/cantrip-cards/src/cards/firestorm_phoenix.rs)
- [Forcefield](../crates/cantrip-cards/src/cards/forcefield.rs)
- [Freyalise's Winds](../crates/cantrip-cards/src/cards/freyalises_winds.rs)
- [Glyph of Reincarnation](../crates/cantrip-cards/src/cards/glyph_of_reincarnation.rs)
- [Guardian Angel](../crates/cantrip-cards/src/cards/guardian_angel.rs)
- [Hipparion](../crates/cantrip-cards/src/cards/hipparion.rs)
- [Ice Cauldron](../crates/cantrip-cards/src/cards/ice_cauldron.rs)
- [Illusionary Presence](../crates/cantrip-cards/src/cards/illusionary_presence.rs)
- [Jade Monolith](../crates/cantrip-cards/src/cards/jade_monolith.rs)
- [Johan](../crates/cantrip-cards/src/cards/johan.rs)
- [Land Equilibrium](../crates/cantrip-cards/src/cards/land_equilibrium.rs)
- [Lava Burst](../crates/cantrip-cards/src/cards/lava_burst.rs)
- [Melee](../crates/cantrip-cards/src/cards/melee.rs)
- [Merseine](../crates/cantrip-cards/src/cards/merseine.rs)
- [Nafs Asp](../crates/cantrip-cards/src/cards/nafs_asp.rs)
- [Night Soil](../crates/cantrip-cards/src/cards/night_soil.rs)
- [Nova Pentacle](../crates/cantrip-cards/src/cards/nova_pentacle.rs)
- [Personal Incarnation](../crates/cantrip-cards/src/cards/personal_incarnation.rs)
- [Power Leak](../crates/cantrip-cards/src/cards/power_leak.rs)
- [Raging River](../crates/cantrip-cards/src/cards/raging_river.rs)
- [Rasputin Dreamweaver](../crates/cantrip-cards/src/cards/rasputin_dreamweaver.rs)
- [Reflecting Mirror](../crates/cantrip-cards/src/cards/reflecting_mirror.rs)
- [Rock Hydra](../crates/cantrip-cards/src/cards/rock_hydra.rs)
- [Runesword](../crates/cantrip-cards/src/cards/runesword.rs)
- [Sacred Boon](../crates/cantrip-cards/src/cards/sacred_boon.rs)
- [Silhouette](../crates/cantrip-cards/src/cards/silhouette.rs)
- [Soul Burn](../crates/cantrip-cards/src/cards/soul_burn.rs)
- [Takklemaggot](../crates/cantrip-cards/src/cards/takklemaggot.rs)
- [Tawnos's Coffin](../crates/cantrip-cards/src/cards/tawnoss_coffin.rs)
- [Whippoorwill](../crates/cantrip-cards/src/cards/whippoorwill.rs)
- [Worms of the Earth](../crates/cantrip-cards/src/cards/worms_of_the_earth.rs)

## Checks and reference follow-ups

The full native test script passes (27,853 Rust tests, managed tests and client build). The standalone generator passes 77 Rust tests; lane and tooling Python suites pass 437 and 52 tests. Printed basic-land mana abilities use the owned intrinsic classifier to avoid duplicate activations. No engine source or card module changed for this audit.

The MTGish adoption checks all 987 amended reference records. 985 agree under the owned semantic-card projection. Two exact differences remain visible and reported: [Dance of Many’s linked creation relation](https://github.com/vishvananda/oracle-grammar/issues/20#issuecomment-6934199376) and [Frankenstein’s Monster selector normalization](https://github.com/vishvananda/oracle-grammar/issues/33#issuecomment-6934200147). [MTGish #302](https://github.com/i5jb/mtgish/issues/302) reports preserving a single replacement identity for Naked Singularity / Reality Twist; the fork implements that grouped Source form.

Machine-readable names, paths, gap strings and input hashes: [audit JSON](../tools/oracle-codegen/data/ice-age-backlog-20261117.json). This is an API and generator audit. No workers were started and no cards were reserved.

The Source integration suite passes **607 tests** (12 ignored) against MTGish
`f7915260`. Seven generated examples additionally type-check against the actual
native API, including Library of Leng, Divine Intervention, Tetravus, Krovikan
Vampire, Wood Elemental, Stardew Valley and Infernal Denizen. This runs in one
small crate without the engine or card registry. The same check is now in CI.

All 43 card-addition commits are authored by `jschibel`; their exact commits are
in the audit JSON. Shared foundation commits include our work too. See the
[API shape review](ice-age-api-shape-review-20261117.md) for requirement/interface
comparisons and the remaining generator work. Fork `main` subsequently advanced
to `9889dd28f9` with upstream host/client/test changes; the audited engine API
is unchanged from the tested `5441ecd742` baseline.
