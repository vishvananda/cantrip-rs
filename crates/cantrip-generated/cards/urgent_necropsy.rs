// Urgent Necropsy
// status: planned
// requires: cost part Keyword
// requires: earlier objects Targeted
// As an additional cost to cast this spell, collect evidence X, where X is the total mana value of the permanents this spell targets.
// Destroy up to one target artifact, up to one target creature, up to one target enchantment, and up to one target planeswalker.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Artifact))) }
#[allow(unused_variables)] fn ability_1_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_target_2(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Enchantment))) }
#[allow(unused_variables)] fn ability_1_target_3(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Planeswalker))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1),target(ability_1_target_1).between(0, 1),target(ability_1_target_2).between(0, 1),target(ability_1_target_3).between(0, 1)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let t_2_raw = announced(g, item, 2, false); let t_2 = announced(g, item, 2, true);
let t_3_raw = announced(g, item, 3, false); let t_3 = announced(g, item, 3, true);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (t_0_raw.get(0).copied().and_then(|t: Target| t.object()) == Some(id) || t_1_raw.get(0).copied().and_then(|t: Target| t.object()) == Some(id) || t_2_raw.get(0).copied().and_then(|t: Target| t.object()) == Some(id) || t_3_raw.get(0).copied().and_then(|t: Target| t.object()) == Some(id))).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Urgent Necropsy", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part Keyword", Cost::NONE => Cost::keyword(CostKeyword::CollectEvidence, Some(|q: &dyn Query, asker: &Asker| (planned_value!("earlier objects Targeted", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Targeted))).into_iter().map(|id: ObjectId| i64::from(q.mana_value(id))).sum::<i64>()))))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
