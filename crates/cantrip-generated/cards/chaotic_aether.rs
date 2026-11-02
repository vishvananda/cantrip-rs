// Chaotic Aether
// status: planned
// requires: card type Phenomenon
// requires: duration UntilEvent
// requires: layout Planar
// requires: operation GameEffect
// requires: trigger event Player EncountersPhenomenon
// requires: trigger event Player PlaneswalksAway
// When you encounter Chaotic Aether, each blank roll of the planar die is a {CHAOS} roll until a player planeswalks away from a plane. (Then planeswalk away from this phenomenon.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables)] fn lasting_until_0(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-game-effect" => cantrip_mechanics::game_rules::for_a_time(g, asker, planned_value!("duration UntilEvent", Duration::Indefinite => { let q: &dyn Query = g; Duration::UntilEvent({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) }), &Vec::<ObjectId>::new(), &Vec::<PlayerId>::new(), GameEffect::PlanarDieBlanksRollsAreChaos));
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-encounters-phenomenon" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::EncounteredPhenomenon { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("trigger-event-player-planeswalks-away" => matcher: #[allow(unused_variables)] fn lasting_until_0(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::PlaneswalkedAway { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("duration-until-event" => lasting_until: LastingUntil { text: "a player planeswalks away from a plane", ends: when_asked(planned_value!("trigger event Player PlaneswalksAway", EventMask(0) => EventMask(EventMask::PLANESWALKED.0)), lasting_until_0) });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Chaotic Aether", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Phenomenon", CardType::Land => CardType::Phenomenon)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player EncountersPhenomenon", EventMask(0) => EventMask(EventMask::PHENOMENON.0)), ability_0_event), ability_0, "When you encounter Chaotic Aether, each blank roll of the planar die is a {CHAOS} roll until a player planeswalks away from a plane.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
