// Krosan Verge
// status: ready
// This land enters tapped.
// {T}: Add {C}.
// {2}, {T}, Sacrifice this land: Search your library for a Forest card and a Plains card, put them onto the battlefield tapped, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::C, mana_1_0);}
 Ok(()) }
#[allow(unused_variables)] fn clauses_2(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let asker = &Asker::new(choice.source, q.controller(choice.source)); clauses_verdict(choice, &mut [((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Forest))), ((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Plains)))]) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let found_2 = cantrip_mechanics::library::search(g, searcher_2, owner_2, asker.source, SearchDestination::BattlefieldTapped, cantrip_mechanics::select::Selection::new(cantrip_mechanics::select::up_to((1i64).max(0) as u32 + (1i64).max(0) as u32)).with(cantrip_api::select::part(&(clauses_2 as SelectFn), 0, "one for each thing searched for")))?;
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; found_2.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, player_3, true)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn clauses_fit(cards: &[ObjectId], clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> bool { let Some((&card, rest)) = cards.split_first() else { return true; }; for i in 0..clauses.len() { if clauses[i].0 > 0 && (clauses[i].1)(card) { clauses[i].0 -= 1; let fits = clauses_fit(rest, clauses); clauses[i].0 += 1; if fits { return true; } } } false }
fn clauses_verdict(choice: &Choice, clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> Verdict { let complete = clauses_fit(choice.selected, clauses); let addable = Bits::of(choice.candidates.len(), |i| { let card = choice.candidates[i]; !choice.selected.contains(&card) && { let with: Vec<ObjectId> = choice.selected.iter().copied().chain([card]).collect(); clauses_fit(&with, clauses) } }); Verdict { complete, addable } }

pub static CARD: CardDef = CardDef { name: "Krosan Verge", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_1_mana) }, "{T}: Add {C}."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{2}, {T}, Sacrifice this land: Search your library for a Forest card and a Plains card, put them onto the battlefield tapped, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
