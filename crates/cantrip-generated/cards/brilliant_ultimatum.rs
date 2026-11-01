// Brilliant Ultimatum
// status: planned
// requires: a land played by what it is as a spell
// Exile the top five cards of your library. An opponent separates those cards into two piles. You may play lands and cast spells from one of those piles. If you cast a spell this way, you cast it without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((5i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
g.end_sentence();
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if !among.is_empty() { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let cards_2 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_0.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let separator_2 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let chooser_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (first_2, second_2) = cantrip_mechanics::choose::separate(g, Some(separator_2), asker.source, cards_2)?; let (pile_2, other_2) = cantrip_mechanics::choose::pile(g, chooser_2, asker.source, first_2, second_2)?;

g.end_sentence();
let played_3: Vec<ObjectId> = planned_value!("a land played by what it is as a spell", Vec::<ObjectId>::new() => cantrip_mechanics::playing::play_each(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, { let q: &dyn Query = g; (pile_2.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Land) || true)).collect::<Vec<ObjectId>>() }, CardPlayAction::Either, PlayCount::AnyNumber, PlayCost::Free, false, false, asker.source)?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Brilliant Ultimatum", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 3, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
