// Explosion of Riches
// status: planned
// requires: target constraint AtRandom
// Draw a card, then each other player may draw a card. Whenever a card is drawn this way, Explosion of Riches deals 5 damage to target opponent chosen at random from among your opponents.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn reflexive_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static REFLEXIVE_2_TARGETS: &[cantrip_api::TargetSpec] = &[planned_value!("target constraint AtRandom", target(reflexive_2_target_0) => target(reflexive_2_target_0).at_random())];
fn reflexive_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; (5i64).max(0) as u16 }; let recipients_3 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn reflexive_9_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static REFLEXIVE_9_TARGETS: &[cantrip_api::TargetSpec] = &[planned_value!("target constraint AtRandom", target(reflexive_9_target_0) => target(reflexive_9_target_0).at_random())];
fn reflexive_9(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let amount_10 = { let q: &dyn Query = g; (5i64).max(0) as u16 }; let recipients_10 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_10 { let batch: Vec<(ObjectId, Target, u16)> = recipients_10.iter().map(|t| (source, *t, amount_10)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_0: i64 = 0; let mut drawn_0: Vec<ObjectId> = Vec::new(); for player in players_0 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_0); done_0 += g.history().drawn(player) as i64 - before; drawn_0.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
if done_0 > 0 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; { let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } };
let mut guard_4_0 = vec![false; players_4.len()];
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7 = vec![Default::default(); players_4.len()];
let mut done_total_7: i64 = 0;
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_4_0[index_4] = true;
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
let mut done_5: i64 = 0; let mut drawn_5: Vec<ObjectId> = Vec::new(); for player in players_5 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_5); done_5 += g.history().drawn(player) as i64 - before; drawn_5.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
 each_6[index_4] = drawn_5.clone(); all_6.extend(each_6[index_4].iter().copied());
each_7[index_4] = done_5;
done_total_7 += done_5;
  }
 } 
g.end_sentence();
if done_total_7 > 0 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Explosion of Riches", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_2, "Draw a card, then each other player may draw a card. Whenever a card is drawn this way, Explosion of Riches deals 5 damage to target opponent chosen at random from among your opponents.").targets(REFLEXIVE_2_TARGETS),delayed_trigger(reflexive_9, "Draw a card, then each other player may draw a card. Whenever a card is drawn this way, Explosion of Riches deals 5 damage to target opponent chosen at random from among your opponents.").targets(REFLEXIVE_9_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
