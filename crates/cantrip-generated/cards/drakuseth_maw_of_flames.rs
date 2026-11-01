// Drakuseth, Maw of Flames
// status: ready
// Flying
// Whenever Drakuseth attacks, Drakuseth deals 4 damage to any target and 3 damage to each of up to two other targets.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { ((cantrip_mechanics::targets::ANY.filter)(q, asker, t)) && !q.chosen_targets().iter().skip(0).take(1).copied().collect::<Vec<Target>>().contains(&t) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY,target(ability_1_target_1).between(0, 2)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let mut batch_0: Vec<(ObjectId, Target, u16)> = Vec::new();
let amount_0_0 = { let q: &dyn Query = g; (4i64).max(0) as u16 }; if let Some(source) = source_0 { for t in { let q: &dyn Query = g; t_0.clone() } { batch_0.push((source, t, amount_0_0)); } }
let amount_0_1 = { let q: &dyn Query = g; (3i64).max(0) as u16 }; if let Some(source) = source_0 { for t in { let q: &dyn Query = g; t_1.clone() } { batch_0.push((source, t, amount_0_1)); } }
g.damage_together(&batch_0);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Drakuseth, Maw of Flames", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 3, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Drakuseth attacks, Drakuseth deals 4 damage to any target and 3 damage to each of up to two other targets.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
