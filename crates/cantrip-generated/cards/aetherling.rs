// Aetherling
// status: ready
// {U}: Exile this creature. Return it to the battlefield under its owner's control at the beginning of the next end step.
// {U}: This creature can't be blocked this turn.
// {1}: This creature gets +1/-1 until end of turn.
// {1}: This creature gets -1/+1 until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = ({ let q: &dyn Query = g; (about(q, item, 0)).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::EndOfTurn;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::CantBeBlocked };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let duration_5 = Duration::EndOfTurn;
let kind_5_0_0 = { let q: &dyn Query = g; EffectKind::Pump((1i64) as i16, (-1i64) as i16) };
if let Some(source) = source_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.add_effect(source, object, kind_5_0_0, duration_5);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_6 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let duration_6 = Duration::EndOfTurn;
let kind_6_0_0 = { let q: &dyn Query = g; EffectKind::Pump((-1i64) as i16, (1i64) as i16) };
if let Some(source) = source_6 { cantrip_api::instruction(g, |g| { for &object in &objects_6 { g.add_effect(source, object, kind_6_0_0, duration_6);  } }); }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Aetherling", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{U}: Exile this creature. Return it to the battlefield under its owner's control at the beginning of the next end step.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{U}: This creature can't be blocked this turn.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{1}: This creature gets +1/-1 until end of turn.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{1}: This creature gets -1/+1 until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "Return it to the battlefield under its owner's control at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
