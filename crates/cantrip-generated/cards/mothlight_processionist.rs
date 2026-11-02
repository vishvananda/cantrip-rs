// Mothlight Processionist
// status: planned
// requires: operation Conjure
// Enchantment spells you cast have convoke.
// Eerie — Whenever an enchantment you control enters and whenever you fully unlock a Room, conjure a card named Mothlight Processionist into your hand. Discard that card at the beginning of the next end step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_pay_with(q: &dyn Query, source: ObjectId, caster: PlayerId, spell: ObjectId) -> PayWith { let asker = &Asker::new(source, q.controller(source)); if Some(asker.controller) == Some(caster) && (q.chars(spell).is(CardType::Enchantment)) { PayWith { convoke: true, ..PayWith::NONE } } else { PayWith::NONE } }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Enchantment) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
if let Event::DoorUnlocked { permanent, player, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if q.state().obj(id).marks.doors == DOOR_LEFT | DOOR_RIGHT && Some(asker.controller) == Some(*player) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Room)) { return Some(Fired { subject, data }); } }
 None }
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let cards_3 = { let q: &dyn Query = g; about(q, item, 0) }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_0.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Mothlight Processionist"), count_0, ConjureTo::Hand)?)); }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; (conjured_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Mothlight Processionist", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::DOOR_UNLOCKED.0), ability_1_event), ability_1, "Eerie — Whenever an enchantment you control enters and whenever you fully unlock a Room, conjure a card named Mothlight Processionist into your hand. Discard that card at the beginning of the next end step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "Discard that card at the beginning of the next end step.")], enters: None, modal: None, enchant: None, grants_pay_with: &[ability_0_0_pay_with], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
