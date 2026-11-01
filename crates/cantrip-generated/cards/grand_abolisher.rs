// Grand Abolisher
// status: ready
// During your turn, your opponents can't cast spells or activate abilities of artifacts, creatures, or enchantments.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !((Some(asker.controller) == Some(q.state().turn.active)) && player != asker.controller && true) }
#[allow(unused_variables)] #[allow(unused_variables)] fn ability_0_0_1_rule(q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef) -> bool { let player = q.controller(object); !((Some(asker.controller) == Some(q.state().turn.active)) && player != asker.controller && q.is_on_battlefield(object) && (q.chars(object).is(CardType::Artifact) || q.chars(object).is(CardType::Creature) || q.chars(object).is(CardType::Enchantment))) }

pub static CARD: CardDef = CardDef { name: "Grand Abolisher", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, activation_rules: &[ability_0_0_1_rule], spell_rules: &[ability_0_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
