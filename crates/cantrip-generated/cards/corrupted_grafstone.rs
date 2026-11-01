// Corrupted Grafstone
// status: planned
// requires: a choice of Color outside its body
// requires: color choice ColorsOfCardsInPlayersGraveyard
// This artifact enters tapped.
// {T}: Choose a color of a card in your graveyard. Add one mana of that color.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { planned!("color-choice-colors-of-cards-in-players-graveyard" => cantrip_mechanics::choose::value(g, asker.source, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, ChoiceKind::Color, 1, &[ChoosableColor::ColorsOfCardsInPlayersGraveyard(|q: &dyn Query, asker: &Asker, id: ObjectId| true, { let q: &dyn Query = g; Some(asker.controller) })])?);
let mana_players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_3_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_3 {
if let Some(color) = { let q: &dyn Query = g; planned_value!("a choice of Color outside its body", None::<Color> => q.state().chosen_color(q.linked_to(asker))) } { g.add_mana(player, color.mana(), mana_3_0); }
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Corrupted Grafstone", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana), "{T}: Choose a color of a card in your graveyard. Add one mana of that color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
