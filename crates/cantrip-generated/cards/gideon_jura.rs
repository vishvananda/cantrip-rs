// Gideon Jura
// status: planned
// requires: duration DuringNextTurn
// requires: player rule for a while MustAttack
// [+2]: During target opponent's next turn, creatures that player controls attack Gideon Jura if able.
// [−2]: Destroy target tapped creature.
// [0]: Until end of turn, Gideon Jura becomes a 6/6 Human Soldier creature that's still a planeswalker. Prevent all damage that would be dealt to him this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("player-rule-for-a-while-must-attack" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::MustAttack { attackers: |q: &dyn Query, asker: &Asker, attacker: ObjectId| q.chars(attacker).is(CardType::Creature), defenders: |q: &dyn Query, asker: &Asker, defender: Target| matches!(defender, Target::Object(id) if Some(asker.source) == Some(id)), count: None }), planned_value!("duration DuringNextTurn", Duration::Indefinite => Duration::DuringNextTurn(Target { slot: Slot(0), index: 0 }))); } });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.state().obj(id).tapped && q.chars(id).is(CardType::Creature)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_2 = g.destroy_all(&objects_2);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn made_5_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((6i64) as i16, (6i64) as i16) };
let kind_3_1_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Human)) };
let kind_3_2_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Soldier)) };
let kind_3_3_0 = { let q: &dyn Query = g; EffectKind::AddTypes(TypeSet::of(&[CardType::Creature])) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3); g.add_effect(source, object, kind_3_1_0, duration_3); g.add_effect(source, object, kind_3_2_0, duration_3); g.add_effect(source, object, kind_3_3_0, duration_3);  } }); }
g.end_sentence();
{  let to = { let q: &dyn Query = g; ((Some(asker.source)).map(Target::Object).into_iter().collect::<Vec<Target>>()).first().copied() }; let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Gideon Jura", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Gideon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(6), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(2)), ability_0, "+2: During target opponent's next turn, creatures that player controls attack Gideon Jura if able.").targets(ABILITY_0_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_1, "−2: Destroy target tapped creature.").targets(ABILITY_1_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(0)), ability_2, "0: Until end of turn, Gideon Jura becomes a 6/6 Human Soldier creature that's still a planeswalker. Prevent all damage that would be dealt to him this turn.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "0: Until end of turn, Gideon Jura becomes a 6/6 Human Soldier creature that's still a planeswalker. Prevent all damage that would be dealt to him this turn.", damage: "damage", of_it: None, applies: made_5_applies, then: None, text: "Prevent all damage that would be dealt to him this turn." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
