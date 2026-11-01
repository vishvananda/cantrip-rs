// Shipwreck Sentry
// status: ready
// Defender
// As long as an artifact entered the battlefield under your control this turn, this creature can attack as though it didn't have defender.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((q.history().entered().filter(|e| e.is(CardType::Artifact) && Some(asker.controller) == Some(e.controller)).count() as i64) >= (1i64)) && (source == affected) }
#[allow(unused_variables)] fn ability_1_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { ((q.history().entered().filter(|e| e.is(CardType::Artifact) && Some(asker.controller) == Some(e.controller)).count() as i64) >= (1i64)) && id == asker.source }

pub static CARD: CardDef = CardDef { name: "Shipwreck Sentry", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Pirate]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, attacks_despite_defender: &[ability_1_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
