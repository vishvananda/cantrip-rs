// Bend or Break
// status: ready
// Each player separates all nontoken lands they control into two piles. For each player, one of their piles is chosen by one of their opponents of their choice. Destroy all lands in the chosen piles. Tap all lands in the other piles.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut piles_0: Vec<(PlayerId, Vec<Vec<ObjectId>>, Option<usize>, Vec<PlayerId>)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let separator_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let mut rest_1: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut piles_1: Vec<Vec<ObjectId>> = Vec::new(); for _ in 1..2 { let (first, second) = cantrip_mechanics::choose::separate(g, Some(separator_1), asker.source, rest_1)?; piles_1.push(first); rest_1 = second; } piles_1.push(rest_1); let choosers_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
piles_0.push((separator_1, piles_1, None, choosers_1));
    } 
for row in piles_0.iter_mut() { if let Some(by) = cantrip_mechanics::choose::player(g, row.0, asker.source, row.3.clone(), "chooses one of your piles", 0)? { let (chosen, _) = cantrip_mechanics::choose::pile(g, by, asker.source, row.1[0].clone(), row.1[1].clone())?; row.2 = Some(usize::from(chosen != row.1[0])); } }
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| piles_0.iter().any(|row| row.2.is_some_and(|at| row.1[at].contains(&id)))).collect::<Vec<ObjectId>>() }; let destroyed_2 = g.destroy_all(&objects_2);
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| piles_0.iter().any(|row| row.1.iter().enumerate().any(|(at, pile)| row.2.is_some_and(|chosen| chosen != at) && pile.contains(&id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_3: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Bend or Break", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
