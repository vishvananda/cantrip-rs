// Phyrexian Vindicator
// status: planned
// requires: engine: no static prevention with an effect besides
// Flying
// If damage would be dealt to this creature, prevent that damage. When damage is prevented this way, this creature deals that much damage to any other target.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn reflexive_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { match t { Target::Player(p) => true, Target::Object(id) => q.is_on_battlefield(id) && { let c = q.chars(id); c.is(CardType::Creature) || c.is(CardType::Planeswalker) || c.is(CardType::Battle) } && (!(Some(asker.source) == Some(id))), _ => false } }
static REFLEXIVE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_1_target_0)];
fn reflexive_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u16 }; let recipients_2 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_2 { let batch: Vec<(ObjectId, Target, u16)> = recipients_2.iter().map(|t| (source, *t, amount_2)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { if done_prevented > 0 && true { g.reflexive_trigger_counted(asker.source, asker.controller, AbilityRef::delayed({ let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, 0), Target::None, ((done_prevented as i64)).clamp(0, i64::from(u16::MAX)) as u16); }
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, parts: &[Prevented]) { let Some(first) = parts.first() else { return }; let prevented = Prevented::total(parts).min(u16::MAX.into()) as u16; let e = &Prevented { amount: prevented, ..*first }; let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_1_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return true; } }
 false }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-static-prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "If damage would be dealt to this creature, prevent that damage. When damage is prevented this way, this creature deals that much damage to any other target.", applies: ability_1_applies, amount: None, then: Some(ability_1_then) });

pub static CARD: CardDef = CardDef { name: "Phyrexian Vindicator", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [4, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_1, "When damage is prevented this way, this creature deals that much damage to any other target.").targets(REFLEXIVE_1_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
