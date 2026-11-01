// Whims of the Fates
// status: planned
// requires: object filter InPile
// requires: operation ChoosePile
// requires: operation Piles
// requires: per-player Piles
// Starting with you, each player separates all permanents they control into three piles. Then each player chooses one of their piles at random and sacrifices those permanents. (Piles can be empty.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_0 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_0.iter().position(|&p| Some(p) == first_0) { players_0.rotate_left(at); }
planned!("per-player-piles" => g.begin_batch(););
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("operation-piles" => cantrip_mechanics::choose::into_piles(g, { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>() }, Some({ let q: &dyn Query = g; (3i64).max(0) as u32 }), &[], None)?);
    } 
planned!("per-player-piles" => g.end_batch(););
g.end_sentence();
planned!("operation-choose-pile" => cantrip_mechanics::choose::of_piles(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, Some({ let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }), true, &[], None, None)?);
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| planned_value!("object filter InPile", false => q.in_pile(asker, id, None))).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Whims of the Fates", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
