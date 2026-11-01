// Goblins of the Flarg
// status: ready
// Mountainwalk (This creature can't be blocked as long as defending player controls a Mountain.)
// When you control a Dwarf, sacrifice this creature.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_state(q: &dyn Query, asker: &Asker) -> bool { ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dwarf)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_0 { let controller = g.controller(object); if players_0.contains(&controller) && g.can_sacrifice(controller, object) { picks_0.push((controller, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Goblins of the Flarg", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Mountainwalk]), protection: ColorSet(0), spell: None, abilities: &[state_triggered(ability_1_state, ability_1, "When you control a Dwarf, sacrifice this creature.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
