// Averna, the Chaos Bloom
// status: planned
// requires: player rule CascadePut
// As you cascade, you may put a land card from among the exiled cards onto the battlefield tapped. (Do this after the last card is exiled but before deciding whether to cast a nonland card.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-cascade-put" => statics: player_rule(ability_0_0_applies, PlayerRule::CascadePut { cards: |q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Land), flags: &[EnterFlag::EntersTapped] }));

pub static CARD: CardDef = CardDef { name: "Averna, the Chaos Bloom", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
