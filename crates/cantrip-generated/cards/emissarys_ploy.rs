// Emissary's Ploy
// status: planned
// requires: ability BeforeGame
// requires: card type Conspiracy
// requires: layout Conspiracy
// (Start the game with this conspiracy face up in the command zone.)
// Before drawing your opening hand, choose 1, 2, or 3.
// You may spend mana as though it were mana of any color to cast creature spells with mana value equal to the chosen number.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, s: &Spending) -> bool { let player = s.player; player == asker.controller && s.purpose == SpendingFor::Casting && s.source.is_some_and(|spell| (q.chars(spell).is(CardType::Creature) && (i64::from(q.mana_value(spell))) == (q.state().chosen_number(q.linked_to(asker)).map_or(0i64, i64::from)))) }
planned_item!("ability-before-game" => before_game: (BeforeGameMoment::OpeningHand, &[|g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let least_0 = 1u32; let most_0 = 3u32; let chosen_0: u32 = cantrip_mechanics::choose::number(g, player_0, asker.source, least_0, most_0, "Choose a number")?;
g.end_sentence();
 Ok(()) }]));
planned_item!("layout-conspiracy" => variant: Some(Variant::ConspiracyDraft));

pub static CARD: CardDef = CardDef { name: "Emissary's Ploy", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Conspiracy", CardType::Land => CardType::Conspiracy)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spend_any_color: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
