// Festival
// status: planned
// requires: engine: no resolving rule on an open set: Prohibit Attack
// Cast this spell only during an opponent's upkeep.
// Creatures can't attack this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| Some(bound_0) == Some(q.state().turn.active)) && q.state().turn.step() == Step::Upkeep) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-attack" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; q.chars(id).is(CardType::Creature) });
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-attack" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-attack" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Creatures can't attack this turn.", rule: LastingRuleKind::CantAttack, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Festival", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_only_if: Some(ability_0_if), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
