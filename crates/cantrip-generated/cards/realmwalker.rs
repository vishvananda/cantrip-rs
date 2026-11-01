// Realmwalker
// status: ready
// Changeling (This card is every creature type.)
// As this creature enters, choose a creature type.
// You may look at the top card of your library any time.
// You may cast creature spells of the chosen type from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let Some(chosen_0) = cantrip_mechanics::chosen_type::choose_creature_type(g, entering)? else { return Ok(()) };
 Ok(()) }
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { (q.chars(spell).is(CardType::Creature) && q.state().chosen_subtype(q.linked_to(asker)).is_some_and(|chosen| q.chars(spell).has_subtype(chosen))) }

pub static CARD: CardDef = CardDef { name: "Realmwalker", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Changeling]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_3_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
