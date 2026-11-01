// Depth Charge Colossus
// status: ready
// Prototype {4}{U}{U} — 6/6 (You may cast this spell with different mana cost, color, and size. It keeps its abilities and types.)
// This creature doesn't untap during your untap step.
// {3}: Untap this creature.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_0 { g.untap(object); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Depth Charge Colossus", token_words: &[], oracle: true, cost: ManaCost { generic: 9, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dreadnought]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(9), toughness: Some(9), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::DoesNotUntap]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{3}: Untap this creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, prototype: Some(Prototype { cost: ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, power: 6, toughness: 6, text: "Prototype {4}{U}{U} — 6/6" }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
