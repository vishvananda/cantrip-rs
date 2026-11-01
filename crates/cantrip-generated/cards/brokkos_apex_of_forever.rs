// Brokkos, Apex of Forever
// status: planned
// requires: cast the card itself on these terms
// requires: engine: no Mutate
// Mutate {2}{U/B}{G}{G} (If you cast this spell for its mutate cost, put it over or under target non-Human creature you own. They mutate into the creature on top plus all abilities from under it.)
// Trample
// You may cast this card from your graveyard using its mutate ability.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-mutate" => ways: cantrip_mechanics::mutate::way(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [Some((Mana::U, Mana::B)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Mutate {2}{U/B}{G}{G}"));
planned_item!("cast-the-card-itself-on-these-terms" => own_casts: OwnCast { zone: ZoneKind::Graveyard, condition: None, grant: PlayGrant { PlayGrant { using: Some(CostedKeyword::Mutate), ..PlayGrant::CAST }..PlayGrant::PLAIN } });

pub static CARD: CardDef = CardDef { name: "Brokkos, Apex of Forever", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Nightmare,Subtype::Beast,Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
