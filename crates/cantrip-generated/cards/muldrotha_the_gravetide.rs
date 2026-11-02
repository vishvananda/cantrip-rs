// Muldrotha, the Gravetide
// status: planned
// requires: play permission of one card of each permanent type
// During each of your turns, you may play a land and cast a permanent spell of each permanent type from your graveyard. (If a card has multiple permanent types, choose one as you play it.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("play-permission-of-one-card-of-each-permanent-type" => #[allow(unused_variables)] fn ability_0_0_cost(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> Option<Cost> { (player == asker.controller && q.state().turn.active == player && !cantrip_mechanics::history::permanent_type_used_this_turn(q, asker.source, card)).then(|| Cost::mana(q.card(q.chars(card).card).cost)) });
planned_item!("play-permission-of-one-card-of-each-permanent-type" => granted_ways: GrantedWay { text: "During each of your turns, you may play a land and cast a permanent spell of each permanent type from your graveyard.", permission: true, lands: true, cost: ability_0_0_cost, ..GrantedWay::CAST });

pub static CARD: CardDef = CardDef { name: "Muldrotha, the Gravetide", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
