// Mishra, Claimed by Gix
// status: planned
// requires: layout MeldPiece
// requires: operation Meld
// Whenever you attack, each opponent loses X life and you gain X life, where X is the number of attacking creatures. If Mishra, Claimed by Gix and a creature named Phyrexian Dragon Engine are attacking, and you both own and control them, exile them, then meld them into Mishra, Lost to Phyrexia. It enters tapped and attacking.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let value_0: i64 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| ((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).len() as i64 };
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (value_0).clamp(0, 2147483647) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (value_0).clamp(0, 2147483647) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.gain_life(player, amount_2); }
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().attackers.iter().any(|a| a.id == candidate) || q.was_attacking(candidate))) && (((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).name == "Phyrexian Dragon Engine")).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) && (Some(asker.source).is_some_and(|candidate: ObjectId| (Some(asker.controller) == Some(q.state().obj(candidate).owner) && Some(asker.controller) == Some(q.controller(candidate)))) && ((q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) && q.chars(id).name == "Phyrexian Dragon Engine" && (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id))) && Some(asker.controller) == Some(q.state().obj(id).owner) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64))) } {
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).name == "Phyrexian Dragon Engine" && (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)))).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::ChooseAsEnters, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let objects_4 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in Some(asker.source).into_iter().collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in selected_3.clone() { if !all.contains(&id) { all.push(id); } } all } }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
let melding_6 = { let q: &dyn Query = g; moved_4.clone() };
planned!("operation-meld" => g.meld(&melding_6, "Mishra, Lost to Phyrexia"););
g.end_sentence();
hoisted_7.extend(moved_4.clone());
hoisted_8.extend(selected_3.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
planned_item!("layout-meld-piece" => melds_into: Some("Mishra, Lost to Phyrexia"));

pub static CARD: CardDef = CardDef { name: "Mishra, Claimed by Gix", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Human,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_0_event), ability_0, "Whenever you attack, each opponent loses X life and you gain X life, where X is the number of attacking creatures. If Mishra, Claimed by Gix and a creature named Phyrexian Dragon Engine are attacking, and you both own and control them, exile them, then meld them into Mishra, Lost to Phyrexia. It enters tapped and attacking.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
