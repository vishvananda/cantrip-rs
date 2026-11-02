// Finale of Revelation
// status: ready
// Draw X cards. If X is 10 or more, instead shuffle your graveyard into your library, draw X cards, untap up to five lands, and you have no maximum hand size for the rest of the game.
// Exile Finale of Revelation.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) >= (10i64) } {
let objects_0 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_0 = g.put_into_library(&objects_0, LibraryPlace::Top);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (i64::from(item.x)) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((5i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::Untap, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let objects_4 = { let q: &dyn Query = g; (selected_3.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_4: Vec<ObjectId> = objects_4.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_4 { g.untap(object); } });
g.end_sentence();
if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_player_effect(asker.controller, source, EffectKind::MaximumHandSize(None), Duration::Indefinite); }
g.end_sentence();
hoisted_6.extend(moved_0.clone());
hoisted_7.extend(turned_4.clone());
hoisted_8.extend(selected_3.clone());
hoisted_9.extend(objects_4.clone());
hoisted_10.extend({ let q: &dyn Query = g; objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_11.extend({ let q: &dyn Query = g; turned_4.clone() });
} else {
let amount_12 = { let q: &dyn Query = g; (i64::from(item.x)) as u32 }; let players_12 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_12 { g.draw(player, amount_12); }
g.end_sentence();
}
g.end_sentence();
let objects_13 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_13 = g.exile_all(&objects_13);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Finale of Revelation", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
