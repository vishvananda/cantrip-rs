// Revival Experiment
// status: ready
// For each permanent type, return up to one card of that type from your graveyard to the battlefield. You lose 3 life for each card returned this way. Exile Revival Experiment.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chooser_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let pool_0: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut picked_0: Vec<ObjectId> = Vec::new();
let types_0 = cantrip_mechanics::targets::PERMANENT_CARD_TYPES.to_vec(); for t in types_0 { let eligible: Vec<ObjectId> = pool_0.iter().copied().filter(|&c| !picked_0.contains(&c) && g.chars(c).is(t)).collect(); if eligible.is_empty() { continue; } picked_0.extend(cantrip_mechanics::choose::cards_from(g, chooser_0, asker.source, Purpose::PutOntoBattlefield, pool_0.clone(), eligible, 1, true)?); }
let objects_1 = ({ let q: &dyn Query = g; picked_0.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; ((3i64 * (moved_1.clone()).len() as i64)).min(i32::MAX.into()) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.lose_life(player, amount_2); }
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Revival Experiment", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
