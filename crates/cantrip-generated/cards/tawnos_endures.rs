// Tawnos Endures
// status: planned
// requires: operation EffectOn
// Exile target creature. It gains "At the beginning of your upkeep, if this card is exiled, it perpetually gets +1/+1, then you may put it onto the battlefield."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
planned!("operation-effect-on" => cantrip_mechanics::effects::effect_on(g, asker, EffectOn::Objects({ let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }), &[ExiledCardEffect::AddAbility("Ability(Scoped { zones: [Exile], condition: None, ability: Triggered { event: Turn { point: UpkeepBegins, players: One(AbilityController) }, intervening_if: Some(ObjectMatches { object: Source, filter: InZone(Shared(Exile)) }), limits: TriggerLimits { per_turn: None, total: None }, announcement: Announcement { targets: [], modes: None }, body: Program { instructions: [Effect { result: ResultId(0), operation: CreatePerpetualEffect { objects: One(Source), changes: [Pump { power: Constant(1), toughness: Constant(1) }] } }, Optional { decider: AbilityController, body: Program { instructions: [Effect { result: ResultId(1), operation: Move { objects: InZone { objects: One(Source), zone: Exile }, destination: Battlefield(AbilityController), entry: EntrySpec { tapped: false, counters: [], states: [] }, face_down: false } }] } }] } } })")], None)?);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Tawnos Endures", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
