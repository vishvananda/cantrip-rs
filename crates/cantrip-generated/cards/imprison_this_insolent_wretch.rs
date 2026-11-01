// Imprison This Insolent Wretch
// status: planned
// requires: a choice of Player outside its body
// requires: card type Scheme
// requires: layout Scheme
// requires: operation AbandonScheme
// requires: supertype Ongoing
// requires: trigger event Player SetsSchemeInMotion
// (An ongoing scheme remains face up until it's abandoned.)
// When you set this scheme in motion, choose an opponent.
// Permanents the chosen player controls don't untap during their untap step.
// When the chosen player is attacked or becomes the target of a spell or ability, abandon this scheme.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_0) = cantrip_mechanics::choose::player(g, player_0, asker.source, candidates_0, "Choose", 0)? else { return Ok(()) };
planned!("a-choice-of-player-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Player(chosen_0)););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked_to(asker.source))) == Some(q.controller(affected)) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked_to(asker))) == Some(q.controller(id)) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if true && match *defender { Target::Player(p) => planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked_to(asker))) == Some(p), _ => false } { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-abandon-scheme" => cantrip_mechanics::scheme::abandon(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source));
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::BecameTarget { target: Target::Player(target), by, controller, .. } = event { let by = *by; let controller = *controller; let subject = Target::Player(*target); let data = [by.0, u32::from(controller.0) | (q.state().stack.len().saturating_sub(1) as u32) << 8]; if planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked_to(asker))) == Some(*target) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-abandon-scheme" => cantrip_mechanics::scheme::abandon(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source));
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Imprison This Insolent Wretch", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype Ongoing", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, ongoing: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, choose an opponent.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "When the chosen player is attacked or becomes the target of a spell or ability, abandon this scheme.").targets(&[]).once_per_batch().batch_by_data(),triggered(when_asked(EventMask(EventMask::BECAME_TARGET.0), ability_3_event), ability_3, "When the chosen player is attacked or becomes the target of a spell or ability, abandon this scheme.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[ability_1_0_rule], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
