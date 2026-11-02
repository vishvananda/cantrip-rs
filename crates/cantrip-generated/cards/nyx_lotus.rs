// Nyx Lotus
// status: ready
// Nyx Lotus enters tapped.
// {T}: Choose a color. Add an amount of mana of that color equal to your devotion to that color. (Your devotion to a color is the number of mana symbols of that color in the mana costs of permanents you control.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Some(chosen_1) = chosen else { return Ok(()) };
let mana_players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_3_0 = { let q: &dyn Query = g; (Some(asker.controller).zip(Some(chosen_1)).map_or(0i64, |(p, color): (PlayerId, Color)| i64::from(cantrip_mechanics::devotion::devotion(q, p, color)))) as u32 };
for player in mana_players_3 {
if let Some(color) = { let q: &dyn Query = g; Some(chosen_1) } { g.add_mana(player, color.mana(), mana_3_0); }
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Nyx Lotus", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_any_color(1).by(ability_1_mana), "{T}: Choose a color. Add an amount of mana of that color equal to your devotion to that color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
