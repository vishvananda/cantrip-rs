// Corrupted Shapeshifter
// status: ready
// Devoid (This card has no color.)
// As this creature enters, it becomes your choice of a 3/3 creature with flying, a 2/5 creature with vigilance, or a 0/12 creature with defender.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.colors = ColorSet::NONE; }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..3)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => { entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(3, 3).with_type(CardType::Creature).with_keyword(Keyword::Flying) });
 }
Some(1) => { entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(2, 5).with_type(CardType::Creature).with_keyword(Keyword::Vigilance) });
 }
Some(2) => { entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(0, 12).with_type(CardType::Creature).with_keyword(Keyword::Defender) });
 }
_ => {} }
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Corrupted Shapeshifter", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi,Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[characteristic_defining(Layer::Color, ability_0_apply_0, "Devoid")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_1_enters), labels: &["3/3 creature with flying", "2/5 creature with vigilance", "0/12 creature with defender"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
