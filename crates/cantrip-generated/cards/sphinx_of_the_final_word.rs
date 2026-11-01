// Sphinx of the Final Word
// status: ready
// This spell can't be countered.
// Flying
// Hexproof (This creature can't be the target of spells or abilities your opponents control.)
// Instant and sorcery spells you control can't be countered.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(affected).became.is_none() && q.state().obj(affected).zone.kind() == ZoneKind::Stack) && (((*so_far).is(CardType::Instant) || (*so_far).is(CardType::Sorcery)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::CantBeCountered); }

pub static CARD: CardDef = CardDef { name: "Sphinx of the Final Word", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeCountered,Keyword::Flying,Keyword::Hexproof]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_3_affects, ability_3_apply_0, "Instant and sorcery spells you control can't be countered.").everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
