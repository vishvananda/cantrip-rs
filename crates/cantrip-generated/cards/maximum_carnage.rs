// Maximum Carnage
// status: planned
// requires: continuous rule RequireCombat Attack
// requires: engine: no resolving rule on an open set: RequireCombat Attack
// requires: engine: no resolving rule on an open set: RequireCombat Attack beyond lasting rules
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Until your next turn, each creature attacks each combat if able and attacks a player other than you if able.
// II — Add {R}{R}{R}.
// III — This Saga deals 5 damage to each opponent.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; q.chars(id).is(CardType::Creature) });
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack-beyond-lasting-rules" => #[allow(unused_variables)] fn lasting_rule_1(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; q.chars(id).is(CardType::Creature) });
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
planned!("engine-no-resolving-rule-on-an-open-set-require-combat-attack-beyond-lasting-rules" => cantrip_api::instruction(g, |g| { g.add_lasting_rule(item, 0, about_0, until_0); g.add_lasting_rule(item, 1, about_0, until_0); }););
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::R, mana_2_0);}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; (5i64).max(0) as u16 }; let recipients_3 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-attack-beyond-lasting-rules" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Until your next turn, each creature attacks each combat if able and attacks a player other than you if able.", rule: LastingRuleKind::MustAttack, applies: lasting_rule_0 },LastingRule { text: "Until your next turn, each creature attacks each combat if able and attacks a player other than you if able.", rule: LastingRuleKind::ObjectRule(0), applies: lasting_rule_1 }];);
planned_item!("continuous-rule-require-combat-attack" => lasting_object_rules: LastingObjectRule { text: "Until your next turn, each creature attacks each combat if able and attacks a player other than you if able.", rule: Rule::Must(ObjectAction::Attack(|q: &dyn Query, asker: &Asker, defender: Target| (match defender { Target::Player(p) => ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&p), Target::Object(id) => false && q.chars(id).is(CardType::Planeswalker) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)), _ => false })), Required::AtLeast(1i64)) });

pub static CARD: CardDef = CardDef { name: "Maximum Carnage", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Until your next turn, each creature attacks each combat if able and attacks a player other than you if able.").targets(&[]),chapter(2, ability_0_1, "II — Add {R}{R}{R}.").targets(&[]),chapter(3, ability_0_2, "III — This Saga deals 5 damage to each opponent.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
