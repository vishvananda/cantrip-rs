// Comet, Stellar Pup
// status: planned
// requires: player rule for a while ActivationGrant
// requires: resolution cost Loyalty
// [0]: Roll a six-sided die.
// 1 or 2 — [+2], then create two 1/1 green Squirrel creature tokens. They gain haste until end of turn.
// 3 — [−1], then return a card with mana value 2 or less from your graveyard to your hand.
// 4 or 5 — Comet deals damage equal to the number of loyalty counters on him to a creature or player, then [−2].
// 6 — [+1], and you may activate Comet's loyalty ability two more times this turn.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Squirrel).name, token_words: token!(Squirrel).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Squirrel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 6).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_0) >= (1i64) && (value_0) <= (2i64)) } {
let paid_1 = planned_value!("resolution cost Loyalty", false => cantrip_mechanics::effects::pay(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(2)))?);
g.end_sentence();
let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = g.words(asker.source);
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (created_2.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::EndOfTurn;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
hoisted_5.extend(objects_4.clone());
hoisted_6.extend(created_2.clone());
}
if { let q: &dyn Query = g; (value_0) == (3i64) } {
let paid_7 = planned_value!("resolution cost Loyalty", false => cantrip_mechanics::effects::pay(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-1)))?);
g.end_sentence();
let selected_8_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (i64::from(q.mana_value(id))) <= (2i64)).collect::<Vec<ObjectId>>() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::PutIntoHand, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
let objects_9 = { let q: &dyn Query = g; selected_8.clone() }; let moved_9 = g.return_to_hand(&objects_9);
g.end_sentence();
hoisted_10.extend(moved_9.clone());
hoisted_11.extend(selected_8.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
if { let q: &dyn Query = g; ((value_0) >= (4i64) && (value_0) <= (5i64)) } {
let player_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_13: Vec<Target> = { let q: &dyn Query = g; { let mut all: Vec<Target> = Vec::new(); for t in (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() { if !all.contains(&t) { all.push(t); } } for t in (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() { if !all.contains(&t) { all.push(t); } } all } }; let chosen_13: Target = match candidates_13.len() { 0 => Target::None, 1 => candidates_13[0], _ => match g.ask(player_13, RequestKind::ChooseTarget { label: "", slot_player: None, together: None, source: asker.source, purpose: Purpose::Targets, candidates: candidates_13.clone(), optional: false, least: 1, count: 1, any_number: false })? { Answer::Targets(chosen) => chosen.first().copied().unwrap_or(Target::None), _ => Target::None } };
g.end_sentence();
let source_14 = { let q: &dyn Query = g; Some(asker.source) }; let amount_14 = { let q: &dyn Query = g; ((Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Loyalty))).sum::<i64>()).clamp(0, 65535) as u16 }; let recipients_14 = { let q: &dyn Query = g; vec![chosen_13] }; if let Some(source) = source_14 { let batch: Vec<(ObjectId, Target, u16)> = recipients_14.iter().map(|t| (source, *t, amount_14)).collect(); g.damage_together(&batch); }
g.end_sentence();
let paid_15 = planned_value!("resolution cost Loyalty", false => cantrip_mechanics::effects::pay(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)))?);
g.end_sentence();
}
if { let q: &dyn Query = g; (value_0) == (6i64) } {
let paid_16 = planned_value!("resolution cost Loyalty", false => cantrip_mechanics::effects::pay(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)))?);
g.end_sentence();
planned!("player-rule-for-a-while-activation-grant" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::ActivationGrant { abilities: |q: &dyn Query, asker: &Asker, permanent: ObjectId, def: Option<&ActivatedDef>| (def.is_some_and(|d| d.is_loyalty_ability()) && (def.is_some_and(|d| d.from == ZoneKind::Battlefield) && Some(asker.source) == Some(permanent))), grant: ActivationGrant::AdditionalTimes(2i64) }), Duration::EndOfTurn); } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Comet, Stellar Pup", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Comet]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(5), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(0)), ability_0, "0: Roll a six-sided die.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
