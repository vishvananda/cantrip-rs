// Aang's Journey
// status: planned
// requires: multiple spell bodies
// requires: search with several find clauses
// Kicker {2} (You may pay an additional {2} as you cast this spell.)
// Search your library for a basic land card. If this spell was kicked, instead search your library for a basic land card and a Shrine card. Reveal those cards, put them into your hand, then shuffle.
// You gain 2 life.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let found_0: Vec<ObjectId> = planned_value!("search with several find clauses", Vec::<ObjectId>::new() => cantrip_mechanics::library::search_clauses(g, searcher_0, owner_0, asker.source, &[(Selection::of({ let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }, &[]), |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))), (Selection::of({ let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }, &[]), |q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Shrine)))])?);
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_4 = cantrip_mechanics::library::searched_cards(g, owner_4); let can_4: Vec<ObjectId> = { let q: &dyn Query = g; shown_4.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_4 = cantrip_mechanics::library::search(g, searcher_4, owner_4, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_4, &shown_4, can_4))?;
let objects_5 = { let q: &dyn Query = g; found_4.clone() }; let mut owners_5: Vec<PlayerId> = objects_5.iter().map(|o| g.state().obj(*o).owner).collect(); owners_5.dedup(); for owner in owners_5 { let cards: Vec<ObjectId> = objects_5.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_6 = { let q: &dyn Query = g; found_4.clone() }; let moved_6 = g.return_to_hand_revealed(&objects_6);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.kicked > 0 { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }
fn ability_1_then(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_8 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.gain_life(player, amount_8); }
g.end_sentence();
 Ok(()) }
planned_item!("multiple-spell-bodies" => spell_after: spell(ability_1_then).targets(&[]));

pub static CARD: CardDef = CardDef { name: "Aang's Journey", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[Subtype::Lesson]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: kicker(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
