// Bristlepack Sentry
// status: ready
// Defender
// As long as you control a creature with power 4 or greater, this creature can attack as though it didn't have defender.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) >= (4i64)))).count() >= 1) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) >= (4i64)))).count() >= 1) && id == asker.source }

pub static CARD: CardDef = CardDef { name: "Bristlepack Sentry", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Plant,Subtype::Wolf]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, attacks_despite_defender: &[ability_1_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
