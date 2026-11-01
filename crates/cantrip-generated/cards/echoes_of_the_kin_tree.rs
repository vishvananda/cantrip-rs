// Echoes of the Kin Tree
// status: ready
// {2}{W}: Bolster 1. (Choose a creature with the least toughness among creatures you control and put a +1/+1 counter on it.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut bolstered_0: Vec<ObjectId> = Vec::new();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; let creatures: Vec<ObjectId> = g.permanents_of(player).into_iter().filter(|&o| g.chars(o).is(CardType::Creature)).collect(); let least = creatures.iter().map(|&o| g.chars(o).toughness.unwrap_or(0)).min(); let fewest: Vec<ObjectId> = creatures.into_iter().filter(|&o| Some(g.chars(o).toughness.unwrap_or(0)) == least).collect(); if let Some(creature) = cantrip_mechanics::choose::card(g, player, asker.source, Purpose::ChooseAsEnters, fewest)? { g.add_counters(creature, CounterKind::PlusOne, amount); bolstered_0.push(creature); }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Echoes of the Kin Tree", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{2}{W}: Bolster 1.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
