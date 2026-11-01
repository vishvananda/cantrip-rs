// Brisela, Voice of Nightmares
// status: planned
// requires: layout Melded
// Flying, first strike, vigilance, lifelink
// Your opponents can't cast spells with mana value 3 or less.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_4_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player != asker.controller && (i64::from(q.mana_value(spell))) <= (3i64)) }
planned_item!("layout-melded" => melded: true);

pub static CARD: CardDef = CardDef { name: "Brisela, Voice of Nightmares", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi,Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(9), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::FirstStrike,Keyword::Vigilance,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_4_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
