// Grimdancer
// status: ready
// This creature enters with your choice of two different counters on it from among menace, deathtouch, and lifelink.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..3)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => { entering.counters[CounterKind::Menace as usize] = entering.counters[CounterKind::Menace as usize].saturating_add(1); entering.counters[CounterKind::Deathtouch as usize] = entering.counters[CounterKind::Deathtouch as usize].saturating_add(1);  }
Some(1) => { entering.counters[CounterKind::Menace as usize] = entering.counters[CounterKind::Menace as usize].saturating_add(1); entering.counters[CounterKind::Lifelink as usize] = entering.counters[CounterKind::Lifelink as usize].saturating_add(1);  }
Some(2) => { entering.counters[CounterKind::Deathtouch as usize] = entering.counters[CounterKind::Deathtouch as usize].saturating_add(1); entering.counters[CounterKind::Lifelink as usize] = entering.counters[CounterKind::Lifelink as usize].saturating_add(1);  }
_ => {} }
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Grimdancer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Nightmare]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["Named and Named", "Named and Named", "Named and Named"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
