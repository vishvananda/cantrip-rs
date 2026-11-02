// Stoneshock Giant
// status: planned
// requires: engine: no resolving rule on an open set: Prohibit Block
// {6}{R}{R}: Monstrosity 3. (If this creature isn't monstrous, put three +1/+1 counters on it and it becomes monstrous.)
// When this creature becomes monstrous, creatures without flying your opponents control can't block this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as i16 }; g.monstrosity(asker.source, amount_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Monstrous { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-block" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && !(q.chars(id).keywords.has(Keyword::Flying)) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) });
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-block" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-block" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "When this creature becomes monstrous, creatures without flying your opponents control can't block this turn.", rule: LastingRuleKind::CantBlock, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Stoneshock Giant", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Giant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{6}{R}{R}: Monstrosity 3.").targets(&[]),triggered(when_asked(EventMask(EventMask::MONSTROUS.0), ability_1_event), ability_1, "When this creature becomes monstrous, creatures without flying your opponents control can't block this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
