// Mortipede
// status: planned
// requires: engine: no resolving rule on an open set: RequireCombat Block beyond lasting rules
// {2}{G}: All creatures able to block this creature this turn do so.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-block-beyond-lasting-rules" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; q.chars(id).is(CardType::Creature) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = { let q: &dyn Query = g; Some(asker.source) }.map(Target::Object).unwrap_or(Target::None); let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-require-combat-block-beyond-lasting-rules" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-require-combat-block-beyond-lasting-rules" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "{2}{G}: All creatures able to block this creature this turn do so.", rule: LastingRuleKind::MustBlockAbout, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Mortipede", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Insect]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{2}{G}: All creatures able to block this creature this turn do so.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
