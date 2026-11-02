// Ria Ivor, Bane of Bladehold
// status: ready
// Battle cry (Whenever this creature attacks, each other attacking creature gets +1/+0 until end of turn.)
// At the beginning of combat on your turn, the next time target creature would deal combat damage to one or more players this combat, prevent that damage. If damage is prevented this way, create that many 1/1 colorless Phyrexian Mite artifact creature tokens with toxic 1 and "This token can't block."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Phyrexian Mite).name, token_words: token!(Phyrexian Mite).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Mite]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBlock]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, toxic: 1, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
#[allow(unused_variables, unused_mut)] fn made_1_then_steps(g: &mut dyn Game, asker: &Asker, affected: ObjectId, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { if { let q: &dyn Query = g; ((done_prevented as i64)) > (0i64) } {
let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; ((done_prevented as i64)) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = g.words(asker.source);
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
}
 Ok(()) }
#[allow(unused_variables, dead_code)] fn made_1_then(g: &mut dyn Game, asker: &Asker, affected: ObjectId, parts: &[Prevented]) { let Some(first) = parts.first() else { return }; let prevented = Prevented::total(parts).min(u16::MAX.into()) as u16; let e = &Prevented { amount: prevented, ..*first }; let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = made_1_then_steps(g, asker, affected, subject, data, prevented); }
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(affected) == Some(source) && matches!(*target, Target::Player(p) if true) { return true; } }
 false }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
{  let to = Some(Target::None); let about = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::NextTime }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Ria Ivor, Bane of Bladehold", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::battle_cry::TRIGGER,triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of combat on your turn, the next time target creature would deal combat damage to one or more players this combat, prevent that damage. If damage is prevented this way, create that many 1/1 colorless Phyrexian Mite artifact creature tokens with toxic 1 and \"This token can't block.\"").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "At the beginning of combat on your turn, the next time target creature would deal combat damage to one or more players this combat, prevent that damage. If damage is prevented this way, create that many 1/1 colorless Phyrexian Mite artifact creature tokens with toxic 1 and \"This token can't block.\"", damage: "damage", of_it: None, applies: made_1_applies, then: Some(made_1_then), text: "At the beginning of combat on your turn, the next time target creature would deal combat damage to one or more players this combat, prevent that damage. If damage is prevented this way, create that many 1/1 colorless Phyrexian Mite artifact creature tokens with toxic 1 and \"This token can't block.\"" }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
