// Fixed Point in Time
// status: planned
// requires: card type Phenomenon
// requires: keyword action ChaosEnsues
// requires: layout Planar
// requires: replacement of PlaneswalkByDie
// requires: trigger event Player EncountersPhenomenon
// When you encounter Fixed Point in Time, until your next turn, if a player would planeswalk as a result of rolling the planar die, chaos ensues instead. (Then planeswalk away from this phenomenon.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables, unused_mut)] fn made_1_then_steps(g: &mut dyn Game, asker: &Asker, affected: ObjectId, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { planned!("keyword-action-chaos-ensues" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::planechase::chaos_ensues(g, player)?; });
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") })); } }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-encounters-phenomenon" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::EncounteredPhenomenon { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("replacement-of-planeswalk-by-die" => callbacks: #[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::PlaneswalkByDie(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if true { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::PlaneswalkByDie(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::PlaneswalkByDie(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn made_1_then(g: &mut dyn Game, asker: &Asker, affected: ObjectId, before: &Proposal, _: &Proposal) { if let Proposal::PlaneswalkByDie(p) = before { let q: &dyn Query = g; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; let _ = made_1_then_steps(g, asker, affected, subject, data); } }
);
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Fixed Point in Time", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Phenomenon", CardType::Land => CardType::Phenomenon)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player EncountersPhenomenon", EventMask(0) => EventMask(EventMask::PHENOMENON.0)), ability_0_event), ability_0, "When you encounter Fixed Point in Time, until your next turn, if a player would planeswalk as a result of rolling the planar die, chaos ensues instead.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[planned_value!("replacement of PlaneswalkByDie", granted_replacement(ReplacementKind::Damage, |_, _, _, _| false, |_, _, _, _| (), "") => granted_replacement(ReplacementKind::PlaneswalkByDie, made_1_applies, made_1_apply, "When you encounter Fixed Point in Time, until your next turn, if a player would planeswalk as a result of rolling the planar die, chaos ensues instead.").then(made_1_then))], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
