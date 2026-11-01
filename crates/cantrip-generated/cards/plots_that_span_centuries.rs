// Plots That Span Centuries
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: replacement of SetSchemeInMotion
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, the next time you would set a scheme in motion, set three schemes in motion instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("replacement-of-set-scheme-in-motion" => callbacks: #[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::SetSchemeInMotion(p) = proposal { let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if Some(asker.controller) == Some(p.player) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_1_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::SetSchemeInMotion(seen) = &seen { let p = seen; let subject = Target::Player(p.player); let data = [u32::MAX; 2]; if let Proposal::SetSchemeInMotion(p) = proposal { p.replaced = true;p.times = (3i64).max(0) as u32; } } }
);
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Plots That Span Centuries", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, the next time you would set a scheme in motion, set three schemes in motion instead.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[planned_value!("replacement of SetSchemeInMotion", granted_replacement(ReplacementKind::Damage, |_, _, _, _| false, |_, _, _, _| (), "") => granted_replacement(ReplacementKind::SetSchemeInMotion, made_1_applies, made_1_apply, "When you set this scheme in motion, the next time you would set a scheme in motion, set three schemes in motion instead.").once())], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
