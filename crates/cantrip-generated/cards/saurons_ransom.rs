// Sauron's Ransom
// status: planned
// requires: engine: no Ring temptation
// Choose an opponent. They look at the top four cards of your library and separate them into a face-down pile and a face-up pile. Put one pile into your hand and the other into your graveyard. The Ring tempts you.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let player_1 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_1, &objects_1);
let separator_2 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let cards_2 = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let (face_down_2, face_up_2) = cantrip_mechanics::choose::separate_face_down_and_face_up(g, separator_2, asker.source, cards_2)?;
if !(face_down_2.is_empty() && face_up_2.is_empty()) { let chooser_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (keep_3, _) = cantrip_mechanics::choose::face_down_or_face_up_pile(g, chooser_3, asker.source, face_down_2, face_up_2.clone())?; if keep_3 == face_up_2 { g.return_to_hand_revealed(&keep_3); } else { g.return_to_hand(&keep_3); } }
let objects_4 = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let moved_4 = { let moves: Vec<(ObjectId, Zone)> = objects_4.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };

g.end_sentence();
planned!("engine-no-ring-temptation" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::ring::tempt(g, player, asker.source)?; });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sauron's Ransom", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
