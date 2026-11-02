// Marang River Prowler
// status: planned
// requires: engine: a card's own terms to cast it from its graveyard
// This creature can't block and can't be blocked.
// You may cast this card from your graveyard as long as you control a black or green permanent.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.controller).unwrap_or(PlayerId(u8::MAX)) && (q.chars(id).colors.has(q.words(asker.source).color(Color::Black)) || q.chars(id).colors.has(q.words(asker.source).color(Color::Green)))).count() >= 1 }
planned_item!("engine-a-card-s-own-terms-to-cast-it-from-its-graveyard" => ways: way(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Graveyard, "Cast from your graveyard").only_from().condition(ability_1_if));

pub static CARD: CardDef = CardDef { name: "Marang River Prowler", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBlock,Keyword::CantBeBlocked]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
