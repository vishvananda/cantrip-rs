// Crystal Spray
// status: planned
// requires: instruction ChooseOne
// requires: operation ChangeSpell
// Change the text of target spell or permanent by replacing all instances of one color word with another or one basic land type with another until end of turn.
// Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (true)) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { ability_0_mode_0_target_0(q, asker, t) || ability_0_mode_1_target_0(q, asker, t) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
match planned_value!("instruction ChooseOne", 0u8 => { let card_0 = item.ability.card; cantrip_mechanics::choose::one_of(g, decider_0, asker.source, vec![Label::Card(card_0, 0), Label::Card(card_0, 1)])? }) {
0 => {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let words_1 = cantrip_mechanics::text_change::color_word(g, player_1, asker.source)?;
g.end_sentence();
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { let changes = { let q: &dyn Query = g; [SpellChange::EffectUntil(EffectKind::ChangeText { from: words_1.0, to: words_1.1 }, Duration::EndOfTurn)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
}
_ => {
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let words_3 = cantrip_mechanics::text_change::basic_land_type(g, player_3, asker.source)?;
g.end_sentence();
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { let changes = { let q: &dyn Query = g; [SpellChange::EffectUntil(EffectKind::ChangeText { from: words_3.0, to: words_3.1 }, Duration::EndOfTurn)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
}
}
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
match planned_value!("instruction ChooseOne", 0u8 => { let card_6 = item.ability.card; cantrip_mechanics::choose::one_of(g, decider_6, asker.source, vec![Label::Card(card_6, 0), Label::Card(card_6, 1)])? }) {
0 => {
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let words_7 = cantrip_mechanics::text_change::color_word(g, player_7, asker.source)?;
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_8 = { let q: &dyn Query = g; Some(asker.source) }; let duration_8 = Duration::EndOfTurn;
let kind_8_0_0 = { let q: &dyn Query = g; EffectKind::ChangeText { from: words_7.0, to: words_7.1 } };
if let Some(source) = source_8 { cantrip_api::instruction(g, |g| { for &object in &objects_8 { g.add_effect(source, object, kind_8_0_0, duration_8);  } }); }
g.end_sentence();
hoisted_9.extend(objects_8.clone());
}
_ => {
let player_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let words_10 = cantrip_mechanics::text_change::basic_land_type(g, player_10, asker.source)?;
g.end_sentence();
let objects_11 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_11 = { let q: &dyn Query = g; Some(asker.source) }; let duration_11 = Duration::EndOfTurn;
let kind_11_0_0 = { let q: &dyn Query = g; EffectKind::ChangeText { from: words_10.0, to: words_10.1 } };
if let Some(source) = source_11 { cantrip_api::instruction(g, |g| { for &object in &objects_11 { g.add_effect(source, object, kind_11_0_0, duration_11);  } }); }
g.end_sentence();
hoisted_12.extend(objects_11.clone());
}
}
g.end_sentence();
let amount_13 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_13 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_13 { g.draw(player, amount_13); }
g.end_sentence();
 Ok(()) }
fn ability_0_implied(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); let chosen = (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == 0).map(|i| g.state().target(item, i)).next().unwrap_or(Target::None);
if ability_0_mode_0_target_0(g, asker, chosen) { return ability_0_mode_0(g, item); }
if ability_0_mode_1_target_0(g, asker, chosen) { return ability_0_mode_1(g, item); }
ability_0_mode_1(g, item) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Crystal Spray", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0_implied), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
