// Assemble the Team
// status: ready
// Search the top third of your library, rounded up, for a card. Put that card into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = searcher_0; let may_0 = g.may_search(searcher_0, owner_0); let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_0: Vec<ObjectId> = if may_0 { g.state().zone(Zone::library(owner_0)).iter().copied().take(({ let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).saturating_add(2).div_euclid(3) }).max(0) as usize).collect::<Vec<ObjectId>>() } else { Vec::new() }; let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| true).collect() };
let found_0: Vec<ObjectId> = { let rule = cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0.clone()); let found = if shown_0.is_empty() { Vec::new() } else { match g.ask(searcher_0, RequestKind::SearchLibrary { source: asker.source, library: shown_0.clone(), rule: Box::new(rule), destination: SearchDestination::Hand })? { Answer::Cards(cards) => cards, _ => Vec::new() } }; if may_0 { g.searched(searcher_0, owner_0); } found };
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let moved_1 = g.return_to_hand(&objects_1);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Assemble the Team", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
