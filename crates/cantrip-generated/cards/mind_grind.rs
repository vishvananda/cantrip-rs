// Mind Grind
// status: planned
// requires: object reference Bound
// requires: operation EachPlayer
// requires: operation LimitX
// requires: per-player ChangeZone
// requires: per-player EachPlayer
// Each opponent reveals cards from the top of their library until they reveal X land cards, then puts all cards revealed this way into their graveyard. X can't be 0.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-limit-x" => cantrip_mechanics::announce::limit_x(g, item, Comparison::Greater, { let q: &dyn Query = g; 0i64 }));
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
planned!("per-player-each-player" => g.begin_batch(););
 for (index_1, bound_9000) in players_1.iter().copied().enumerate() { planned!("operation-each-player", EachPlayer, "action" = "{\"args\":{\"at_most\":(),\"count\":{\"kind\":\"AnnouncedX\"},\"filter\":{\"args\":\"Land\",\"kind\":\"CardType\"},\"library\":{\"kind\":\"Acting\"}},\"kind\":\"RevealUntil\"}", "players" = { let q: &dyn Query = g; planned_value!("object reference Bound", None::<ObjectId> => bound_9000).into_iter().collect::<Vec<ObjectId>>() });
    } 
planned!("per-player-each-player" => g.end_batch(););
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
planned!("per-player-change-zone" => g.begin_batch(););
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let objects_5 = { let q: &dyn Query = g; ({ let mut all: Vec<ObjectId> = q.state().battlefield().to_vec(); all.extend_from_slice(q.state().zone(Zone::STACK)); all.extend_from_slice(q.state().zone(Zone::COMMAND)); for p in q.state().seats() { for zone in [Zone::library(p), Zone::hand(p), Zone::graveyard(p), Zone::exile(p)] { all.extend_from_slice(q.state().zone(zone)); } } all }).into_iter().filter(|&id| (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1)).contains(&id)).collect::<Vec<ObjectId>>() }; let moved_5 = { let moves: Vec<(ObjectId, Zone)> = objects_5.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
 each_6[index_3] = moved_5.clone(); all_6.extend(each_6[index_3].iter().copied());
each_7[index_3] = { let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_7.extend(each_7[index_3].iter().copied());
   } 
planned!("per-player-change-zone" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mind Grind", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
