// Ancient Excavation
// status: ready
// Draw cards equal to the number of cards in your hand, then discard a card for each card drawn this way.
// Basic landcycling {2} ({2}, Discard this card: Search your library for a basic land card, reveal it, put it into your hand, then shuffle.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_0: i64 = 0; let mut drawn_0: Vec<ObjectId> = Vec::new(); for player in players_0 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_0); done_0 += g.history().drawn(player) as i64 - before; drawn_0.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; ((done_0 as i64)) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_1)?); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_2 = cantrip_mechanics::library::searched_cards(g, owner_2); let can_2: Vec<ObjectId> = { let q: &dyn Query = g; shown_2.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_2 = cantrip_mechanics::library::search(g, searcher_2, owner_2, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_2, &shown_2, can_2))?;
let objects_3 = { let q: &dyn Query = g; found_2.clone() }; let mut owners_3: Vec<PlayerId> = objects_3.iter().map(|o| g.state().obj(*o).owner).collect(); owners_3.dedup(); for owner in owners_3 { let cards: Vec<ObjectId> = objects_3.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_4 = { let q: &dyn Query = g; found_2.clone() }; let moved_4 = g.return_to_hand_revealed(&objects_4);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ancient Excavation", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(DISCARD_THIS), ability_1, "Basic landcycling {2}").targets(&[]).cycling().from(ZoneKind::Hand)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
