// Honor's Reward
// status: ready
// You gain 4 life. Bolster 2. (Choose a creature with the least toughness among creatures you control and put two +1/+1 counters on it.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
let mut bolstered_1: Vec<ObjectId> = Vec::new();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; let creatures: Vec<ObjectId> = g.permanents_of(player).into_iter().filter(|&o| g.chars(o).is(CardType::Creature)).collect(); let least = creatures.iter().map(|&o| g.chars(o).toughness.unwrap_or(0)).min(); let fewest: Vec<ObjectId> = creatures.into_iter().filter(|&o| Some(g.chars(o).toughness.unwrap_or(0)) == least).collect(); if let Some(creature) = cantrip_mechanics::choose::card(g, player, asker.source, Purpose::ChooseAsEnters, fewest)? { g.add_counters(creature, CounterKind::PlusOne, amount); bolstered_1.push(creature); }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Honor's Reward", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
