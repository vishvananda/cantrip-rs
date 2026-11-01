// Tibalt, Wicked Tormentor
// status: planned
// requires: operation DraftFromSpellbook
// [+1]: Add {R}{R}. Draft a card from Tibalt, Wicked Tormentor's spellbook, then exile it. Until end of turn, you may cast that card.
// [+1]: Tibalt deals 4 damage to target creature or planeswalker unless its controller has Tibalt deal 4 damage to them. If they do, you may discard a card. If you do, draw a card.
// [−X]: Create X 1/1 red Devil creature tokens with "When this token dies, it deals 1 damage to any target."
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::R, mana_1_0);}
g.end_sentence();
let times_2: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut drafted_2: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { drafted_2.extend(planned_value!("operation DraftFromSpellbook", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::draft_from_spellbook(g, player, asker.source, "Tibalt, Wicked Tormentor", times_2)?)); }
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (drafted_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let cards_4_0 = { let q: &dyn Query = g; (moved_3.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_4_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_4_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_4_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let payer_5 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") };
let paid_5 = cantrip_mechanics::effects::ask_may(g, payer_5, asker.source, Purpose::Optional)?; if paid_5 {
let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let amount_6 = { let q: &dyn Query = g; (4i64).max(0) as u16 }; let recipients_6 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_6 { let batch: Vec<(ObjectId, Target, u16)> = recipients_6.iter().map(|t| (source, *t, amount_6)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
let payer_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_7, after_7, before_7, picked_7): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_9 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_9 = Vec::new(); for player in players_9 { cards_9.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_9, true)? }); } let discarded_9 = g.discard_together(&cards_9);
let paid_8 = !discarded_9.clone().is_empty();
(paid_8, { let q: &dyn Query = g; (discarded_9).clone() }, { let q: &dyn Query = g; (discarded_9.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_7 as i64) != 0) } {
let amount_10 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_10 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_10 { g.draw(player, amount_10); }
g.end_sentence();
}
g.end_sentence();
hoisted_11.extend(after_7.clone());
hoisted_12.extend({ let q: &dyn Query = g; before_7.clone() });
} else {
let source_13 = { let q: &dyn Query = g; Some(asker.source) }; let amount_13 = { let q: &dyn Query = g; (4i64).max(0) as u16 }; let recipients_13 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_13 { let batch: Vec<(ObjectId, Target, u16)> = recipients_13.iter().map(|t| (source, *t, amount_13)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
pub mod token_15 {
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_0 = { let q: &dyn Query = g; item.subject.object() }; let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: token!(Devil).name, token_words: token!(Devil).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Devil]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(8)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this token dies, it deals 1 damage to any target.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_14 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_14 = Vec::new();
let count_14_0 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u32 }; let def_14_0 = g.card_id(&token_15::CARD);
for player in players_14 {
tokens_14.extend(std::iter::repeat((def_14_0,player)).take(count_14_0 as usize));
} let words_14 = g.words(asker.source);
let created_14 = g.create_tokens_worded(&tokens_14, false, None, words_14)?;
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Tibalt, Wicked Tormentor", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Tibalt]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(3), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_0, "+1: Add {R}{R}. Draft a card from Tibalt, Wicked Tormentor's spellbook, then exile it. Until end of turn, you may cast that card.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: Tibalt deals 4 damage to target creature or planeswalker unless its controller has Tibalt deal 4 damage to them. If they do, you may discard a card. If you do, draw a card.").targets(ABILITY_1_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty_minus_x()), ability_2, "−X: Create X 1/1 red Devil creature tokens with \"When this token dies, it deals 1 damage to any target.\"").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_15::CARD];
