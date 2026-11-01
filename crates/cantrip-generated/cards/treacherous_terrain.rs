// Treacherous Terrain
// status: ready
// Treacherous Terrain deals damage to each opponent equal to the number of lands that player controls.
// Basic landcycling {2} ({2}, Discard this card: Search your library for a basic land card, reveal it, put it into your hand, then shuffle.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(ObjectId, Target, u16)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 65535) as u16 }; if let Some(source) = source_2 { for t in { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { gathered_1.push((source, t, amount_2)); } }
    } 
g.damage_together(&gathered_1);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_3 = cantrip_mechanics::library::searched_cards(g, owner_3); let can_3: Vec<ObjectId> = { let q: &dyn Query = g; shown_3.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_3 = cantrip_mechanics::library::search(g, searcher_3, owner_3, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_3, &shown_3, can_3))?;
let objects_4 = { let q: &dyn Query = g; found_3.clone() }; let mut owners_4: Vec<PlayerId> = objects_4.iter().map(|o| g.state().obj(*o).owner).collect(); owners_4.dedup(); for owner in owners_4 { let cards: Vec<ObjectId> = objects_4.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_5 = { let q: &dyn Query = g; found_3.clone() }; let moved_5 = g.return_to_hand_revealed(&objects_5);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Treacherous Terrain", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(DISCARD_THIS), ability_1, "Basic landcycling {2}").targets(&[]).cycling().from(ZoneKind::Hand)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
