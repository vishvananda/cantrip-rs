// Sanctum Prelate
// status: ready
// As this creature enters, choose a number.
// Noncreature spells with mana value equal to the chosen number can't be cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let least_0 = { let q: &dyn Query = g; (0i64).max(0) as u32 } + 1; let most_0 = i32::MAX as u32; cantrip_mechanics::choose::number_as_enters(g, entering, least_0, most_0, "Choose a number")?; let chosen_0: u32 = entering.chosen_number.unwrap_or(0);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && (!(q.chars(spell).is(CardType::Creature)) && (i64::from(q.mana_value(spell))) == (q.state().chosen_number(q.linked_to(asker)).map_or(0i64, i64::from)))) }

pub static CARD: CardDef = CardDef { name: "Sanctum Prelate", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, spell_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
