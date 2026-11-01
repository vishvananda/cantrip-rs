// Embrace My Diabolical Vision
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, each player shuffles their hand and graveyard into their library. You draw seven cards, then each other player draws four cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend({ let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } });
    } 
let objects_3 = ({ let q: &dyn Query = g; gathered_1.clone() }).into_iter().rev().collect::<Vec<ObjectId>>(); let moved_3 = g.put_into_library(&objects_3, LibraryPlace::Top);
g.end_sentence();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
    } 
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; { let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Embrace My Diabolical Vision", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, each player shuffles their hand and graveyard into their library. You draw seven cards, then each other player draws four cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
