// The Dark Barony
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: subtype Ulgrotha
// requires: trigger event Player ChaosEnsues
// Whenever a nonblack card is put into a player's graveyard from anywhere, that player loses 1 life.
// Whenever chaos ensues, each opponent discards a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && (!(q.chars(id).colors.has(q.words(asker.source).color(Color::Black))) && true && !(q.state().obj(id).token)) && !q.state().obj(id).token { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in g.state().apnap().filter(|p| players_1.contains(p)).collect::<Vec<PlayerId>>() { cards_1.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_1)?); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "The Dark Barony", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Ulgrotha", Subtype::Plains => Subtype::Ulgrotha)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever a nonblack card is put into a player's graveyard from anywhere, that player loses 1 life.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, each opponent discards a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
