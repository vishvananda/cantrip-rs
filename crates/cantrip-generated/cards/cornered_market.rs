// Cornered Market
// status: ready
// Players can't cast spells with the same name as a nontoken permanent.
// Players can't play nonbasic lands with the same name as a nontoken permanent.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && (q.state().battlefield().iter().copied().filter(|&id| !(q.state().obj(id).token)).collect::<Vec<ObjectId>>()).iter().any(|&o| cantrip_mechanics::same_name(q, spell, o))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, land: ObjectId, face: CardId) -> bool { !(true && (!(q.chars(land).supertypes.basic) && q.chars(land).is(CardType::Land) && (q.state().battlefield().iter().copied().filter(|&id| !(q.state().obj(id).token)).collect::<Vec<ObjectId>>()).iter().any(|&o| cantrip_mechanics::same_name(q, land, o)))) }

pub static CARD: CardDef = CardDef { name: "Cornered Market", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, land_play_rules: &[ability_1_0_rule], spell_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
