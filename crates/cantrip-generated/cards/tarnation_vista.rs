// Tarnation Vista
// status: ready
// This land enters tapped. As it enters, choose a color.
// {T}: Add one mana of the chosen color.
// {1}, {T}: For each color among monocolored permanents you control, add one mana of that color.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
let player_1 = entering.controller; let Answer::Colors(colors_1) = g.ask(player_1, RequestKind::ChooseColors { source: entering.object, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_1 = colors_1[0];
entering.chosen_color = Some(chosen_1);
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_2);}
 Ok(()) }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, _: Option<Color>) -> Result<(), Suspend> { let colors = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).colors.iter().count() as i64) == (1i64) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().fold(ColorSet::NONE, |all: ColorSet, o: ObjectId| ColorSet(all.0 | q.chars(o).colors.0)) }; for color in colors.iter() { g.add_mana(asker.controller, color.mana(), 1); } Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tarnation Vista", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana).choose_color().colors(ColorSet(31)).yields_any(1), "{T}: Add one mana of the chosen color.").colors_from(cantrip_mechanics::lands::chosen_color),mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_2_mana), "{1}, {T}: For each color among monocolored permanents you control, add one mana of that color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
