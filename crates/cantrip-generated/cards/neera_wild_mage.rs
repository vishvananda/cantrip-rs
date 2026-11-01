// Neera, Wild Mage
// status: ready
// Whenever you cast a spell, you may put it on the bottom of its owner's library. If you do, reveal cards from the top of your library until you reveal a nonland card. You may cast that card without paying its mana cost. Then put all revealed cards not cast this way on the bottom of your library in a random order. This ability triggers only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let objects_1 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = g.put_into_library(&objects_1, LibraryPlace::Bottom);
g.end_sentence();
hoisted_2.extend(moved_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let owner_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_4 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_4, rest_4) = cantrip_mechanics::library::reveal_until_n(g, owner_4, count_4, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_4 = found_4.len() + rest_4.len(); let objects_4: Vec<ObjectId> = g.state().zone(Zone::library(owner_4)).iter().copied().take(shown_4).collect();
let caster_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_5: Vec<ObjectId> = { let q: &dyn Query = g; (objects_4.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_5: u32 = 1; let mut played_5: Vec<ObjectId> = Vec::new();
while left_5 > 0 {
let mut open_5: Vec<ObjectId> = Vec::new(); for &c in &pool_5 { let ways = g.cast_ways_during_resolution_by(c, caster_5); if ways.iter().any(|w| !g.card(w.face).types.has(CardType::Land)) { open_5.push(c); } }
let picked_5 = if open_5.is_empty() { None } else if open_5.len() > 1 || (true && (!false || pool_5.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_5, asker.source, Purpose::CastFree, pool_5.clone(), open_5.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_5, open_5[0], Purpose::CastFree)? { Some(open_5[0]) } else { None };
let Some(card_5) = picked_5 else { break; };
let ways_5: Vec<CastWay> = g.cast_ways_during_resolution_by(card_5, caster_5).into_iter().filter(|w| !g.card(w.face).types.has(CardType::Land)).collect();
let done_5 = if ways_5.is_empty() { g.play_land_during_resolution(card_5)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_5, card_5, ways_5)?.map(|w| w.way); g.cast_during_resolution_as(card_5, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_5), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_5 { left_5 -= 1; played_5.push(g.state().obj(card_5).became.unwrap_or(card_5)); } else if 1i64 == 1 { break; } if done_5 || !(true && true) { pool_5.retain(|&c| c != card_5); }
}
let library_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_6: Vec<ObjectId> = { let q: &dyn Query = g; (objects_4.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_6)).collect(); let placed_6 = cards_6.clone();
g.bottom_in_random_order(library_6, &cards_6);
g.end_sentence();
hoisted_7.extend(placed_6.clone());
hoisted_8.extend(objects_4.clone());
hoisted_9.extend({ let q: &dyn Query = g; placed_6.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Neera, Wild Mage", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Elf,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever you cast a spell, you may put it on the bottom of its owner's library. If you do, reveal cards from the top of your library until you reveal a nonland card. You may cast that card without paying its mana cost. Then put all revealed cards not cast this way on the bottom of your library in a random order. This ability triggers only once each turn.").targets(&[]).triggers_each_turn(1)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
