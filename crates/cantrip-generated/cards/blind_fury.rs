// Blind Fury
// status: ready
// All creatures lose trample until end of turn. If a creature would deal combat damage to a creature this turn, it deals double that damage to that creature instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::Damage(e) = proposal { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && q.chars(source).is(CardType::Creature) && matches!(*target, Target::Object(o) if q.chars(o).is(CardType::Creature)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_2_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Damage(seen) = &seen { let e = seen; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if let Proposal::Damage(e) = proposal { e.amount = ((2i64 * i64::from(data[0]))).clamp(0, 65535) as u16; } } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::LoseKeywords(Keywords::of(&[Keyword::Trample])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Blind Fury", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::Damage, made_2_applies, made_2_apply, "All creatures lose trample until end of turn. If a creature would deal combat damage to a creature this turn, it deals double that damage to that creature instead.").text("If a creature would deal combat damage to a creature this turn, it deals double that damage to that creature instead.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
