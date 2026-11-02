// Extrapolate the Impossible
// status: ready
// You may reveal exactly two cards you own with different names from outside the game. An opponent chooses one of them. You put that card into your hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let pool_1 = { let q: &dyn Query = g; (q.outside_the_game(asker.controller)).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }; let count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let picked_1 = cantrip_mechanics::choose::select(g, player_1, asker.source, Purpose::Reveal, pool_1, cantrip_mechanics::select::Selection::of(count_1, &[]).with(cantrip_mechanics::select::DIFFERENT_NAMES))?;
let picked_1: Vec<ObjectId> = if picked_1.len() == 2 { picked_1 } else { Vec::new() };
let objects_2 = { let q: &dyn Query = g; picked_1.clone() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let selected_4_player = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; objects_2.clone() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::PutIntoHand, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };

g.end_sentence();
let objects_6 = { let q: &dyn Query = g; selected_4.clone() }; let moved_6 = { let (outside, inside): (Vec<ObjectId>, Vec<ObjectId>) = objects_6.iter().partition(|&&o| g.state().obj(o).zone.is(ZoneKind::Sideboard)); let mut moved = g.bring_into_hand_revealed(&outside); moved.extend(g.return_to_hand_revealed(&inside)); moved };
g.end_sentence();
hoisted_7.extend(moved_6.clone());
hoisted_8.extend(picked_1.clone());
hoisted_9.extend(objects_2.clone());
hoisted_10.extend(selected_4.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Extrapolate the Impossible", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
