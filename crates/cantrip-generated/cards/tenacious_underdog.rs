// Tenacious Underdog
// status: planned
// requires: cast the card itself on these terms
// requires: engine: no Blitz
// Blitz—{2}{B}{B}, Pay 2 life. (If you cast this spell for its blitz cost, it gains haste and "When this creature dies, draw a card." Sacrifice it at the beginning of the next end step.)
// You may cast this card from your graveyard using its blitz ability.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-blitz" => ways: cantrip_mechanics::blitz::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(2)), "Blitz—{2}{B}{B}, Pay 2 life."));
planned_item!("cast-the-card-itself-on-these-terms" => own_casts: OwnCast { zone: ZoneKind::Graveyard, condition: None, grant: PlayGrant { using: Some(CostedKeyword::Blitz), ..PlayGrant::CAST } });

pub static CARD: CardDef = CardDef { name: "Tenacious Underdog", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
