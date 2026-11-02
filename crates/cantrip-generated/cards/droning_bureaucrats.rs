// Droning Bureaucrats
// status: planned
// requires: engine: no resolving rule on an open set: Prohibit Attack
// requires: engine: no resolving rule on an open set: Prohibit Block
// requires: numeric expression AnnouncedX
// {X}, {T}: Each creature with mana value X can't attack or block this turn.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-attack" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) == (planned_value!("numeric expression AnnouncedX", 0i64 => i64::from(q.announced_x().unwrap_or(0))))) });
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-block" => #[allow(unused_variables)] fn lasting_rule_1(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) == (planned_value!("numeric expression AnnouncedX", 0i64 => i64::from(q.announced_x().unwrap_or(0))))) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-attack" => cantrip_api::instruction(g, |g| { g.add_lasting_rule(item, 0, about_0, until_0); g.add_lasting_rule(item, 1, about_0, until_0); }););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-attack" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Each creature with mana value X can't attack or block this turn.", rule: LastingRuleKind::CantAttack, applies: lasting_rule_0 },LastingRule { text: "Each creature with mana value X can't attack or block this turn.", rule: LastingRuleKind::CantBlock, applies: lasting_rule_1 }];);

pub static CARD: CardDef = CardDef { name: "Droning Bureaucrats", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{X}, {T}: Each creature with mana value X can't attack or block this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
