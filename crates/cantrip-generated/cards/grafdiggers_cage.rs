// Grafdigger's Cage
// status: ready
// Creature cards in graveyards and libraries can't enter the battlefield.
// Players can't cast spells from graveyards or libraries.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enter(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { !(q.obj(id).zone.is(ZoneKind::Graveyard) && (|q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature))(q, asker, id)) }
#[allow(unused_variables)] fn ability_1_enter(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { !(q.obj(id).zone.is(ZoneKind::Library) && (|q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature))(q, asker, id)) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, from: ZoneKind) -> bool { !(true && from == ZoneKind::Graveyard) }
#[allow(unused_variables)] fn ability_2_1_rule(q: &dyn Query, asker: &Asker, player: PlayerId, from: ZoneKind) -> bool { !(true && from == ZoneKind::Library) }

pub static CARD: CardDef = CardDef { name: "Grafdigger's Cage", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_rules: &[ability_2_0_rule,ability_2_1_rule], enter_rules: &[ability_0_enter,ability_1_enter], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
