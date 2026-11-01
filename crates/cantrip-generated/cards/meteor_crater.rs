// Meteor Crater
// status: planned
// requires: a choice of Color outside its body
// requires: color choice ColorAmoungPermanents
// {T}: Choose a color of a permanent you control. Add one mana of that color.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { planned!("color-choice-color-amoung-permanents" => cantrip_mechanics::choose::value(g, asker.source, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, ChoiceKind::Color, 1, &[ChoosableColor::ColorAmoungPermanents({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>() })])?);
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
if let Some(color) = { let q: &dyn Query = g; planned_value!("a choice of Color outside its body", None::<Color> => q.state().chosen_color(q.linked_to(asker))) } { g.add_mana(player, color.mana(), mana_2_0); }
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Meteor Crater", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana), "{T}: Choose a color of a permanent you control. Add one mana of that color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
