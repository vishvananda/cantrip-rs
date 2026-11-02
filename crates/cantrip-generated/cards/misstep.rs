// Misstep
// status: planned
// requires: duration DuringControllersNextUntapStep
// requires: engine: no resolving rule on an open set: UntapStep
// Creatures target player controls don't untap during that player's next untap step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
planned_item!("engine-no-resolving-rule-on-an-open-set-untap-step" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && about.player() == Some(q.controller(id))) });
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let about_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) }.map(Target::Player).unwrap_or(Target::None); let until_0 = planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringNextUntapStepOf({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }));
planned!("engine-no-resolving-rule-on-an-open-set-untap-step" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-resolving-rule-on-an-open-set-untap-step" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Creatures target player controls don't untap during that player's next untap step.", rule: LastingRuleKind::DoesntUntap, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Misstep", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
