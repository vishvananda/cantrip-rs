// War Tax
// status: planned
// requires: continuous rule CombatCost
// requires: engine: no resolving rule on an open set: CombatCost
// {X}{U}: This turn, creatures can't attack unless their controller pays {X} for each attacking creature they control.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-resolving-rule-on-an-open-set-combat-cost" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; q.chars(id).is(CardType::Creature) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-combat-cost" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-combat-cost" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "{X}{U}: This turn, creatures can't attack unless their controller pays {X} for each attacking creature they control.", rule: LastingRuleKind::ObjectRule(0), applies: lasting_rule_0 }];);
planned_item!("continuous-rule-combat-cost" => lasting_object_rules: LastingObjectRule { text: "{X}{U}: This turn, creatures can't attack unless their controller pays {X} for each attacking creature they control.", rule: Rule::CostTo(ObjectAction::Attack(|q: &dyn Query, asker: &Asker, defender: Target| true), None::<PlayerId>, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))) });

pub static CARD: CardDef = CardDef { name: "War Tax", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{X}{U}: This turn, creatures can't attack unless their controller pays {X} for each attacking creature they control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
