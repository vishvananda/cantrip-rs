// Case of the Market Melee
// status: planned
// requires: event value in a target filter beyond its subject
// requires: target with a divided amount read as chosen
// When this Case enters, it deals 1 damage to any target.
// Damage isn't removed from creatures during cleanup steps.
// To solve — Three or more creatures are damaged.
// Solved — Whenever you attack with one or more creatures, this Case deals that much damage divided as you choose among any number of targets.
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_0 = { let q: &dyn Query = g; item.subject.object() }; let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (*so_far).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
fn ability_2_to_solve(q: &dyn Query, asker: &Asker) -> bool { (((q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).damage > 0).collect::<Vec<ObjectId>>()).len() as i64) >= (3i64) }
static ABILITY_2_SOLVED_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
#[allow(unused_variables)] fn ability_2_solved_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && true && q.is_solved(asker.source) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_solved_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::damage_divided(g, item);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Case of the Market Melee", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Case]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this Case enters, it deals 1 damage to any target.").targets(ABILITY_0_TARGETS),cantrip_mechanics::solved::to_solve(ability_2_to_solve, "To solve (If unsolved, solve at the beginning of your end step.)"),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_solved_0_event), ability_2_solved_0, "To solve — Three or more creatures are damaged.").targets(ABILITY_2_SOLVED_0_TARGETS).once_per_batch().divide(Divide { total: planned_value!("target with a divided amount read as chosen", 1u32 => Divide::read(|q: &dyn Query, asker: &Asker| (planned_value!("event value in a target filter beyond its subject", 0i64 => q.event_total_at(asker))).max(0) as u32)), of_x: None })], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, damage_stays: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
