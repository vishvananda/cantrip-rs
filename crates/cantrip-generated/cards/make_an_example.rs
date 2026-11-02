// Make an Example
// status: ready
// Each opponent separates the creatures they control into two piles. For each opponent, you choose one of their piles. Each opponent sacrifices the creatures in their chosen pile. (Piles can be empty.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut piles_0: Vec<(PlayerId, Vec<Vec<ObjectId>>, Option<usize>, Vec<PlayerId>)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let separator_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let mut rest_1: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut piles_1: Vec<Vec<ObjectId>> = Vec::new(); for _ in 1..2 { let (first, second) = cantrip_mechanics::choose::separate(g, Some(separator_1), asker.source, rest_1)?; piles_1.push(first); rest_1 = second; } piles_1.push(rest_1); let choosers_1: Vec<PlayerId> = Vec::<PlayerId>::new();
piles_0.push((separator_1, piles_1, None, choosers_1));
    } 
g.end_sentence();
let chooser_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let from_2 = Some({ let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }); let mut chosen_2: Vec<ObjectId> = Vec::new(); for row in piles_0.iter_mut() { if from_2.as_ref().is_some_and(|from| !from.contains(&row.0)) { continue; } let at = { let (chosen, _) = cantrip_mechanics::choose::pile(g, chooser_2, asker.source, row.1[0].clone(), row.1[1].clone())?; usize::from(chosen != row.1[0]) }; row.2 = Some(at); chosen_2.extend(row.1[at].iter().copied()); }
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| piles_0.iter().any(|row| row.2.is_some_and(|at| row.1[at].contains(&id)))).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Make an Example", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
