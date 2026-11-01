// Chief Engineer
// status: ready
// Artifact spells you cast have convoke. (Your creatures can help cast those spells. Each creature you tap while casting an artifact spell pays for {1} or one mana of that creature's color.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_pay_with(q: &dyn Query, source: ObjectId, caster: PlayerId, spell: ObjectId) -> PayWith { let asker = &Asker::new(source, q.controller(source)); if Some(asker.controller) == Some(caster) && (q.chars(spell).is(CardType::Artifact)) { PayWith { convoke: true, ..PayWith::NONE } } else { PayWith::NONE } }

pub static CARD: CardDef = CardDef { name: "Chief Engineer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vedalken,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, grants_pay_with: &[ability_0_0_pay_with], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
