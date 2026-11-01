// Archon of Valor's Reach
// status: planned
// requires: a choice of CardType
// requires: a choice of CardType kept on a permanent
// Flying, vigilance, trample
// As this creature enters, choose artifact, enchantment, instant, sorcery, or planeswalker.
// Players can't cast spells of the chosen type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_3_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_0: CardType = planned_value!("a choice of CardType", CardType::Creature => cantrip_mechanics::choose::card_type(g, entering.controller, entering.object, vec![CardType::Artifact, CardType::Enchantment, CardType::Instant, CardType::Sorcery, CardType::Planeswalker])?);
planned!("a-choice-of-card-type" => entering.chosen_card_type = Some(chosen_0););
 Ok(()) }
#[allow(unused_variables)] fn ability_4_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && planned_value!("a choice of CardType kept on a permanent", None::<CardType> => q.state().chosen_card_type(q.linked_to(asker))).is_some_and(|chosen| q.chars(spell).is(chosen))) }

pub static CARD: CardDef = CardDef { name: "Archon of Valor's Reach", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Archon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_3_enters), modal: None, enchant: None, spell_rules: &[ability_4_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
