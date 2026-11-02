// Damping Matrix
// status: ready
// Activated abilities of artifacts and creatures can't be activated unless they're mana abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Artifact) || (*so_far).is(CardType::Creature)) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { let id = object; !(q.is_on_battlefield(id) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (!(ability.is_mana_ability()))) }

pub static CARD: CardDef = CardDef { name: "Damping Matrix", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, activation_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
