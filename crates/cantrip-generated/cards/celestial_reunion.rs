// Celestial Reunion
// status: planned
// requires: a choice of CreatureType outside its body
// requires: cost part Behold
// requires: cost part PlayerAction
// As an additional cost to cast this spell, you may choose a creature type and behold two creatures of that type.
// Search your library for a creature card with mana value X or less, reveal it, put it into your hand, then shuffle. If this spell's additional cost was paid and the revealed card is the chosen type, put that card onto the battlefield instead of putting it into your hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_1 = cantrip_mechanics::library::searched_cards(g, owner_1); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (i64::from(item.x)))).collect() }; let found_1 = cantrip_mechanics::library::search(g, searcher_1, owner_1, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1))?;
let objects_2 = { let q: &dyn Query = g; found_1.clone() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
if { let q: &dyn Query = g; (((found_1.clone()).into_iter().filter(|&id| planned_value!("a choice of CreatureType outside its body", None::<Subtype> => q.state().chosen_subtype(q.linked(item))).is_some_and(|chosen| q.chars(id).has_subtype(chosen))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; found_1.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, player_3, false)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
let objects_6 = { let q: &dyn Query = g; found_1.clone() }; let moved_6 = g.return_to_hand_revealed(&objects_6);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_8 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_8 = cantrip_mechanics::library::searched_cards(g, owner_8); let can_8: Vec<ObjectId> = { let q: &dyn Query = g; shown_8.iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (i64::from(item.x)))).collect() }; let found_8 = cantrip_mechanics::library::search(g, searcher_8, owner_8, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_8, &shown_8, can_8))?;
let objects_9 = { let q: &dyn Query = g; found_8.clone() }; let mut owners_9: Vec<PlayerId> = objects_9.iter().map(|o| g.state().obj(*o).owner).collect(); owners_9.dedup(); for owner in owners_9 { let cards: Vec<ObjectId> = objects_9.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_10 = { let q: &dyn Query = g; found_8.clone() }; let moved_10 = g.return_to_hand_revealed(&objects_10);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.paid > 0 { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }

pub static CARD: CardDef = CardDef { name: "Celestial Reunion", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Cast without choosing", "Choose a creature type and behold two creatures of that type"], late_targets: true, optional_cost: Some(OptionalCost::new(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part PlayerAction", Cost::NONE => Cost::performed(|q: &dyn Query, asker: &Asker| Some(asker.controller), |g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::chosen_type::creature_type_now(g, player_0, asker.source)?;
g.end_sentence();
 Ok(()) }))).and(planned_value!("cost part Behold", Cost::NONE => Cost::beholds(|q: &dyn Query, asker: &Asker| 2i64, CardsInHand::And(CardsInHand::IsCardtype(CardType::Creature), CardsInHand::IsCreatureTypeVariable(CreatureTypeVariable::TheChosenCreatureType))))))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
