// Impromptu Raid
// status: ready
// {2}{R/G}: Reveal the top card of your library. If it isn't a creature card, put it into your graveyard. Otherwise, put that card onto the battlefield. That creature gains haste. Sacrifice it at the beginning of the next end step.
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; about(q, item, 0) }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_5 { let controller = g.controller(object); if players_5.contains(&controller) && g.can_sacrifice(controller, object) { picks_5.push((controller, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_0: Vec<PlayerId> = objects_0.iter().map(|o| g.state().obj(*o).owner).collect(); owners_0.dedup(); for owner in owners_0 { let cards: Vec<ObjectId> = objects_0.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
if { let q: &dyn Query = g; (((objects_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; ((moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::Indefinite;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
hoisted_6.extend(moved_1.clone());
hoisted_7.extend(objects_2.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let objects_9 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_9 = { let moves: Vec<(ObjectId, Zone)> = objects_9.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_10.extend(moved_9.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Impromptu Raid", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::G)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::G)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{2}{R/G}: Reveal the top card of your library. If it isn't a creature card, put it into your graveyard. Otherwise, put that card onto the battlefield. That creature gains haste. Sacrifice it at the beginning of the next end step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_4, "Sacrifice it at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
