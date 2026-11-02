// Archon of Valor's Reach
// status: ready
// Flying, vigilance, trample
// As this creature enters, choose artifact, enchantment, instant, sorcery, or planeswalker.
// Players can't cast spells of the chosen type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_3_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..5)?; let chosen_0: CardType = [CardType::Artifact, CardType::Enchantment, CardType::Instant, CardType::Sorcery, CardType::Planeswalker][usize::from(entering.chosen_option.map_or(0, |chosen| chosen.index))];
 Ok(()) }
#[allow(unused_variables)] fn ability_4_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && q.state().chosen_option(q.linked_to(asker)).filter(|c| std::ptr::eq(q.card(c.card), &CARD)).map(|c| [CardType::Artifact, CardType::Enchantment, CardType::Instant, CardType::Sorcery, CardType::Planeswalker][usize::from(c.index)]).is_some_and(|chosen| q.chars(spell).is(chosen))) }

pub static CARD: CardDef = CardDef { name: "Archon of Valor's Reach", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Archon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_3_enters), modal: None, enchant: None, spell_rules: &[ability_4_0_rule], labels: &["Artifact", "Enchantment", "Instant", "Sorcery", "Planeswalker"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
