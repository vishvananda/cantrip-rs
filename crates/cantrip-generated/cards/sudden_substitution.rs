// Sudden Substitution
// status: ready
// Split second (As long as this spell is on the stack, players can't cast spells or activate abilities that aren't mana abilities.)
// Exchange control of target noncreature spell and target creature. Then the spell's controller may choose new targets for it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (!(q.chars(id).is(CardType::Creature)))) }
#[allow(unused_variables)] fn ability_1_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0),target(ability_1_target_1)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let (spell_0, permanent_0) = ({ let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }, { let q: &dyn Query = g; t_1_raw.get(0).copied().and_then(|t: Target| t.object()) }); if let (Some(spell), Some(permanent)) = (spell_0, permanent_0) { if g.target_legal(item, 0) && g.target_legal(item, 1) { let by = g.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == spell).map(|i| i.controller); let other = g.controller(permanent); if let Some(by) = by { if by != other { cantrip_api::instruction(g, |g| { g.change_controller(spell, other); g.add_effect(asker.source, permanent, EffectKind::Control(by), Duration::Indefinite); }); } } } }
g.end_sentence();
let chooser_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let retarget_1: Option<Target> = t_0_raw.get(0).copied().filter(|t| t_0.contains(t)); if let Some(t) = retarget_1 {
if let Target::Object(spell) = t { g.choose_new_targets(spell, chooser_1)?; }
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Sudden Substitution", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::SplitSecond]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
