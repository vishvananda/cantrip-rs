// King Narfi's Betrayal
// status: planned
// requires: play permission with mana as though any color
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Each player mills four cards. Then you may exile a creature or planeswalker card from each graveyard.
// II, III — Until end of turn, you may cast spells from among cards exiled with this Saga, and you may spend mana as though it were mana of any color to cast those spells.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(bound_0) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::Exile, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
 each_3[index_1] = selected_2.clone(); all_3.extend(each_3[index_1].iter().copied());
   } 
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; all_3.clone() }; let moved_4 = { let by = g.linked(item); g.exile_with(&objects_4, by, false) };
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("play-permission-with-mana-as-though-any-color" => let players_5_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let linked_5_0 = g.linked(item); cantrip_api::instruction(g, |g| { for &player in &players_5_0 { g.allow_play_exiled_with(linked_5_0, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, any_color: true, ..PlayGrant::CAST }, false); } }););
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("play-permission-with-mana-as-though-any-color" => let players_6_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let linked_6_0 = g.linked(item); cantrip_api::instruction(g, |g| { for &player in &players_6_0 { g.allow_play_exiled_with(linked_6_0, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, any_color: true, ..PlayGrant::CAST }, false); } }););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "King Narfi's Betrayal", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Each player mills four cards. Then you may exile a creature or planeswalker card from each graveyard.").targets(&[]),chapter(2, ability_0_1, "II, III — Until end of turn, you may cast spells from among cards exiled with this Saga, and you may spend mana as though it were mana of any color to cast those spells.").targets(&[]),chapter(3, ability_0_2, "II, III — Until end of turn, you may cast spells from among cards exiled with this Saga, and you may spend mana as though it were mana of any color to cast those spells.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
