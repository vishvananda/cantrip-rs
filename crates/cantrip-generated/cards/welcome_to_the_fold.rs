// Welcome to the Fold
// status: planned
// requires: cost branch Madness
// Madness {X}{U}{U} (If you discard this card, discard it into exile. When you do, cast it for its madness cost or put it into your graveyard.)
// Gain control of target creature if its toughness is 2 or less. If this spell's madness cost was paid, instead gain control of that creature if its toughness is X or less.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_branch_paid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_BRANCH_PAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_branch_paid_target_0)];
fn ability_0_branch_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_1: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).is_some_and(|candidate: ObjectId| q.chars(candidate).toughness.is_some_and(|v| (i64::from(v)) <= (i64::from(item.x)))) } {
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::Indefinite;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(asker.controller).expect("a reference that always names a player")) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
hoisted_1.extend(objects_0.clone());
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_branch_unpaid_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_BRANCH_UNPAID_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_branch_unpaid_target_0)];
fn ability_0_branch_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).is_some_and(|candidate: ObjectId| q.chars(candidate).toughness.is_some_and(|v| (i64::from(v)) <= (2i64))) } {
let objects_2 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::Indefinite;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(asker.controller).expect("a reference that always names a player")) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
hoisted_3.extend(objects_2.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("cost-branch-madness" => cost_branches: CostBranch { keyword: "Madness", cost: Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), paid: spell(ABILITY_0_BRANCH_PAID_TARGETS, ability_0_branch_paid), unpaid: spell(ABILITY_0_BRANCH_UNPAID_TARGETS, ability_0_branch_unpaid) });

pub static CARD: CardDef = CardDef { name: "Welcome to the Fold", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
