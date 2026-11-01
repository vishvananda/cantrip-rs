// Suq'Ata Firewalker
// status: ready
// This creature can't be the target of red spells or abilities from red sources.
// {T}: This creature deals 1 damage to any target.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); subject == asker.source }
#[allow(unused_variables)] fn ability_0_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (by.spell && (*by_source).colors.has(q.words(asker.source).color(Color::Red))) }
#[allow(unused_variables)] fn ability_0_1_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); subject == asker.source }
#[allow(unused_variables)] fn ability_0_1_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(by.spell) && (!by.spell && (*by_source).colors.has(q.words(asker.source).color(Color::Red)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Suq'Ata Firewalker", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: This creature deals 1 damage to any target.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, targeting_rules: &[TargetingRule { kind: TargetingRuleKind::Forbid(ability_0_0_rule_by), applies: ability_0_0_rule_on },TargetingRule { kind: TargetingRuleKind::Forbid(ability_0_1_rule_by), applies: ability_0_1_rule_on }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
