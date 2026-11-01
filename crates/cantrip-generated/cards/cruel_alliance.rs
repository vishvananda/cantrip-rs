// Cruel Alliance
// status: planned
// requires: cost branch Teamwork with targets of its own
// Teamwork 2 (As an additional cost to cast this spell, you may tap any number of creatures you control with total power 2 or more.)
// Exile target creature with mana value 3 or less. If this spell was cast using teamwork, instead exile target creature and you gain 3 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_branch_paid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_BRANCH_PAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_branch_paid_target_0)];
fn ability_0_branch_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_branch_unpaid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (3i64)))) }
static ABILITY_0_BRANCH_UNPAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_branch_unpaid_target_0)];
fn ability_0_branch_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("cost-branch-teamwork-with-targets-of-its-own" => cost_branches: CostBranch { keyword: "Teamwork", cost: Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap_creatures: Some(TapCreatures { power: 2, greater_with: None }), ..Cost::NONE }), paid: spell(ABILITY_0_BRANCH_PAID_TARGETS, ability_0_branch_paid), unpaid: spell(ABILITY_0_BRANCH_UNPAID_TARGETS, ability_0_branch_unpaid) });

pub static CARD: CardDef = CardDef { name: "Cruel Alliance", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Cast without teamwork", "Teamwork 2: Tap any number of creatures you control with total power 2 or more"], late_targets: true, optional_cost: Some(cantrip_mechanics::teamwork::optional(2)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
