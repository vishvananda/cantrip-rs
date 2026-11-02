// Chef's Kiss
// status: planned
// requires: new targets chosen at random
// Gain control of target spell that targets only a single permanent or player. Copy it, then reselect the targets at random for the spell and the copy. The new targets can't be you or a permanent you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| { let chosen = q.state().targets(i); !chosen.is_empty() && chosen.iter().all(|t| *t == chosen[0]) }))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("new-targets-chosen-at-random" => if let Some(spell) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { g.change_controller(spell, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }); cantrip_mechanics::copy::retarget(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, spell, NewTargets::Random)?; });
g.end_sentence();
planned!("new-targets-chosen-at-random" => if let Some(spell) = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) } { cantrip_mechanics::copy::on_stack(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, spell, { let q: &dyn Query = g; (1i64).max(0) as u32 }, NewTargets::Random)?; });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Chef's Kiss", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
