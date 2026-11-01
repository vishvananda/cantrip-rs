// War of the Last Alliance
// status: planned
// requires: engine: no Ring temptation
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I, II — Search your library for a legendary creature card, reveal it, put it into your hand, then shuffle.
// III — Creatures you control gain double strike until end of turn. The Ring tempts you.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Creature))).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_4 = cantrip_mechanics::library::searched_cards(g, owner_4); let can_4: Vec<ObjectId> = { let q: &dyn Query = g; shown_4.iter().copied().filter(|&id| (q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Creature))).collect() }; let found_4 = cantrip_mechanics::library::search(g, searcher_4, owner_4, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_4, &shown_4, can_4))?;
let objects_5 = { let q: &dyn Query = g; found_4.clone() }; let mut owners_5: Vec<PlayerId> = objects_5.iter().map(|o| g.state().obj(*o).owner).collect(); owners_5.dedup(); for owner in owners_5 { let cards: Vec<ObjectId> = objects_5.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_6 = { let q: &dyn Query = g; found_4.clone() }; let moved_6 = g.return_to_hand_revealed(&objects_6);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_8 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_8 = { let q: &dyn Query = g; Some(asker.source) }; let duration_8 = Duration::EndOfTurn;
let kind_8_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::DoubleStrike])) };
if let Some(source) = source_8 { cantrip_api::instruction(g, |g| { for &object in &objects_8 { g.add_effect(source, object, kind_8_0_0, duration_8);  } }); }
g.end_sentence();
planned!("engine-no-ring-temptation" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::ring::tempt(g, player, asker.source)?; });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "War of the Last Alliance", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II — Search your library for a legendary creature card, reveal it, put it into your hand, then shuffle.").targets(&[]),chapter(2, ability_0_1, "I, II — Search your library for a legendary creature card, reveal it, put it into your hand, then shuffle.").targets(&[]),chapter(3, ability_0_2, "III — Creatures you control gain double strike until end of turn. The Ring tempts you.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
