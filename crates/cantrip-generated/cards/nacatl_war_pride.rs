// Nacatl War-Pride
// status: planned
// requires: static rule Unique
// This creature must be blocked by exactly one creature if able.
// Whenever this creature attacks, create X tokens that are copies of it and that are tapped and attacking, where X is the number of creatures defending player controls. Exile the tokens at the beginning of the next end step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; id == asker.source }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let original_0 = { let q: &dyn Query = g; item.subject.object() }; let values_0 = { let q: &dyn Query = g; original_0.map(|original: ObjectId| q.copiable_values(original)) }; let count_0 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_0 = match values_0 { Some(values) => g.create_token_copies_tapped(values, player_0, count_0, true)?, None => Vec::new() };
for &token in &created_0 { let controller = g.controller(token); if let Some(defender) = cantrip_mechanics::combat::choose_attack_target(g, controller, asker.source)? { g.enter_attacking(&[token], defender); } }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_0.clone() }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
planned_item!("static-rule-unique" => statics: object_rule(ability_0_0_rule, Rule::BlockedByExactlyOne));

pub static CARD: CardDef = CardDef { name: "Nacatl War-Pride", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, create X tokens that are copies of it and that are tapped and attacking, where X is the number of creatures defending player controls. Exile the tokens at the beginning of the next end step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "Exile the tokens at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
