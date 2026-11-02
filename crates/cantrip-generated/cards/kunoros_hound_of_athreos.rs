// Kunoros, Hound of Athreos
// status: ready
// Vigilance, menace, lifelink
// Creature cards in graveyards can't enter the battlefield.
// Players can't cast spells from graveyards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_enter(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { !(q.obj(id).zone.is(ZoneKind::Graveyard) && (|q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature))(q, asker, id)) }
#[allow(unused_variables)] fn ability_4_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, from: ZoneKind) -> bool { !(true && from == ZoneKind::Graveyard) }

pub static CARD: CardDef = CardDef { name: "Kunoros, Hound of Athreos", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dog]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance,Keyword::Menace,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_rules: &[ability_4_0_rule], enter_rules: &[ability_3_enter], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
