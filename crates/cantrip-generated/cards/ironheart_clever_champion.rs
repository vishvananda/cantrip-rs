// Ironheart, Clever Champion
// status: ready
// Improvise (Your artifacts can help cast this spell. Each artifact you tap after you're done activating mana abilities pays for {1}.)
// Flying
// Noncreature spells you cast have improvise.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_pay_with(q: &dyn Query, source: ObjectId, caster: PlayerId, spell: ObjectId) -> PayWith { let asker = &Asker::new(source, q.controller(source)); if Some(asker.controller) == Some(caster) && (!(q.chars(spell).is(CardType::Creature))) { PayWith { improvise: true, ..PayWith::NONE } } else { PayWith::NONE } }

pub static CARD: CardDef = CardDef { name: "Ironheart, Clever Champion", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, grants_pay_with: &[ability_2_0_pay_with], pay_with: PayWith { improvise: true, ..PayWith::NONE }, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
