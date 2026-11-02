// Celestine Reef
// status: planned
// requires: card type Plane
// requires: duration UntilEvent
// requires: layout Planar
// requires: player rule for a while can't lose the game
// requires: player rule for a while can't win the game
// requires: subtype Luvion
// requires: trigger event Player ChaosEnsues
// requires: trigger event Player PlaneswalksAway
// Creatures without flying or islandwalk can't attack.
// Whenever chaos ensues, until a player planeswalks, you can't lose the game and your opponents can't win the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && !((*so_far).keywords.has(Keyword::Flying)) && !((*so_far).keywords.has(Keyword::Islandwalk))) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !((q.chars(attacker).is(CardType::Creature) && !(q.chars(attacker).keywords.has(Keyword::Flying)) && !(q.chars(attacker).keywords.has(Keyword::Islandwalk)))) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables)] fn lasting_until_0(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables)] fn lasting_until_1(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-rule-for-a-while-can-t-lose-the-game" => let players_0_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_0_0 { g.add_player_effect(asker.controller, source, EffectKind::CantLoseGame(player), planned_value!("duration UntilEvent", Duration::Indefinite => { let q: &dyn Query = g; Duration::UntilEvent({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) })); } });
g.end_sentence();
planned!("player-rule-for-a-while-can-t-win-the-game" => let players_1_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_1_0 { g.add_player_effect(asker.controller, source, EffectKind::CantWinGame(player), planned_value!("duration UntilEvent", Duration::Indefinite => { let q: &dyn Query = g; Duration::UntilEvent({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 1) })); } });
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("trigger-event-player-planeswalks-away" => matcher: #[allow(unused_variables)] fn lasting_until_0(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::PlaneswalkedAway { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("trigger-event-player-planeswalks-away" => matcher: #[allow(unused_variables)] fn lasting_until_1(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::PlaneswalkedAway { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("duration-until-event" => lasting_until: LastingUntil { text: "a player planeswalks", ends: when_asked(planned_value!("trigger event Player PlaneswalksAway", EventMask(0) => EventMask(EventMask::PLANESWALKED.0)), lasting_until_0) });
planned_item!("duration-until-event" => lasting_until: LastingUntil { text: "a player planeswalks", ends: when_asked(planned_value!("trigger event Player PlaneswalksAway", EventMask(0) => EventMask(EventMask::PLANESWALKED.0)), lasting_until_1) });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Celestine Reef", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Luvion", Subtype::Plains => Subtype::Luvion)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, until a player planeswalks, you can't lose the game and your opponents can't win the game.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_0_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
