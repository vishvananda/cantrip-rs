// Intellectual Offering
// status: ready
// Choose an opponent. You and that player each draw three cards.
// Choose an opponent. Untap all nonland permanents you control and all nonland permanents that player controls.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_0) = cantrip_mechanics::choose::player(g, player_0, asker.source, candidates_0, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; { let mut all: Vec<PlayerId> = Vec::new(); for p in Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } for p in Some(chosen_0).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } all } };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_2) = cantrip_mechanics::choose::player(g, player_2, asker.source, candidates_2, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && (Some(asker.controller) == Some(q.controller(id)) || Some(chosen_2) == Some(q.controller(id))))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_3: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_3 { g.untap(object); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Intellectual Offering", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
