// Crystal Spray
// status: planned
// requires: instruction ChooseOne
// requires: modes implied by the target chosen
// requires: operation ChangeSpell
// Change the text of target spell or permanent by replacing all instances of one color word with another or one basic land type with another until end of turn.
// Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (true)) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_0, asker.source, vec![Label::Card(card_0, 0), Label::Card(card_0, 1)])?) {
0 => {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let words_1 = cantrip_mechanics::text_change::color_word(g, player_1, asker.source)?;
g.end_sentence();
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { cantrip_mechanics::spell_changes::change(g, asker, spell, &[SpellChange::Continuing(&[SpellChange::Effect(EffectKind::ChangeText { from: words_1.0, to: words_1.1 })], Duration::EndOfTurn)]); });
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
}
_ => {
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let words_4 = cantrip_mechanics::text_change::basic_land_type(g, player_4, asker.source)?;
g.end_sentence();
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { cantrip_mechanics::spell_changes::change(g, asker, spell, &[SpellChange::Effect(EffectKind::ChangeText { from: words_4.0, to: words_4.1 })]); });
g.end_sentence();
}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
static ABILITY_0_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_1_target_0)];
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_6, asker.source, vec![Label::Card(card_6, 0), Label::Card(card_6, 1)])?) {
0 => {
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let words_7 = cantrip_mechanics::text_change::color_word(g, player_7, asker.source)?;
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_8 = { let q: &dyn Query = g; Some(asker.source) }; let duration_8 = Duration::Indefinite;
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
let amount_12 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_12 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_12 { g.draw(player, amount_12); }
g.end_sentence();
hoisted_13.extend(objects_11.clone());
}
}
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = planned_value!("modes implied by the target chosen", modes(&[mode(ability_0_mode_0, "").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, "").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1) => modes(&[mode(ability_0_mode_0, "").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, "").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1).implied_by_target());
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Crystal Spray", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
