// Darksteel Monolith
// status: planned
// requires: alternate spell cost rule
// Indestructible
// Once each turn, you may pay {0} rather than pay the mana cost for a colorless spell you cast from your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("alternate-spell-cost-rule" => statics: player_rule(ability_1_0_applies, PlayerRule::SpellCost { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| (q.chars(spell).colors.is_empty() && cast_of(q, spell).is_some_and(|cast| cast.from == ZoneKind::Hand && Some(asker.controller).into_iter().collect::<Vec<PlayerId>>().contains(&q.state().obj(cast.spell).owner))), change: CostModifier::Alternate { cost: PlayCost::Alternate(&[ManaSymbol::ManaCostGeneric(0)]), limit: Some(PlayLimit::EachTurn(1)) } }));

pub static CARD: CardDef = CardDef { name: "Darksteel Monolith", token_words: &[], oracle: true, cost: ManaCost { generic: 8, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
