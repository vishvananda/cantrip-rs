// Timeline Culler
// status: planned
// requires: Warp cost other than mana
// requires: cast the card itself on these terms
// Haste
// You may cast this card from your graveyard using its warp ability.
// Warp—{B}, Pay 2 life. (You may cast this card from your hand or graveyard for its warp cost. If you do, exile this creature at the beginning of the next end step, then you may cast it from exile on a later turn.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("cast-the-card-itself-on-these-terms" => own_casts: OwnCast { zone: ZoneKind::Graveyard, condition: None, grant: PlayGrant { PlayGrant { using: Some(CostedKeyword::Warp), ..PlayGrant::CAST }..PlayGrant::PLAIN } });
planned_item!("warp-cost-other-than-mana" => ways: cantrip_mechanics::warp::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(2)), "Warp"));

pub static CARD: CardDef = CardDef { name: "Timeline Culler", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Drix,Subtype::Warlock]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::warp::EXILE], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
