// Fires of Invention
// status: ready
// You can cast spells only during your turn and you can cast no more than two spells each turn.
// You may cast spells with mana value less than or equal to the number of lands you control without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player == asker.controller && true) || q.state().turn.active == player }
#[allow(unused_variables)] fn ability_0_1_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player == asker.controller && true && q.history().casts_by(player).filter(|c| c.spell != spell && true).count() >= 2) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { player == asker.controller && (i64::from(q.mana_value(spell))) <= ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) }

pub static CARD: CardDef = CardDef { name: "Fires of Invention", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_alternatives: &[SpellAlternative { text: "Without paying its mana cost", applies: ability_1_0_rule, cost: InsteadOfManaCost::WithoutPaying, flash: false }], spell_rules: &[ability_0_0_rule,ability_0_1_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
