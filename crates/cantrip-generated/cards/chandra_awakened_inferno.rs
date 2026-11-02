// Chandra, Awakened Inferno
// status: ready
// This spell can't be countered.
// [+2]: Each opponent gets an emblem with "At the beginning of your upkeep, this emblem deals 1 damage to you."
// [−3]: Chandra deals 3 damage to each non-Elemental creature.
// [−X]: Chandra deals X damage to target creature or planeswalker. If a permanent dealt damage this way would die this turn, exile it instead.
#[allow(unused_imports)] use cantrip_api::*;
pub mod emblem_1 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Chandra, Awakened Inferno emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, this emblem deals 1 damage to you.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_1::CARD); }
    } 
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; (3i64).max(0) as u16 }; let recipients_2 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Elemental))) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_2 { let batch: Vec<(ObjectId, Target, u16)> = recipients_2.iter().map(|t| (source, *t, amount_2)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)))) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_target_0)];
#[allow(unused_variables)] fn made_6_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Graveyard) && z.from.is(ZoneKind::Battlefield) && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Planeswalker)) && Some(affected) == Some(id)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_6_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::exile(q.state().obj(z.object).owner); } } }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut done_3: i64 = 0; let mut dealt_3: Vec<ObjectId> = Vec::new(); let mut hurt_3: Vec<PlayerId> = Vec::new(); let before_3 = g.history().head() as usize; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u16 }; let recipients_3 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); done_3 += g.damage_together(&batch).iter().map(|&n| i64::from(n)).sum::<i64>(); dealt_3.extend(cantrip_mechanics::history::permanents_dealt_damage_by(g.history(), source, before_3)); for r in &g.history().all()[before_3..] { if let Event::Damage { source: s, target: Target::Player(p), amount, .. } = r.event { if s == source && amount > 0 && !hurt_3.contains(&p) { hurt_3.push(p); } } } }
g.end_sentence();
for bound_1005 in dealt_3.clone() {
{  let to = Some(Target::None); let about = { let q: &dyn Query = g; Some(bound_1005) }; if let (Some(to), Some(about)) = (to, about) { let _ = to; g.add_effect(item.object, about, EffectKind::Replacement { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Chandra, Awakened Inferno", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Chandra]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(6), defense: None, keywords: Keywords::of(&[Keyword::CantBeCountered]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(2)), ability_1, "+2: Each opponent gets an emblem with \"At the beginning of your upkeep, this emblem deals 1 damage to you.\"").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-3)), ability_2, "−3: Chandra deals 3 damage to each non-Elemental creature.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty_minus_x()), ability_3, "−X: Chandra deals X damage to target creature or planeswalker. If a permanent dealt damage this way would die this turn, exile it instead.").targets(ABILITY_3_TARGETS).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ZoneChange, made_6_applies, made_6_apply, "−X: Chandra deals X damage to target creature or planeswalker. If a permanent dealt damage this way would die this turn, exile it instead.").text("If a permanent dealt damage this way would die this turn, exile it instead.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&emblem_1::CARD];
