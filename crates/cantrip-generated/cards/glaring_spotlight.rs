// Glaring Spotlight
// status: planned
// requires: engine: no resolving rule on an open set: Prohibit BeBlocked
// requires: static rule TargetableDespite
// Creatures your opponents control with hexproof can be the targets of spells and abilities you control as though they didn't have hexproof.
// {3}, Sacrifice this artifact: Creatures you control gain hexproof until end of turn and can't be blocked this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(affected)) && ((*so_far).keywords.has(Keyword::Hexproof))) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)) && (q.chars(id).keywords.has(Keyword::Hexproof))) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) });
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Hexproof])) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Creatures you control gain hexproof until end of turn and can't be blocked this turn.", rule: LastingRuleKind::CantBeBlocked, applies: lasting_rule_0 }];);
planned_item!("static-rule-targetable-despite" => statics: object_rule(ability_0_0_rule, Rule::TargetableDespite { by: |q: &dyn Query, asker: &Asker, by: &Asker, by_source: &Chars| Some(asker.controller) == Some(by.controller), keyword: Keyword::Hexproof }));

pub static CARD: CardDef = CardDef { name: "Glaring Spotlight", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS), ability_1, "{3}, Sacrifice this artifact: Creatures you control gain hexproof until end of turn and can't be blocked this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
