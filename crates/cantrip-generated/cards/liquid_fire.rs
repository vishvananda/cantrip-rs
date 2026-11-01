// Liquid Fire
// status: planned
// requires: cost part PlayerAction
// As an additional cost to cast this spell, choose a number between 0 and 5.
// Liquid Fire deals X damage to target creature and 5 minus X damage to that creature's controller, where X is the chosen number.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let mut batch_1: Vec<(ObjectId, Target, u16)> = Vec::new();
let amount_1_0 = { let q: &dyn Query = g; (q.state().chosen_number(q.linked(item)).map_or(0i64, i64::from)).clamp(0, 65535) as u16 }; if let Some(source) = source_1 { for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { batch_1.push((source, t, amount_1_0)); } }
let amount_1_1 = { let q: &dyn Query = g; ((5i64 - q.state().chosen_number(q.linked(item)).map_or(0i64, i64::from))).max(0) as u16 }; if let Some(source) = source_1 { for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { batch_1.push((source, t, amount_1_1)); } }
g.damage_together(&batch_1);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Liquid Fire", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part PlayerAction", Cost::NONE => Cost::performed(|q: &dyn Query, asker: &Asker| Some(asker.controller), |g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let least_0 = 0u32; let most_0 = 5u32; let chosen_0: u32 = cantrip_mechanics::choose::number(g, player_0, asker.source, least_0, most_0, "Choose a number")?;
g.end_sentence();
 Ok(()) })))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
