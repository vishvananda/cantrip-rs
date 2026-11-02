// The Seventh Doctor
// status: planned
// requires: condition Outcome
// requires: engine: a two-word subtype in a card's type line
// requires: operation Guess
// Whenever The Seventh Doctor attacks, choose a card in your hand. Defending player guesses whether that card's mana value is greater than the number of artifacts you control. If they guessed wrong, you may cast it without paying its mana cost. If you don't cast a spell this way, investigate. (Create a Clue token. It's an artifact with "{2}, Sacrifice this token: Draw a card.")
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::ChooseAsEnters, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
g.end_sentence();
if let Some(present_3_0) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { planned!("operation-guess" => cantrip_mechanics::guess::guess(g, { let q: &dyn Query = g; present_3_0 }, asker.source, { let q: &dyn Query = g; (selected_0.clone()).get(0 as usize).copied() }, Some(|q: &dyn Query, asker: &Asker, id: ObjectId| (i64::from(q.mana_value(id))) > ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64)), None)?);
 }
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition Outcome", false => q.outcome(asker, Outcome::GuessedWrong)) } {
let caster_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_4: Vec<ObjectId> = { let q: &dyn Query = g; ((selected_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_4: u32 = 1; let mut played_4: Vec<ObjectId> = Vec::new();
while left_4 > 0 {
let mut open_4: Vec<ObjectId> = Vec::new(); for &c in &pool_4 { let ways = g.cast_ways_during_resolution_by(c, caster_4); if ways.iter().any(|w| true) { open_4.push(c); } }
let picked_4 = if open_4.is_empty() { None } else if open_4.len() > 1 || (false && (!false || pool_4.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_4, asker.source, Purpose::CastFree, pool_4.clone(), open_4.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_4, open_4[0], Purpose::CastFree)? { Some(open_4[0]) } else { None };
let Some(card_4) = picked_4 else { break; };
let ways_4: Vec<CastWay> = g.cast_ways_during_resolution_by(card_4, caster_4).into_iter().filter(|w| true).collect();
let done_4 = if ways_4.is_empty() { g.play_land_during_resolution(card_4)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_4, card_4, ways_4)?.map(|w| w.way); g.cast_during_resolution_as(card_4, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_4), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_4 { left_4 -= 1; played_4.push(g.state().obj(card_4).became.unwrap_or(card_4)); } else if true { break; } if done_4 || !(true && false) { pool_4.retain(|&c| c != card_4); }
}
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition Outcome", false => q.outcome(asker, Outcome::NoSpellCast)) } {
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; for _ in 0..times { cantrip_mechanics::tokens::investigate(g, player)?; }; }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Seventh Doctor", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[planned_value!("engine: a two-word subtype in a card's type line", Subtype::TimeLord => Subtype::TimeLord),Subtype::Doctor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever The Seventh Doctor attacks, choose a card in your hand. Defending player guesses whether that card's mana value is greater than the number of artifacts you control. If they guessed wrong, you may cast it without paying its mana cost. If you don't cast a spell this way, investigate.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::tokens::CLUE];
