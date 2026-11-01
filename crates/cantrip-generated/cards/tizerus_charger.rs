// Tizerus Charger
// status: ready
// Escape—{4}{B}, Exile five other cards from your graveyard. (You may cast this card from your graveyard for its escape cost.)
// This creature escapes with your choice of a +1/+1 counter or a flying counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_escaped(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => { entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(1);  }
Some(1) => { entering.counters[CounterKind::Flying as usize] = entering.counters[CounterKind::Flying as usize].saturating_add(1);  }
_ => {} }
 Ok(()) }
fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let escaped = { let q: &dyn Query = g; cantrip_mechanics::escape::escaped(q, entering.object) }; if escaped { ability_1_escaped(g, entering)?; } Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tizerus Charger", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Pegasus]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ways: &[cantrip_mechanics::escape::way(ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, 5, "Escape—{4}{B}, Exile five other cards from your graveyard.")], labels: &["PowerToughness", "Named"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
