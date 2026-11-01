// Windriddle Palaces
// status: planned
// requires: a library's top card open to view
// requires: card type Plane
// requires: layout Planar
// requires: play permission LibraryTops
// requires: subtype Belenon
// requires: trigger event Player ChaosEnsues
// Players play with the top card of their libraries revealed.
// You may play lands and cast spells from the top of any player's library.
// Whenever chaos ensues, each player mills a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_1_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
 Ok(()) }
planned_item!("a-library-s-top-card-open-to-view" => play_rules: PlayRule::LibraryTopRevealed(|q, asker, player| true));
planned_item!("play-permission-library-tops" => statics: player_rule(ability_1_0_applies, PlayerRule::Play { zone: ZoneKind::Library, cards: CardsOfRule::Asked(|q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { lands: true, spells: false, ..PlayGrant::CAST } }));
planned_item!("play-permission-library-tops" => statics: player_rule(ability_1_1_applies, PlayerRule::Play { zone: ZoneKind::Library, cards: CardsOfRule::Asked(|q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { ..PlayGrant::CAST } }));
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Windriddle Palaces", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Belenon", Subtype::Plains => Subtype::Belenon)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_2_event), ability_2, "Whenever chaos ensues, each player mills a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
