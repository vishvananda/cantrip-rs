// Darksteel Monolith
// status: ready
// Indestructible
// Once each turn, you may pay {0} rather than pay the mana cost for a colorless spell you cast from your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { player == asker.controller && (q.chars(spell).colors.is_empty() && (cantrip_mechanics::zone_cast_from(q, spell) == Some(ZoneKind::Hand) && Some(asker.controller).into_iter().collect::<Vec<PlayerId>>().contains(&q.state().obj(spell).owner))) && !cantrip_mechanics::history::permission_used_this_turn(q, asker.source) }

pub static CARD: CardDef = CardDef { name: "Darksteel Monolith", token_words: &[], oracle: true, cost: ManaCost { generic: 8, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_alternatives: &[SpellAlternative { text: "Pay {0}", applies: ability_1_0_rule, cost: InsteadOfManaCost::Mana(ManaCost::ZERO), flash: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
