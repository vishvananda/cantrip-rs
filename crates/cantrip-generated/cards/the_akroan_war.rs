// The Akroan War
// status: planned
// requires: engine: no resolving rule on an open set: RequireCombat Attack
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Gain control of target creature for as long as this Saga remains on the battlefield.
// II — Until your next turn, creatures your opponents control attack each combat if able.
// III — Each tapped creature deals damage to itself equal to its power.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0)];
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::WhileSourceOnBattlefield;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(asker.controller).expect("a reference that always names a player")) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) });
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
planned!("engine-no-resolving-rule-on-an-open-set-require-combat-attack" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let sources_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.state().obj(id).tapped && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let mut batch_2: Vec<(ObjectId, Target, u16)> = Vec::new(); for &source in &sources_2 { let amount = { let q: &dyn Query = g; (Some(source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u16 }; for t in { let q: &dyn Query = g; (Some(source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { batch_2.push((source, t, amount)); } } let dealt_2 = g.damage_together(&batch_2); let done_2: i64 = dealt_2.iter().map(|&n| i64::from(n)).sum();
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Until your next turn, creatures your opponents control attack each combat if able.", rule: LastingRuleKind::MustAttack, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "The Akroan War", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Gain control of target creature for as long as this Saga remains on the battlefield.").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "II — Until your next turn, creatures your opponents control attack each combat if able.").targets(&[]),chapter(3, ability_0_2, "III — Each tapped creature deals damage to itself equal to its power.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
