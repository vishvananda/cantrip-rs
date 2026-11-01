// Fertile Thicket
// status: ready
// This land enters tapped.
// When this land enters, you may look at the top five cards of your library. If you do, reveal up to one basic land card from among them, then put that card on top of your library and the rest on the bottom in any order.
// {T}: Add {G}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_3 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((5i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_3, &objects_3);
let selected_4_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; ((objects_3.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::Reveal, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
let objects_5 = { let q: &dyn Query = g; selected_4.clone() }; let mut owners_5: Vec<PlayerId> = objects_5.iter().map(|o| g.state().obj(*o).owner).collect(); owners_5.dedup(); for owner in owners_5 { let cards: Vec<ObjectId> = objects_5.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let library_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_6: Vec<ObjectId> = { let q: &dyn Query = g; selected_4.clone() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_6)).collect(); let placed_6 = cards_6.clone();
let chooser_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_6 = cantrip_mechanics::library::ask_library_order(g, chooser_6, asker.source, &cards_6, DestinationKind::LibraryTop)?; g.arrange_library(library_6, &order_6, &[]);
let library_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_7: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in placed_6.clone() { if !all.contains(&id) { all.push(id); } } all }; (objects_3.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_7)).collect(); let placed_7 = cards_7.clone();
let chooser_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_7 = cantrip_mechanics::library::ask_library_order(g, chooser_7, asker.source, &cards_7, DestinationKind::LibraryBottom)?; g.arrange_library(library_7, &[], &order_7);
g.end_sentence();
hoisted_8.extend(placed_6.clone());
hoisted_9.extend(placed_7.clone());
hoisted_10.extend(objects_3.clone());
hoisted_11.extend(selected_4.clone());
hoisted_12.extend(objects_5.clone());
hoisted_13.extend({ let q: &dyn Query = g; placed_6.clone() });
hoisted_14.extend({ let q: &dyn Query = g; placed_7.clone() });
}
g.end_sentence();
 Ok(()) }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_15 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_15_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_15 {
g.add_mana(player, Mana::G, mana_15_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fertile Thicket", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this land enters, you may look at the top five cards of your library. If you do, reveal up to one basic land card from among them, then put that card on top of your library and the rest on the bottom in any order.").targets(&[]),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 1, 0]), ..add_by(ability_2_mana) }, "{T}: Add {G}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
