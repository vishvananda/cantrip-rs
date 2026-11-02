// Meteor Crater
// status: ready
// {T}: Choose a color of a permanent you control. Add one mana of that color.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mana_colors(q: &dyn Query, asker: &Asker) -> ColorSet { (q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).into_iter().fold(ColorSet::NONE, |all: ColorSet, o: ObjectId| ColorSet(all.0 | q.chars(o).colors.0)) }
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(), Suspend> { if let Some(color) = chosen { g.add_mana(asker.controller, color.mana(), 1); } Ok(()) }

pub static CARD: CardDef = CardDef { name: "Meteor Crater", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().yields_any(1), "{T}: Choose a color of a permanent you control. Add one mana of that color.").colors_from(ability_0_mana_colors)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
