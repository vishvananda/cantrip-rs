// Ignite the Cloneforge!
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, create a token that's a copy of target permanent an opponent controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_0 = match original_0 { Some(original) => g.create_token_copies(original, player_0, count_0)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Ignite the Cloneforge!", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, create a token that's a copy of target permanent an opponent controls.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
