// Worldheart Phoenix
// status: planned
// requires: cast the card itself on these terms
// Flying
// You may cast this card from your graveyard by paying {W}{U}{B}{R}{G} rather than paying its mana cost. If you do, it enters with two +1/+1 counters on it.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("cast-the-card-itself-on-these-terms" => own_casts: OwnCast { zone: ZoneKind::Graveyard, condition: None, grant: PlayGrant { PlayGrant { alternate: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [1, 1, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), rider: Some(&[&[EnterFlag::EntersWithNumberCounters({ let q: &dyn Query = g; 2i64 }, CounterType::PTCounter(1, 1))]]), ..PlayGrant::CAST }..PlayGrant::PLAIN } });

pub static CARD: CardDef = CardDef { name: "Worldheart Phoenix", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phoenix]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
