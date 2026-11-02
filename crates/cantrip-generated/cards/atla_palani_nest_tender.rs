// Atla Palani, Nest Tender
// status: ready
// {2}, {T}: Create a 0/1 green Egg creature token with defender.
// Whenever an Egg you control dies, reveal cards from the top of your library until you reveal a creature card. Put that card onto the battlefield and the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Egg).name, token_words: token!(Egg).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Egg]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Egg)) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let owner_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_2, rest_2) = cantrip_mechanics::library::reveal_until_n(g, owner_2, count_2, &|q: &dyn Query, id: ObjectId| q.chars(id).is(CardType::Creature)); let shown_2 = found_2.len() + rest_2.len(); let objects_2: Vec<ObjectId> = g.state().zone(Zone::library(owner_2)).iter().copied().take(shown_2).collect();
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; (objects_2.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, player_3, false)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
let library_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_4: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_2.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_4)).collect(); let placed_4 = cards_4.clone();
g.bottom_in_random_order(library_4, &cards_4);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Atla Palani, Nest Tender", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{2}, {T}: Create a 0/1 green Egg creature token with defender.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever an Egg you control dies, reveal cards from the top of your library until you reveal a creature card. Put that card onto the battlefield and the rest on the bottom of your library in a random order.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
