// Reverse the Polarity
// status: planned
// requires: engine: no resolving rule on an open set: Prohibit BeBlocked
// Choose one —
// • Counter all other spells.
// • Switch each creature's power and toughness until end of turn.
// • Creatures can't be blocked this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().zone(Zone::STACK).to_vec()).into_iter().filter(|&id| true && !(Some(asker.source) == Some(id))).collect::<Vec<ObjectId>>() }; cantrip_api::instruction(g, |g| { for spell in objects_0 { g.counter(spell); } });
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::SwitchPt };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; q.chars(id).is(CardType::Creature) });
fn ability_0_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Counter all other spells.").targets(&[]),mode(ability_0_mode_1, "Switch each creature's power and toughness until end of turn.").targets(&[]),mode(ability_0_mode_2, "Creatures can't be blocked this turn.").targets(&[])]).choose(1, 1);
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Creatures can't be blocked this turn.", rule: LastingRuleKind::CantBeBlocked, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Reverse the Polarity", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
