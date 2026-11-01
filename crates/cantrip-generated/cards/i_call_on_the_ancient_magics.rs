// I Call on the Ancient Magics
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, each other player searches their library for a card, reveals it, and puts it into their hand. Then you search your library for two cards and put them into your hand. Each player shuffles.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; { let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let searcher_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_1 = cantrip_mechanics::library::searched_cards(g, owner_1); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| true).collect() }; let found_1 = cantrip_mechanics::library::search(g, searcher_1, owner_1, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1))?;
 each_2[index_0] = found_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; all_2.clone() }; let mut owners_3: Vec<PlayerId> = objects_3.iter().map(|o| g.state().obj(*o).owner).collect(); owners_3.dedup(); for owner in owners_3 { let cards: Vec<ObjectId> = objects_3.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; all_2.clone() }; let moved_4 = g.return_to_hand_revealed(&objects_4);
g.end_sentence();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
    } 
g.end_sentence();
let searcher_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_6 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let shown_6 = cantrip_mechanics::library::searched_cards(g, owner_6); let can_6: Vec<ObjectId> = { let q: &dyn Query = g; shown_6.iter().copied().filter(|&id| true).collect() }; let found_6 = cantrip_mechanics::library::search(g, searcher_6, owner_6, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_6, &shown_6, can_6))?;
let objects_7 = { let q: &dyn Query = g; found_6.clone() }; let moved_7 = g.return_to_hand_revealed(&objects_7);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "I Call on the Ancient Magics", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, each other player searches their library for a card, reveals it, and puts it into their hand. Then you search your library for two cards and put them into your hand. Each player shuffles.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
