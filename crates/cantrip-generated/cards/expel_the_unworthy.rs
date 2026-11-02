// Expel the Unworthy
// status: ready
// Kicker {2}{W} (You may pay an additional {2}{W} as you cast this spell.)
// Choose target creature with mana value 3 or less. If this spell was kicked, instead choose target creature. Exile the chosen creature, then its controller gains life equal to its mana value.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_paid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_PAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_paid_target_0)];
#[allow(unused_variables)] fn ability_0_unpaid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (3i64)))) }
static ABILITY_0_UNPAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_unpaid_target_0)];
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { if q.obj(asker.source).kicked > 0 { (ability_0_paid_target_0)(q, asker, t) } else { (ability_0_unpaid_target_0)(q, asker, t) } }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[TargetSpec { min: 1, max: 1, filter: ability_0_target_0, label: "", if_kicked: Some((1, 1)), together: None }];
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) as i32 }; let players_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) as i32 }; let players_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.gain_life(player, amount_3); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.kicked > 0 { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Expel the Unworthy", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: kicker(ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), late_targets: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
