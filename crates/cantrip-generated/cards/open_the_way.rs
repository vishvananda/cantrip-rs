// Open the Way
// status: planned
// requires: a limit on the X announced
// X can't be greater than the number of players in the game.
// Reveal cards from the top of your library until you reveal X land cards. Put those land cards onto the battlefield tapped and the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_0 = { let q: &dyn Query = g; (i64::from(item.x)) as usize }; let (found_0, rest_0) = cantrip_mechanics::library::reveal_until_n(g, owner_0, count_0, &|q: &dyn Query, id: ObjectId| q.chars(id).is(CardType::Land)); let shown_0 = found_0.len() + rest_0.len(); let objects_0: Vec<ObjectId> = g.state().zone(Zone::library(owner_0)).iter().copied().take(shown_0).collect();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; (objects_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, true)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
let library_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_2: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_0.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_2)).collect(); let placed_2 = cards_2.clone();
g.bottom_in_random_order(library_2, &cards_2);
g.end_sentence();
 Ok(()) }
planned_item!("a-limit-on-the-x-announced", "(kind:Casting,args:Require((kind:Compare,args:(left:(kind:AnnouncedX),comparison:LessOrEqual,right:(kind:CountPlayers,args:(kind:All))))))");

pub static CARD: CardDef = CardDef { name: "Open the Way", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
