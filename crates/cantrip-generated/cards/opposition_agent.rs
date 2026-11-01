// Opposition Agent
// status: planned
// requires: ability ExilesSearchedCards
// requires: player rule Decides
// Flash
// You control your opponents while they're searching their libraries.
// While an opponent is searching their library, they exile each card they find. You may play those cards for as long as they remain exiled, and you may spend mana as though it were mana of any color to cast them.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-decides" => statics: player_rule(ability_1_0_applies, PlayerRule::Decides(Decision::SearchesOf(|q: &dyn Query, asker: &Asker, player: PlayerId| player != asker.controller))));
planned_item!("ability-exiles-searched-cards" => statics: exiles_searched_cards(|q: &dyn Query, asker: &Asker, player: PlayerId| player != asker.controller, |q: &dyn Query, asker: &Asker| Some(asker.controller)));

pub static CARD: CardDef = CardDef { name: "Opposition Agent", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
