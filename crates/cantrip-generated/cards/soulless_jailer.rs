// Soulless Jailer
// status: ready
// Permanent cards in graveyards can't enter the battlefield.
// Players can't cast noncreature spells from graveyards or exile.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enter(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { !(q.obj(id).zone.is(ZoneKind::Graveyard) && (|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)))(q, asker, id)) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && q.obj(spell).zone.is(ZoneKind::Graveyard) && !(q.chars(spell).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && q.obj(spell).zone.is(ZoneKind::Exile) && !(q.chars(spell).is(CardType::Creature))) }

pub static CARD: CardDef = CardDef { name: "Soulless Jailer", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Golem]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, enter_rules: &[ability_0_enter], spell_rules: &[ability_1_0_rule,ability_1_1_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
