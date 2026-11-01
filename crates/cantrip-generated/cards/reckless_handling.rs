// Reckless Handling
// status: ready
// Search your library for an artifact card, reveal it, put it into your hand, shuffle, then discard a card at random. If an artifact card was discarded this way, Reckless Handling deals 2 damage to each opponent.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| q.chars(id).is(CardType::Artifact)).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let mut hand_4 = g.state().zone(Zone::hand(player_4)).to_vec(); let mut picked_4: Vec<ObjectId> = Vec::new(); while (picked_4.len() as u32) < amount_4 && !hand_4.is_empty() { let at = g.choose_at_random(hand_4.len()); picked_4.push(hand_4.remove(at)); } let discarded_4 = g.discard_together(&picked_4);
g.end_sentence();
if { let q: &dyn Query = g; (((discarded_4.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let amount_5 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_5 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_5 { let batch: Vec<(ObjectId, Target, u16)> = recipients_5.iter().map(|t| (source, *t, amount_5)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Reckless Handling", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
