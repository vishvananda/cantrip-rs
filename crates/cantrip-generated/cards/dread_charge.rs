// Dread Charge
// status: planned
// requires: engine: no resolving rule on an open set: Prohibit BeBlocked beyond lasting rules
// Black creatures you control can't be blocked this turn except by black creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn block_rule_0(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(attacker == source && !((q.chars(blocker).colors.has(q.words(asker.source).color(Color::Black)) && q.chars(blocker).is(CardType::Creature)))) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked-beyond-lasting-rules" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).colors.has(q.words(asker.source).color(Color::Black)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked-beyond-lasting-rules" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked-beyond-lasting-rules" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Black creatures you control can't be blocked this turn except by black creatures.", rule: LastingRuleKind::BlockRestriction(0), applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Dread Charge", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, lasting_block_rules: &[LastingBlockRule { text: "Can't be blocked except by black creatures", allows: block_rule_0 }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
