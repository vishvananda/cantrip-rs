// No Witnesses
// status: ready
// Each player who controls the most creatures investigates. Then destroy all creatures. (To investigate, create a Clue token. It's an artifact with "{2}, Sacrifice this token: Draw a card.")
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) == ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == p && q.chars(id).is(CardType::Creature)).count() as i64).max().unwrap_or(0))).collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { { let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let times = 1u32; for _ in 0..times { cantrip_mechanics::tokens::investigate(g, player)?; }; }
    } 
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let destroyed_2 = g.destroy_all(&objects_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "No Witnesses", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::tokens::CLUE];
