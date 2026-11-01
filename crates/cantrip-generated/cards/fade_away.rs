// Fade Away
// status: planned
// requires: numeric expression RepeatedCostsUnpaid
// requires: operation PlayersPayRepeatedly
// requires: per-player PlayersPayRepeatedly
// For each creature, its controller sacrifices a permanent of their choice unless they pay {1}.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-players-pay-repeatedly" => g.begin_batch(););
 for (index_0, bound_9000) in players_0.iter().copied().enumerate() { planned!("operation-players-pay-repeatedly" => cantrip_mechanics::players::each_pays_repeatedly(g, { let q: &dyn Query = g; Some(bound_9000).into_iter().collect::<Vec<PlayerId>>() }, asker.source, "Cost { parts: [Mana([Generic(1)])] }", { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_9000) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 65535) as u16 })?);
    } 
planned!("per-player-players-pay-repeatedly" => g.end_batch(););
g.end_sentence();
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_3: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() }; let players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_4: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((planned_value!("numeric expression RepeatedCostsUnpaid", 0i64 => Some(bound_0).map_or(0i64, |p: PlayerId| unpaid_0.iter().filter(|who| **who == p).count() as i64))).max(0) as u32) }; for player in players_4 { let eligible: Vec<ObjectId> = objects_4.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_4.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_4.clone(), &[]))? }; for object in taken { picks_4.push((player, object)); } }
gathered_3.extend(picks_4.iter().copied());
    } 
let sacrificed_3 = g.sacrifice_each(&gathered_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fade Away", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
