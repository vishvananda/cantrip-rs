// Brutal Expulsion
// status: ready
// Devoid (This card has no color.)
// Choose one or both —
// • Return target spell or creature to its owner's hand.
// • Brutal Expulsion deals 2 damage to target creature or planeswalker. If that creature or planeswalker would die this turn, exile it instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.colors = ColorSet::NONE; }
#[allow(unused_variables)] fn ability_1_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack || q.state().obj(id).zone.kind() == ZoneKind::Battlefield) && ((((q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Stack) && true) || ((q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Battlefield) && q.chars(id).is(CardType::Creature))))) }
static ABILITY_1_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_0_target_0)];
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
static ABILITY_1_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_1_target_0)];
#[allow(unused_variables)] fn made_3_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Graveyard) && z.from.is(ZoneKind::Battlefield) && Some(affected) == Some(id) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_3_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::exile(q.state().obj(z.object).owner); } } }
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_1 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); g.damage_together(&batch); }
g.end_sentence();
{  let to = Some(Target::None); let about = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; if let (Some(to), Some(about)) = (to, about) { let _ = to; g.add_effect(item.object, about, EffectKind::Replacement { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
static ABILITY_1_MODES: ModalDef = modes(&[mode(ability_1_mode_0, "Return target spell or creature to its owner's hand.").targets(ABILITY_1_MODE_0_TARGETS),mode(ability_1_mode_1, "Brutal Expulsion deals 2 damage to target creature or planeswalker. If that creature or planeswalker would die this turn, exile it instead.").targets(ABILITY_1_MODE_1_TARGETS)]).choose(1, 2);
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Brutal Expulsion", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[characteristic_defining(Layer::Color, ability_0_apply_0, "Devoid")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_1_MODES), enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ZoneChange, made_3_applies, made_3_apply, "Choose one or both — Return target spell or creature to its owner's hand; or Brutal Expulsion deals 2 damage to target creature or planeswalker. If that creature or planeswalker would die this turn, exile it instead.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
