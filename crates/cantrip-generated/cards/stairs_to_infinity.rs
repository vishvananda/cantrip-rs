// Stairs to Infinity
// status: planned
// requires: a maximum hand size of other players
// requires: card type Plane
// requires: keyword action RevealTopOfPlanarDeck
// requires: keyword action TopOfPlanarDeckToBottom
// requires: layout Planar
// requires: subtype Xerex
// requires: trigger event Player ChaosEnsues
// requires: trigger event Player RollsPlanarDie
// Players have no maximum hand size.
// Whenever you roll the planar die, draw a card.
// Whenever chaos ensues, reveal the top card of your planar deck. You may put it on the bottom of your planar deck.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("keyword-action-reveal-top-of-planar-deck" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::planechase::reveal_top(g, player)?; });
g.end_sentence();
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? {
planned!("keyword-action-top-of-planar-deck-to-bottom" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::planechase::top_to_bottom(g, player)?; });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("a-maximum-hand-size-of-other-players" => hand_size_rules: HandSizeRule { applies: |q, asker, player| true, size: HandSize::Unlimited });
planned_item!("trigger-event-player-rolls-planar-die" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::PlanarDieRolled { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None });
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Stairs to Infinity", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Xerex", Subtype::Plains => Subtype::Xerex)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player RollsPlanarDie", EventMask(0) => EventMask(EventMask::PLANAR_DIE.0)), ability_1_event), ability_1, "Whenever you roll the planar die, draw a card.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_2_event), ability_2, "Whenever chaos ensues, reveal the top card of your planar deck. You may put it on the bottom of your planar deck.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
