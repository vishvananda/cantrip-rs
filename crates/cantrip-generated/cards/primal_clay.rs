// Primal Clay
// status: ready
// As this creature enters, it becomes your choice of a 3/3 artifact creature, a 2/2 artifact creature with flying, or a 1/6 Wall artifact creature with defender in addition to its other types. (A creature with defender can't attack.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..3)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => { entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(3, 3).with_type(CardType::Artifact).with_type(CardType::Creature) });
 }
Some(1) => { entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(2, 2).with_type(CardType::Artifact).with_type(CardType::Creature).with_keyword(Keyword::Flying) });
 }
Some(2) => { entering.copy = Some({ let q: &dyn Query = g; q.copiable_values(entering.object).sized(1, 6).with_subtype(Subtype::Wall).with_type(CardType::Artifact).with_type(CardType::Creature).with_keyword(Keyword::Defender) });
 }
_ => {} }
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Primal Clay", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_0_enters), labels: &["3/3 artifact creature", "2/2 artifact creature with flying", "1/6 Wall artifact creature with defender"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
