// Danitha Capashen, Paragon
// status: ready
// First strike, vigilance, lifelink
// Aura and Equipment spells you cast cost {1} less to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) && q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))) { CostDelta::less((1i64).max(0) as u32) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Danitha Capashen, Paragon", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike,Keyword::Vigilance,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_3_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
