// Preferred Selection
// status: ready
// At the beginning of your upkeep, look at the top two cards of your library. You may sacrifice this enchantment and pay {2}{G}{G}. If you do, put one of those cards into your hand. If you don't, put one of those cards on the bottom of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((2i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_1, &objects_1);
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_2 = g.may_pay(payer_2, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS), &PAY_QUESTION)?;
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutIntoHand, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; selected_3.clone() }; let moved_4 = g.return_to_hand(&objects_4);
g.end_sentence();
let library_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_5: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_1.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_5)).collect(); let placed_5 = cards_5.clone();
let chooser_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_5 = cantrip_mechanics::library::ask_library_order(g, chooser_5, asker.source, &cards_5, DestinationKind::LibraryTop)?; g.arrange_library(library_5, &order_5, &[]);
g.end_sentence();
hoisted_6.extend(moved_4.clone());
hoisted_7.extend(placed_5.clone());
hoisted_8.extend(selected_3.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_10.extend({ let q: &dyn Query = g; placed_5.clone() });
} else {
let selected_11_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_11_candidates = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let selected_11_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_11 = { cantrip_mechanics::choose::select(g, selected_11_player, asker.source, Purpose::PutOnBottom, selected_11_candidates, cantrip_mechanics::select::Selection::of(selected_11_count, &[]))? };
g.end_sentence();
let library_12 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_12: Vec<ObjectId> = { let q: &dyn Query = g; selected_11.clone() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_12)).collect(); let placed_12 = cards_12.clone();
let chooser_12 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_12 = cantrip_mechanics::library::ask_library_order(g, chooser_12, asker.source, &cards_12, DestinationKind::LibraryBottom)?; g.arrange_library(library_12, &[], &order_12);
g.end_sentence();
let library_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_13: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in placed_12.clone() { if !all.contains(&id) { all.push(id); } } all }; (objects_1.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_13)).collect(); let placed_13 = cards_13.clone();
let chooser_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_13 = cantrip_mechanics::library::ask_library_order(g, chooser_13, asker.source, &cards_13, DestinationKind::LibraryTop)?; g.arrange_library(library_13, &order_13, &[]);
g.end_sentence();
hoisted_14.extend(placed_12.clone());
hoisted_15.extend(placed_13.clone());
hoisted_16.extend(selected_11.clone());
hoisted_17.extend({ let q: &dyn Query = g; placed_12.clone() });
hoisted_18.extend({ let q: &dyn Query = g; placed_13.clone() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Preferred Selection", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, look at the top two cards of your library. You may sacrifice this enchantment and pay {2}{G}{G}. If you do, put one of those cards into your hand. If you don't, put one of those cards on the bottom of your library.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
