// Worldpurge
// status: planned
// requires: engine: no effect empties a mana pool
// Return all permanents to their owners' hands. Each player chooses up to seven cards in their hand, then shuffles the rest into their library. Each player loses all unspent mana.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_2: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let all_3: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((7i64).max(0) as u32) }; let kept_3 = cantrip_mechanics::choose::select(g, bound_0, asker.source, Purpose::Keep, all_3.clone(), cantrip_mechanics::select::Selection::of(count_3, &[]))?; gathered_2.extend(all_3.iter().copied().filter(|o| !kept_3.contains(o)));
    } 
let objects_4 = { let q: &dyn Query = g; gathered_2.clone() }; let moved_4 = g.put_into_library(&objects_4, LibraryPlace::ShuffledIn);
g.end_sentence();
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { planned!("engine-no-effect-empties-a-mana-pool" => { let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; g.empty_mana_pool(player); });
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Worldpurge", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::W, Mana::U)),Some((Mana::W, Mana::U)),Some((Mana::W, Mana::U)),Some((Mana::W, Mana::U)),None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
