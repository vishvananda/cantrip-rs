// Aquamorph Entity
// status: planned
// requires: ability AsEvent
// As this creature enters or is turned face up, it becomes your choice of 5/1 or 1/5.
// Morph {2}{U} (You may cast this card face down as a 2/2 creature for {3}. Turn it face up any time for its morph cost.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-as-event" => as_events: AsEventDef { event: AsEvent::EntersOrTurnsFaceUp, object: |g: &mut dyn Game, asker: &Asker| { let q: &dyn Query = g; Some(asker.source) }, actions: &[EnterOrFaceUpAction::EntersWithPTOfChoice(PT::PT(5, 1), PT::PT(1, 5))] });

pub static CARD: CardDef = CardDef { name: "Aquamorph Entity", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::morph::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Morph {2}{U}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
