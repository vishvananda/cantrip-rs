// Tip the Scales
// status: ready
// Sacrifice a creature. When you do, all creatures get -X/-X until end of turn, where X is the sacrificed creature's toughness.
#[allow(unused_imports)] use cantrip_api::*;
fn reflexive_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::Pump((((-1i64 * item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.chars(o).toughness.unwrap_or(0))))).clamp(-32768, 32767) as i16).min(0), (((-1i64 * item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.chars(o).toughness.unwrap_or(0))))).clamp(-32768, 32767) as i16).min(0)) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_0 { let eligible: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_0.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_0.clone(), &[]))? }; for object in taken { picks_0.push((player, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
if !sacrificed_0.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; (objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).get(0).copied() }.map_or(Target::None, Target::Object)); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tip the Scales", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_2, "When you do, all creatures get -X/-X until end of turn, where X is the sacrificed creature's toughness.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
