// Wingfold Pteron
// status: ready
// This creature enters with your choice of a flying counter or a hexproof counter on it. (A creature with hexproof can't be the target of spells or abilities your opponents control.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => { entering.counters[CounterKind::Flying as usize] = entering.counters[CounterKind::Flying as usize].saturating_add(1);  }
Some(1) => { entering.counters[CounterKind::Hexproof as usize] = entering.counters[CounterKind::Hexproof as usize].saturating_add(1);  }
_ => {} }
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wingfold Pteron", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["Named", "Named"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
